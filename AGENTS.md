# AGENTS.md

## Project Overview

`issueflow` is an artifact advancement system for AI coding workflows. It is designed to work with tools such as `Codex` and `OpenCode` to move GitLab work objects such as issues, milestones, labels, and merge requests forward through explicit artifact state, structured memory, `skills`, and agent coordination. The stable core is artifact progression control rather than code generation itself.

## Tech Stack

Rust (edition 2024) + tokio + axum 0.8 / Vue 3 + Naive UI + Vite / reqwest-based GitLab REST API client / async-openai / minijinja / OIDC / TOML + env config

## Build & Run Commands

```bash
# backend dev
GIT_WEBHOOK_SECRET=local-dev-secret cargo run

# frontend dev
npm --prefix web run dev

# targeted checks
cargo test
npm --prefix web test -- --run

# full quality gate
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test && npm --prefix web run lint && npm --prefix web run format:check && npm --prefix web run build && npm --prefix web test -- --run
```

Use `PATH="$HOME/.cargo/bin:$PATH"` when `cargo` is not on `PATH`.

## Quality Gates

Before committing, CI enforces:

```bash
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test && npm --prefix web run lint && npm --prefix web run format:check && npm --prefix web run build && npm --prefix web test -- --run
```

## Project Structure

```
src/
  main.rs             Entry point
  error.rs            AppError + RFC 7807
  config.rs           Config loading
  http/               axum server, routes, handlers
  gitlab/             GitLab REST client, webhook, commands, issues, projects
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
- For `AG-UI` + `A2UI` integration work, use the local skill at `.agent/skills/ag-ui-a2ui-integration/SKILL.md`.
- `ToolCall*` events are only for real logical tools such as GitLab, wiki, or repository reads/writes.
- `A2UI` payloads travel only through `CustomEvent` with a required `kind` field.
- Use `kind: "a2ui_render"` for agent-authored UI and `kind: "a2ui_submit"` for user interaction data returned from that UI.
- Do not put `A2UI` payloads into `ToolCallArgs` or `ToolCallResult`.

## Architecture Decisions

- Keep all GitLab integration code in `src/gitlab/`.
- Use direct REST API calls through the project-owned `reqwest` client, and expose only project-owned DTOs outside `src/gitlab/`.
- Never use service-side GitLab API tokens.
- Analysis and preparation may be triggered by agent session, tool call, webhook, or scheduled job.
- When no authenticated user session is available, the system may only do preparation work such as read-only analysis, requirement structuring, draft generation, and persisting pending actions for later confirmation.
- Any GitLab write operation must run with the current authenticated user's session access token and stay within that user's effective permissions.
- MVP default: login-free triggers stop at preparation and wait for a user to log in and explicitly confirm before any GitLab write call executes.
- Future optional mode: the system may persist pending work for background continuation, but continuation must re-bind to a user authorization context, revalidate current permissions at execution time, and keep auditability plus explicit allowlists intact.
- Future optional mode: a user may explicitly configure a personal GitLab PAT to enable background automation on that user's behalf. Such PAT usage must stay opt-in, be encrypted at rest, be revocable, be bound to explicit project or workspace scope, and remain constrained by operation allowlists plus the user's effective GitLab permissions.
- Use `async-openai` for OpenAI API integration and `minijinja` for agent prompt templating.
- Keep AG-UI code split by workspace boundary: `agui-protocol` for protocol types/events, `agui-runtime` for agent runtime/orchestration, `agui-axum` for Axum/SSE transport glue.
- `agui-protocol` must stay transport-agnostic and provider-agnostic.
- `agui-runtime` may depend on `async-openai` for now, but `agui-axum` must not depend on OpenAI provider code.
- Agent-facing GitLab access must stay controlled: explicit allowlists, validation, and no unrestricted pass-through API surface.
- If a change needs to break these rules, update this file first and state the reason.

## Testing

- Integration tests for router/handler behavior. Unit tests for pure logic.
- **HTTP feature gate**: write a failing `tests/` integration test first, then implement.
- Use `common::test_app(config)` + `.oneshot(request).await`. Cover success + auth rejection + invalid input.

## Configuration

1. Built-in defaults → 2. `config/issueflow.toml` → 3. `.env` → 4. environment variables

## Git Hygiene

- Don't commit `target/`, `.env`, or secret files.
- Scoped, focused commits.
