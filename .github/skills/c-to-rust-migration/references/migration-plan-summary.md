# Migration Plan Summary

Summarized from the authoritative
[migration plan](../../../../docs/c-to-rust-migration-plan.md), checked against
local source on 2026-10-09. Remaining C inventory: **87 `.c` files**, including
partial ports. See the main plan for details; this summary does not supersede it.

## Phases

| Phase | Risk | Focus | Status |
|-------|------|-------|--------|
| 1. Pure functions & utilities | Low | pascal.rs, rng/random.rs, text_lines | Listed utilities done |
| 2. Data & configuration | Low-Med | Trap templates, monster templates, store doors | Mostly done |
| 3. Isolated game logic | Medium | Casino games, effects, help | Not started |
| 4. Player actions | Med-High | 22 C action files, including use_magic.c | Partial (attack.rs exists) |
| 5. Magic systems | Med-High | arcane, divine, nature, song, chakra, spells, blow | Not started |
| 6. Combat & creatures | High | combat/ranged.c, creature.c, monsters.c | Partial (Rust hit calc and templates) |
| 7. Map generation | High | rooms, dungeon layout, rivers, towns | Not started |
| 8. Stores & economy | High | Stores, trade, bank, house, loot | Partial (bank display helpers) |
| 9. Core systems | Very high | Trap activation, misc, I/O, player, inventory | Partial (trap data/placement only) |
| 10. Initialization & main loop | Final | init, pregame, main_loop, main.c | Partial (Rust pregame menu/character creation) |

## Already migrated

- `pascal.rs` — Pascal helper functions; `pascal.c` removed
- `rng/random.rs` — RNG with `_with_rng` injection
- `text_lines` — String utilities
- `dungeon/trap/data.rs` — Trap templates (unified list)
- `dungeon/trap/placement.rs`, `globals.rs`, `interop.rs` — Trap placement and C ABI wrappers; activation remains C
- `generate_monster/` — Monster templates
- `model/` — Core data structures
- `data/` — Static game data
- `conversion/` — C-Rust type mappings
- `combat/fighting.rs` — `managed_to_hit` hit calculation
- `player/` — Partial (attributes, stats, skills, regeneration)
- `save/`, `persistence/` — Save/load system
- `inventory/` — Partial (display)
- `equipment.rs`, `identification.rs`
- `pregame/menu.rs` — Menu/high-score display, not a standalone `highscore.rs`; pregame still includes C
- `pregame/create_character/` — Rust character creation modules
- `town_level/enter_bank.rs` — Display helpers only; operations remain C

## Next up (recommended order)

1. Store door definitions (Phase 2, low risk)
2. Casino slot machine (Phase 3, isolated, good practice target)
3. Effects system (Phase 3, ~207 lines, moderate dependencies)
4. Simple player actions: rest, search, look (Phase 4)

## Guiding principles

1. **Test first** — RED → GREEN → REFACTOR
2. **Small increments** — one function or concern at a time
3. **Preserve behavior** — match C unless explicitly fixing a bug
4. **Interop-friendly** — use `extern "C"` wrappers so C callers don't change yet
