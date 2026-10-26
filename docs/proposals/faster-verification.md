# Faster verification

Stop running the full test suite on every change.

## Problem

`make check` is required per behavior (Definition of Done and several skills),
and agents rerun it repeatedly. One `make check` currently:

* compiles the single 62k-line crate about six ways (Clippy lib and tests,
  `cargo test`, default build, `terminal-test-support` build,
  `save-test-support` build; the harnesses alternate features),
* runs all 921 Rust tests (171 `#[serial]`),
* links and runs 12 C harnesses serially, each against a ~54 MB
  `libomoria.a`.

Any Rust edit recompiles the whole crate because there is only one crate.

## Approach

The full `make check` becomes required **once before each commit/PR** (and
always in CI).

### Part A: Tiered verification

| Tier | When | Command |
| --- | --- | --- |
| T0 focused | Each RED/GREEN/REFACTOR edit | `cargo test <module::filter>` (later `cargo test -p <crate>`) |
| T1 fast | End of each behavior or delegated task | `make check-fast` (`cargo fmt --check`, `cargo clippy --all-targets`, `cargo test`, `make omoria`), plus only the harness targets mapped to touched C boundaries |
| T2 full | Once before commit/PR; CI | `make check` (unchanged contents) |

* Add `check-fast` to the [Makefile](../../Makefile) (no C harnesses).
* Add a "source area → harness target" table to [testing.md](../testing.md),
  e.g. hunger → `test-hunger`, save/persistence → `test-save` and
  `test-headless-persistence`, main loop/command → `test-headless*`.
* Rewrite the gate wording in
  [copilot-instructions.md](../../.github/copilot-instructions.md) (REFACTOR
  rule, Definition of Done "Green", Delegated task contract, Quality gates),
  the skills `add-rng-injection`, `c-to-rust-migration`, `delegated-task` and
  `port-c-function`, and the [README](../../README.md).
* The reviewer agent keeps "`make check` would pass" (pre-merge). The
  container workflow ([containers.md](../agents/containers.md)) keeps
  `make check`, since it is the pre-PR gate.

### Part B: Cargo workspace split

* Make the root `Cargo.toml` a workspace while keeping the `omoria` staticlib
  package at the root, so `target/debug/libomoria.a` and all Makefile/C paths
  stay unchanged. New crates live in `crates/`.
* Makefile and CI cargo calls cover the whole workspace (`--workspace` or
  `default-members`, `cargo fmt --all`).
* Spike: map `crate::` dependencies between modules and rank leaf candidates.
  Findings so far:
  * `generate_monster/monster_template` (~8.8k lines, 110 tests) has **no
    `crate::` dependencies** and is the first extraction.
  * `model` (with `thirdparty`, `constants`, `misc::rs2item_name`) has small
    dependencies but declares `extern "C"` statics (`m_list`); second
    candidate.
  * `data` depends on `generate_item`, `identification` and `conversion`;
    later.
* Extract `crates/omoria-monster-templates` (types, data table, data and
  attribute tests). Keep the `#[no_mangle]` FFI (`interop.rs`) in the root
  crate and re-export via `pub use`, to avoid symbol-export pitfalls for
  dependency crates in the staticlib.
* This is a behavior-preserving move. Verify an identical Rust test count,
  that all harnesses pass, and that touching e.g. `src/player_action/**` no
  longer recompiles or retests the new crate.

## Tasks

1. Add `make check-fast`.
2. Document the tiers and the area → harness table in `docs/testing.md`.
3. Update instructions, skills and README to the T0/T1/T2 policy (after 1, 2).
4. Convert to a workspace with workspace-wide cargo flags in the Makefile
   (after 1).
5. Map module dependencies and rank leaf candidates in `docs/STRUCTURE.md`.
6. Extract the monster-templates crate (after 4, 5).
7. Confirm test-count parity, `make check`, and reduced rebuild/test scope
   (after 6).
8. Document the crate layout and `cargo test -p` focused tests in
   STRUCTURE, testing, the migration skill and instructions (after 3, 6).

## Notes

* No CHANGELOG entries: tooling and refactoring only, no runtime behavior
  change.
* The TDD RED phase doesn't apply to tooling/config moves; evidence is
  structural (test-count parity, gates green).
* Not selected, but good follow-ups if T2 is still too slow: a per-step timing
  script, change-aware `make check-changed` driven by `git diff`, and a single
  shared test-support library build with parallel (`make -j`) harnesses.
* Follow-up crates after the first extraction: `model`, then
  `data`/`generate_item`.
