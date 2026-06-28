# GitLab REST Client Auth Migration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the `gitlab` crate integration with a project-owned `reqwest` REST client, remove service-side GitLab API token usage, and require all GitLab API calls to use the authenticated user's session access token.

**Architecture:** Keep the GitLab boundary concentrated in `src/gitlab/`, but swap the internal transport to a reusable `reqwest` client that handles base URL parsing, user-token auth, JSON decoding, and paginated GET requests. Preserve project-owned DTOs and route/agent call sites, while changing issue creation and agent orchestration to source GitLab authority exclusively from `Session.access_token`.

**Tech Stack:** Rust 2024, tokio, axum, reqwest, serde, sqlx

---

## File Map

- Modify: `Cargo.toml`
- Modify: `AGENTS.md`
- Modify: `src/config.rs`
- Modify: `src/config/raw.rs`
- Modify: `src/config/sources.rs`
- Modify: `src/gitlab/client.rs`
- Modify: `src/gitlab/issues.rs`
- Modify: `src/gitlab/projects.rs`
- Modify: `src/gitlab/repository.rs`
- Modify: `src/gitlab/wiki.rs`
- Modify: `src/http/handlers/issues_handler.rs`
- Modify: `src/agent/gitlab_tools.rs`
- Modify: `src/agent/orchestrator.rs`
- Modify: `tests/config_loading.rs`
- Modify: `tests/gitlab_issue_creation.rs`
- Modify: `docs/CONFIG.md`
- Modify: `docs/CONFIG_zh.md`
- Modify: `docs/local-development.md`

## Task 1: Lock in auth behavior with failing tests

- [ ] Add a failing integration test showing `POST /api/issues` rejects requests without an authenticated session.
- [ ] Add a failing integration test that spins up a local HTTP listener, calls `POST /api/issues` with a signed session JWT, and asserts the outbound GitLab request uses `Authorization: Bearer <session access token>` without relying on `git.token`.
- [ ] Run `PATH="$HOME/.cargo/bin:$PATH" cargo test --test gitlab_issue_creation` and confirm the new tests fail for the expected auth reasons.

## Task 2: Replace GitLab transport with project-owned REST client

- [ ] Remove the `gitlab` crate dependency from `Cargo.toml`.
- [ ] Rebuild `src/gitlab/client.rs` around `reqwest` with helpers for base URL normalization, user-token auth headers, JSON requests, raw JSON GETs, and pagination.
- [ ] Migrate `src/gitlab/issues.rs`, `projects.rs`, `repository.rs`, and `wiki.rs` to explicit GitLab REST paths while keeping project-owned DTOs unchanged at the boundary.
- [ ] Re-run focused GitLab tests and confirm the new client behavior passes.

## Task 3: Remove service-side GitLab API authority

- [ ] Delete `git.token` from config structs and environment loading while preserving `git.base_url` and `git.webhook_secret`.
- [ ] Change issue creation handlers and agent tool execution to require `Session` and pass `session.access_token` through the GitLab layer.
- [ ] Stop `src/agent/orchestrator.rs` from manufacturing a fake session from config; it must use persisted user session credentials instead.
- [ ] Re-run focused config and agent/issue tests and confirm server-token paths are gone.

## Task 4: Update docs and verify

- [ ] Rewrite config docs to remove `GIT_TOKEN` and document that GitLab API access comes from the logged-in user's OIDC session token.
- [ ] Run `PATH="$HOME/.cargo/bin:$PATH" cargo test --test gitlab_issue_creation --test config_loading`
- [ ] Run `PATH="$HOME/.cargo/bin:$PATH" cargo test`
- [ ] Run `PATH="$HOME/.cargo/bin:$PATH" cargo fmt -- --check`
