#! /usr/bin/env bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd "$SCRIPT_DIR/.."
ROOT_DIR=$(pwd)

# Install git hooks
cd "$ROOT_DIR/git-hooks"
for hook in *; do
    ln -sf "$ROOT_DIR/git-hooks/$hook" "$ROOT_DIR/.git/hooks/$hook"
done
echo "Git hooks installed successfully."