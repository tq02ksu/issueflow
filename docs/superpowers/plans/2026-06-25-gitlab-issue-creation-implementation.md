# GitLab Issue Creation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first phase of chat-driven GitLab issue creation: draft in the workbench flow, explicit confirmation, and final Gateway-backed issue creation in GitLab.

**Architecture:** Keep the first implementation narrow. The Vue workbench exposes a minimal draft-confirm-result flow, while the Rust Gateway adds only the configuration and `POST /api/issues` path needed to perform the final controlled GitLab write through the `gitlab` crate. The page is not a direct CRUD form and the agent/Gateway boundary stays explicit.

**Tech Stack:** Rust, Axum, Tokio, Serde, `gitlab` crate, Vue 3, Pinia, Vue Router, Vitest

---

### Task 1: Add the failing Gateway configuration tests for GitLab issue creation settings

**Files:**
- Modify: `tests/config_loading.rs`
- Reference: `src/config.rs`
- Reference: `src/config/raw.rs`
- Reference: `src/config/sources.rs`

- [ ] **Step 1: Read the existing config-loading tests**

Read: `tests/config_loading.rs`
Expected: Existing tests show the current style for env/TOML/default precedence.

- [ ] **Step 2: Add failing tests for `GIT_BASE_URL` and `GIT_TOKEN` loading**

Add test cases that verify:

```rust
#[test]
fn load_config_reads_gitlab_api_settings_from_dotenv() {
    // verifies GIT_BASE_URL and GIT_TOKEN are loaded
}

#[test]
fn load_config_prefers_environment_over_dotenv_for_gitlab_api_settings() {
    // verifies env overrides dotenv values
}
```

These tests should assert the loaded config exposes:

- `git.base_url`
- `git.token`

- [ ] **Step 3: Run the focused config test file and verify the new tests fail**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test config_loading`
Expected: FAIL because `Config` and raw config types do not yet support the new GitLab API settings.

### Task 2: Implement GitLab API configuration loading

**Files:**
- Modify: `src/config/raw.rs`
- Modify: `src/config/sources.rs`
- Modify: `src/config.rs`
- Test: `tests/config_loading.rs`

- [ ] **Step 1: Extend raw config to hold GitLab API settings**

Add fields under the raw Git config for:

```rust
pub struct RawGitConfig {
    pub webhook_secret: Option<String>,
    pub base_url: Option<String>,
    pub token: Option<String>,
}
```

and update merge behavior so these values merge like the existing config fields.

- [ ] **Step 2: Load `GIT_BASE_URL` and `GIT_TOKEN` from environment sources**

Update the environment mapping so it reads:

```rust
git: Some(RawGitConfig {
    webhook_secret: values.get("GIT_WEBHOOK_SECRET").cloned(),
    base_url: values.get("GIT_BASE_URL").cloned(),
    token: values.get("GIT_TOKEN").cloned(),
})
```

- [ ] **Step 3: Extend the concrete config types**

Update `src/config.rs` so the concrete Git config holds:

```rust
pub struct GitConfig {
    pub webhook_secret: String,
    pub base_url: Option<String>,
    pub token: Option<String>,
}
```

Do not make `base_url` and `token` globally required at startup yet; phase 1 only requires them when final issue creation is used.

- [ ] **Step 4: Run the focused config test file and verify it passes**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test config_loading`
Expected: PASS

### Task 3: Add the failing HTTP tests for GitLab issue creation

**Files:**
- Create: `tests/gitlab_issue_creation.rs`
- Reference: `tests/oidc_handler.rs`
- Reference: `src/http/routes.rs`

- [ ] **Step 1: Write failing tests for the new issue creation route**

Add tests covering at least:

```rust
#[tokio::test]
async fn create_issue_rejects_empty_title() {
    // POST /api/issues with blank title returns 400
}

#[tokio::test]
async fn create_issue_returns_internal_server_error_when_gitlab_api_config_is_missing() {
    // POST /api/issues with missing git.base_url or git.token returns 500
}
```

The payload should use the minimal phase-1 shape:

```json
{
  "project_id": 123,
  "title": "Draft onboarding issue",
  "description": "Created by issueflow agent"
}
```

- [ ] **Step 2: Run the focused HTTP test file and verify it fails**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test gitlab_issue_creation`
Expected: FAIL because the route and handler do not exist yet.

### Task 4: Implement the minimal Gateway issue creation path

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/gitlab/mod.rs`
- Create: `src/gitlab/issues.rs`
- Modify: `src/http/handlers/mod.rs`
- Create: `src/http/handlers/issues_handler.rs`
- Modify: `src/http/routes.rs`
- Test: `tests/gitlab_issue_creation.rs`

- [ ] **Step 1: Add the `gitlab` crate dependency**

Add to `Cargo.toml`:

```toml
gitlab = "0.1600"
```

- [ ] **Step 2: Add a minimal GitLab issue creation module**

Create `src/gitlab/issues.rs` with a small function that:

- checks `config.git.base_url`
- checks `config.git.token`
- initializes a `gitlab::Gitlab` client
- creates one issue with `project_id`, `title`, and `description`

Keep the surface small, for example with:

```rust
pub struct CreateIssueInput {
    pub project_id: u64,
    pub title: String,
    pub description: String,
}
```

and a function returning a minimal created issue payload.

- [ ] **Step 3: Export the new GitLab issue module**

Update `src/gitlab/mod.rs` to include:

```rust
pub mod issues;
```

- [ ] **Step 4: Add the HTTP handler and route**

Create `src/http/handlers/issues_handler.rs` with:

- request struct for `project_id`, `title`, `description`
- title trimming/validation
- call into `gitlab::issues`
- `201`, `400`, `500`, `502` status handling as defined in the spec

Update:

```rust
src/http/handlers/mod.rs
src/http/routes.rs
```

to expose and route:

```text
POST /api/issues
```

- [ ] **Step 5: Run the focused HTTP issue-creation test file and verify it passes**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test gitlab_issue_creation`
Expected: PASS

### Task 5: Add a narrow test seam for successful final-creation behavior

**Files:**
- Modify: `src/gitlab/issues.rs`
- Modify: `src/http/handlers/issues_handler.rs`
- Modify: `tests/gitlab_issue_creation.rs`

- [ ] **Step 1: Add one success-path HTTP test**

Extend `tests/gitlab_issue_creation.rs` with a test that verifies a valid request reaches the final creation path and returns a created issue payload.

Use a narrow seam that avoids real GitLab traffic. The test should verify the final HTTP contract rather than the external SDK.

- [ ] **Step 2: Run the focused issue-creation test file and verify it fails for the expected reason**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test gitlab_issue_creation`
Expected: FAIL because the handler path does not yet expose a testable success seam.

- [ ] **Step 3: Implement the smallest success seam**

Adjust the issue creation module/handler boundary so tests can verify:

- valid payload reaches the creation path
- returned response includes `id`, `iid`, `project_id`, `title`, `web_url`

Keep the seam internal and small. Do not build a large abstraction layer.

- [ ] **Step 4: Run the focused issue-creation test file and verify it passes**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test gitlab_issue_creation`
Expected: PASS

### Task 6: Add the failing frontend tests for draft, confirmation, and result states

**Files:**
- Modify: `web/src/tests/app.spec.ts`
- Create: `web/src/tests/workbench-issue-flow.spec.ts`
- Reference: `web/src/views/WorkbenchView.vue`
- Reference: `web/src/stores/session.ts`

- [ ] **Step 1: Add failing tests for the workbench issue flow states**

Write tests that verify the workbench can show:

- a structured draft state
- a confirmation action
- a created-result state

The tests can use a minimal store-driven model rather than a real agent transport.

- [ ] **Step 2: Run the focused frontend test file and verify it fails**

Run: `npm test -- --run web/src/tests/workbench-issue-flow.spec.ts`
Workdir: `web`
Expected: FAIL because the workbench currently only renders a static skeleton.

### Task 7: Implement the minimal web workbench issue flow

**Files:**
- Modify: `web/src/stores/session.ts`
- Modify: `web/src/views/WorkbenchView.vue`
- Create: `web/src/components/issue/IssueDraftCard.vue`
- Test: `web/src/tests/workbench-issue-flow.spec.ts`

- [ ] **Step 1: Add minimal workbench state for draft, confirmation, and result**

Extend the store with the smallest useful state model for:

- issue draft present/absent
- confirmation in progress
- created issue result

Do not add full agent protocol logic yet.

- [ ] **Step 2: Replace the static workbench text with a minimal draft-confirm-result flow**

Update `WorkbenchView.vue` so it can render:

- a chat-driven draft placeholder state
- a structured draft card
- a confirm action
- a created issue result panel

The UI can be simple; the main requirement is that it reflects the interaction model.

- [ ] **Step 3: Add a focused presentational component for the structured draft**

Create `web/src/components/issue/IssueDraftCard.vue` to display:

- project id
- title
- description

Keep the component dumb and presentation-only.

- [ ] **Step 4: Run the focused frontend issue-flow test file and verify it passes**

Run: `npm test -- --run src/tests/workbench-issue-flow.spec.ts`
Workdir: `web`
Expected: PASS

### Task 8: Update local development documentation for GitLab-backed issue creation

**Files:**
- Modify: `docs/local-development.md`
- Modify: `README.md`
- Modify: `README_zh.md`

- [ ] **Step 1: Add the failing documentation verification step**

Define the exact documentation requirements before editing:

- `docs/local-development.md` must mention `GIT_BASE_URL`
- `docs/local-development.md` must mention `GIT_TOKEN`
- the docs must explain that final issue creation happens through the Gateway after chat-driven confirmation

- [ ] **Step 2: Update the local development document**

Add the new GitLab API config requirements and a short section explaining the phase-1 flow:

- chat draft
- user confirmation
- Gateway final write to GitLab

- [ ] **Step 3: Add a short pointer in README files if needed**

If the current README wording is too generic, add one short sentence linking the local development doc to chat-driven GitLab issue creation. Keep the README concise.

- [ ] **Step 4: Verify the docs with targeted searches**

Run:

```bash
rg "GIT_BASE_URL|GIT_TOKEN|chat-driven|Gateway" README.md README_zh.md docs/local-development.md
```

Expected: matches confirm the new issue creation flow and config are documented.

### Task 9: Run final focused verification

**Files:**
- Verify: `tests/config_loading.rs`
- Verify: `tests/gitlab_issue_creation.rs`
- Verify: `web/src/tests/workbench-issue-flow.spec.ts`
- Verify: `docs/local-development.md`

- [ ] **Step 1: Run Rust config and issue-creation tests together**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test config_loading --test gitlab_issue_creation`
Expected: PASS

- [ ] **Step 2: Run frontend tests for the workbench issue flow**

Run: `npm test -- --run src/tests/workbench-issue-flow.spec.ts src/tests/app.spec.ts`
Workdir: `web`
Expected: PASS

- [ ] **Step 3: Re-read the spec and check plan coverage**

Read: `docs/superpowers/specs/2026-06-25-gitlab-issue-creation-design.md`
Expected: Every in-scope requirement maps to a completed task.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml src tests web README.md README_zh.md docs/local-development.md docs/superpowers/specs/2026-06-25-gitlab-issue-creation-design.md docs/superpowers/plans/2026-06-25-gitlab-issue-creation-implementation.md
git commit -m "feat: add chat-driven gitlab issue creation"
```
