#!/usr/bin/env bash
# Import a headless worker's result bundle as local branch agent/<task-id>.
# Pushing stays a manual step after review.
# usage: scripts/agent-import.sh <task-id>
set -euo pipefail

[[ $# -eq 1 ]] || { echo "usage: $0 <task-id>" >&2; exit 2; }
task_id=$1
[[ $task_id =~ ^[A-Za-z0-9._-]+$ ]] || { echo "invalid task id: $task_id" >&2; exit 2; }

repo_root=$(cd "$(dirname "$0")/.." && pwd)
bundle="$repo_root/agent-results/$task_id/agent.bundle"
branch="agent/$task_id"

[[ -f $bundle && ! -L $bundle ]] || { echo "no bundle at $bundle" >&2; exit 1; }
if git -C "$repo_root" show-ref --verify --quiet "refs/heads/$branch"; then
    echo "local branch $branch already exists" >&2
    exit 1
fi

git -C "$repo_root" fetch --quiet origin
git -C "$repo_root" bundle verify --quiet "$bundle"
git -C "$repo_root" fetch --quiet "$bundle" "refs/heads/$branch:refs/heads/$branch"

echo "Imported $branch. The code is untrusted until reviewed: read the diff before"
echo "building or running it (Makefile, build scripts, .cargo/, .vscode/ and"
echo ".github/ changes run on your machine or in CI)."
echo "  review:  git log -p $branch --not --remotes=origin"
echo "  publish: git push -u origin $branch"
