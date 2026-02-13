# Monster Template Migration Plan

This document details the migration of `src/generate_monster/monster_template.c` (561 lines) to Rust.

## Overview

`monster_template.c` contains:

1. **Monster template data** (~450 lines): A large static array of ~300 monster definitions
2. **Attribute query functions** (~90 lines): `monster_template_has_attribute()` and `monster_template_has_attributes()`
3. **Level index array** (`m_level`): Used for monster generation by level

## Current C Structure

### Data Types (`monster_template.h`)

```c
typedef struct monster_attributes {
  bool multiplies;
  bool can_move;
} monster_attributes;

typedef struct monster_template {
  uint8_t area_effect_radius;
  uint8_t ac;
  char name[28];
  uint64_t cmove;                   // Bit field for movement/behavior
  uint64_t spells;                  // Creature spells
  uint64_t cdefense;                // Bit field for defenses/vulnerabilities
  int16_t sleep;                    // Inactive counter when spawned
  int64_t mexp;                     // Exp value for kill
  int8_t speed;                     // Movement speed
  char symbol;                      // Symbol on map
  char hit_die[7];                  // e.g. "20d8"
  char damage[36];                  // e.g. "1 1 3d3|1 1 4d4"
  int8_t level;                     // Minimum level creature is found at
  uint8_t magic_resistance;
  monster_attributes attributes;
} monster_template_t;
```

### Enum (`monster_attribute`)

50+ attributes covering:
- Movement flags (random movement %, water/land, flying, etc.)
- Behavior (multiplies, picks up objects, carries gold, etc.)
- Defense/weakness (vulnerable to frost/fire/etc., undead, demon, etc.)

## Dependencies Analysis

### Used by (consumers of `monster_templates[]`)
- `misc.c` - Monster symbol lookup, naming
- `creature.c` - Monster movement, visibility, behavior flags
- `quest.c` - Quest rewards, monster selection
- `generate_monster/generate_monster.c` - Monster spawning

### Dependencies (what this module needs)
- Model types only (no runtime dependencies)
- Constants: `MAX_MONS_LEVEL` (100)

## Target Rust Structure

```
src/
├── generate_monster/
│   └── monster_template/              # New Rust module
│       ├── mod.rs                     # Public API
│       ├── data.rs                    # Static monster template array
│       ├── attribute.rs               # MonsterAttribute enum
│       ├── template.rs                # MonsterTemplate struct
│       ├── query.rs                   # has_attribute() logic
│       ├── interop.rs                 # C ABI wrappers
│       └── tests/
│           ├── mod.rs
│           ├── data_tests.rs          # Validate data matches C
│           └── attribute_tests.rs     # Test attribute queries
```

---

## Migration Phases

### Phase 1: Template Struct & Attribute Enum (RED)
**Complexity**: Low  
**Risk**: Low  
**Deliverable**: Rust types that mirror the C structures

#### 1.1 Port `monster_attribute` enum → `attribute.rs`

```rust
/// Monster attribute flags.
///
/// These map to bit fields in `cmove`, `cdefense`, and the `attributes` struct.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MonsterAttribute {
    MoveOnlyToAttack,
    RandomMovement20pc,
    RandomMovement40pc,
    RandomMovement75pc,
    WaterBased,
    LandBased,
    DiesInWrongElement,
    SurvivesInWater,
    SurvivesOnLand,
    GoodMonster,
    Unspawnable,
    InvisibleMovement,
    MovesThroughDoor,
    MovesThroughWall,
    MovesThroughCreatures,
    PicksUpObjects,
    Multiplies,
    AnchorsInWater,
    Flying,
    CarriesObjects,
    CarriesGold,
    Carries60pc,
    Carries90pc,
    Carries1d2Things,
    Carries2d2Things,
    Carries4d2Things,
    WinsTheGame,
    Dragon,
    Monster,
    Evil,
    Undead,
    Demon,
    VulnerableToFrost,
    VulnerableToFire,
    VulnerableToPoison,
    VulnerableToAcid,
    VulnerableToLightning,
    VulnerableToStoneToMud,
    Uncharmable,
    VisibleWithInfravision,
    MaxHitPoints,
    Regenerates,
}
```

#### 1.2 Port `monster_template_t` → `template.rs`

```rust
/// Monster template (static definition).
///
/// Mirrors `monster_template_t` from C with Rust-friendly types.
#[derive(Debug, Clone)]
pub struct MonsterTemplate {
    pub area_effect_radius: u8,
    pub ac: u8,
    pub name: &'static str,
    pub cmove: u64,
    pub spells: u64,
    pub cdefense: u64,
    pub sleep: i16,
    pub mexp: i64,
    pub speed: i8,
    pub symbol: char,
    pub hit_die: &'static str,
    pub damage: &'static str,
    pub level: i8,
    pub magic_resistance: u8,
    pub multiplies: bool,
    pub can_move: bool,
}
```

**Testing (Phase 1)**:
- Test that struct can be instantiated
- Test field sizes match C (for FFI correctness)

---

### Phase 2: Attribute Query Logic (RED → GREEN)
**Complexity**: Medium  
**Risk**: Low-Medium  

#### 2.1 Port `monster_template_has_attribute()` → `query.rs`

The C function is a 100-line switch statement mapping attributes to bit masks.
Port this as a method on `MonsterTemplate`:

```rust
impl MonsterTemplate {
    /// Check if this monster has the given attribute.
    pub fn has_attribute(&self, attr: MonsterAttribute) -> bool {
        match attr {
            MonsterAttribute::MoveOnlyToAttack => !self.can_move,
            MonsterAttribute::RandomMovement20pc => (self.cmove & 0x00000002) != 0,
            MonsterAttribute::RandomMovement40pc => (self.cmove & 0x00000004) != 0,
            // ... etc
        }
    }
}
```

**Testing (Phase 2)**:
- Test each attribute against known monsters from the C data
- Test compound attributes (SurvivesInWater depends on WaterBased, DiesInWrongElement, Flying)

---

### Phase 3: Static Data Array (RED → GREEN)
**Complexity**: Medium (tedious, not complex)  
**Risk**: Low  

Port the 300+ monster definitions to `data.rs`:

```rust
pub const MONSTER_TEMPLATES: &[MonsterTemplate] = &[
    // Level 0
    MonsterTemplate {
        area_effect_radius: 10, ac: 1, name: "<<Placeholder>>",
        cmove: 0x0010C000, spells: 0x00009F52, cdefense: 0x3000,
        sleep: 25000, mexp: 50, speed: 1, symbol: 'p',
        hit_die: "20d8", damage: "1 1 3d3",
        level: 0, magic_resistance: 20,
        multiplies: false, can_move: true,
    },
    // ... ~300 more entries
];

pub const MONSTER_TEMPLATE_COUNT: usize = MONSTER_TEMPLATES.len();
```

**Testing (Phase 3)**:
- Validate count matches C (`monster_template_size`)
- Spot-check key monsters (Balrog, Town Guard, etc.)
- Verify level distribution

---

### Phase 4: C Interop Layer
**Complexity**: Medium  
**Risk**: Medium  

Create C ABI wrappers so the existing C code can call into Rust.

#### 4.1 Option A: Keep C array, add Rust query functions

Initially, keep `monster_templates[]` in C but expose Rust query functions:

```rust
// interop.rs

/// C ABI wrapper for has_attribute.
/// Returns true if the monster at index `mptr` has the given attribute.
#[no_mangle]
pub extern "C" fn rust_monster_has_attribute(mptr: libc::c_long, attr: libc::c_int) -> bool {
    // Convert C enum value to Rust enum
    let attr = MonsterAttribute::from_c(attr as u32);
    MONSTER_TEMPLATES.get(mptr as usize)
        .map(|t| t.has_attribute(attr))
        .unwrap_or(false)
}
```

#### 4.2 Option B: Full replacement with repr(C) struct

For full replacement, we'd need a `#[repr(C)]` struct that matches the C layout exactly.
This is more complex due to fixed-size char arrays.

```rust
#[repr(C)]
pub struct MonsterTemplateC {
    pub area_effect_radius: u8,
    pub ac: u8,
    pub name: [libc::c_char; 28],
    pub cmove: u64,
    // ... etc
}

#[no_mangle]
pub static MONSTER_TEMPLATES_C: &[MonsterTemplateC] = /* ... */;
```

**Recommendation**: Start with Option A (add Rust queries, keep C data), then migrate data in a later phase.

---

### Phase 5: Level Index (`m_level`)
**Complexity**: Low  
**Risk**: Low  

The `m_level` array is computed at startup to index monsters by level.
This can be a Rust function:

```rust
/// Build level index: m_level[L] = index of first monster at level >= L
pub fn build_level_index() -> [usize; MAX_MONS_LEVEL + 2] {
    let mut index = [0usize; MAX_MONS_LEVEL + 2];
    // ... port logic from C init code
    index
}
```

---

## Suggested Implementation Order

Following TDD (RED → GREEN → REFACTOR):

| Step | Description | Complexity | Files |
|------|-------------|------------|-------|
| 1 | Create module structure with stub types | Low | `mod.rs`, `attribute.rs`, `template.rs` |
| 2 | Write failing tests for attribute enum | Low | `tests/attribute_tests.rs` |
| 3 | Implement `MonsterAttribute` enum | Low | `attribute.rs` |
| 4 | Write failing tests for `has_attribute()` | Medium | `tests/attribute_tests.rs` |
| 5 | Implement `has_attribute()` | Medium | `query.rs` |
| 6 | Write failing tests for data correctness | Low | `tests/data_tests.rs` |
| 7 | Port static monster data | Tedious | `data.rs` |
| 8 | Add C interop wrappers | Medium | `interop.rs` |
| 9 | Delete C implementation | Low | Update `monster_template.c` |

---

## Risk Mitigation

### Data Accuracy
- Generate Rust data from C using a script (avoid manual transcription errors)
- Write tests that compare Rust values to C values via FFI

### Binary Compatibility
- If using `#[repr(C)]`, verify struct layout matches with `static_assertions`
- Test field offsets match C expectations

### Incremental Migration
- Phase 4 (interop) allows coexistence: C code can gradually switch to Rust functions
- Keep C array until all consumers are migrated

---

## Design Decisions (Resolved)

1. **Data generation**: **Manual port** — allows cleanup during migration, typo risk mitigated by tests comparing to C via FFI.

2. **Interop strategy**: **Option B (full replacement)** — go straight to `#[repr(C)]` struct so we can fully delete the C implementation.

3. **Scope**: **Keep `generate_monster.c` separate** — this migration covers only `monster_template.c`.

4. **Attribute representation**: **Keep bit fields** — use the same `cmove`/`cdefense` bit fields internally for 1:1 C compatibility, with a clean Rust `has_attribute()` API on top.
