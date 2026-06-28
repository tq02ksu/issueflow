# Agent Prompt File And OpenAI SDK Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Move the agent system prompt into a fixed repository text file, inject it into model requests, switch the OpenAI client to `async-openai`, and consolidate agent issue reads behind `src/gitlab/` helpers.

**Architecture:** Add a small prompt-loading module with a fixed repository path and a pure helper that prepends the system prompt to model messages. Replace the handwritten OpenAI streaming client with an `async-openai` adapter that still emits the existing internal `ProviderDelta` stream. Move agent issue read HTTP calls out of `gitlab_tools.rs` into `src/gitlab/issues.rs` so the agent tool layer becomes orchestration-only.

**Tech Stack:** Rust 2024, tokio, async-openai, reqwest, axum, sqlx, serde_json

---

## File Map

- Create: `prompts/agent_system.j2`
- Create: `src/agent/prompt.rs`
- Modify: `Cargo.toml`
- Modify: `src/agent/mod.rs`
- Modify: `src/agent/openai.rs`
- Modify: `src/agent/orchestrator.rs`
- Modify: `src/agent/gitlab_tools.rs`
- Modify: `src/gitlab/issues.rs`
- Modify: `src/config.rs`
- Modify: `tests/config_loading.rs`
- Create: `tests/agent_prompt.rs`

## Task 1: Prompt file loading and request message shaping

- [ ] Write a failing test that loads the fixed prompt file path and asserts the contents are non-empty.
- [ ] Run `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_prompt` and confirm it fails because the module/file does not exist yet.
- [ ] Add `prompts/agent_system.txt`, `src/agent/prompt.rs`, and a pure helper that returns model messages with the system prompt prepended.
- [ ] Re-run `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_prompt` and confirm it passes.

## Task 2: OpenAI client migration

- [ ] Write a failing unit test around the OpenAI message builder or delta translation helper.
- [ ] Run the focused Rust test and confirm it fails for the expected reason.
- [ ] Add `async-openai` to `Cargo.toml` and replace the handwritten SSE parsing in `src/agent/openai.rs` with an adapter built on the SDK streaming API.
- [ ] Re-run the focused Rust test and confirm it passes.

## Task 3: Agent issue tool consolidation

- [ ] Write a failing test that expects agent issue reads to go through reusable helpers in `src/gitlab/issues.rs`.
- [ ] Run the focused Rust test and confirm it fails before the helpers exist.
- [ ] Move `list_issues` / `get_issue` HTTP access into `src/gitlab/issues.rs` and update `src/agent/gitlab_tools.rs` to call those helpers.
- [ ] Re-run the focused Rust test and confirm it passes.

## Task 4: Verification

- [ ] Run `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_prompt --test config_loading --test agent_tools --test agent_run`
- [ ] Run `PATH="$HOME/.cargo/bin:$PATH" cargo test`
- [ ] If dependency or formatting changes require it, run `PATH="$HOME/.cargo/bin:$PATH" cargo fmt`
