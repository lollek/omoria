---
name: implement
description: "Implement a task end-to-end: TDD, verification, self-review, branch, commit, PR."
argument-hint: "What to implement (e.g. PA1 from docs/migration/backlog.md)"
user-invocable: true
disable-model-invocation: true
---

Implement the given task end-to-end and autonomously.

1. Follow the task contract and completion report in [delegated-task](../delegated-task/SKILL.md). Use subagents where they help (Explore for research, parallel work on independent files).
2. If the task comes from a tracked epic, backlog, or story card, update that source card after its acceptance checks pass: mark its status accurately and record concise implementation, verification, and unverified-boundary notes. Do not change dependent or out-of-scope cards.
3. After full implementation and verification, review the diff with the "Reviewer" agent. Fix any issues or inconsistencies it finds, then rerun `make check`. Never run the reviewer or other terminal-using subagents while `make check` is running.
4. When the code is PR-ready: create a branch from `main`, commit, push, and open a PR targeting `main` with `gh pr create`. Put the completion report in the PR body.
5. Minimize approval prompts: one plain command per terminal call, no heredocs, no chained or piped commands; use the built-in read/search tools instead of the shell for reading files.
6. Ask only for blockers, scope changes, or design decisions.
