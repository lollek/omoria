---
name: retrospective
description: "Run a retrospective on the current agent session and apply improvements to agent customization files (copilot-instructions.md, AGENTS.md, *.instructions.md, SKILL.md, *.agent.md) so the next session performs better. Use when: session retro, retrospective, lessons learned, post-mortem, improve instructions, update instructions from this session, what went wrong."
argument-hint: "Optional: a focus area or specific friction to address"
user-invocable: true
disable-model-invocation: true
---

# Session Retrospective

Turn friction from **this conversation** into small, durable improvements to the agent customization files, apply them directly, then summarize.

## Ground rules

- **Evidence is the current conversation only.** Do not mine session history, git logs, or PRs for evidence. Reading customization files in order to edit them is expected. If the context was compacted, use the checkpoint summaries as evidence too.
- **Only edit customization files** (listed in Step 3). Never touch product code, tests, changelogs, or build config.
- **"No changes needed" is a valid outcome.** Don't invent lessons to justify the run.
- If the user passed a focus argument, prioritize signals related to it.

## Step 1: Harvest signals

Scan the conversation from the start and list concrete events:

| Signal | Examples |
|---|---|
| User correction / redirect | "no", "don't", "I meant…", reverted or rejected edits |
| Repeated instruction | User had to say the same thing more than once |
| Wasted effort | Failed or retried commands, wrong tool, guessing paths/commands, dead-end exploration, redundant file reads, unnecessary subagents |
| Knowledge discovered late | Build/test commands, file locations, conventions, gotchas that would have saved time if known upfront |
| Rule ignored | An instruction existed but wasn't followed |
| Approval friction | Command styles that triggered avoidable approval prompts |
| Clarifying questions | Questions that a documented default would have made unnecessary |
| Worked well | A non-obvious approach worth reinforcing (only if not already documented) |

For each, note: what happened, what it cost, and the likely root cause.

## Step 2: Diagnose and filter

Classify each signal's root cause:

- **Missing rule** → add one.
- **Rule exists but wasn't loaded** → fix the `description` trigger keywords or `applyTo` glob, not the body.
- **Rule exists but is ambiguous, buried, or contradicted** → rewrite, move, or resolve the conflict.
- **Rule is wrong or obsolete** → correct or delete it.
- **One-off** (typo, transient network error, unique task detail) → discard.

Keep a lesson only if it is **likely to recur**, **actionable as an instruction**, and **generalizes beyond this specific task**. Pick at most ~5 highest-impact changes.

## Step 3: Choose the target file

Inventory what exists before deciding (in the repo root and the personal folder):

- Repo: `.github/copilot-instructions.md`, `AGENTS.md`, `CLAUDE.md`, `.github/instructions/`, `.github/skills/`, `.github/agents/`, `.github/prompts/`
- Personal: `~/.copilot/instructions/`, `~/.copilot/skills/`, `~/.copilot/agents/`

| Lesson scope | Target |
|---|---|
| All work in this repo | `.github/copilot-instructions.md` or `AGENTS.md`, whichever already exists |
| Specific file types or directories in this repo | `.github/instructions/<topic>.instructions.md` with a narrow `applyTo` |
| A specific repeatable workflow | That skill's `SKILL.md` (or the skill that should have been used) |
| A custom agent's behavior | Its `*.agent.md` |
| Personal preference across all repos | `~/.copilot/instructions/<topic>.instructions.md` |
| The retrospective process itself | This file |

Prefer editing an existing file over creating one, and the most specific scope that fits. Repo-specific facts never go in personal files; personal preferences never go in repo files.

## Step 4: Apply edits

Read each target file fully first. If a rule already covers the lesson, sharpen it instead of adding a duplicate.

Writing rules:

- Imperative, specific, and checkable: "Run `make check` before reporting completion", not "be careful about testing".
- Include exact commands, paths, and names learned in the session.
- Add a short *why* clause only when the reason is non-obvious.
- Match the file's voice and structure; place the rule under the most relevant existing heading.
- **Keep files lean:** prefer replacing or merging over appending. Delete rules the session proved wrong or obsolete. Instruction bloat degrades every future session.
- Never weaken safety, security, or content-exclusion rules, and never contradict an explicit instruction the user gave in this session.
- Never record secrets, tokens, personal data, or internal URLs from the conversation.
- If a file is blocked by content exclusion, skip it and report that.

When creating a new file:

- Frontmatter between `---` markers; quote values containing colons; spaces, not tabs.
- `description` uses "Use when…" phrasing with concrete trigger keywords.
- Skills: `name` matches the folder name.
- Instructions: narrow `applyTo` globs; avoid `**` unless truly universal.

## Step 5: Validate

- Frontmatter parses as YAML in every touched file.
- Search the touched files and their siblings for the key terms of each new rule to confirm there are no duplicates or contradictions.
- Relative links resolve.
- If the repo has a check that covers these files, run it.

## Step 6: Report

Keep it concise:

1. Table: **Signal → Root cause → Change → File** (link each file).
2. Discarded signals, one line each with the reason.
3. For repo files: note the changes are uncommitted (`git --no-pager diff --stat`). Do not commit unless asked.
4. For personal files (not version-controlled): show each change as a short before/after snippet so the user can revert it.
