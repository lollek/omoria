# Shared image for the omoria Dev Container and headless agent workers.
# Mirrors the CI toolchain (.github/workflows/ci.yml): stable Rust + rustfmt/clippy,
# a C toolchain and ncurses, plus git, gh and the Copilot CLI for delegated work.
# Build context is the repository root.
FROM node:22-bookworm-slim AS node

FROM rust:1-bookworm

ARG COPILOT_CLI_VERSION=1.0.100
ARG USERNAME=dev
ARG USER_UID=1000
ARG USER_GID=1000

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        build-essential \
        ca-certificates \
        curl \
        git \
        jq \
        less \
        libncurses-dev \
        sudo \
    && install -d -m 0755 /etc/apt/keyrings \
    && curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg \
        -o /etc/apt/keyrings/githubcli-archive-keyring.gpg \
    && chmod a+r /etc/apt/keyrings/githubcli-archive-keyring.gpg \
    && echo "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" \
        > /etc/apt/sources.list.d/github-cli.list \
    && apt-get update \
    && apt-get install -y --no-install-recommends gh \
    && rm -rf /var/lib/apt/lists/*

COPY --from=node /usr/local/bin/node /usr/local/bin/node
COPY --from=node /usr/local/lib/node_modules /usr/local/lib/node_modules
RUN ln -s ../lib/node_modules/npm/bin/npm-cli.js /usr/local/bin/npm \
    && ln -s ../lib/node_modules/npm/bin/npx-cli.js /usr/local/bin/npx \
    && npm install -g "@github/copilot@${COPILOT_CLI_VERSION}" \
    && npm cache clean --force

RUN rustup component add rustfmt clippy

RUN groupadd --gid "${USER_GID}" "${USERNAME}" \
    && useradd --uid "${USER_UID}" --gid "${USER_GID}" -m -s /bin/bash "${USERNAME}" \
    && echo "${USERNAME} ALL=(root) NOPASSWD:ALL" > "/etc/sudoers.d/${USERNAME}" \
    && chmod 0440 "/etc/sudoers.d/${USERNAME}" \
    && chown -R "${USERNAME}:${USERNAME}" "${CARGO_HOME}"

COPY scripts/agent-worker.sh /usr/local/bin/agent-worker
RUN chmod 0755 /usr/local/bin/agent-worker

USER ${USERNAME}
WORKDIR /home/${USERNAME}

# Pre-fetch crates so fresh clones build without hitting crates.io for every worker.
COPY --chown=${USERNAME}:${USERNAME} Cargo.toml Cargo.lock /tmp/omoria-deps/
RUN mkdir -p /tmp/omoria-deps/src \
    && touch /tmp/omoria-deps/src/lib.rs \
    && cargo fetch --manifest-path /tmp/omoria-deps/Cargo.toml \
    && rm -rf /tmp/omoria-deps
