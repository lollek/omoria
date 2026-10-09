# Testing

What the automated checks cover, how to pick a verification level, and known
gaps. Bounded headless gameplay coverage exists for scripted movement, ration
pickup, and save/reload scenarios. It does not cover a complete gameplay
workflow, general combat or creature interactions, interactive terminal input,
or real disk persistence. Unit tests and successful linking are useful
evidence, but do not establish that a player can complete a gameplay workflow.

The task cards that built this infrastructure are in the git history of
`docs/testing-roadmap.md`.

## Current Verification

The `make check` gate runs:

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
make
make test-movement
make test-messages
make test-save
make test-headless
make test-headless-interaction
make test-headless-persistence
```

This checks formatting, Rust linting and tests, and the full C/Rust build and
link, plus terminal-free C checks for movement, messages, save-record apply,
bounded main-loop control, scripted movement, ration pickup, and save/reload.
The interaction harness also verifies close-door targets through the production
C/Rust boundary with isolated globals, captured messages, and headless drawing.
Clippy does not build the `save-test-support` feature, so the test-support
functions behind it are not linted. The clean gate passes locally on macOS.
CI runner results remain unverified. Existing Clippy warnings are
non-blocking. Six scoped legacy lint allowances preserve behavior; they are not
evidence that those paths are safe. See the [build instructions](../README.md).

Before changing the allowed legacy paths, add focused tests for ability input,
flag-width assumptions, RNG rounding, terminal input-buffer validity, and C
string-pointer contracts. In particular, terminal input currently casts an
immutable buffer to a mutable C pointer; the allowance does not make that safe.

## Verification Levels

| Level | Scope | Required evidence |
| --- | --- | --- |
| L0: pure logic | Explicit inputs and outputs; injected or seeded RNG; no globals, terminal, filesystem, or clock. | Deterministic assertions on results and invariants, including relevant boundaries. |
| L1: isolated globals | Small integration slices that still use shared C/Rust state. | Serialize every test sharing that state, establish initial state, and restore it even on failure; repeat runs without order dependence. No real terminal or personal files. |
| L2: save fixtures | Compatibility and round trips through actual save record types using sanitized, tracked JSON. | Deserialize known existing-format data, assert meaningful fields, serialize and deserialize again, and compare semantic values. Invalid input fails predictably. |
| L3: headless turns | Execute bounded turns through production game logic with controlled state, commands, RNG, persistence, and messages. | Assert observable state changes and ordered messages without curses, interactive input, wall-clock dependencies, or personal files. |

Choose the smallest level that covers a change; a compile-only check is not a
substitute for a behavioral assertion.

## Test Infrastructure

* **Save JSON (L2).** Sanitized fixture `tests/fixtures/save_record_v1.json`;
  pure `parse_save`/`serialize_save` tests are inline in
  [save.rs](../src/save/save.rs). Debug-build `SaveRecord` deserialization
  needs more than 2 MiB of stack (tests use `with_large_stack`). Invalid
  `identified` types and subtypes return decode errors
  ([identification.rs](../src/identification.rs)).
* **Save apply (L1/L2).** `make test-save`
  ([save_apply_ffi.c](../tests/save_apply_ffi.c)) applies the fixture to C/Rust
  globals and reads it back. Loading rejects saves whose UID differs from the
  selected character.
* **Persistence (L0/L1).** Crate-private
  [PersistenceEngine](../src/persistence/main.rs) covers master records and
  typed character load/write/delete/list. Production is file-backed
  ([filestorage.rs](../src/persistence/filestorage.rs)) and writes character
  saves to a temporary file before renaming it over the save. The JSON-backed
  in-memory engine ([memory.rs](../src/persistence/memory.rs)) is built only
  for tests and the `save-test-support` feature.
* **Message capture (L0/L1).** [message.rs](../src/message.rs) provides
  `capture_messages()` (Rust guard) and the
  `C_message_capture_begin`/`end`/`count`/`get` C API. The newest live capture
  wins and dropping it restores the previous one, even on panic. Captured
  `msg_print` calls skip rendering and the ` -more-` prompt.
* **Seeded C-facing RNG (L1).** [rng/random.rs](../src/rng/random.rs) and
  [random_extern.rs](../src/random_extern.rs) provide a thread-local seeded
  override for `randint`, `rand_rep`, and `randnor`, with a Rust guard and C
  begin/end. Rust code that calls `thread_rng()` directly is not affected.
* **Item generation (L0).** Dungeon item generation takes an injected RNG
  through every roll ([generate_item.rs](../src/generate_item/generate_item.rs)).
* **Movement (L0/L1).** [movement/step.rs](../src/player_action/movement/step.rs)
  resolves steps against explicit state; `make test-movement`
  ([movement_ffi.c](../tests/movement_ffi.c)) runs the C caller for walls, map
  edges, and an open-floor move.
* **Headless terminal doubles.** Scripted input, drawing counters, and
  fail-fast prompts; see [tests/support](../tests/support/README.md).
* **Headless scenarios (L3).** `main_loop_with_commands`
  ([main_loop.h](../src/main_loop/main_loop.h)) runs the main loop with an
  injected command source and a turn limit. `make test-headless` covers loop
  control and movement ([headless_turn.c](../tests/headless_turn.c)),
  `make test-headless-interaction` a ration pickup and L1 close-door targets
  ([headless_interaction.c](../tests/headless_interaction.c)), and
  `make test-headless-persistence` save/reload through the in-memory engine
  ([headless_persistence.c](../tests/headless_persistence.c)). Each runs twice
  in one process. Before adding a scenario, check the
  [seam inventory](migration/headless-turn-seams.md) for terminal, input,
  clock, file, and RNG calls on its path.

## Known Gaps

* `load_character` itself is untested; C callers and terminal display of save
  decode errors are unverified.
* Cave data longer than the current map hits `debug::fatal`, but shorter data
  panics with an out-of-bounds index in `load_cave`
  ([dungeon.rs](../src/save/dungeon.rs)) before that check.
* The parent directory is not synced after a save rename, so the rename may
  not survive a power loss.
* The movement transition `apply_step` is `#[cfg(test)]` only; production
  still uses `move_creature`.
* The C begin/end RNG test exercises only `randint`.
* `headless_interaction.c` duplicates setup and command scripting instead of
  reusing `tests/support/headless_scenario.*`, and checks its two runs against
  fixed expectations rather than comparing them.
* The three headless Make targets share `target/debug/headless-*.o`; do not
  run them concurrently with `make -j`.
