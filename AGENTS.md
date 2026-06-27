# AGENTS.md

## Project Overview

`issueflow` is a chat-driven issue management agent. Users describe needs in chat; the agent fills in context, drafts structured issues, and writes them into GitLab.

## Tech Stack

Rust (edition 2024) + tokio + axum 0.8 / Vue 3 + Naive UI + Vite / gitlab crate 0.1600 / OIDC / TOML + env config

## Build & Run Commands

```bash
cargo build
GIT_WEBHOOK_SECRET=local-dev-secret cargo run
cargo test
cargo test status_route_returns_ok -- --exact
cargo clippy -- -D warnings
cargo fmt -- --check
cargo build --release
cd web && npm ci && npm run build && npm test
```

Use `PATH="$HOME/.cargo/bin:$PATH"` when `cargo` is not on `PATH`.

## Quality Gates

Before committing, CI enforces:

```bash
cargo fmt -- --check     # formatting must be clean (run `cargo fmt` if fails)
cargo clippy -- -D warnings  # no warnings
cargo test               # all tests pass

# Frontend
npm run build            # compile clean
npm test -- --run        # all pass
```

## Project Structure

```
src/
  main.rs             Entry point
  error.rs            AppError + RFC 7807
  config.rs           Config loading
  http/               axum server, routes, handlers
  gitlab/             Webhook, commands, issues, projects
  workflow/           State machines + permissions
  oidc/mod.rs         OIDC config + discovery
  session/mod.rs      JWT session
tests/                Integration tests
web/                  Vue 3 + Naive UI frontend
```

## Code Conventions

- Follow existing patterns. Prefer smallest correct change.
- Keep logic in Rust, not shell or frontend.

## Agent Session Architecture

- Agent Session uses `AG-UI` + `A2UI` together: `AG-UI` for runtime communication and `A2UI` for UI description.
- `ToolCall*` events are only for real logical tools such as GitLab, wiki, or repository reads/writes.
- `A2UI` payloads travel only through `CustomEvent` with a required `kind` field.
- Use `kind: "a2ui_render"` for agent-authored UI and `kind: "a2ui_submit"` for user interaction data returned from that UI.
- Do not put `A2UI` payloads into `ToolCallArgs` or `ToolCallResult`.

## Testing

- Integration tests for router/handler behavior. Unit tests for pure logic.
- **HTTP feature gate**: write a failing `tests/` integration test first, then implement.
- Use `common::test_app(config)` + `.oneshot(request).await`. Cover success + auth rejection + invalid input.

## Configuration

1. Built-in defaults → 2. `config/issueflow.toml` → 3. `.env` → 4. environment variables

## Git Hygiene

- Don't commit `target/`, `.env`, or secret files.
- Scoped, focused commits.
