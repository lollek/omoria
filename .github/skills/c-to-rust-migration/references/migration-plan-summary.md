# Migration Plan Summary

Summarized from [docs/c-to-rust-migration-plan.md](/docs/c-to-rust-migration-plan.md).

## Phases

| Phase | Risk | Focus | Status |
|-------|------|-------|--------|
| 1. Pure functions & utilities | Low | pascal.rs, random.rs, text_lines | Done |
| 2. Data & configuration | Low-Med | Trap templates, monster templates, store doors | Mostly done |
| 3. Isolated game logic | Medium | Casino games, effects, help | Not started |
| 4. Player actions | Med-High | 22 action files (rest, search, move, eat, etc.) | Not started |
| 5. Magic systems | Med-High | arcane, divine, nature, song, chakra, spells, blow | Not started |
| 6. Combat & creatures | High | fighting.c, ranged.c, creature.c, monsters.c | Partial (hit calc done) |
| 7. Map generation | High | rooms, dungeon layout, rivers, towns | Not started |

## Already migrated

- `pascal.rs` — Pascal helper functions
- `rng/random.rs` — RNG with `_with_rng` injection
- `text_lines` — String utilities
- `dungeon/trap/data.rs` — Trap templates (unified list)
- `dungeon/trap/placement.rs` — Trap placement
- `generate_monster/` — Monster templates
- `model/` — Core data structures
- `data/` — Static game data
- `conversion/` — C-Rust type mappings
- `combat/fighting.rs` — `managed_to_hit` hit calculation
- `player/` — Partial (attributes, stats, skills, regeneration)
- `save/`, `persistence/` — Save/load system
- `inventory/` — Partial (display)
- `equipment.rs`, `identification.rs`

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
