---
description: "Implement a task end-to-end: TDD, verification, self-review, branch, commit, PR."
agent: "agent"
argument-hint: "What to implement (e.g. PA1 from docs/migration/backlog.md)"
---

Implement the given task end-to-end and autonomously.

1. Follow the task contract and completion report in [delegated-task](./delegated-task.prompt.md). Use subagents where they help (Explore for research, parallel work on independent files).
2. After full implementation and verification, review the diff with the "Olle's Reviewer" agent. Fix any issues or inconsistencies it finds, then rerun `make check`. Never run the reviewer or other terminal-using subagents while `make check` is running.
3. When the code is PR-ready: create a branch from `main`, commit, push, and open a PR targeting `main` with `gh pr create`. Put the completion report in the PR body.
4. Minimize approval prompts: one plain command per terminal call, no heredocs, no chained or piped commands; use the built-in read/search tools instead of the shell for reading files.
5. Ask only for blockers, scope changes, or design decisions.
