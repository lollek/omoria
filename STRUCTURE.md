# Structure

This document describes the target architecture for the Rust port of omoria.
The migration is incremental — not all modules exist yet, and some C code remains.

## Design Principles

### 1. Vertical Slicing by Game Domain

Code is organized by **game feature / domain** rather than technical layer.
Each module contains its own types, logic, and tests together.

**Why?**
- To understand "how inventory works", you look in one place: `inventory/`
- Makes modules easier to test in isolation
- Reduces coupling between unrelated subsystems

### 2. Testable Core, Thin UI Shell

```
┌─────────────────────────────────────────────────┐
│  main.c / main.rs  (wiring, initialization)     │
├─────────────────────────────────────────────────┤
│  UI layer (ncurses, terminal I/O)               │
│  Hard to test — keep this thin                  │
├─────────────────────────────────────────────────┤
│  Game logic modules (pure functions)            │
│  Easy to test with injected RNG                 │
├─────────────────────────────────────────────────┤
│  Model / shared types                           │
└─────────────────────────────────────────────────┘
```

The goal: most game logic should be pure functions that take inputs and return
outputs, with no direct terminal I/O or global state. This makes them trivially
testable.

### 3. Dependency Direction

```
UI → Game Logic → Model
         ↓
       RNG (injected)
```

- **Model** depends on nothing (or only other model types)
- **Game logic** depends on model, and accepts RNG as a parameter
- **UI** depends on game logic and model, handles I/O
- **main** wires everything together

## Module Overview

### Core Modules

| Module | Purpose |
|--------|---------|
| `model/` | Shared foundational types used across 3+ domains |
| `rng/` | Random number generation with injectable RNG pattern |
| `game/` | Game orchestration: turn processing, command dispatch |

### Game Domain Modules

| Module | Purpose |
|--------|---------|
| `combat/` | Melee, ranged attacks, damage calculation |
| `magic/` | Spells, mana, casting (arcane, divine, nature, song) |
| `inventory/` | Items, equipment, encumbrance, consumables |
| `dungeon/` | Map generation, cave structure, lighting |
| `creature/` | Monster AI, spawning, templates |
| `town/` | Stores, NPCs, town generation |
| `casino/` | Mini-games (blackjack, slots, horse racing) |
| `player/` | Player-specific: stats, level-up, hunger, regeneration |

### Infrastructure Modules

| Module | Purpose |
|--------|---------|
| `save/` | Save/load serialization |
| `persistence/` | File I/O abstraction |
| `ui/` | Terminal rendering, messages, menus |
| `init/` | One-time initialization |
| `pregame/` | Character creation, main menu |

## Target Directory Structure

```
src/
├── lib.rs                    # Rust library root
├── main.c                    # C entry point (until fully ported)
│
├── model/                    # ═══ Shared foundational types ═══
│   ├── mod.rs
│   ├── player.rs             # Player struct, PlayerFlags
│   ├── monster.rs            # Monster struct
│   ├── item.rs               # Item struct (base)
│   ├── stat.rs               # Stats, StatBlock
│   ├── class.rs              # Character classes
│   ├── race.rs               # Races
│   ├── time.rs               # GameTime
│   └── currency.rs           # Wallet, gold
│
├── combat/                   # ═══ Combat system ═══
│   ├── mod.rs
│   ├── melee.rs              # Melee attack logic
│   ├── ranged.rs             # Ranged attack logic
│   ├── damage.rs             # Damage calculation
│   └── interop.rs            # C FFI bridge (fighting.c)
│
├── magic/                    # ═══ Magic system ═══
│   ├── mod.rs
│   ├── spell.rs              # Spell definitions
│   ├── casting.rs            # Casting logic, mana costs
│   ├── arcane.rs             # Arcane school
│   ├── divine.rs             # Divine school
│   ├── nature.rs             # Nature school
│   └── song.rs               # Bard songs
│
├── inventory/                # ═══ Items & equipment ═══
│   ├── mod.rs
│   ├── item_type.rs          # Item categories, subtypes
│   ├── equipment.rs          # Equip slots, equip/unequip
│   ├── use_item.rs           # Consumable logic
│   └── display.rs            # Inventory formatting
│
├── dungeon/                  # ═══ Dungeon & maps ═══
│   ├── mod.rs
│   ├── generate.rs           # Map generation algorithms
│   ├── rooms.rs              # Room placement
│   ├── cave.rs               # Cave data structure
│   ├── light.rs              # Lighting calculations
│   └── river.rs              # River generation
│
├── creature/                 # ═══ Monsters ═══
│   ├── mod.rs
│   ├── spawn.rs              # Monster placement
│   ├── ai.rs                 # Monster behavior
│   └── template.rs           # Monster templates
│
├── town/                     # ═══ Town & stores ═══
│   ├── mod.rs
│   ├── store.rs              # Store inventory, pricing
│   ├── generate.rs           # Town map generation
│   └── blackmarket.rs        # Black market logic
│
├── casino/                   # ═══ Mini-games ═══
│   ├── mod.rs
│   ├── blackjack.rs
│   ├── horseracing.rs
│   └── slotmachine.rs
│
├── player/                   # ═══ Player-specific ═══
│   ├── mod.rs
│   ├── attributes.rs         # Stat calculations
│   ├── level_up.rs           # Level progression
│   ├── regeneration.rs       # HP/mana regen
│   ├── hunger.rs             # Food system
│   └── skills.rs             # Skill system
│
├── save/                     # ═══ Persistence ═══
│   ├── mod.rs
│   ├── serialize.rs          # Save format
│   ├── load.rs               # Load logic
│   └── migration.rs          # Save version upgrades
│
├── ui/                       # ═══ User interface ═══
│   ├── mod.rs
│   ├── terminal.rs           # ncurses wrapper
│   ├── messages.rs           # Message log
│   ├── menu.rs               # Menu rendering
│   └── help.rs               # Help screens
│
├── game/                     # ═══ Orchestration ═══
│   ├── mod.rs
│   ├── turn.rs               # Turn processing
│   ├── commands.rs           # Player command dispatch
│   └── state.rs              # Game state machine
│
├── rng/                      # ═══ Randomness ═══
│   ├── mod.rs
│   └── random.rs             # RNG wrappers (_with_rng pattern)
│
├── init/                     # ═══ Initialization ═══
│   └── ...                   # One-time setup
│
├── pregame/                  # ═══ Pre-game ═══
│   ├── mod.rs
│   ├── menu.rs               # Main menu
│   └── create_character/     # Character creation flow
│
└── interop/                  # ═══ C FFI (optional consolidation) ═══
    └── *_extern.rs           # All C interop in one place
```

## Module Dependency Rules

### `model/` — Shared Types
- **Can use:** Other types from `model/`
- **Cannot use:** Any logic modules
- **Tests:** Unit tests for type invariants, serialization

### `combat/`, `magic/`, `inventory/`, etc. — Game Logic
- **Can use:** `model/`, `rng/`, other logic modules (with care)
- **Cannot use:** `ui/`, direct terminal I/O
- **Tests:** Unit tests with seeded RNG, no I/O mocking needed

### `ui/` — User Interface
- **Can use:** Any module
- **Cannot use:** —
- **Tests:** Minimal; test formatting logic only, not ncurses calls

### `game/` — Orchestration
- **Can use:** All game logic modules, `ui/`, `save/`
- **Tests:** Integration tests for turn sequences

## Migration Strategy

Since we're mid-migration from C → Rust:

1. **Don't reorganize everything at once** — keep diffs reviewable
2. **When porting a C file**, place the Rust code in the target structure
3. **Move existing Rust files incrementally** when touching them anyway
4. **Keep `*_extern.rs` files** as C interop bridges during transition
5. **Legacy C files stay in `src/`** until ported; new Rust goes in modules

## RNG Pattern (Deterministic Testing)

All game logic that uses randomness should follow this pattern:

```rust
// Injectable version (for testing)
fn roll_damage_with_rng(rng: &mut impl Rng, weapon: &Weapon) -> u32 {
    rng.gen_range(weapon.min_damage..=weapon.max_damage)
}

// Convenience wrapper (for production)
fn roll_damage(weapon: &Weapon) -> u32 {
    roll_damage_with_rng(&mut rand::thread_rng(), weapon)
}
```

Tests use a seeded RNG for deterministic results:
```rust
#[test]
fn test_damage_is_in_range() {
    let mut rng = StdRng::seed_from_u64(42);
    let weapon = Weapon { min_damage: 1, max_damage: 6 };
    let damage = roll_damage_with_rng(&mut rng, &weapon);
    assert!(damage >= 1 && damage <= 6);
}
```

## Current Status

This structure is the **target state**. Current code is a mix of:
- Legacy C files in `src/*.c`
- Partially migrated Rust modules
- Some modules already following the new structure

See `CHANGELOG.md` for migration progress.
