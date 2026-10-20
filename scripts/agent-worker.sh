#!/usr/bin/env bash
# Headless agent worker entrypoint (runs inside the omoria-agent image).
# Clones the repo, lets Copilot CLI work on one task on branch agent/<task-id>,
# runs `make check`, then commits and pushes the branch. Results go to /results.
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
branch="agent/$task_id"

mkdir -p "$results"
exec > >(tee -a "$results/worker.log") 2>&1

export GH_TOKEN=${GH_TOKEN:-${COPILOT_GITHUB_TOKEN:-}}
[[ -n $GH_TOKEN ]] || { echo "GH_TOKEN or COPILOT_GITHUB_TOKEN must be set" >&2; exit 2; }

status() {
    jq -n --arg task "$task_id" --arg branch "$branch" --arg base "$base_branch" \
        --arg agent "$agent_status" --arg check "$check_status" --arg push "$push_status" \
        '{task: $task, branch: $branch, base: $base, agent: $agent, make_check: $check, push: $push}' \
        > "$results/status.json"
}
agent_status=not-run
check_status=not-run
push_status=not-run
trap status EXIT

echo "== [$task_id] cloning $repo_url ($base_branch)"
gh auth setup-git
git clone --branch "$base_branch" "$repo_url" "$workdir"
cd "$workdir"

if [[ -z ${OMORIA_GIT_NAME:-} || -z ${OMORIA_GIT_EMAIL:-} ]]; then
    login=$(gh api user --jq .login)
    uid=$(gh api user --jq .id)
fi
git config user.name "${OMORIA_GIT_NAME:-$login}"
git config user.email "${OMORIA_GIT_EMAIL:-$uid+$login@users.noreply.github.com}"

if git ls-remote --exit-code --heads origin "$branch" >/dev/null; then
    echo "remote branch $branch already exists; pick another task id" >&2
    exit 2
fi
git switch -c "$branch"

prompt=$(cat <<PROMPT
You are a headless worker. Follow the workflow and completion report in
.github/skills/delegated-task/SKILL.md for the task below. Nobody can answer
questions: if requirements are ambiguous, choose the narrowest reasonable scope
and list the assumption in your report. You may commit locally, but do not push
or open pull requests; the harness runs \`make check\` and pushes branch $branch.

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
    --deny-tool 'shell(git push)'
    --deny-tool 'shell(gh pr)'
    "--secret-env-vars=GH_TOKEN,COPILOT_GITHUB_TOKEN,GITHUB_TOKEN"
    --share "$results/transcript.md"
    --log-dir "$results/copilot-logs"
)
[[ -n ${COPILOT_MODEL:-} ]] && copilot_args+=(--model "$COPILOT_MODEL")
[[ -n ${COPILOT_MAX_AI_CREDITS:-} ]] && copilot_args+=(--max-ai-credits "$COPILOT_MAX_AI_CREDITS")

echo "== [$task_id] running Copilot CLI"
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

git --no-pager log --oneline "origin/$base_branch..HEAD" > "$results/commits.txt"
git --no-pager diff --stat "origin/$base_branch...HEAD" > "$results/diffstat.txt"

if [[ ! -s $results/commits.txt ]]; then
    push_status=skipped-no-changes
    echo "== [$task_id] no changes to push"
elif git push -u origin "HEAD:refs/heads/$branch"; then
    push_status=pushed
    echo "== [$task_id] pushed $branch"
else
    push_status=failed
fi

[[ $agent_status == ok && $check_status == passed && $push_status != failed ]]
