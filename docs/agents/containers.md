# Agent containers

One Docker image gives agents a sandbox with the same toolchain as CI: stable
Rust with rustfmt and Clippy, GCC, Make, ncurses, git, `gh` and the Copilot
CLI. Use it in two ways:

* **Dev Container**: open the repository inside the container in VS Code.
* **Headless workers**: start one container per task with
  `scripts/delegate.sh`. The worker clones the repository, lets Copilot CLI
  implement the task on branch `agent/<task-id>`, runs `make check` and saves
  the branch as a git bundle. You import it with `scripts/agent-import.sh`,
  review it and push it yourself.

Files:

| File | Purpose |
| --- | --- |
| [`.devcontainer/Dockerfile`](../../.devcontainer/Dockerfile) | Shared image (`omoria-agent:dev`) |
| [`.devcontainer/devcontainer.json`](../../.devcontainer/devcontainer.json) | VS Code Dev Container settings |
| [`docker/compose.yml`](../../docker/compose.yml) | Headless `worker` service |
| [`scripts/agent-worker.sh`](../../scripts/agent-worker.sh) | Worker entrypoint, baked into the image |
| [`scripts/delegate.sh`](../../scripts/delegate.sh) | Host command that starts a worker |
| [`scripts/agent-import.sh`](../../scripts/agent-import.sh) | Host command that imports a worker's result as a local branch |

## Security model

Agents run with all tools allowed (autopilot). The container limits what they
can reach:

* **Host files**: the repository is cloned inside the container. Workers only
  get `agent-results/<task-id>/` from the host.
* **No root**: the image has no `sudo`.
* **No repository write access**: containers receive only
  `COPILOT_GITHUB_TOKEN`, which can make Copilot requests but cannot push.
  `GH_TOKEN` and `GITHUB_TOKEN` are never forwarded, and workers clone the public
  repository anonymously. You decide what gets pushed.
* **Spending cap**: each worker session stops at 500 AI credits ($5) unless you
  set `COPILOT_MAX_AI_CREDITS`.

Remaining risks:

* The agent can read the Copilot token and use outbound network access freely.
  The worst case is spent credits, up to the cap per worker.
* Everything a worker produces is untrusted, including `status.json`, logs and
  the `make check` verdict. Review the diff before building or running it on
  your machine: `Makefile`, `build.rs`, `.cargo/`, `.vscode/`, `.devcontainer/`
  and `.github/` changes can execute code locally or in CI.
* The Dev Container is less isolated than a headless worker, because the VS Code
  server and its GitHub session run inside it. Use headless workers for
  unattended autopilot.

## Prerequisites

* Docker with Compose v2 (for example Rancher Desktop or Docker Desktop).
  Allow roughly 1.5 GB of memory per concurrent worker.
* A Copilot-only token, passed through the environment and never stored in
  the image. Create a fine-grained personal access token with **Public
  repositories (read-only)** access and only the **Copilot Requests**
  permission. Don't use your `gh` token or any token that can write to
  repositories.

```sh
export COPILOT_GITHUB_TOKEN=github_pat_...
```

Native ARM64 images work: CI also checks the build on ARM64 Linux, where C
`char` is unsigned.

## Dev Container

Run **Dev Containers: Clone Repository in Container Volume…** in VS Code and
pick this repository. VS Code builds the image and forwards only
`COPILOT_GITHUB_TOKEN` from the host. Run `make check` in the container terminal
as usual.

By default, VS Code also shares your host Git credentials with every Dev
Container, which would let an agent push with them. Turn this off in your VS
Code **user** settings (repository files cannot change it):

```jsonc
"dev.containers.copyGitConfig": false,
"dev.containers.gitCredentialHelperConfigLocation": "none",
"dev.containers.dockerCredentialHelper": false
```

VS Code also forwards the host SSH agent whenever one is running, and no
setting disables that. Check that it holds no keys with `ssh-add -l` before you
start autopilot, and clear it with `ssh-add -D` if needed. Without
credentials the Dev Container cannot push. Let a headless worker do autopilot
tasks, or copy work out with `git bundle` and `docker cp`.

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
   anonymously at the base branch and creates `agent/<task-id>`. It stops if
   that remote branch already exists.
2. Runs Copilot CLI non-interactively with `--no-ask-user` and
   `--max-ai-credits`, with tool use allowed and the repository's instructions
   and skills loaded.
3. Runs `make check` and commits any changes the agent left uncommitted.
4. Saves the new commits as `agent.bundle` in the results directory.

Then, on the host:

```sh
scripts/agent-import.sh pa13-search-rust              # creates local branch agent/pa13-search-rust
git log -p agent/pa13-search-rust --not --remotes=origin
git push -u origin agent/pa13-search-rust             # only after review
```

Results appear on the host under `agent-results/<task-id>/` (ignored by git):

| File | Contents |
| --- | --- |
| `task.md` | The task as delegated |
| `worker.log` | Full worker output; follow with `tail -f` |
| `transcript.md` | Copilot session transcript |
| `make-check.log` | `make check` output |
| `commits.txt`, `diffstat.txt` | Work on the branch |
| `agent.bundle` | The branch, for `scripts/agent-import.sh` |
| `status.json` | Agent, `make check`, and bundle outcomes, written on exit |

After pushing, open a pull request as usual, for example with
`gh pr create --head agent/<task-id>`. CI is the trusted check. A passing `make check` in the container
doesn't prove gameplay in a real terminal. The report in `transcript.md`
lists what the agent couldn't verify.

Optional environment variables for `delegate.sh`: `MAX_WORKERS`,
`COPILOT_MODEL`, `COPILOT_MAX_AI_CREDITS` (default 500; 1 credit is $0.01),
`OMORIA_REPO_URL` (must be readable anonymously), `OMORIA_GIT_NAME`,
`OMORIA_GIT_EMAIL`. Commits use your host Git identity unless you set the last
two.

## Maintenance

* Rebuild after changing the Dockerfile or `scripts/agent-worker.sh`:
  `docker compose -f docker/compose.yml build worker`.
* Upgrade Copilot CLI by changing `COPILOT_CLI_VERSION` in the Dockerfile.
* Stop a worker with `docker stop omoria-agent-<task-id>`. Delete imported
  branches with `git branch -D agent/<task-id>`, and pushed ones with
  `git push origin --delete agent/<task-id>`.
