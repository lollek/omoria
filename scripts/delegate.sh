#!/usr/bin/env bash
# Start a headless agent worker container for one task.
# usage: scripts/delegate.sh <task-id> <task-file> [base-branch]
set -euo pipefail

usage() {
    echo "usage: $0 <task-id> <task-file> [base-branch]" >&2
    exit 2
}

[[ $# -ge 2 && $# -le 3 ]] || usage
task_id=$1
task_file=$2
base_branch=${3:-main}
max_workers=${MAX_WORKERS:-5}

[[ $task_id =~ ^[A-Za-z0-9._-]+$ ]] || { echo "invalid task id: $task_id" >&2; exit 2; }
[[ -r $task_file ]] || { echo "task file not readable: $task_file" >&2; exit 2; }
[[ -n ${COPILOT_GITHUB_TOKEN:-} ]] || {
    echo "export COPILOT_GITHUB_TOKEN (a Copilot-only token) first; see docs/agents/containers.md" >&2
    exit 2
}

repo_root=$(cd "$(dirname "$0")/.." && pwd)
compose=(docker compose -f "$repo_root/docker/compose.yml")
container="omoria-agent-$task_id"
results="$repo_root/agent-results/$task_id"

running=$(docker ps -q --filter label=omoria.agent=worker | wc -l | tr -d ' ')
if (( running >= max_workers )); then
    echo "$running workers already running (MAX_WORKERS=$max_workers); try again later" >&2
    exit 1
fi
if docker ps -aq --filter "name=^${container}$" | grep -q .; then
    echo "container $container already exists" >&2
    exit 1
fi
if [[ -e $results ]]; then
    echo "$results already exists; pick another task id or remove it" >&2
    exit 1
fi

if ! docker image inspect omoria-agent:dev >/dev/null 2>&1; then
    "${compose[@]}" build worker
fi

mkdir -p "$results"
cp "$task_file" "$results/task.md"

OMORIA_GIT_NAME=${OMORIA_GIT_NAME:-$(git -C "$repo_root" config user.name || true)}
OMORIA_GIT_EMAIL=${OMORIA_GIT_EMAIL:-$(git -C "$repo_root" config user.email || true)}
export OMORIA_GIT_NAME OMORIA_GIT_EMAIL

"${compose[@]}" run -d --rm --name "$container" \
    -v "$results:/results" \
    worker "$task_id" /results/task.md "$base_branch" >/dev/null

echo "Started $container on branch agent/$task_id (base $base_branch, max ${COPILOT_MAX_AI_CREDITS:-500} AI credits)."
echo "  follow:  tail -f $results/worker.log"
echo "  status:  cat $results/status.json   (written when the worker exits)"
echo "  import:  scripts/agent-import.sh $task_id   (after it finishes)"
echo "  stop:    docker stop $container"
