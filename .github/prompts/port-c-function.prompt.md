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
- Check the [migration backlog](../../docs/migration/backlog.md) and its epic files for the story card that covers this code.

### 2. Choose the Rust target location
- Follow existing module organization (domain-vertical slicing).
- If a module already exists for this domain, add to it.
- If creating a new module, follow the pattern in `src/dungeon/trap/` (mod.rs + data.rs + placement.rs + interop.rs).

### 3. RED phase — write a failing test
- Write a `#[cfg(test)]` test that describes the desired behavior.
- Add minimal stubs/scaffolding to compile, but do NOT implement the real logic.
- If the function uses RNG, write the test against a `_with_rng` variant with a seeded `StdRng`.
- Run the focused test, record the expected RED failure, and proceed.

### 4. GREEN phase — minimal implementation
- Implement the smallest change that makes the test pass.
- Match the C behavior exactly unless the task says "fix".
- If the function is called from C, add an `extern "C"` wrapper in an interop module.
- Run the focused tests successfully and proceed.

### 5. REFACTOR phase — cleanup
- Improve naming, reduce duplication, add doc comments if logic is non-obvious.
- Keep all tests green.
- Rerun the focused tests; continue within the approved scope.

### 6. Finish
- Run `make check` to verify formatting, Clippy, all Rust tests, and the C/Rust build.
- Check for new compiler warnings.
- Update CHANGELOG.md under `## Unreleased` with a brief entry (prefix with "Internal:" for non-player-facing changes).
- Update your story card's `Status:` line in the backlog; report RED evidence and untested integration boundaries.
