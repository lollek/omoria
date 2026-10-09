---
description: "Complete one bounded task with recorded test evidence and the repository verification gate."
agent: "agent"
argument-hint: "Behavior, allowed files, acceptance tests, and excluded scope"
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

Run `make check` before completion. A Rust unit test plus a successful C/Rust link does not prove gameplay, C callers, save restoration, or terminal interactions.

## Completion Report

- Changed files and behavior.
- RED command and expected failure, or why RED is not applicable.
- GREEN command and result.
- Final `make check` result and new warnings.
- Explicit unverified boundaries and remaining blockers.