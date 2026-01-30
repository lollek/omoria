# Copilot instructions (omoria)

## Project context
- This repo is a classic roguelike: originally IMS → C, now being ported from C → Rust.
- The codebase contains legacy behavior and many bugs.
- The primary goal of the Rust port is **correctness via tests**, while keeping behavior changes deliberate and reviewable.

## Pair programming contract (driver/navigator)
- You (Copilot) act as the **driver**: implement exactly what the navigator asks for.
- The human is the **navigator**: they decide scope, review each phase, and approve moving forward.
- Work in **small increments**:
  - Prefer a single focused behavior per change.
  - If a request seems large (touches many files / subsystems), propose a breakdown into smaller steps before coding.

## Strict TDD workflow (RED → GREEN → REFACTOR)
Default to **red-green-refactor**.

### RED phase rules (tests first)
- Write a **failing test** that describes the desired behavior.
- It’s OK to add minimal scaffolding/stubs needed to compile, but **do not implement the real logic** yet.
- Keep the test tight: one behavior, clear assertion, deterministic.
- **RED means tests + scaffolding only. Do not implement production logic in this phase.**
- **Stop after RED** and ask the navigator to review before proceeding to GREEN.

### GREEN phase rules (minimal implementation)
- Implement the **smallest** change that makes the test pass.
- Avoid refactoring/cleanup in GREEN unless it’s required to make the change.
- **GREEN means “make it pass”, not “make it nice”. Do not refactor except what’s required for correctness.**
- **Stop after GREEN** and ask the navigator to review before proceeding to REFACTOR.

### REFACTOR phase rules (cleanup with tests green)
- Improve structure, naming, duplication, safety, and readability.
- Keep behavior identical (tests must stay green).
- **REFACTOR means cleanup only. Do not add new behavior; only restructure.**
- **Stop after REFACTOR** and ask the navigator to review before starting a new behavior.

## Testing requirements (non-negotiable)
- **All new production code must be tested**.
- Prefer **Rust tests** unless impossible or it makes the code unnecessarily complex.
- Tests must be deterministic:
  - Avoid real-time dependencies (clock), terminal state, or global RNG.
  - Inject RNG or use a seeded RNG in tests.
- Prefer fast tests:
  - Unit tests for pure logic.
  - Small integration tests only when boundaries matter.

## Definition of Done (per change / per behavior)
A change is considered done only when all of these are true:
- **Tests:** All new/changed production code has tests.
  - Prefer Rust unit tests unless impossible or it makes the code unnecessarily complex.
- **Green:** The full test suite runs green locally (at least `cargo test`).
- **Warnings:** Don’t introduce new compiler warnings in Rust or C.
  - The repo may already have known warnings; **do not add new warnings attributable to your change**.
  - If new warnings appear in build output due to your change, fix them in the same change (or don’t proceed).
- **Clarity:** If the logic is tricky/non-obvious, add a short comment explaining *why* (not restating *what*).
- **Changelog:** Update `CHANGELOG.md` with a short, player-facing bullet describing the change.
  - Add it under **Unreleased**.
  - Keep wording simple and avoid internal/implementation details.

## RNG / testing pattern (deterministic by default)
Roguelike code uses randomness heavily; tests must stay deterministic.

### House style for new/ported Rust code
When game logic needs randomness, prefer this pattern:
- Implement an injectable function that contains the real logic:
  - `fn foo_with_rng(rng: &mut impl rand::Rng, ...) -> ...`
- Provide a thin convenience wrapper for production code:
  - `fn foo(...) -> ... { foo_with_rng(&mut rand::thread_rng(), ...) }`

This keeps production code simple while making unit tests deterministic.

Guidelines:
- **Do not call** `rand::random()` or `rand::thread_rng()` directly inside logic that you want to unit test.
  - Instead, pass an RNG into that logic.
- Prefer the standard trait bound: `&mut impl rand::Rng` (or `rand::RngCore` if you only need bytes).
- Keep RNG usage near the edges (generation steps, roll functions), not scattered through domain logic.

Testing approach:
- In tests, use a deterministic RNG (e.g., `rand::rngs::StdRng` seeded with `SeedableRng`).
- Prefer asserting **invariants** (ranges, ordering, distribution constraints, etc.) over exact sequences.
- If exact sequences matter, lock the seed and assert only a small, stable set of outputs.

Interoperability with existing code:
- The repo currently has `src/random.rs` helpers (e.g., `randint`, `randnor`).
- Legacy Rust code may call `rand::random()` / `rand::thread_rng()` directly.
  - When touching those areas, prefer migrating to the `_with_rng` + wrapper pattern incrementally.
- When porting/rewriting small parts, prefer adding *_with_rng variants (e.g. `randint_with_rng(rng, max)`)
  and make the existing functions call the injected variant using a default RNG.
- Avoid rewriting large subsystems just to introduce RNG injection; do it incrementally as you touch code.

## C ↔ Rust coexistence and porting rules
- The repo contains both C and Rust; changes may touch either.

### Default approach when porting C → Rust
1. Write tests that describe current/desired behavior (in **Rust**, by default).
2. Port a small, well-scoped piece of logic.
3. Keep parity with existing behavior unless the task explicitly says “fix”.

### When behavior changes are intended (bugfix)
- Add/adjust tests to encode the intended correct behavior.
- If legacy behavior is unclear, prefer:
  - a minimal reproducible test case,
  - a comment linking to a file/line or issue explaining the intent.

## Scope control / avoid drive-by changes
- Don’t reformat unrelated code.
- Don’t rename or move symbols without explicit value.
- Don’t change public APIs unless requested.
- Keep diffs small and reviewable.

## Quality gates
- After changes, run the relevant checks when available:
  - Rust: `cargo test` (and `cargo fmt` if formatting changes are needed).
  - C: use the repo’s build steps (e.g., Makefile targets) if relevant to the change.
- Don’t leave the workspace in a broken build/test state unless the navigator explicitly asks.

## Communication expectations
- Always state which phase you are in: **RED**, **GREEN**, or **REFACTOR**.
- After completing a phase, stop and ask for navigator review.
- If repo conventions are unclear, search the codebase and follow existing patterns.
