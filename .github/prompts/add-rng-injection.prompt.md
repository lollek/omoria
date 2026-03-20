---
description: "Add RNG injection to an existing Rust function that calls rand directly, converting it to the _with_rng pattern."
agent: "agent"
argument-hint: "Which function to add RNG injection to"
---

Convert the specified function from direct `rand::thread_rng()` / `rand::random()` usage to the injectable `_with_rng` + wrapper pattern.

## Procedure

### 1. Identify the function
- Read the current implementation and find all direct RNG calls (`rand::random()`, `rand::thread_rng()`, `rng::randint()` without injection, etc.).

### 2. RED phase — write a deterministic test
- Write a test for the `_with_rng` variant that doesn't exist yet.
- Use a seeded `StdRng`:
  ```rust
  use rand::{SeedableRng, StdRng};
  let seed: &[_] = &[1, 2, 3, 4];
  let mut rng = StdRng::from_seed(seed);
  ```
- Assert invariants (ranges, properties), not exact values — unless testing determinism itself.
- Add a minimal stub of the `_with_rng` function signature so the test compiles but fails.
- **Stop and ask for navigator review.**

### 3. GREEN phase — extract RNG parameter
- Create `fn foo_with_rng(rng: &mut impl Rng, ...) -> ...` with the real logic.
- Change the original `fn foo(...)` to call `foo_with_rng(&mut rand::thread_rng(), ...)`.
- If the function is `extern "C"`, keep the C wrapper calling the non-`_with_rng` version.
- **Stop and ask for navigator review.**

### 4. REFACTOR phase
- Clean up any redundancy.
- Add doc comments to both functions (wrapper references the `_with_rng` variant).
- Verify all tests pass with `cargo test --lib`.
- **Stop and ask for navigator review.**
