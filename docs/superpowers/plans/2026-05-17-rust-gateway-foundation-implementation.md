# Rust Gateway Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the next handoff-ready slice of `issueflow` by extending the existing Rust bootstrap into a minimal Robot Gateway foundation with command parsing, workflow state machines, webhook handling, and lightweight HTML endpoints.

**Architecture:** Keep this plan limited to the Rust Gateway foundation already present in `issueflow`. The implementation should add pure domain modules first, then HTTP handlers and routing on top, while keeping persistence, GitLab CI robot scripts, runtime image work, and the Vue workbench in separate follow-up plans.

**Tech Stack:** Rust 1.95, axum, tokio, serde, serde_json

---

## Scope Boundary

This plan intentionally covers only the Gateway foundation inside the current Rust crate.

This plan does **not** implement:

- PostgreSQL persistence
- GitLab CI robot scripts
- OpenCode runtime image
- Vue 3 + Naive UI workbench
- release pipeline automation

Those areas should be handled in follow-up plans after this foundation lands.

## File Structure

### Application

- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/config.rs`
- Modify: `src/http/mod.rs`
- Modify: `src/http/routes.rs`
- Create: `src/http/handlers/mod.rs`
- Create: `src/http/handlers/status_handler.rs`
- Create: `src/http/handlers/webhook_handler.rs`
- Create: `src/http/handlers/confirm_handler.rs`
- Create: `src/gitlab/mod.rs`
- Create: `src/gitlab/commands.rs`
- Create: `src/gitlab/webhook.rs`
- Create: `src/workflow/mod.rs`
- Create: `src/workflow/types.rs`
- Create: `src/workflow/issue_state_machine.rs`
- Create: `src/workflow/mr_state_machine.rs`
- Create: `src/workflow/release_state_machine.rs`

### Tests

- Create: `tests/gitlab_commands.rs`
- Create: `tests/issue_state_machine.rs`
- Create: `tests/mr_state_machine.rs`
- Create: `tests/release_state_machine.rs`
- Create: `tests/webhook_handler.rs`
- Create: `tests/confirm_handler.rs`
- Keep: `tests/status_handler.rs`

### Templates

- Create: `internal/pages/templates/confirm_result.html`
- Create: `internal/pages/templates/status.html`

## Task 1: Add GitLab Command Parsing

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Create: `src/gitlab/mod.rs`
- Create: `src/gitlab/commands.rs`
- Test: `tests/gitlab_commands.rs`

- [ ] **Step 1: Write the failing command parser tests**

```rust
use issueflow::gitlab::commands::{parse_note_command, RobotCommand};

#[test]
fn parse_note_command_accepts_start_dev_on_first_non_empty_line() {
    let parsed = parse_note_command("\n/start-dev\nplease continue");
    assert_eq!(parsed, Some(RobotCommand::StartDev));
}

#[test]
fn parse_note_command_rejects_unknown_commands() {
    let parsed = parse_note_command("/ship-it");
    assert_eq!(parsed, None);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test gitlab_commands --test gitlab_commands`
Expected: FAIL with missing `issueflow::gitlab` module or missing parser symbols.

- [ ] **Step 3: Add serde dependencies and expose the gitlab module**

```toml
[dependencies]
axum = "0.8"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net"] }

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
```

```rust
pub mod config;
pub mod gitlab;
pub mod http;
pub mod workflow;
```

```rust
pub mod commands;
pub mod webhook;
```

- [ ] **Step 4: Implement the minimal command parser**

```rust
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RobotCommand {
    Triage,
    Validate,
    StartDev,
    Verify,
    PrepareRelease,
    Release,
}

pub fn parse_note_command(note: &str) -> Option<RobotCommand> {
    note.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .and_then(|line| match line {
            "/triage" => Some(RobotCommand::Triage),
            "/validate" => Some(RobotCommand::Validate),
            "/start-dev" => Some(RobotCommand::StartDev),
            "/verify" => Some(RobotCommand::Verify),
            "/prepare-release" => Some(RobotCommand::PrepareRelease),
            "/release" => Some(RobotCommand::Release),
            _ => None,
        })
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test gitlab_commands --test gitlab_commands`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add Cargo.toml src/lib.rs src/gitlab/mod.rs src/gitlab/commands.rs tests/gitlab_commands.rs
git commit -m "feat: add gitlab command parsing"
```

## Task 2: Add Workflow State Machines

**Files:**
- Create: `src/workflow/mod.rs`
- Create: `src/workflow/types.rs`
- Create: `src/workflow/issue_state_machine.rs`
- Create: `src/workflow/mr_state_machine.rs`
- Create: `src/workflow/release_state_machine.rs`
- Test: `tests/issue_state_machine.rs`
- Test: `tests/mr_state_machine.rs`
- Test: `tests/release_state_machine.rs`

- [ ] **Step 1: Write the failing workflow tests**

```rust
use issueflow::workflow::issue_state_machine::{next_issue_stage, IssueEvent, IssueStage};

#[test]
fn issue_state_machine_allows_start_dev_from_validated() {
    let next = next_issue_stage(IssueStage::Validated, IssueEvent::StartDev).unwrap();
    assert_eq!(next, IssueStage::MrOpened);
}
```

```rust
use issueflow::workflow::mr_state_machine::{next_mr_stage, MrEvent, MrStage};

#[test]
fn mr_state_machine_allows_confirm_from_awaiting_plan() {
    let next = next_mr_stage(MrStage::AwaitingPlanConfirm, MrEvent::ConfirmPlan).unwrap();
    assert_eq!(next, MrStage::ApprovedForDev);
}
```

```rust
use issueflow::workflow::release_state_machine::{next_release_stage, ReleaseEvent, ReleaseStage};

#[test]
fn release_state_machine_allows_prepare_from_idle() {
    let next = next_release_stage(ReleaseStage::Idle, ReleaseEvent::PrepareRelease).unwrap();
    assert_eq!(next, ReleaseStage::ReleaseChecking);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test issue_state_machine && PATH="$HOME/.cargo/bin:$PATH" cargo test --test mr_state_machine && PATH="$HOME/.cargo/bin:$PATH" cargo test --test release_state_machine`
Expected: FAIL with missing `workflow` modules or missing transition functions.

- [ ] **Step 3: Add the workflow module skeleton**

```rust
pub mod issue_state_machine;
pub mod mr_state_machine;
pub mod release_state_machine;
pub mod types;
```

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvalidTransition {
    pub machine: &'static str,
    pub from: &'static str,
    pub event: &'static str,
}

impl InvalidTransition {
    pub fn new(machine: &'static str, from: &'static str, event: &'static str) -> Self {
        Self { machine, from, event }
    }
}
```

- [ ] **Step 4: Implement the three state machines**

```rust
use super::types::InvalidTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IssueStage {
    New,
    Triaging,
    NeedsInfo,
    Validated,
    MrOpened,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IssueEvent {
    Triage,
    NeedsInfo,
    Validate,
    StartDev,
}

pub fn next_issue_stage(stage: IssueStage, event: IssueEvent) -> Result<IssueStage, InvalidTransition> {
    match (stage, event) {
        (IssueStage::New, IssueEvent::Triage) => Ok(IssueStage::Triaging),
        (IssueStage::Triaging, IssueEvent::NeedsInfo) => Ok(IssueStage::NeedsInfo),
        (IssueStage::Triaging, IssueEvent::Validate) => Ok(IssueStage::Validated),
        (IssueStage::Validated, IssueEvent::StartDev) => Ok(IssueStage::MrOpened),
        _ => Err(InvalidTransition::new("issue", stage_name(stage), event_name(event))),
    }
}

fn stage_name(stage: IssueStage) -> &'static str {
    match stage {
        IssueStage::New => "new",
        IssueStage::Triaging => "triaging",
        IssueStage::NeedsInfo => "needs-info",
        IssueStage::Validated => "validated",
        IssueStage::MrOpened => "mr-opened",
    }
}

fn event_name(event: IssueEvent) -> &'static str {
    match event {
        IssueEvent::Triage => "triage",
        IssueEvent::NeedsInfo => "needs_info",
        IssueEvent::Validate => "validate",
        IssueEvent::StartDev => "start_dev",
    }
}
```

```rust
use super::types::InvalidTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MrStage {
    DraftPlan,
    AwaitingPlanConfirm,
    ApprovedForDev,
    InDev,
    Verifying,
    Done,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MrEvent {
    PlanReady,
    ConfirmPlan,
    StartImplement,
    StartVerify,
    VerifyPassed,
}

pub fn next_mr_stage(stage: MrStage, event: MrEvent) -> Result<MrStage, InvalidTransition> {
    match (stage, event) {
        (MrStage::DraftPlan, MrEvent::PlanReady) => Ok(MrStage::AwaitingPlanConfirm),
        (MrStage::AwaitingPlanConfirm, MrEvent::ConfirmPlan) => Ok(MrStage::ApprovedForDev),
        (MrStage::ApprovedForDev, MrEvent::StartImplement) => Ok(MrStage::InDev),
        (MrStage::InDev, MrEvent::StartVerify) => Ok(MrStage::Verifying),
        (MrStage::Verifying, MrEvent::VerifyPassed) => Ok(MrStage::Done),
        _ => Err(InvalidTransition::new("mr", stage_name(stage), event_name(event))),
    }
}

fn stage_name(stage: MrStage) -> &'static str {
    match stage {
        MrStage::DraftPlan => "draft-plan",
        MrStage::AwaitingPlanConfirm => "awaiting-plan-confirm",
        MrStage::ApprovedForDev => "approved-for-dev",
        MrStage::InDev => "in-dev",
        MrStage::Verifying => "verifying",
        MrStage::Done => "done",
    }
}

fn event_name(event: MrEvent) -> &'static str {
    match event {
        MrEvent::PlanReady => "plan_ready",
        MrEvent::ConfirmPlan => "confirm_plan",
        MrEvent::StartImplement => "implement_started",
        MrEvent::StartVerify => "verify_started",
        MrEvent::VerifyPassed => "verify_passed",
    }
}
```

```rust
use super::types::InvalidTransition;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReleaseStage {
    Idle,
    ReleaseChecking,
    ReadyForRelease,
    Releasing,
    Released,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReleaseEvent {
    PrepareRelease,
    ReleaseReady,
    Publish,
    Released,
}

pub fn next_release_stage(stage: ReleaseStage, event: ReleaseEvent) -> Result<ReleaseStage, InvalidTransition> {
    match (stage, event) {
        (ReleaseStage::Idle, ReleaseEvent::PrepareRelease) => Ok(ReleaseStage::ReleaseChecking),
        (ReleaseStage::ReleaseChecking, ReleaseEvent::ReleaseReady) => Ok(ReleaseStage::ReadyForRelease),
        (ReleaseStage::ReadyForRelease, ReleaseEvent::Publish) => Ok(ReleaseStage::Releasing),
        (ReleaseStage::Releasing, ReleaseEvent::Released) => Ok(ReleaseStage::Released),
        _ => Err(InvalidTransition::new("release", stage_name(stage), event_name(event))),
    }
}

fn stage_name(stage: ReleaseStage) -> &'static str {
    match stage {
        ReleaseStage::Idle => "idle",
        ReleaseStage::ReleaseChecking => "release-checking",
        ReleaseStage::ReadyForRelease => "ready-for-release",
        ReleaseStage::Releasing => "releasing",
        ReleaseStage::Released => "released",
    }
}

fn event_name(event: ReleaseEvent) -> &'static str {
    match event {
        ReleaseEvent::PrepareRelease => "prepare_release",
        ReleaseEvent::ReleaseReady => "release_ready",
        ReleaseEvent::Publish => "release",
        ReleaseEvent::Released => "released",
    }
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test issue_state_machine && PATH="$HOME/.cargo/bin:$PATH" cargo test --test mr_state_machine && PATH="$HOME/.cargo/bin:$PATH" cargo test --test release_state_machine`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add src/workflow/mod.rs src/workflow/types.rs src/workflow/issue_state_machine.rs src/workflow/mr_state_machine.rs src/workflow/release_state_machine.rs tests/issue_state_machine.rs tests/mr_state_machine.rs tests/release_state_machine.rs
git commit -m "feat: add workflow state machines"
```

## Task 3: Add Webhook Payload Parsing and Webhook Handler

**Files:**
- Modify: `src/config.rs`
- Modify: `src/http/mod.rs`
- Modify: `src/http/routes.rs`
- Create: `src/http/handlers/mod.rs`
- Create: `src/http/handlers/webhook_handler.rs`
- Create: `src/gitlab/webhook.rs`
- Test: `tests/webhook_handler.rs`

- [ ] **Step 1: Write the failing webhook handler tests**

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use issueflow::{config::Config, http::routes};
use tower::ServiceExt;

#[tokio::test]
async fn webhook_route_rejects_invalid_token() {
    let app = routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/gitlab")
                .header("x-gitlab-token", "wrong-token")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"object_kind":"issue"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn webhook_route_accepts_valid_note_hook() {
    let app = routes::router(Config::for_tests("expected-token"));
    let payload = r#"{"object_kind":"note","object_attributes":{"note":"/start-dev","noteable_type":"Issue"}}"#;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/webhooks/gitlab")
                .header("x-gitlab-token", "expected-token")
                .header("content-type", "application/json")
                .body(Body::from(payload))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::ACCEPTED);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test webhook_handler --test webhook_handler`
Expected: FAIL with missing `router(Config)` signature, missing webhook route, or missing handler modules.

- [ ] **Step 3: Extend config and add webhook payload models**

```rust
#[derive(Clone, Debug)]
pub struct Config {
    pub listen_addr: String,
    pub gitlab_webhook_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        let listen_addr = std::env::var("LISTEN_ADDR").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let gitlab_webhook_secret = std::env::var("GITLAB_WEBHOOK_SECRET").unwrap_or_else(|_| "development-secret".to_string());
        Self { listen_addr, gitlab_webhook_secret }
    }

    pub fn for_tests(secret: &str) -> Self {
        Self {
            listen_addr: "127.0.0.1:0".to_string(),
            gitlab_webhook_secret: secret.to_string(),
        }
    }
}
```

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GitlabWebhook {
    pub object_kind: String,
    #[serde(default)]
    pub object_attributes: WebhookAttributes,
}

#[derive(Debug, Default, Deserialize)]
pub struct WebhookAttributes {
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub noteable_type: String,
}
```

- [ ] **Step 4: Implement the handler and router wiring**

```rust
pub mod webhook_handler;
```

```rust
pub mod handlers;
pub mod routes;
pub mod server;
```

```rust
use axum::{extract::State, http::{HeaderMap, StatusCode}, Json};

use crate::{config::Config, gitlab::{commands::parse_note_command, webhook::GitlabWebhook}};

pub async fn handle_webhook(
    State(config): State<Config>,
    headers: HeaderMap,
    Json(payload): Json<GitlabWebhook>,
) -> StatusCode {
    let token = headers
        .get("x-gitlab-token")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    if token != config.gitlab_webhook_secret {
        return StatusCode::UNAUTHORIZED;
    }

    if payload.object_kind == "note" {
        let _ = parse_note_command(&payload.object_attributes.note);
    }

    StatusCode::ACCEPTED
}
```

```rust
use axum::{routing::{get, post}, Router};

use crate::{config::Config, http::handlers::webhook_handler};

pub fn router(config: Config) -> Router {
    Router::new()
        .route("/status/ping", get(|| async { "ok" }))
        .route("/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .with_state(config)
}
```

- [ ] **Step 5: Update `main.rs` and server wiring to use `router(config.clone())`**

```rust
use issueflow::{config::Config, http::server};

#[tokio::main]
async fn main() {
    let config = Config::from_env();
    server::serve(config)
        .await
        .expect("failed to run gateway server");
}
```

```rust
use crate::config::Config;

pub async fn serve(config: Config) -> Result<(), std::io::Error> {
    let listener = tokio::net::TcpListener::bind(&config.listen_addr).await?;
    axum::serve(listener, super::routes::router(config)).await
}
```

- [ ] **Step 6: Run tests to verify they pass**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test webhook_handler --test webhook_handler`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add src/config.rs src/http/mod.rs src/http/routes.rs src/http/handlers/mod.rs src/http/handlers/webhook_handler.rs src/gitlab/webhook.rs tests/webhook_handler.rs src/main.rs src/http/server.rs
git commit -m "feat: add gitlab webhook handling"
```

## Task 4: Add Lightweight Status and Confirm HTML Endpoints

**Files:**
- Create: `src/http/handlers/status_handler.rs`
- Create: `src/http/handlers/confirm_handler.rs`
- Create: `internal/pages/templates/status.html`
- Create: `internal/pages/templates/confirm_result.html`
- Test: `tests/confirm_handler.rs`
- Modify: `tests/status_handler.rs`

- [ ] **Step 1: Add failing HTML endpoint tests**

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use issueflow::{config::Config, http::routes};
use tower::ServiceExt;

#[tokio::test]
async fn confirm_plan_page_returns_ok() {
    let app = routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(Request::builder().uri("/confirm/plan/test-token").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use issueflow::{config::Config, http::routes};
use tower::ServiceExt;

#[tokio::test]
async fn session_status_page_returns_ok() {
    let app = routes::router(Config::for_tests("expected-token"));
    let response = app
        .oneshot(Request::builder().uri("/status/session/demo-session").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test confirm_handler && PATH="$HOME/.cargo/bin:$PATH" cargo test session_status_page_returns_ok --test status_handler -- --exact`
Expected: FAIL with missing handlers or missing routes.

- [ ] **Step 3: Create the HTML template files**

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Issueflow Plan Confirmation</title>
  </head>
  <body>
    <main>
      <h1>Plan Confirmation</h1>
      <p>This placeholder page confirms that the Gateway can serve plan confirmation results.</p>
    </main>
  </body>
</html>
```

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <title>Issueflow Session Status</title>
  </head>
  <body>
    <main>
      <h1>Session Status</h1>
      <p>This placeholder page confirms that the Gateway can serve lightweight status pages.</p>
    </main>
  </body>
</html>
```

- [ ] **Step 4: Implement the minimal HTML handlers**

```rust
use axum::{extract::Path, response::Html};

pub async fn status_ping() -> &'static str {
    "ok"
}

pub async fn session_status(Path(session_id): Path<String>) -> Html<String> {
    Html(format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>Issueflow Session Status</title></head><body><main><h1>Session Status</h1><p>Session: {session_id}</p></main></body></html>"
    ))
}
```

```rust
use axum::{extract::Path, response::Html};

pub async fn confirm_plan(Path(token): Path<String>) -> Html<String> {
    Html(format!(
        "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\" /><title>Issueflow Plan Confirmation</title></head><body><main><h1>Plan Confirmation</h1><p>Token: {token}</p></main></body></html>"
    ))
}
```

```rust
pub mod confirm_handler;
pub mod status_handler;
pub mod webhook_handler;
```

```rust
use axum::{routing::{get, post}, Router};

use crate::{
    config::Config,
    http::handlers::{confirm_handler, status_handler, webhook_handler},
};

pub fn router(config: Config) -> Router {
    Router::new()
        .route("/status/ping", get(status_handler::status_ping))
        .route("/status/session/:session_id", get(status_handler::session_status))
        .route("/confirm/plan/:token", get(confirm_handler::confirm_plan))
        .route("/webhooks/gitlab", post(webhook_handler::handle_webhook))
        .with_state(config)
}
```

- [ ] **Step 5: Run tests to verify they pass**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test confirm_handler && PATH="$HOME/.cargo/bin:$PATH" cargo test session_status_page_returns_ok --test status_handler -- --exact`
Expected: PASS

- [ ] **Step 6: Run the full current Rust test suite**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test`
Expected: PASS with `status_handler`, `confirm_handler`, `gitlab_commands`, `issue_state_machine`, `mr_state_machine`, `release_state_machine`, and `webhook_handler` all green.

- [ ] **Step 7: Commit**

```bash
git add src/http/handlers/status_handler.rs src/http/handlers/confirm_handler.rs internal/pages/templates/status.html internal/pages/templates/confirm_result.html tests/confirm_handler.rs tests/status_handler.rs
git commit -m "feat: add gateway html endpoints"
```

## Follow-Up Plans Required

After this plan lands, create separate implementation plans for:

1. PostgreSQL schema, repositories, and durable session storage
2. GitLab CI robot scripts and `.gitlab-ci.yml`
3. OpenCode runtime image
4. Agent Workbench frontend
