---
name: delegated-task
description: "Complete one bounded task with recorded test evidence and the repository verification gate."
argument-hint: "Behavior, allowed files, acceptance tests, and excluded scope"
user-invocable: true
disable-model-invocation: true
---

Complete the specified task within its approved scope.

## Task Contract

- Behavior or defect:
- Allowed files and ownership:
- Acceptance tests and observable outcomes:
- Excluded behavior and files:
- Integration boundaries requiring evidence:

Resolve missing requirements before editing. Use existing test helpers and small increments.
For behavior changes, run a focused failing test before implementation and record why it failed.
Then implement the minimum fix, rerun the same test, and refactor only with tests green.
Do not pause for phase reviews; ask only for blockers or scope changes.
For docs/configuration changes, use relevant structural checks instead of artificial gameplay tests.

Apply the T0/T1/T2 Quality gates in [copilot-instructions.md](../../copilot-instructions.md). At task completion, run T1 (`make check-fast`) and only the C harnesses mapped to touched C boundaries in [testing.md](../../../docs/testing.md). T2 `make check` is required once before each commit/PR and always in CI. A Rust unit test plus a successful C/Rust link does not prove gameplay, C callers, save restoration, or terminal interactions.

## Completion Report

- Changed files and behavior.
- RED command and expected failure, or why RED is not applicable.
- GREEN command and result.
- Final T1 result, including applicable C harnesses; T2 result when this task is the pre-commit/PR gate; and new warnings.
- Explicit unverified boundaries and remaining blockers.
