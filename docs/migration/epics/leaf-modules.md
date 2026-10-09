# Epic LF: Leaf Modules

Small, independent ports with few dependencies. Every story can start now,
and none depends on another. Good work for light or standard agents. Back to
the [backlog](../backlog.md).

Facts here come from a lighter survey. Each agent should confirm callers and
ABI before starting.

## Stories

### LF1. Store Door Data

Status: open. Depends on: none.
Size: S. Complexity: Medium. Agent: standard.

Owns: the `store_door` definition in [variables.c](../../../src/variables.c)
and its declaration in `variables.h`, plus a new Rust data module.

Behavior: move the `store_door` array to a Rust `#[no_mangle]` static with
the same C layout. Consumers are `generate_map/town.c` (copies records into
`t_list`) and `stores.c` (reads names). Do not edit other definitions in
`variables.c`.

Acceptance checks:

* L0: every entry, including subvals 112, 115, 117, 118, and 120-124, matches
  the C data.
* Town generation and store names still link and read the same values.

### LF2. Floor Definitions

Status: open. Depends on: none.
Size: S. Complexity: Low. Agent: light.

Owns: [floor.c](../../../src/floor.c), `floor.h`, a new Rust data module.

Behavior: move the static floor definitions to Rust statics with the same C
layout and symbol names. They are used by map generation, lighting,
movement, and display.

Acceptance checks:

* L0: every definition matches the C values.
* `floor.c` is deleted; all C users still link.

### LF3. Small System Leaves

Status: open. Depends on: none.
Size: S. Complexity: Low. Agent: light.

Owns: [graphics.c](../../../src/graphics.c) and
[unix.c](../../../src/unix.c), plus their Rust replacements.

Behavior: move the `curses_is_running` global and `user_name` to Rust with
the same symbols and types. Bundled because each is tiny.

Acceptance checks:

* `user_name` returns the same value as C for the current user (L1, no
  personal files written).
* Both C files are deleted.

### LF4. Loot Placement

Status: open. Recommended: PA1.
Size: S. Complexity: Medium. Agent: standard.

Owns: [loot/loot.c](../../../src/loot/loot.c), a new Rust loot module.

Behavior: port `place_gold`, `place_random_dungeon_item`, and
`place_random_loot_near`. They mutate `cave`, `t_list`, and `gold_list`, call
item generation and `popt`/`pusht`, and use RNG. Use F-CELL if PA1 is done.

Acceptance checks:

* L1: seeded placement puts the same items at the same cells as C on a small
  map.
* `loot.c` is deleted; its many callers still link.

### LF5. Help

Status: open. Depends on: none.
Size: S. Complexity: Medium. Agent: standard.

Owns: [help.c](../../../src/help.c), a new Rust help module.

Behavior: move the help text and the `ident_char` symbol table to Rust data
with a pure lookup. C keeps the terminal loops and `get_com` prompts for now.
`moria_help` forks `mhelp.pl` and restores curses; leave it in C.

Acceptance checks:

* L0: every `ident_char` key returns the same description as C.
* L0: help pages match the C text.

### LF6. Kickout Checks (pathfinder: F-CLOCK)

Status: open. Depends on: none.
Size: S. Complexity: Medium. Agent: strong.

Owns: [kickout.c](../../../src/kickout.c), `kickout.h`, a new Rust kickout
module.

Behavior: port the operating-hours and kickout-file checks with an injected
clock and file path. The save-and-exit path calls C save and `exit_game`;
keep it behind FFI and never run it in tests.

Foundation: F-CLOCK. Injectable wall clock for later headless and main loop
work.

Acceptance checks:

* L0: hour boundaries for open and closed times match C.
* L1: kickout-file present and absent cases use a temporary directory.

### LF7. Small Init Files

Status: open. Depends on: none.
Size: S each. Complexity: Low-Medium. Agent: light or standard.

Owns: one of `init/bank.c`, `init/stores.c`, `init/monsters.c`,
`init/trade.c`, `init/death.c`, or `init/kickout.c` per agent.

Behavior: survey not yet done. The first agent per file confirms callers,
globals, and RNG, then ports it or reports a split. Give each its own ID
(LF7a, LF7b, ...) when picked up.
