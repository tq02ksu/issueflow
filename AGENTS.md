# AGENTS.md

## Project Overview

`issueflow` is a chat-driven issue management agent. Users describe needs in chat; the agent fills in context, drafts structured issues, and writes them into GitLab. Agent behavior is governed by version-controlled `skills` stored in Git repositories.

## Tech Stack

- **Language**: Rust (edition 2024)
- **Runtime**: tokio (async, multi-threaded)
- **HTTP Framework**: axum 0.8
- **Frontend**: Vue 3 + Naive UI + Vite
- **GitLab SDK**: gitlab crate 0.1600
- **OIDC**: HMAC-SHA256 state signing, OIDC discovery
- **Config**: TOML + env + `.env` with layered precedence

## Build & Run Commands

```bash
# Build
cargo build

# Run
GIT_WEBHOOK_SECRET=local-dev-secret cargo run

# Tests
cargo test

# Focused test
cargo test status_route_returns_ok -- --exact

# Lint
cargo clippy -- -D warnings

# Format check
cargo fmt -- --check

# Release build
cargo build --release

# Frontend
cd web && npm ci && npm run build && npm test
```

Use `PATH="$HOME/.cargo/bin:$PATH"` when `cargo` is not on `PATH`.

## Project Structure

```
src/
  main.rs             Entry point
  config.rs           Config loading (env, .env, TOML, defaults)
  config/raw.rs       Deserialization types
  config/sources.rs   Loading and merge logic
  http/mod.rs         HTTP module root
  http/server.rs      axum server
  http/routes.rs      Router definition
  http/handlers/      Per-route handlers (spa, status, confirm, oidc, webhook, issues)
  gitlab/mod.rs       GitLab integration
  gitlab/webhook.rs   Webhook parsing
  gitlab/commands.rs  Note command parsing
  gitlab/issues.rs    Issue creation
  workflow/           State machines (issue, MR, release) and permissions
  oidc/mod.rs         OIDC config, discovery, state signing
tests/                Integration tests (13 files)
web/                  Vue 3 + Naive UI frontend
scripts/robot/        GitLab CI integration
```

## Code Conventions

- Follow existing patterns before introducing new abstractions.
- Keep Gateway logic in Rust; do not move behavior into shell scripts or frontend code.
- Prefer the smallest correct change.
- Do not opportunistically refactor unrelated code.
- Keep workflow logic separate from CI-platform-specific adapters.

## Testing

- Prefer integration tests for behavior that touches the router or config.
- Reserve unit tests for pure logic.
- Tests run against embedded defaults (no external services required).
- Use `PATH="$HOME/.cargo/bin:$PATH"` for Rust commands when `cargo` is not on `PATH`.

## Configuration

Configuration loads in this order (later overrides earlier):

1. Built-in defaults
2. `config/issueflow.toml`
3. Project root `.env`
4. Process environment variables

See [docs/CONFIG.md](docs/CONFIG.md) for the full reference.

## Git Hygiene

- Do not commit `target/`.
- Do not commit `.env`.
- Do not overwrite unrelated user changes.
- Avoid destructive git commands unless explicitly requested.
- Keep commits scoped to the work performed.
