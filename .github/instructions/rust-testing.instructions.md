---
description: "Use when writing, editing, or running Rust tests, using cargo test, handling IDE test discovery or No tests found, adding test modules, or making code testable. Covers deterministic RNG, inline test modules, and assertion patterns for omoria."
applyTo: "**/*.rs"
---

# Rust Testing Conventions

## Test location

Tests live **inline** in the same file as the production code, inside a `#[cfg(test)]` module:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn descriptive_test_name() {
        // ...
    }
}
```

Place the test module at the end of the file, after production code. Keep enums
and structs near the top, before the functions that use them.

For large data-driven test suites, a separate `*_tests.rs` file in the same directory is acceptable (e.g., `data_tests.rs`).

## Deterministic RNG (mandatory)

Never call `rand::random()` or `rand::thread_rng()` in test code. Always use a seeded RNG:

```rust
use rand::{SeedableRng, StdRng};

let seed: &[_] = &[1, 2, 3, 4];
let mut rng = StdRng::from_seed(seed);
```

Call the `_with_rng` variant of any function that uses randomness:

```rust
let result = managed_to_hit_with_rng(&mut rng, base_to_hit, level, plus_to_hit, enemy_ac);
```

## Assertion style

- Prefer asserting **invariants** (ranges, ordering, properties) over exact sequences:
  ```rust
  assert!((1..=100).contains(&value));
  assert!(hits > 0, "Expected at least one hit via natural 1");
  ```
- Use exact assertions for deterministic output contracts, including rendered text, spacing, and field positions. For RNG-dependent results, reserve exact assertions for determinism itself (two identical seeds produce identical output).
- Always include a failure message for non-obvious assertions.

## Test naming

Use `snake_case` names that describe the behavior being tested:
- `fn randint_with_rng_is_deterministic_for_a_fixed_seed()`
- `fn managed_to_hit_sometimes_hits_via_natural_one_on_d20()`
- `fn place_trap_sets_tval_based_on_list_type()`

## One behavior per test

Each test should verify a single behavior or edge case. Prefer multiple small tests over one large test with many assertions.

## Running focused tests

- If the IDE test tool reports "No tests found" for Rust tests, switch to `cargo test <test_name_or_module_path> --lib` rather than retrying the same discovery mechanism.

## Shared state

- Every test that reads or writes shared globals must use `#[serial_test::serial]`, including callers in other test modules.
- Initialize all state the scenario depends on before assertions and restore or reset it afterward.
- A mutex around individual accesses does not isolate a multi-step test scenario.
- Prefer caller-owned state over globals. Keep tests against pure input structs parallel.
- Verify isolation with repeated `cargo test` runs using the default thread count, not `--test-threads=1`.
- When testing registered callbacks or FFI handlers, keep the handler function shared between production and tests. Exercise event enqueue, hit testing, and handler dispatch; substitute only the global/map/FFI state adapter with a deterministic fixture. Do not replace the handler itself under `cfg(test)`, which bypasses the behavior the dispatch test should verify.
- Pathfinding fixtures backed by `dungeon::cell::Cells` must keep start and goal strictly inside the map (`1 < y < height`, `1 < x < width`) and initialize `fopen` plus a known-light field (`pl`, `tl`, or `fm`); C display-light flags alone do not make a Rust pathfinding cell known.
- Run focused tests during development; use the Quality gates in `.github/copilot-instructions.md` for completion validation, including its movement-only exception.
