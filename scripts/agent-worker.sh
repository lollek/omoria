#!/usr/bin/env bash
# Headless agent worker entrypoint (runs inside the omoria-agent image).
# Clones the repo anonymously, lets Copilot CLI work on one task on branch
# agent/<task-id>, runs `make check`, and saves the branch as a git bundle in
# /results. The container holds no credentials that can write to the repository;
# a human imports and pushes the bundle with scripts/agent-import.sh.
set -euo pipefail

usage() {
    echo "usage: agent-worker <task-id> <task-file> [base-branch]" >&2
    exit 2
}

[[ $# -ge 2 && $# -le 3 ]] || usage
task_id=$1
task_file=$2
base_branch=${3:-main}

[[ $task_id =~ ^[A-Za-z0-9._-]+$ ]] || { echo "invalid task id: $task_id" >&2; exit 2; }
[[ -r $task_file ]] || { echo "task file not readable: $task_file" >&2; exit 2; }

repo_url=${OMORIA_REPO_URL:-https://github.com/lollek/omoria.git}
results=${OMORIA_RESULTS_DIR:-/results}
workdir=${OMORIA_WORKDIR:-$HOME/omoria}
max_credits=${COPILOT_MAX_AI_CREDITS:-500}
branch="agent/$task_id"

[[ $max_credits =~ ^[0-9]+$ ]] || { echo "invalid COPILOT_MAX_AI_CREDITS: $max_credits" >&2; exit 2; }

mkdir -p "$results"
exec > >(tee -a "$results/worker.log") 2>&1

# Only a Copilot-scoped token is accepted; repo-writing tokens never enter the session.
unset GH_TOKEN GITHUB_TOKEN
[[ -n ${COPILOT_GITHUB_TOKEN:-} ]] || { echo "COPILOT_GITHUB_TOKEN must be set" >&2; exit 2; }
export GIT_TERMINAL_PROMPT=0

status() {
    jq -n --arg task "$task_id" --arg branch "$branch" --arg base "$base_branch" \
        --arg agent "$agent_status" --arg check "$check_status" --arg bundle "$bundle_status" \
        --argjson credits "$max_credits" \
        '{task: $task, branch: $branch, base: $base, max_ai_credits: $credits,
          agent: $agent, make_check: $check, bundle: $bundle}' \
        > "$results/status.json"
}
agent_status=not-run
check_status=not-run
bundle_status=not-run
trap status EXIT

echo "== [$task_id] cloning $repo_url ($base_branch)"
git clone --branch "$base_branch" "$repo_url" "$workdir"
cd "$workdir"
git config user.name "${OMORIA_GIT_NAME:-omoria-agent}"
git config user.email "${OMORIA_GIT_EMAIL:-omoria-agent@localhost}"

if git ls-remote --exit-code --heads origin "$branch" >/dev/null; then
    echo "remote branch $branch already exists; pick another task id" >&2
    exit 2
fi
git switch -c "$branch"

prompt=$(cat <<PROMPT
You are a headless worker. Follow the workflow and completion report in
.github/skills/delegated-task/SKILL.md for the task below. Nobody can answer
questions: if requirements are ambiguous, choose the narrowest reasonable scope
and list the assumption in your report. Commit your work locally on branch
$branch. You cannot push or open pull requests; the harness runs
\`make check\` and hands the branch to a human for review.

## Task ($task_id)

$(cat "$task_file")
PROMPT
)

copilot_args=(
    -p "$prompt"
    --trust-folder
    --allow-all-tools
    --no-ask-user
    --no-auto-update
    --max-ai-credits "$max_credits"
    --deny-tool 'shell(git push)'
    --deny-tool 'shell(gh pr)'
    "--secret-env-vars=COPILOT_GITHUB_TOKEN"
    --share "$results/transcript.md"
    --log-dir "$results/copilot-logs"
)
[[ -n ${COPILOT_MODEL:-} ]] && copilot_args+=(--model "$COPILOT_MODEL")

echo "== [$task_id] running Copilot CLI (max $max_credits AI credits)"
if copilot "${copilot_args[@]}"; then agent_status=ok; else agent_status=failed; fi

echo "== [$task_id] running make check"
if make check > "$results/make-check.log" 2>&1; then check_status=passed; else check_status=failed; fi
echo "make check: $check_status (see make-check.log)"

git add -A
if ! git diff --cached --quiet; then
    git commit -q -F - <<MSG
agent: $task_id (uncommitted agent changes)

Co-authored-by: Copilot <223556219+Copilot@users.noreply.github.com>
MSG
fi
# The agent may have switched branches; the bundle always carries the final HEAD.
git update-ref "refs/heads/$branch" HEAD

git --no-pager log --oneline "origin/$base_branch..$branch" > "$results/commits.txt"
git --no-pager diff --stat "origin/$base_branch...$branch" > "$results/diffstat.txt"

if [[ ! -s $results/commits.txt ]]; then
    bundle_status=skipped-no-changes
    echo "== [$task_id] no changes to bundle"
elif git bundle create "$results/agent.bundle" "origin/$base_branch..$branch"; then
    bundle_status=created
    echo "== [$task_id] saved $branch to agent.bundle; import with scripts/agent-import.sh $task_id"
else
    bundle_status=failed
fi

[[ $agent_status == ok && $check_status == passed && $bundle_status != failed ]]
