# Agent containers

One Docker image gives agents a sandbox with the same toolchain as CI: stable
Rust with rustfmt and Clippy, GCC, Make, ncurses, git, `gh` and the Copilot
CLI. Use it in two ways:

* **Dev Container**: open the repository inside the container in VS Code.
* **Headless workers**: start one container per task with
  `scripts/delegate.sh`. The worker clones the repository, lets Copilot CLI
  implement the task on branch `agent/<task-id>`, runs `make check` and pushes
  the branch for review.

Files:

| File | Purpose |
| --- | --- |
| [`.devcontainer/Dockerfile`](../../.devcontainer/Dockerfile) | Shared image (`omoria-agent:dev`) |
| [`.devcontainer/devcontainer.json`](../../.devcontainer/devcontainer.json) | VS Code Dev Container settings |
| [`docker/compose.yml`](../../docker/compose.yml) | Headless `worker` service |
| [`scripts/agent-worker.sh`](../../scripts/agent-worker.sh) | Worker entrypoint, baked into the image |
| [`scripts/delegate.sh`](../../scripts/delegate.sh) | Host command that starts a worker |

## Prerequisites

* Docker with Compose v2 (for example Rancher Desktop or Docker Desktop).
  Allow roughly 1.5 GB of memory per concurrent worker.
* A GitHub token, passed through the environment and never stored in the
  image. Prefer a fine-grained personal access token limited to this
  repository with:
  * **Contents**: read and write, so the worker can clone and push `agent/*`
    branches.
  * **Copilot Requests**: so Copilot CLI can run.

```sh
export GH_TOKEN=github_pat_...   # or COPILOT_GITHUB_TOKEN
```

Protect `main` with branch protection so that a worker token cannot update it.

Native ARM64 images work: CI also checks the build on ARM64 Linux, where C
`char` is unsigned.

## Dev Container

Run **Dev Containers: Clone Repository in Container Volume…** in VS Code and
pick this repository. VS Code builds the image, forwards `GH_TOKEN` and
`COPILOT_GITHUB_TOKEN` from the host, and shares the `omoria-cargo-registry`
volume with the workers. Run `make check` in the container terminal as usual.

## Headless workers

Write the task in a Markdown file. Follow the
[delegated-task contract](../../.github/skills/delegated-task/SKILL.md): state the
behavior, allowed files, acceptance tests and excluded scope. Then run:

```sh
scripts/delegate.sh pa13-search-rust tasks/pa13.md        # base branch: main
scripts/delegate.sh fix-rest-prompt task.md feat/rest     # explicit base branch
```

The script refuses to start a worker when `MAX_WORKERS` workers (default 5)
are already running. It builds `omoria-agent:dev` the first time. Each worker:

1. Clones `OMORIA_REPO_URL` (default `https://github.com/lollek/omoria.git`)
   at the base branch and creates `agent/<task-id>`. It stops if that remote
   branch already exists.
2. Runs Copilot CLI non-interactively with `--no-ask-user`, with tool use
   allowed and the repository's instructions and skills loaded. The agent may
   not run `git push` or `gh pr`, and the token is hidden from its shell.
3. Runs `make check` and commits any changes the agent left uncommitted.
4. Pushes `agent/<task-id>` if there are new commits.

Results appear on the host under `agent-results/<task-id>/` (ignored by git):

| File | Contents |
| --- | --- |
| `task.md` | The task as delegated |
| `worker.log` | Full worker output; follow with `tail -f` |
| `transcript.md` | Copilot session transcript |
| `make-check.log` | `make check` output |
| `commits.txt`, `diffstat.txt` | Work on the branch |
| `status.json` | Agent, `make check`, and push outcomes, written on exit |

Review the branch like any other contribution, for example with
`gh pr create --head agent/<task-id>`. A passing `make check` in the container
doesn't prove gameplay in a real terminal. The report in `transcript.md`
lists what the agent couldn't verify.

Optional environment variables for `delegate.sh`: `MAX_WORKERS`,
`COPILOT_MODEL`, `COPILOT_MAX_AI_CREDITS`, `OMORIA_REPO_URL`,
`OMORIA_GIT_NAME`, `OMORIA_GIT_EMAIL`. Without the last two, commits use the
token owner's GitHub login and noreply address.

## Maintenance

* Rebuild after changing the Dockerfile or `scripts/agent-worker.sh`:
  `docker compose -f docker/compose.yml build worker`.
* Upgrade Copilot CLI by changing `COPILOT_CLI_VERSION` in the Dockerfile.
* Stop a worker with `docker stop omoria-agent-<task-id>`. Delete pushed
  branches with `git push origin --delete agent/<task-id>`.
