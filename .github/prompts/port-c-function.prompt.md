---
description: "Port a single C function or small piece of C logic to Rust, following the project's TDD workflow."
agent: "agent"
argument-hint: "Which C function or file to port"
---

Port the specified C function to Rust following the project's strict TDD workflow.

## Procedure

### 1. Analyze the C code
- Read the C source to understand the function's behavior, inputs, outputs, and edge cases.
- Identify dependencies (other C functions called, globals accessed, types used).
- Check [docs/c-to-rust-migration-plan.md](docs/c-to-rust-migration-plan.md) for context on where this fits.

### 2. Choose the Rust target location
- Follow existing module organization (domain-vertical slicing).
- If a module already exists for this domain, add to it.
- If creating a new module, follow the pattern in `src/dungeon/trap/` (mod.rs + data.rs + placement.rs + interop.rs).

### 3. RED phase — write a failing test
- Write a `#[cfg(test)]` test that describes the desired behavior.
- Add minimal stubs/scaffolding to compile, but do NOT implement the real logic.
- If the function uses RNG, write the test against a `_with_rng` variant with a seeded `StdRng`.
- **Stop and ask for navigator review.**

### 4. GREEN phase — minimal implementation
- Implement the smallest change that makes the test pass.
- Match the C behavior exactly unless the task says "fix".
- If the function is called from C, add an `extern "C"` wrapper in an interop module.
- **Stop and ask for navigator review.**

### 5. REFACTOR phase — cleanup
- Improve naming, reduce duplication, add doc comments if logic is non-obvious.
- Keep all tests green.
- **Stop and ask for navigator review.**

### 6. Finish
- Run `cargo test --lib` to verify all tests pass.
- Check for new compiler warnings.
- Update CHANGELOG.md under `## Unreleased` with a brief entry (prefix with "Internal:" for non-player-facing changes).
