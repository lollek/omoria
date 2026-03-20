---
description: "Use when writing or editing Rust tests, adding test modules, or making code testable. Covers deterministic RNG, inline test modules, and assertion patterns for omoria."
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
- Use exact assertions only when testing determinism itself (two identical seeds produce identical output).
- Always include a failure message for non-obvious assertions.

## Test naming

Use `snake_case` names that describe the behavior being tested:
- `fn randint_with_rng_is_deterministic_for_a_fixed_seed()`
- `fn managed_to_hit_sometimes_hits_via_natural_one_on_d20()`
- `fn place_trap_sets_tval_based_on_list_type()`

## One behavior per test

Each test should verify a single behavior or edge case. Prefer multiple small tests over one large test with many assertions.
