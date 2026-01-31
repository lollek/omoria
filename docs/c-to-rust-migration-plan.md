# C to Rust Migration Plan

This document outlines a controlled, incremental strategy for migrating the omoria codebase from C to Rust. The goal is to prioritize correctness via tests while keeping behavior changes deliberate and reviewable.

## Guiding Principles

1. **Test-first approach**: Write tests for behavior before porting code (RED → GREEN → REFACTOR)
2. **Small increments**: Port one module or function at a time
3. **Isolate first**: Start with modules that have few dependencies
4. **Preserve behavior**: Match existing C behavior unless explicitly fixing bugs
5. **Interop-friendly**: Use `extern "C"` wrappers for Rust functions called from C

## Current State Overview

### Already Migrated to Rust
The following modules are already in Rust (partially or fully):
- `model/` - Core data structures (Item, Player, Monster, etc.)
- `data/` - Static data (currency values, class info, race info)
- `conversion/` - Type conversions between C and Rust
- `logic/` - Some game logic (wallet, level_up, stat_modifiers, use_item)
- `random.rs` - RNG functions (but without `_with_rng` variants yet)
- `identification.rs` - Item identification system
- `save/`, `persistence/` - Save/load system
- `player/` - Partial player logic (attributes, stats, skills, regeneration)
- `inventory/` - Partial (display_inventory.rs)
- `highscore.rs` - High score system
- `equipment.rs` - Equipment handling

### Still in C (89 files)
Major C modules remaining:
- **Core game loop**: `main.c`, `main_loop/`, `variables.c`
- **Combat**: `fighting/`, `spells.c`, `blow.c`
- **Map generation**: `generate_map/`, `dungeon/`
- **Monsters**: `creature.c`, `monsters.c`, `generate_monster/`
- **Player actions**: `player_action/` (22 files)
- **UI/IO**: `io.c`, `screen.c`, `term.c`, `graphics.c`
- **Stores/Trading**: `stores.c`, `trade.c`, `blackmarket.c`
- **Misc**: `misc.c` (2319 lines), `pascal.c`, `traps.c`, `effects.c`
- **Casino**: `casino/` (4 files)
- **Magic**: `magic/` (5 files)
- **Init**: `init/` (10 files)

---

## Migration Phases

### Phase 1: Pure Functions & Utilities (Low Risk)
**Goal**: Migrate small, pure functions that are easy to test and have minimal dependencies.

#### 1.1 Pascal Helper Functions
- [x] `pascal.c` → `pascal.rs`
  - `pindex()` - Pascal-style index function
  - `is_in()` - Set membership test
  - `is_vowel()` - Character test
  - **Complexity**: Low (~80 lines)
  - **Dependencies**: None (pure functions)
  - **Testing**: Unit tests with edge cases

#### 1.2 Random with RNG Injection
- [x] Enhance `random.rs` with `_with_rng` pattern
  - Add `randint_with_rng()`, `rand_rep_with_rng()`, `randnor_with_rng()`
  - Keep existing functions as wrappers
  - **Complexity**: Low
  - **Testing**: Deterministic tests with seeded RNG

#### 1.3 Text/String Utilities
- [x] `text_lines.c` → port string utilities to Rust
  - Extract pure string manipulation functions
  - **Complexity**: Low-Medium
  - **Dependencies**: Minimal

### Phase 2: Data & Configuration (Low-Medium Risk)
**Goal**: Migrate static data and configuration that doesn't change during gameplay.

#### 2.1 Trap Definitions
- [ ] Trap templates from `traps.c` → `data/traps.rs`
  - Static trap definitions (treasure_type arrays)
  - **Complexity**: Low (data only)
  - **Testing**: Validate data matches C definitions

#### 2.2 Monster Templates
- [ ] `generate_monster/monster_template.c` → Rust
  - Static creature definitions
  - **Complexity**: Low-Medium
  - **Dependencies**: Model types

#### 2.3 Store Door Definitions  
- [ ] Store entry definitions from `variables.c`
  - Static store_door array
  - **Complexity**: Low

### Phase 3: Isolated Game Logic (Medium Risk)
**Goal**: Port self-contained game logic with clear inputs/outputs.

#### 3.1 Casino Games
- [ ] `casino/slotmachine.c` → `casino/slotmachine.rs`
  - Relatively isolated, self-contained logic
  - Uses RNG, IO
  - **Complexity**: Medium (~400 lines)
  - **Dependencies**: random, io, term
  - **Testing**: Mock IO, seeded RNG

- [ ] `casino/horseracing.c` → `casino/horseracing.rs`
  - Similar isolation level
  - **Complexity**: Medium

- [ ] `casino/blackjack.c` → `casino/blackjack.rs`
  - Card game logic
  - **Complexity**: Medium-High (~710 lines)

- [ ] `casino/casino.c` → `casino/mod.rs`
  - Main casino menu/dispatcher
  - **Complexity**: Low

#### 3.2 Effects System
- [ ] `effects.c` → `effects.rs`
  - Status effects on player (acid damage to armor, etc.)
  - **Complexity**: Medium (~207 lines)
  - **Dependencies**: inventory, player, random
  - **Testing**: Unit tests for each effect type

#### 3.3 Help System
- [ ] `help.c` → `help.rs`
  - Help text display
  - **Complexity**: Low-Medium
  - **Dependencies**: IO only

### Phase 4: Player Actions (Medium-High Risk)
**Goal**: Port individual player actions, which are somewhat modular.

Each player action file in `player_action/` can be migrated independently:

#### 4.1 Simple Actions (start here)
- [ ] `rest.c` → port to Rust
- [ ] `search.c` → port to Rust
- [ ] `look.c` → port to Rust
- [ ] `close.c` → port to Rust
- [ ] `open.c` → port to Rust

#### 4.2 Movement & Navigation
- [ ] `move.c` → port to Rust
- [ ] `ascend_stairs.c` → port to Rust
- [ ] `descend_stairs.c` → port to Rust

#### 4.3 Item Usage Actions
- [ ] `eat.c` → port to Rust
- [ ] `quaff_potion.c` → port to Rust
- [ ] `read_scroll.c` → port to Rust
- [ ] `aim_wand.c` → port to Rust
- [ ] `use_staff.c` → port to Rust
- [ ] `refill_lamp.c` → port to Rust
- [ ] `toggle_light_source.c` → port to Rust

#### 4.4 Combat Actions
- [ ] `attack.c` → port to Rust (partial - already has `attack.rs`)
- [ ] `bash.c` → port to Rust
- [ ] `tunnel.c` → port to Rust

#### 4.5 Trap & Door Actions
- [ ] `disarm_trap.c` → port to Rust
- [ ] `jam_door.c` → port to Rust

#### 4.6 Inventory Actions
- [ ] `drop.c` → port to Rust

### Phase 5: Magic Systems (Medium-High Risk)
**Goal**: Port the spell/magic subsystems.

- [ ] `magic/arcane.c` → Rust (mage spells)
- [ ] `magic/divine.c` → Rust (priest spells)  
- [ ] `magic/nature.c` → Rust (druid spells)
- [ ] `magic/song.c` → Rust (bard songs)
- [ ] `magic/chakra.c` → Rust (monk abilities)
- [ ] `spells.c` → Rust (spell effects and casting)
- [ ] `blow.c` → Rust (chimes, horns, gems - ~572 lines)

### Phase 6: Combat & Creatures (High Risk)
**Goal**: Port the combat system and creature AI.

- [ ] `fighting/fighting.c` → Rust
- [ ] `fighting/ranged.c` → Rust
- [ ] `creature.c` → Rust (creature AI and movement)
- [ ] `monsters.c` → Rust (monster data/handling)
- [ ] `generate_monster/generate_monster.c` → Rust

### Phase 7: Map Generation (High Risk)
**Goal**: Port dungeon and town generation.

- [ ] `generate_map/misc.c` → Rust (utility functions)
- [ ] `generate_map/rooms.c` → Rust (room generation)
- [ ] `generate_map/dungeon.c` → Rust (dungeon layout)
- [ ] `generate_map/river.c` → Rust (water features)
- [ ] `generate_map/town.c` → Rust (town generation)
- [ ] `generate_map/generate_map.c` → Rust (main entry point)
- [ ] `dungeon/light.c` → Rust (lighting)

### Phase 8: Stores & Economy (High Risk)
**Goal**: Port the store and trading systems.

- [ ] `stores.c` → Rust
- [ ] `trade.c` → Rust
- [ ] `blackmarket.c` → Rust
- [ ] `town_level/enter_bank.c` → Rust
- [ ] `town_level/enter_house.c` → Rust
- [ ] `loot/loot.c` → Rust

### Phase 9: Core Systems (Very High Risk)
**Goal**: Port the remaining core systems. Do these last.

- [ ] `traps.c` → Rust (trap activation logic)
- [ ] `misc.c` → Rust (2319 lines - break into smaller modules)
- [ ] `io.c` → Rust
- [ ] `screen.c` → Rust
- [ ] `graphics.c` → Rust
- [ ] `player.c` → Rust
- [ ] `inventory/inven.c` → Rust
- [ ] `inventory/equip.c` → Rust

### Phase 10: Initialization & Main Loop (Final)
**Goal**: Port the startup and main game loop.

- [ ] `init/` → Rust (all initialization)
- [ ] `pregame/` → Rust (character creation, menus)
- [ ] `main_loop/` → Rust
- [ ] `main.c` → Rust

---

## Migration Checklist (Per Module)

For each module migration, follow this checklist:

### Before Starting
- [ ] Read and understand the C code
- [ ] Identify all dependencies (includes, extern functions, global variables)
- [ ] Identify all functions that need to be exposed to C (extern)
- [ ] Plan test cases covering key behaviors

### RED Phase
- [ ] Write failing tests describing desired behavior
- [ ] Add minimal scaffolding/stubs to compile
- [ ] **STOP** - Review with navigator

### GREEN Phase  
- [ ] Implement minimal code to make tests pass
- [ ] Add `extern "C"` wrappers if called from C
- [ ] **STOP** - Review with navigator

### REFACTOR Phase
- [ ] Improve structure, naming, safety
- [ ] Add `_with_rng` variants if RNG is used
- [ ] Ensure tests still pass
- [ ] **STOP** - Review with navigator

### Completion
- [ ] Remove corresponding C code (or mark as deprecated)
- [ ] Update Makefile if needed
- [ ] Run full test suite (`cargo test`)
- [ ] Update CHANGELOG.md
- [ ] Mark the action as done in this migration plan
- [ ] No new compiler warnings

---

## Dependency Graph (Simplified)

```
main.c
├── main_loop/
│   └── command.c
├── init/
├── pregame/
└── [all game systems]

Game Systems:
├── player.c ←──────────────────┐
├── creature.c                   │
├── fighting/ ←── spells.c ←── blow.c
├── inventory/ ←── effects.c    │
├── stores.c ←── trade.c        │
├── generate_map/ ←── generate_monster/
├── traps.c                      │
└── misc.c ──────────────────────┘ (used everywhere)

IO/Display:
├── io.c
├── screen.c
├── term.c
└── graphics.c
```

---

## Global Variables Strategy

The C code uses many global variables (see `variables.h`). Strategy:
1. **Short term**: Access globals via `extern` from Rust
2. **Medium term**: Wrap globals in accessor functions
3. **Long term**: Move state into Rust structs, pass explicitly

Key globals to track:
- `cave[][]` - The dungeon map
- `char_row`, `char_col` - Player position
- `dun_level` - Current dungeon depth
- `turn` - Game turn counter
- `equipment[]` - Player equipment slots
- `inventory_list` - Player inventory
- `stores[]` - Store inventories

---

## Notes

### Why This Order?
1. **Pure functions first**: Easy to test, no state, no side effects
2. **Data second**: Read-only, validates model correctness
3. **Isolated systems third**: Casino is self-contained, good practice
4. **Actions fourth**: Each action is somewhat independent
5. **Combat later**: Complex interactions, needs stable foundations
6. **Core last**: Everything depends on these, migrate when foundations are solid

### Risk Mitigation
- Keep C code working alongside Rust during migration
- Use integration tests to verify C↔Rust interop
- Migrate in branches, merge only when tests pass
- Keep diffs small and reviewable

### When to Fix Bugs vs Preserve Behavior
- **Default**: Preserve existing behavior, even if buggy
- **Exception**: If a bug is trivial and the fix is obvious, fix it
- **Always**: Document any behavior changes in CHANGELOG.md
- **If unclear**: Ask the navigator before changing behavior
