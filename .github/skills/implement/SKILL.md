---
name: implement
description: "Implement a task end-to-end: TDD, verification, self-review, and GitHub handoff."
argument-hint: "What to implement (e.g. PA1 from docs/epics/backlog.md)"
user-invocable: true
disable-model-invocation: true
---

Implement the given task end-to-end and autonomously.

1. Follow the task contract and completion report in [delegated-task](../delegated-task/SKILL.md). Before broad exploration, inspect the story's owned implementation, its nearest production caller, and the cheapest relevant test; state one falsifiable hypothesis and the focused RED check, then start editing. Expand only when the check exposes an unresolved nearby boundary or failure. Use subagents where they help (Explore for research, parallel work on independent files).
2. If the task comes from a tracked epic, backlog, or story card, update that source card after its acceptance checks pass: mark its status accurately and record concise implementation, verification, and unverified-boundary notes. Do not change dependent or out-of-scope cards.
3. After full implementation and verification, review the diff with the "Reviewer" agent. Fix any issues or inconsistencies it finds, then rerun `CARGO_TERM_QUIET=true make -s check`. Never run the reviewer or other terminal-using subagents while the check is running. Require an explicit process exit status before reporting the gate as passed; if the runner omits it, inspect captured output or rerun the quiet command before claiming success.
4. Follow the user's requested Git handoff. If they request a PR, create a branch from `main`, commit, push, and open a PR targeting `main` with `gh pr create`; put the completion report in the PR body. If they decline a PR, do not create a branch, commit, or push unless they explicitly request those actions; leave the changes in the worktree.
5. Minimize approval prompts: one plain command per terminal call, no heredocs, no chained or piped commands; use the built-in read/search tools instead of the shell for reading files.
6. Ask only for blockers, scope changes, or design decisions.
