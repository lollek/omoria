# Testing Roadmap

Status: planned work. A headless gameplay harness is **not implemented**.
Existing unit tests and successful linking are useful evidence, but do not
establish that a player can complete a gameplay workflow.

## Current Verification

The `make check` gate runs:

```sh
cargo fmt --check
cargo clippy --all-targets
cargo test
make
```

This checks formatting, Rust linting and tests, and the full C/Rust build and
link. It proves the unit-tested logic and link, not gameplay or headless turns.
The clean gate passes locally on macOS. CI runner results remain unverified.
Existing Clippy warnings are non-blocking. Six scoped legacy lint allowances
preserve behavior; they are not evidence that those paths are safe. See the
[build instructions](../README.md).

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

These levels describe evidence, not a claim that every level exists today.
Choose the smallest level that covers a change; a compile-only check is not a
substitute for a behavioral assertion.

## Ordered Handoffs

Each numbered task is a separate implementation handoff. Keep its scope small,
record the failing behavioral check before implementation, and report the
focused command, result, and `make check` result (including any failures).
Do not combine these tasks into a wholesale game-state or loop rewrite.

### 1. Existing Save JSON Compatibility (L2)

Status: done. Fixture: `tests/fixtures/save_record_v1.json`; tests are inline in
[save.rs](../src/save/save.rs). Known gap: invalid `identified` entries panic
during deserialization instead of returning an error.

Start at [SaveRecord](../src/save/save_record.rs) and the
[save reader/writer](../src/save/save.rs).

Acceptance checks:

* Add a sanitized, tracked fixture under `tests/fixtures/` representing an
  existing save format, not merely freshly serialized defaults. Existing local
  saves are ignored/personal: do not blindly commit them. Remove personal names,
  identifiers, and sensitive content while retaining representative structure.
* Deserialize that JSON into `SaveRecord` without curses, global-state mutation,
  or master-file access. Assert player fields, a nonempty inventory/equipment
  example, dungeon state, identification, and monster data.
* Serialize the parsed record, deserialize it again, and compare semantic JSON
  values or corresponding fields, not whitespace or object-key order.
* Test malformed/truncated JSON and a missing required field with explicit
  errors rather than panics. Use an inline Rust test if crate-private record
  types make an external integration test unnecessarily complex.

### 2. Persistence Injection (L0/L1, Then L2)

Status: master-engine injection (2a) is done. Master operations use caller-owned
engines in tests, with shared record-update rules and JSON encoding. Character
saves also use the JSON codec, but still read and write files directly; their
storage boundary and in-memory fixture checks remain the next handoff (2b).

Start at [PersistenceEngine](../src/persistence/main.rs) and
[FileStorageEngine](../src/persistence/filestorage.rs). The current trait covers
master records; character JSON still uses direct file I/O in the save module.
Make master-engine injection one handoff, then connect character save I/O in a
small follow-up rather than assuming the trait already covers both.

Acceptance checks:

* Inject an in-memory engine into the exercised master operations; assert
  load/save calls, `allow_new` behavior, and propagated storage errors.
* Keep the file-backed production default. If injection replaces a global
  engine, use serial tests and scoped restoration, including panic cleanup.
* Add an explicit character-save storage boundary, with an in-memory adapter
  reusing task 1's fixture. Save then load and assert the same semantic record;
  cover missing data and write/read failures without touching the real save
  directory or master file.

### 3. Message Stream Capture (L0/L1)

Status: partial. Rust recording capture is implemented in
[message.rs](../src/message.rs) with serialized inline tests. `capture_messages()`
returns a guard with an ordered, unbounded `messages()` snapshot, including empty
and space-only messages; the separate history still retains only the last 50.
Capture is process-wide: the newest live guard receives messages, and dropping
it restores the previous live capture or history-only recording, including on
panic. Tests sharing recording or capture must serialize and restore history.
The Rust terminal test stub also feeds recording while preserving its last-message
helper. Capture does not disable interactive output: the C rendering/input bypass
in `io.c` is deferred to task 6. C callers and terminal behavior remain unverified.

Start at [message recording](../src/message.rs) and the C message path in
[io.c](../src/io.c). Recording history alone does not bypass terminal rendering.

Acceptance checks:

* Capture ordered messages from the production message path into a test sink
  without curses calls or input prompts; preserve interactive output by default.
* Assert exact order and contents for two emitted messages and an empty stream.
  Specify how stream capture differs from the existing 50-message history cap,
  and test that history retention remains unchanged.
* Isolate/reset shared sinks and history between tests; prove a second test
  starts empty and the production sink is restored after scoped capture.

### 4. Item Generation RNG Injection (L0)

Status: done. Dungeon-level item generation now threads rand 0.4 RNG through
level, category, template, quality, and item creation rolls. Tests cover seeded
repeatability, allowed item types, item levels, counts/charges, zero dungeon
level, and zero tries. Zero dungeon level deliberately returns item level zero
without consuming RNG or attempting the legacy full-table roll.

Start at [item generation](../src/generate_item/generate_item.rs) and
[template quality/magic application](../src/generate_item/item_template.rs).
The project currently uses rand 0.4; use its supported RNG APIs, not examples
requiring an unrelated dependency upgrade.

Acceptance checks:

* Begin with one item-generation entry point and add an injectable RNG variant
  plus the existing production wrapper. Pass that RNG through selection and
  every quality/magic roll reachable from that entry point.
* Identical explicit seeds and inputs yield identical items; assert allowed
  categories, level bounds, and valid counts/charges across several seeds.
* Cover zero dungeon level and zero tries explicitly with deliberate semantics
  and no modulo-by-zero panic. Preserve legacy distributions unless a behavior
  change is separately approved; do not assert flaky statistical thresholds.

### 5. Minimal State Slice (L0/L1)

Start with the state used by one chosen command in the
[current main loop](../src/main_loop/main_loop.c), not every global variable.
Define a small owned state slice containing only the player position, relevant
map cells, turn counter, and any additional fields that command actually uses.

Acceptance checks:

* Run a legal movement command against explicit state and assert its new
  position and turn consumption. Run a blocked movement command and assert the
  documented position/turn outcome; decide existing semantics from the code.
* Two independent states do not affect each other. If a temporary globals
  adapter is necessary, serialize it and prove restoration after failure.
* Preserve production command behavior through a thin adapter; do not duplicate
  movement/combat rules in test-only code or introduce a full-state rewrite.

### 6. Bounded Headless Turn Loop (L3)

Depends on tasks 1-5. Extract only enough control from the
[current main loop](../src/main_loop/main_loop.c) to execute a bounded command
sequence through production dispatch with explicit dependencies.

Acceptance checks:

* Load a sanitized fixture, provide a fixed RNG and scripted commands, capture
  messages, and execute a fixed turn limit without curses initialization,
  interactive input, real time, or personal-file access.
* Assert concrete outcomes: successful movement changes position, a blocked move
  obeys existing turn semantics, and a chosen interaction changes HP or inventory
  and emits the expected ordered message. Fix the setup so results are stable.
* Save/reload through in-memory persistence and assert the relevant state is
  retained. Repeat the same scenario and obtain identical outcomes.
* Command exhaustion and the turn limit terminate predictably without waiting
  for input; state and injected dependencies are restored between scenarios.
* Keep the interactive game working. Only call this headless gameplay coverage
  once these behavioral tests exist and pass; extending `make check` to include
  them is a follow-up, not a claim about the current gate.