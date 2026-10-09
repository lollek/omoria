# Traps Migration Plan

The [main migration plan](../c-to-rust-migration-plan.md) is the source of truth
for status. This document records the trap-specific boundaries verified against
[traps.c](../../src/traps.c), [traps.h](../../src/traps.h), and the current Rust
modules on 2026-10-09. This is a **partial port**, not completed trap activation.

## Current Status

| Concern | Current implementation | Status |
|---------|------------------------|--------|
| Templates and rubble | [data.rs](../../src/dungeon/trap/data.rs) | Ported |
| Placement into supplied tile/item lists | [placement.rs](../../src/dungeon/trap/placement.rs) | Ported |
| Global placement, visibility, rubble | [globals.rs](../../src/dungeon/trap/globals.rs) | Ported |
| C placement ABI | [interop.rs](../../src/dungeon/trap/interop.rs) | Ported |
| Effect handlers (`ht__*`) | [traps.c](../../src/traps.c) | Pending |
| Activation dispatcher (`hit_trap`) | [traps.c](../../src/traps.c) | Pending |
| Chest activation (`trigger_trap`) | [traps.c](../../src/traps.c) | Pending |
| Town/special-tile dispatch | Cases in C `hit_trap` | Pending |

```text
src/dungeon/trap/
├── mod.rs           # Existing public API and re-exports
├── data.rs          # TrapTemplate, TRAP_LIST, RUBBLE, constants
├── placement.rs     # TrapList and place_trap_into_lists
├── globals.rs       # Unsafe wrappers around dungeon globals
├── interop.rs       # place_trap, change_trap, place_rubble C ABI
├── data_tests.rs    # Template and placement tests
└── test_support.rs  # Shared test helpers
```

`effect.rs`, `chest.rs`, and a separate town entrance module are possible future
homes, not existing implementations. Their design must be reviewed before porting.

## Completed Data and Placement

The historical `trap_lista` / `trap_listb` data is represented by one
`TRAP_LIST` of 20 `TrapTemplate` entries plus standalone `RUBBLE`.
`TrapTemplate` stores `name`, `level`, `subval`, `damage`, and `cost`.
Placement sets the item fields and clears the fields that are constant for traps.

`TrapList::A` selects unseen traps and `TrapList::B` selects seen traps, except:

- Subval 1 (open pit) is always `TVAL_SEEN_TRAP`.
- Subval 19 (closed door) is always `TVAL_CLOSED_DOOR`.

Placement subvals are **1-based**: `template_for` uses `subval - 1` to index the
zero-based Rust slice. Valid template inputs are 1 through 20; zero and out-of-range
inputs are not validated by the current placement API. Activation values such as
99 and 101-123 are not indices into `TRAP_LIST`.

Existing [data_tests.rs](../../src/dungeon/trap/data_tests.rs) covers template
values, tval selection, item copying, and global placement/visibility behavior.
It does not establish parity for unported activation.

## ABI Contracts

Keep these declarations from [traps.h](../../src/traps.h) unchanged during an
activation port:

```c
void hit_trap(const long *y, const long *x);
void trigger_trap(long y, long x);
void place_trap(long y, long x, long typ, long subval);
void change_trap(long y, long x);
void place_rubble(long y, long x);
```

- `hit_trap` takes pointers to C `long`, not integer coordinates by value. A future
  Rust export must preserve `*const libc::c_long` parameters and explicitly handle
  pointer validity before dereferencing. It is still defined by C today.
- `trigger_trap` takes only two coordinates. It reads chest flags from
  `t_list[cave[y][x].tptr].flags`; there is no flags argument in its ABI.
- Existing Rust placement exports use `libc::c_long` for every argument, not
  `c_int`. `typ == 1` selects `TrapList::A`; other values select `TrapList::B`.
- The current wrappers cast coordinates/subvals to `usize` and mutate C global
  state through unsafe helpers. They do not validate negative coordinates or
  invalid template indices. Do not describe them as checked safe interfaces.
- Do not add duplicate Rust activation exports while C still defines those symbols.

## Pending Activation: Preserve C Dispatch

Before dispatch, C `hit_trap` stops active searching/resting, calls `change_trap`,
refreshes the player's tile, clears `find_flag`, and rolls damage from the item's
damage string. These side effects are part of the behavior to characterize.

### Dungeon and Special Traps

| Subval | Current C handler |
|--------|-------------------|
| 1 | `ht__open_pit` |
| 2 | `ht__arrow` |
| 3 | `ht__covered_pit` |
| 4 | `ht__trap_door` |
| 5 | `ht__sleep_gas` |
| 6 | `ht__hidden_object` |
| 7 | `ht__str_dart` |
| 8 | `ht__teleport` |
| 9 | `ht__rockfall` |
| 10 | `ht__corrode_gas` |
| 11 | `ht__summon_monster` |
| 12 | `ht__fire` |
| 13 | `ht__acid` |
| 14 | `ht__poison_gas` |
| 15 | `ht__blind_gas` |
| 16 | `ht__confuse_gas` |
| 17 | `ht__slow_dart` |
| 18 | `ht__con_dart` |
| 19 | `ht__secret_door` (empty handler; visibility handled earlier) |
| 20 | `ht__chute` |
| 99 | `ht__scare_monster` (empty handler for player activation) |
| 123 | `ht__whirlpool` |

For covered pits, C replaces the trap with `place_trap(y, x, 2, 1)` **only in the
non-feather-fall branch**, after damage. Do not move replacement outside that branch
or change the list argument while claiming behavior parity.

### Town and Special Tiles

| Subval | Current C behavior |
|--------|--------------------|
| 101 | General store |
| 102 | Armory |
| 103 | Weaponsmith |
| 104 | Temple |
| 105 | Alchemy shop |
| 106 | Magic shop |
| 107 | Inn |
| 108 | Check trading-post hours, then `enter_trading_post` |
| 109 | Library |
| 110 | Music shop |
| 111 | Insurance shop closed: message only |
| 112 | Check bank hours, then `enter_bank` |
| 113 | Gem shop |
| 114 | Check money-changer hours, then redirect-to-bank message |
| 115 | Check casino hours, then `enter_casino` |
| 116 | Deli |
| 117 | `enter_fortress` |
| 118 | Black market |
| 119 | No explicit case; falls through to unknown-value message |
| 120, 121, 122 | `enter_house(*y, *x)` |
| 123 | Whirlpool, not a store entrance |

Ordinary shops above use `check_store_hours_and_enter`; special entrances have
distinct paths. Do not replace all 101-123 values with one generic store operation.
Unknown values print `You got lucky: unknown trap value.`

### Chest Traps

C `trigger_trap` reads the flags once and checks each bit independently, in order:

| Flag | Current behavior |
|------|------------------|
| `0x10` | Strength-loss needle; damage if stat loss succeeds |
| `0x20` | Poison needle damage and poisoned duration |
| `0x40` | Paralysis gas, with free-action protection |
| `0x80` | Delete chest and apply explosion damage |
| `0x100` | Three summon attempts, selecting water/land by terrain |

These are separate `if` branches, not an exclusive match. Preserve ordering and
combined-flag behavior, including summons after an explosion.

## Remaining Migration Order and Tests

1. Characterize one C effect at a time with deterministic tests before porting.
2. Introduce injectable RNG and explicit state/message dependencies for that effect.
3. Characterize special-tile dispatch, including hours, closed insurance, houses,
   missing case 119, and whirlpool behavior before extracting town logic.
4. Port chest activation with combined flags, free action, terrain, and deletion tests.
5. Port the dispatcher only after its prerequisites and pre-dispatch side effects
   are covered; preserve the pointer-based C ABI and verify callers at the boundary.
6. Remove replaced C definitions only after Rust behavior and full linking are verified.

## Migration Checklist

### Phase 2.1: Data and Placement

- [x] Unified `TRAP_LIST`, `TrapTemplate`, and `RUBBLE`
- [x] `TrapList`, tval selection, and `place_trap_into_lists`
- [x] `place_trap_global`, `change_trap_global`, and `place_rubble_global`
- [x] C ABI placement wrappers using `c_long`
- [x] Data and placement tests

### Later: Activation and Town Dispatch

- [ ] Effect handlers, one behavior at a time
- [ ] Town/special-tile dispatch and hours behavior
- [ ] Chest activation and combined flags
- [ ] `hit_trap` dispatcher and pre-dispatch state changes
- [ ] Activation C ABI wrappers and integration tests
- [ ] Remove replaced C code and update build inputs as needed
- [ ] Full tests/link verification and changelog update for the actual port

Data/placement completion must not be used to mark these activation tasks complete.# Traps Migration Plan

This document details the migration of `src/traps.c` (590 lines) to Rust.

## Overview

`traps.c` contains several distinct concerns that should be **split into separate modules** during migration:

| Concern | C Location | Rust Target |
|---------|-----------|-------------|
| Trap templates (data) | `trap_lista`, `trap_listb`, `some_rubble` | `dungeon/trap/data.rs` |
| Trap placement | `place_trap`, `change_trap`, `place_rubble` | `dungeon/trap/placement.rs` |
| Trap effects | `ht__*` functions (20+ handlers) | `dungeon/trap/effect.rs` |
| Main dispatcher | `hit_trap` | `dungeon/trap/mod.rs` |
| Town entrances | Cases 101-123 in `hit_trap` | `town/entrance.rs` |
| Chest traps | `trigger_trap` | `dungeon/trap/chest.rs` |

## Target Rust Structure

```
src/
├── dungeon/
│   └── trap/                    # New module for traps
│       ├── mod.rs               # Public API: place_trap, hit_trap, trigger_trap
│       ├── data.rs              # Trap templates (trap_lista, trap_listb, rubble)
│       ├── effect.rs            # Trap effect handlers (ht__* functions)
│       ├── placement.rs         # place_trap, change_trap, place_rubble
│       └── chest.rs             # trigger_trap (chest-specific traps)
│
└── town/
    └── entrance.rs              # Store entrance logic (cases 101-123 from hit_trap)
```

## Dependencies Analysis

### External Dependencies (from includes)
```c
#include "casino/casino.h"       // enter_casino()
#include "constants.h"
#include "debug.h"
#include "effects.h"             // various effect functions
#include "fighting/fighting.h"   // managed_to_hit()
#include "generate_monster/generate_monster.h"
#include "io.h"                  // msg_print(), get_com()
#include "loot/loot.h"
#include "magic.h"
#include "misc.h"                // take_hit(), damroll()
#include "pascal.h"
#include "player.h"
#include "quest.h"
#include "random.h"              // randint()
#include "screen.h"
#include "spells.h"
#include "stores.h"              // store_maint(), store functions
#include "town_level/enter_bank.h"
#include "town_level/enter_house.h"
#include "trade.h"
#include "types.h"
#include "variables.h"           // globals: dun_level, cave, etc.
```

### Global Variables Used
- `dun_level` - Current dungeon level
- `cave[][]` - The dungeon map
- `char_row`, `char_col` - Player position
- `find_flag` - Search mode flag
- `t_list[]` - Treasure list
- `player_flags()` - Player status flags (ffall, blind, etc.)

## Migration Order

### Step 1: Trap Data (`data.rs`) — Phase 2.1 in main plan
**Complexity**: Low  
**Risk**: Low  
**Dependencies**: Model types only

Port the static trap definitions:
```rust
// dungeon/trap/data.rs

pub struct TrapTemplate {
    pub name: &'static str,
    pub tval: TreasureType,
    pub flags: u32,
    pub flags2: u32,
    pub p1: i16,
    pub cost: i16,
    pub subval: i16,
    pub weight: i16,
    pub number: i16,
    pub tohit: i16,
    pub todam: i16,
    pub ac: i16,
    pub toac: i16,
    pub damage: &'static str,
    pub level: i16,
    pub special: i16,
}

pub const TRAP_LIST_A: &[TrapTemplate] = &[
    TrapTemplate { name: "an open pit", tval: SeenTrap, /* ... */ },
    TrapTemplate { name: "an arrow trap", tval: UnseenTrap, /* ... */ },
    // ... all 19 traps
];

pub const TRAP_LIST_B: &[TrapTemplate] = &[
    // ... trap_listb entries
];

pub const RUBBLE: TrapTemplate = TrapTemplate {
    name: "some rubble",
    // ...
};
```

**Testing**: Validate counts and key values match C definitions.

### Step 2: Town Entrances (`town/entrance.rs`) — New, split from traps
**Complexity**: Medium  
**Risk**: Low-Medium  
**Dependencies**: stores, bank, casino, house modules

Extract town entrance logic from `hit_trap`:
```rust
// town/entrance.rs

pub fn enter_location(subval: i32) -> bool {
    match subval {
        101 => check_and_enter_store(StoreType::General),
        102 => check_and_enter_store(StoreType::Armory),
        103 => check_and_enter_store(StoreType::WeaponShop),
        104 => check_and_enter_store(StoreType::Temple),
        105 => check_and_enter_store(StoreType::Alchemist),
        106 => check_and_enter_store(StoreType::MagicShop),
        107 => check_and_enter_store(StoreType::Inn),
        108 => check_and_enter_store(StoreType::Tavern),
        109 => check_and_enter_store(StoreType::Library),
        110 => check_and_enter_store(StoreType::MusicShop),
        111 => check_and_enter_store(StoreType::Pawnshop),
        112 => check_and_enter_bank(),
        113 => check_and_enter_store(StoreType::BlackMarket),
        114 => enter_house(),
        115 => check_and_enter_casino(),
        116 => check_and_enter_store(StoreType::Jeweler),
        117 => check_and_enter_store(StoreType::Pet),
        118 => check_and_enter_store(StoreType::Junkyard),
        119 => check_and_enter_store(StoreType::Church),
        120 => check_and_enter_store(StoreType::Bookstore),
        121 => check_and_enter_store(StoreType::Toyshop),
        122..=123 => false, // Empty stores
        _ => false,
    }
}

fn check_and_enter_store(store_type: StoreType) -> bool {
    if check_store_hours(store_type) {
        store_maint();
        enter_store(store_type);
        true
    } else {
        false
    }
}
```

**Testing**: Unit tests for store hour checking, integration tests for store entry.

### Step 3: Trap Effects (`effect.rs`) — Phase 9 (part of traps.c)
**Complexity**: Medium-High  
**Risk**: Medium  
**Dependencies**: player, misc (take_hit), effects, io, random

Port each `ht__*` function as a separate handler:

```rust
// dungeon/trap/effect.rs

use crate::rng;

/// Open pit trap - player falls into a pit
pub fn open_pit(dam: i32) {
    msg_print("You fell into a pit!");
    if player_flags().ffall {
        msg_print("You gently float down.");
    } else {
        take_hit(dam, "an open pit");
    }
}

/// Arrow trap - arrow shoots at player
pub fn arrow(dam: i32) {
    if managed_to_hit(125, 0, 0, player_ac()) {
        take_hit(dam, "an arrow trap");
        msg_print("An arrow hits you.");
    } else {
        msg_print("An arrow barely misses you.");
    }
}

/// Covered pit - player falls through concealed floor
pub fn covered_pit(dam: i32, y: i32, x: i32) {
    msg_print("You fell into a covered pit.");
    if player_flags().ffall {
        msg_print("You gently float down.");
    } else {
        take_hit(dam, "a covered pit");
    }
    place_trap(y, x, TrapListType::A, 1); // Replace with open pit
}

// ... continue for all ht__* functions:
// - ht__trap_door
// - ht__gas
// - ht__loose_rock
// - ht__dart
// - ht__strange_rune
// - ht__gas_chest (9-12)
// - ht__mist_hazard (13-14)
// - ht__flash_ball
// - ht__acid_ball
// - ht__fire_ball
// - ht__poison_gas
// - ht__blindness_ball
// - ht__confusion_ball
// - ht__scare_monster
// - ht__create_monster
```

**Testing**: 
- Unit test each effect in isolation
- Mock `msg_print`, `take_hit`, `player_flags`
- Use seeded RNG for deterministic tests

### Step 4: Trap Placement (`placement.rs`)
**Complexity**: Low-Medium  
**Risk**: Low  
**Dependencies**: cave data, trap templates

```rust
// dungeon/trap/placement.rs

use super::data::{TRAP_LIST_A, TRAP_LIST_B, RUBBLE};

pub fn place_trap(y: i32, x: i32, list_type: TrapListType, subval: i32) {
    let template = match list_type {
        TrapListType::A => &TRAP_LIST_A[subval as usize],
        TrapListType::B => &TRAP_LIST_B[subval as usize],
    };
    // Create treasure from template and place at (y, x)
    // ...
}

pub fn change_trap(y: i32, x: i32) {
    // Make hidden trap visible
    // ...
}

pub fn place_rubble(y: i32, x: i32) {
    // Place rubble obstacle
    // ...
}
```

### Step 5: Chest Traps (`chest.rs`)
**Complexity**: Medium  
**Risk**: Medium  
**Dependencies**: effects, player, rng

```rust
// dungeon/trap/chest.rs

pub fn trigger_trap(flags: u32, y: i32, x: i32) {
    // Handle chest trap flags:
    // - CH_LOSE_STR
    // - CH_POISON
    // - CH_PARALYSIS
    // - CH_SUMMON
    // - CH_EXPLODE
    // ...
}
```

### Step 6: Main Dispatcher (`mod.rs`)
**Complexity**: Medium  
**Risk**: Medium  
**Dependencies**: All of the above

```rust
// dungeon/trap/mod.rs

mod data;
mod effect;
mod placement;
mod chest;

pub use data::*;
pub use placement::*;
pub use chest::trigger_trap;

pub fn hit_trap(y: i32, x: i32) {
    placement::change_trap(y, x);
    lite_spot(char_row(), char_col());
    set_find_flag(false);

    let trap = get_trap_at(y, x);
    let dam = damroll(&trap.damage);

    match trap.subval {
        // Dungeon traps (1-20)
        1 => effect::open_pit(dam),
        2 => effect::arrow(dam),
        3 => effect::covered_pit(dam, y, x),
        4 => effect::trap_door(dam),
        5 => effect::gas(trap.p1),
        6 => effect::loose_rock(),
        7 => effect::dart(dam),
        8 => effect::strange_rune(trap.p1),
        9..=12 => effect::gas_chest(trap.subval, dam),
        13..=14 => effect::mist_hazard(trap.subval),
        15 => effect::flash_ball(),
        16 => effect::acid_ball(dam),
        17 => effect::fire_ball(dam),
        18 => effect::poison_gas(dam),
        19 => effect::blindness_ball(),
        20 => effect::confusion_ball(),
        
        // Special traps (99)
        99 => { /* scare_monster - nothing for player */ }
        
        // Town entrances (101-123)
        101..=123 => {
            crate::town::entrance::enter_location(trap.subval);
        }
        
        _ => msg_print("You got lucky: unknown trap value."),
    }
}
```

## Testing Strategy

### Unit Tests (per module)

#### `data.rs` tests
```rust
#[test]
fn trap_list_a_has_correct_count() {
    assert_eq!(TRAP_LIST_A.len(), 19); // MAX_TRAPA
}

#[test]
fn open_pit_has_correct_damage() {
    assert_eq!(TRAP_LIST_A[1].damage, "2d6");
}
```

#### `effect.rs` tests
```rust
#[test]
fn open_pit_with_ffall_does_not_damage() {
    // Setup: player has feather fall
    // Act: trigger open pit
    // Assert: no damage taken, float message shown
}

#[test]
fn arrow_trap_can_miss() {
    // Use seeded RNG to force miss
    // Assert: miss message, no damage
}

#[test]
fn covered_pit_replaces_with_open_pit() {
    // Assert: trap at location changes to subval=1
}
```

#### `town/entrance.rs` tests
```rust
#[test]
fn closed_store_shows_message_and_returns_false() {
    // Setup: set time to closed hours
    // Act: try to enter store
    // Assert: returns false, "closed" message
}
```

### Integration Tests
- Verify trap placement creates correct treasure entries
- Verify `hit_trap` dispatches to correct handler
- Verify C interop with `extern "C"` wrappers

## C Interop

During migration, maintain C compatibility:

```rust
// dungeon/trap/interop.rs

#[no_mangle]
pub extern "C" fn hit_trap(y: libc::c_int, x: libc::c_int) {
    super::hit_trap(y as i32, x as i32);
}

#[no_mangle]
pub extern "C" fn place_trap(y: libc::c_int, x: libc::c_int, typ: libc::c_int, subval: libc::c_int) {
    let list_type = if typ == 1 { TrapListType::A } else { TrapListType::B };
    super::placement::place_trap(y as i32, x as i32, list_type, subval as i32);
}

#[no_mangle]
pub extern "C" fn trigger_trap(flags: libc::c_ulong, y: libc::c_int, x: libc::c_int) {
    super::chest::trigger_trap(flags as u32, y as i32, x as i32);
}
```

## Migration Checklist

### Phase 2.1: Trap Data (DONE)
- [x] Create `src/dungeon/trap/` directory structure
- [x] Port `TrapTemplate` struct
- [x] Port `TRAP_LIST_A` constant array
- [x] Port `TRAP_LIST_B` constant array
- [x] Port `RUBBLE` constant
- [x] Add supporting constants used by trap placement (`TVAL_*` incl. `secret_door`)
- [x] Implement and test trap placement wrappers:
  - [x] `place_trap_into_lists`
  - [x] `place_trap_global` (unsafe global wrapper)
  - [x] `change_trap_global` (unsafe global wrapper, incl. `secret_door`)
  - [x] `place_rubble_global` (unsafe global wrapper)
- [x] Write validation tests (deterministic; serial where globals are involved)
- [x] Review with navigator

### Phase (later): Town Entrances
- [ ] Create `src/town/entrance.rs`
- [ ] Port store hour checking logic
- [ ] Port `enter_location` dispatcher
- [ ] Write unit tests
- [ ] Review with navigator

### Phase 9: Trap Logic
 - [ ] Port `effect.rs` handlers (one at a time, TDD)
 - [ ] Port `placement.rs` functions
 - [ ] Port `chest.rs` trap logic
 - [ ] Port `hit_trap` dispatcher
 - [ ] Add C interop wrappers
   - [x] Provide C ABI wrappers for `place_trap`, `change_trap`, and `place_rubble` (Rust: `dungeon::trap::interop`)
 - [ ] Integration tests
 - [ ] Remove C code
 - [ ] Update Makefile
 - [ ] Update CHANGELOG.md

## Notes

### Why Split Town Entrances?
The town entrance code (cases 101-123) is conceptually unrelated to dungeon traps. It uses the trap mechanism as a hack for "special floor tiles". Moving it to `town/entrance.rs` makes the code more discoverable and testable.

### Trap Subval Ranges
- `1-20`: Standard dungeon traps
- `99`: Scare monster (glyph of warding)
- `101-123`: Town store entrances (not really traps)

### Potential Bug: Unknown Trap Subval
The C code has a `default: msg_print("You got lucky")` which suggests unknown trap values are possible. Consider adding telemetry or assertions to catch this in testing.
