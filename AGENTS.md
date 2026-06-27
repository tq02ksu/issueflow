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

## Pre-Commit Quality Gates

Before committing Rust changes, run these in order. CI enforces the same gates:

1. **`cargo fmt -- --check`** — formatting must be clean. Run `cargo fmt` if it fails.
2. **`cargo clippy -- -D warnings`** — no warnings allowed (unused imports, unused variables, etc.).
3. **`cargo test`** — all tests must pass.

Frontend gates:

1. **`npm run build`** in `web/` — must compile without errors.
2. **`npm test -- --run`** in `web/` — all tests must pass.

Common pitfalls these gates catch:

- **Unused imports** in test files (e.g., importing `http::routes` when only `Config` is needed).
  Tests pass because the import compiles, but clippy rejects it.
- **Formatting drift** — code works but `cargo fmt` wants different line breaks. Always run
  `cargo fmt` after edits, not just before committing.

## Project Structure

```
src/
  main.rs             Entry point
  error.rs            Unified error type (AppError + RFC 7807)
  config.rs           Config loading (env, .env, TOML, defaults)
  config/raw.rs       Deserialization types
  config/sources.rs   Loading and merge logic
  http/mod.rs         HTTP module root
  http/server.rs      axum server
  http/routes.rs      Router definition
  http/handlers/      Per-route handlers (auth, spa, status, confirm, oidc, webhook, issues, workbench)
  gitlab/mod.rs       GitLab integration
  gitlab/webhook.rs   Webhook parsing
  gitlab/commands.rs  Note command parsing
  gitlab/issues.rs    Issue creation
  workflow/           State machines (issue, MR, release) and permissions
  oidc/mod.rs         OIDC config, discovery, state signing
  session/mod.rs      JWT session token sign/verify and extractor
tests/                Integration tests (14 files)
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

### HTTP Integration Test Gate

**Before implementing any HTTP service feature** (new route, handler, auth behavior,
middleware), write an HTTP integration test in `tests/` first. The test must exercise
the full request→router→handler→extractor chain and assert the HTTP response directly.

- Use `common::test_app(config)` to build a test router, then `.oneshot(request).await`.
- Verify `response.status()` and `response body` against expected outcomes.
- Cover at minimum: success path + auth rejection + invalid input.
- Run the failing test first to confirm it catches the missing behavior, then implement.

Existing tests: `tests/auth_me.rs`, `tests/oidc_handler.rs`, `tests/status_handler.rs`,
`tests/webhook_handler.rs`, `tests/spa_handler.rs`, `tests/gitlab_issue_creation.rs`,
`tests/confirm_handler.rs`, `tests/session_cookie.rs`.

Run a single test:
```bash
cargo test auth_me_accepts_valid_jwt -- --exact
```

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
