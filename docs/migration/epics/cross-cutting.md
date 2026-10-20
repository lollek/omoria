# Epic XC: Cross-Cutting Follow-ups

Follow-ups found while closing epics that do not belong to a single epic.
Back to the [backlog](../backlog.md).

XC1-XC3 have no solid dependencies and can join any wave. XC4 is deferred.
XC1, XC2, and XC3 all edit
[headless_interaction.c](../../../tests/headless_interaction.c); run them one
at a time or coordinate edits to that file.

## Stories

### XC1. Seeded RNG for Hit and Confusion Rolls (fan-out)

Status: done (2026-10-20). Depends on: none.
Size: S. Complexity: Medium. Agent: standard.

Owns: [rng/random.rs](../../../src/rng/random.rs),
[combat/fighting.rs](../../../src/combat/fighting.rs),
[movement/step.rs](../../../src/player_action/movement/step.rs),
[headless_interaction.c](../../../tests/headless_interaction.c).
Must not touch: `generate_item/` (XC2).

Behavior: the production wrappers `managed_to_hit` and `resolve_direction`
route through the active RNG used by `randint`. Without a seeded override this
still uses the thread RNG, so gameplay is unchanged. The C ABI of
`managed_to_hit` is unchanged.

Acceptance checks:

* L0: with `use_seeded_rng`, `managed_to_hit` and `resolve_direction` repeat
  for the same seed.
* L1: a seeded headless bash on a closed door gives a repeatable success and
  failure through `C_player_action_bash_target`.

Out of scope: changing hit formulas or RNG call order.

### XC2. Seeded RNG for Item Generation (fan-out)

Status: open. Depends on: none. Recommended: XC1 (same helper).
Size: S. Complexity: Medium. Agent: standard.

Owns: [generate_item.rs](../../../src/generate_item/generate_item.rs),
[template/misc_usable.rs](../../../src/generate_item/template/misc_usable.rs),
[headless_interaction.c](../../../tests/headless_interaction.c).
Must not touch: `combat/`, `player_action/movement/` (XC1). If XC1 has not
merged, coordinate on the `rng/random.rs` helper.

Behavior: the production wrappers in `generate_item/` call `thread_rng()`
directly, so chest treasure (PA9) and rubble loot (PA11) are not repeatable
under `begin_c_seeded_rng`. Route them through the active RNG. Gameplay is
unchanged without a seeded override.

Acceptance checks:

* L0: with `use_seeded_rng`, `generate_item_for_dungeon_level` returns the
  same item for the same seed.
* L1: a seeded headless open of a treasure chest, or tunnel through rubble,
  produces the same item attributes on repeated runs.

Out of scope: changing item tables or generation rules.

### XC3. Headless C-Caller Checks for Search and Stairs (fan-out)

Status: open. Depends on: none.
Size: S. Complexity: Low. Agent: light.

Owns: [headless_interaction.c](../../../tests/headless_interaction.c).

Behavior: tests only. PA1 and PA3 have Rust tests but no check through the
real C symbols.

Acceptance checks:

* L1: seeded `player_action_search` next to an unseen trap and a secret door
  reveals both, with the C messages; blind search prints the blind message.
* L1: `player_action_ascend_stairs` and `player_action_descend_stairs` on the
  matching stair change `dun_level` and set `moria_flag`; on the wrong tile
  they print the C message and change nothing.

Out of scope: production code changes.

### XC4. Shared Open-Door Transition (fan-out)

Status: deferred until a third caller needs it (IU4 or monster door opening
in `creature.c`). Depends on: PA9, PA12.
Size: S. Complexity: Low. Agent: light.

Owns: [dungeon/door.rs](../../../src/dungeon/door.rs),
[open.rs](../../../src/player_action/open.rs),
[bash.rs](../../../src/player_action/bash.rs), and the new caller.

Behavior: open and bash both replace a closed door with `door_list[0]` and
set `fopen`; bash then sets `p1 = 1`, open keeps the template's `p1`. Move the
shared transition into `door.rs` next to `close`, keeping each caller's `p1`
rule and message order.

Acceptance checks:

* L0: the helper's state changes match the existing open and bash tests.
* Existing open and bash Rust and headless checks stay green unchanged.

Out of scope: lockpicking, stuck doors, and bash hit rolls.
