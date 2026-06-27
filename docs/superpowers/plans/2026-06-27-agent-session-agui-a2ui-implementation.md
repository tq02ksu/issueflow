# Agent Session AG-UI + A2UI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build long-lived agent sessions with durable AG-UI runs, A2UI rendering, OpenAI-compatible model calls, GitLab issue/context tools, and recovery after browser refresh or server restart.

**Architecture:** The Rust gateway persists 4 tables (sessions, runs, messages, event log). HTTP creates durable runs. A background worker claims runs via lease, executes model/tool work, writes every AG-UI event to durable storage, and fans them to subscribers. The Vue frontend adds `/workbench/agent`, creates durable runs, replays persisted events after refresh, and routes `CustomEvent(kind="a2ui_render"/"a2ui_submit")` through `a2ui-vue`.

**Tech Stack:** Rust 2024, Axum 0.8, Tokio, sqlx AnyPool, reqwest, uuid, Vue 3, Pinia, Vue Router, Naive UI, a2ui-vue, Vitest

## Global Constraints

- Minimum closed loop: 4 tables (agent_sessions, agent_runs, agent_messages, agent_run_events)
- Session must belong to exactly one workbench. No unbound sessions.
- AG-UI + A2UI together. AG-UI for communication, A2UI for UI description.
- ToolCall* events only for real logical tools (GitLab, wiki, repository reads/writes).
- A2UI payloads travel only through CustomEvent with required `kind` field.
- `kind: "a2ui_render"` for agent-authored UI. `kind: "a2ui_submit"` for user interaction return.
- Do not put A2UI into ToolCallArgs or ToolCallResult.
- OpenAI-compatible protocol only.
- HTTP feature gate: write failing `tests/` integration test first, then implement.
- Use `common::test_app(config)` + `.oneshot(request).await`.
- Database: sqlx AnyPool + existing SQLite/PostgreSQL migration directories.
- Frontend route: `/workbench/agent`.
- Runs are durable DB jobs, not request-scoped tasks.
- Every AG-UI event persisted before push to live subscribers.
- `agent_run_events` retained for non-terminal runs; terminal runs cleaned by TTL.

---

## File Map

### Backend create

- `src/agent/mod.rs`
- `src/agent/models.rs` — persisted row types and API DTOs
- `src/agent/events.rs` — AG-UI event enum with `type`-tagged serde + SSE helpers
- `src/agent/sessions.rs` — session SQL persistence
- `src/agent/runs.rs` — durable run + event log + lease SQL persistence
- `src/agent/openai.rs` — OpenAI-compatible streaming client
- `src/agent/gitlab_tools.rs` — tool definitions + dispatch + allowlist
- `src/agent/orchestrator.rs` — process one claimed run
- `src/agent/worker.rs` — claim/reclaim loop
- `src/http/handlers/agent_handler.rs` — session CRUD + run create/subscribe handlers
- `tests/agent_sessions.rs` — integration tests for session CRUD
- `tests/agent_run.rs` — integration tests for run create + event subscription
- `tests/agent_schema.rs` — migration coverage
- `migrations/sqlite/003_create_agent_sessions.sql`
- `migrations/sqlite/004_create_agent_runs_messages_events.sql`
- `migrations/postgres/003_create_agent_sessions.sql`
- `migrations/postgres/004_create_agent_runs_messages_events.sql`

### Backend modify

- `Cargo.toml` — add uuid, tokio-stream
- `src/lib.rs` — export agent module
- `src/main.rs` — start worker alongside server
- `src/config.rs` — add AgentConfig
- `src/config/raw.rs` — add RawAgentConfig + merge
- `src/config/sources.rs` — add env mapping
- `src/http/handlers/mod.rs` — expose agent_handler
- `src/http/routes.rs` — add workbench session routes + durable run routes
- `tests/config_loading.rs` — add agent config tests

### Frontend create

- `web/src/api/agent.api.ts`
- `web/src/stores/agent.store.ts`
- `web/src/composables/useAgentRun.ts`
- `web/src/composables/useA2UIBridge.ts`
- `web/src/views/AgentView.vue`
- `web/src/components/agent/SessionList.vue`
- `web/src/components/agent/ChatPanel.vue`
- `web/src/components/agent/ChatMessages.vue`
- `web/src/components/agent/ChatInput.vue`
- `web/src/components/agent/ToolCallCard.vue`
- `web/src/components/agent/A2UISurfaceHost.vue`
- `web/src/tests/agent.store.spec.ts`
- `web/src/tests/useAgentRun.spec.ts`

### Frontend modify

- `web/package.json` — add a2ui-vue
- `web/src/router/index.ts` — add `/workbench/agent`
- `web/src/main.ts` — register a2ui-vue

---

### Task 1: Add agent config, dependencies, and 4-table schema

**Files:**
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/config.rs`
- Modify: `src/config/raw.rs`
- Modify: `src/config/sources.rs`
- Modify: `tests/config_loading.rs`
- Create: `tests/agent_schema.rs`
- Create: `src/agent/mod.rs`
- Create: `migrations/sqlite/003_create_agent_sessions.sql`
- Create: `migrations/sqlite/004_create_agent_runs_messages_events.sql`
- Create: `migrations/postgres/003_create_agent_sessions.sql`
- Create: `migrations/postgres/004_create_agent_runs_messages_events.sql`

**Produces:**
- `AgentConfig { openai_base_url, openai_api_key, model, max_tool_rounds }`
- Tables: `agent_sessions`, `agent_runs`, `agent_messages`, `agent_run_events`

- [ ] **Step 1: Add failing config test for agent settings**

Append to `tests/config_loading.rs`:

```rust
#[tokio::test]
async fn load_config_reads_agent_openai_settings_from_dotenv() {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("config")).unwrap();
    std::fs::write(
        dir.path().join(".env"),
        "GIT_WEBHOOK_SECRET=test-secret\nAGENT_OPENAI_BASE_URL=https://api.openai.com/v1\nAGENT_OPENAI_API_KEY=sk-test\nAGENT_MODEL=gpt-4o\nAGENT_MAX_TOOL_ROUNDS=9\n",
    ).unwrap();
    let current = std::env::current_dir().unwrap();
    std::env::set_current_dir(dir.path()).unwrap();
    let config = issueflow::config::Config::load().await.unwrap();
    std::env::set_current_dir(current).unwrap();
    assert_eq!(config.agent.openai_base_url.as_deref(), Some("https://api.openai.com/v1"));
    assert_eq!(config.agent.openai_api_key.as_deref(), Some("sk-test"));
    assert_eq!(config.agent.model.as_deref(), Some("gpt-4o"));
    assert_eq!(config.agent.max_tool_rounds, 9);
}
```

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test config_loading`
Expected: FAIL (no `agent` field).

- [ ] **Step 2: Add failing schema test**

Create `tests/agent_schema.rs`:

```rust
mod common;

#[tokio::test]
async fn migrations_create_all_agent_tables() {
    let pool = common::test_pool().await;
    let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_sessions").fetch_one(&pool).await.unwrap();
    let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_runs").fetch_one(&pool).await.unwrap();
    let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_messages").fetch_one(&pool).await.unwrap();
    let _: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM agent_run_events").fetch_one(&pool).await.unwrap();
}
```

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_schema`
Expected: FAIL (no such table).

- [ ] **Step 3: Add `uuid` dependency**

```toml
[dependencies]
uuid = { version = "1", features = ["v4", "serde"] }
tokio-stream = "0.1"
```

- [ ] **Step 4: Extend config types**

Add to `src/config/raw.rs`:

```rust
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct RawAgentConfig {
    pub openai_base_url: Option<String>,
    pub openai_api_key: Option<String>,
    pub model: Option<String>,
    pub max_tool_rounds: Option<u32>,
}
```

Wire into `RawConfig` and its `merge` method. Add `AgentConfig` to `src/config.rs` with `max_tool_rounds` defaulting to `10`.

- [ ] **Step 5: Load env vars**

In `src/config/sources.rs`, add:

```rust
agent: Some(RawAgentConfig {
    openai_base_url: values.get("AGENT_OPENAI_BASE_URL").cloned(),
    openai_api_key: values.get("AGENT_OPENAI_API_KEY").cloned(),
    model: values.get("AGENT_MODEL").cloned(),
    max_tool_rounds: values.get("AGENT_MAX_TOOL_ROUNDS").map(|v| v.parse::<u32>()).transpose()?,
}),
```

- [ ] **Step 6: Create migration SQL files**

`migrations/sqlite/003_create_agent_sessions.sql`:

```sql
CREATE TABLE IF NOT EXISTS agent_sessions (
    id              TEXT PRIMARY KEY,
    user_id         INTEGER NOT NULL,
    workbench_id    INTEGER NOT NULL,
    title           TEXT NOT NULL,
    latest_state    TEXT,
    last_message_at TEXT NOT NULL,
    created_at      TEXT NOT NULL,
    updated_at      TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (workbench_id) REFERENCES workbenches(id)
);
CREATE INDEX IF NOT EXISTS idx_agent_sessions_user_updated ON agent_sessions(user_id, updated_at DESC);
```

`migrations/sqlite/004_create_agent_runs_messages_events.sql`:

```sql
CREATE TABLE IF NOT EXISTS agent_runs (
    id             TEXT PRIMARY KEY,
    session_id     TEXT NOT NULL,
    parent_run_id  TEXT,
    status         TEXT NOT NULL,
    worker_id      TEXT,
    leased_until   TEXT,
    attempt_count  INTEGER NOT NULL DEFAULT 0,
    resume_cursor  TEXT,
    input_payload  TEXT,
    error_code     TEXT,
    error_message  TEXT,
    started_at     TEXT NOT NULL,
    finished_at    TEXT,
    FOREIGN KEY (session_id) REFERENCES agent_sessions(id),
    FOREIGN KEY (parent_run_id) REFERENCES agent_runs(id)
);
CREATE INDEX IF NOT EXISTS idx_agent_runs_session_started ON agent_runs(session_id, started_at DESC);

CREATE TABLE IF NOT EXISTS agent_messages (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    session_id   TEXT NOT NULL,
    run_id       TEXT,
    role         TEXT NOT NULL,
    message_kind TEXT NOT NULL,
    content      TEXT NOT NULL,
    created_at   TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES agent_sessions(id),
    FOREIGN KEY (run_id) REFERENCES agent_runs(id)
);
CREATE INDEX IF NOT EXISTS idx_agent_messages_session_created ON agent_messages(session_id, created_at ASC, id ASC);

CREATE TABLE IF NOT EXISTS agent_run_events (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    run_id     TEXT NOT NULL,
    seq        INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    payload    TEXT NOT NULL,
    created_at TEXT NOT NULL,
    FOREIGN KEY (run_id) REFERENCES agent_runs(id),
    UNIQUE (run_id, seq)
);
CREATE INDEX IF NOT EXISTS idx_agent_run_events_run_seq ON agent_run_events(run_id, seq ASC);
```

Create PostgreSQL equivalents with the same columns.

- [ ] **Step 7: Export module**

Update `src/lib.rs`: `pub mod agent;`
Create placeholder `src/agent/mod.rs`: `// module exports added in later tasks`

- [ ] **Step 8: Run tests and commit**

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test --test config_loading
PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_schema
```

Expected: PASS.

```bash
git add Cargo.toml src/lib.rs src/config.rs src/config/raw.rs src/config/sources.rs tests/config_loading.rs tests/agent_schema.rs migrations/ src/agent/mod.rs
git commit -m "feat: add agent config and 4-table schema"
```

---

### Task 2: Implement workbench-scoped session persistence and CRUD endpoints

**Files:**
- Create: `src/agent/models.rs`
- Create: `src/agent/sessions.rs`
- Create: `src/http/handlers/agent_handler.rs`
- Modify: `src/http/handlers/mod.rs`
- Modify: `src/http/routes.rs`
- Create: `tests/agent_sessions.rs`

**Produces:**
- `AgentSessionRow { id, user_id, workbench_id, title, latest_state, last_message_at, created_at, updated_at }`
- `AgentMessageRow { id, session_id, run_id, role, message_kind, content, created_at }`
- `AgentSessionDetail { session, messages }`
- Routes: `GET/POST /api/workbenches/{workbench_id}/agent-sessions`, `GET/PATCH/DELETE /api/workbenches/{workbench_id}/agent-sessions/{id}`

- [ ] **Step 1: Write failing integration tests**

Create `tests/agent_sessions.rs`:

```rust
mod common;
use axum::{body::Body, http::{Method, Request, StatusCode, header}};
use tower::ServiceExt;

fn auth_header() -> (header::HeaderName, String) {
    let token = issueflow::session::Session {
        user_id: 1, sub: "user-sub".into(), access_token: "gitlab-access-token".into(),
    }.sign("test-jwt-secret").unwrap();
    (header::AUTHORIZATION, format!("Bearer {token}"))
}

#[tokio::test]
async fn create_and_list_agent_sessions() {
    // first, create a workbench for workbench_id = 1 (or use existing fixture)
    let app = common::test_app(issueflow::config::Config::for_tests("secret")).await;
    let (auth_name, auth_value) = auth_header();

    let create = app.clone().oneshot(
        Request::builder().method(Method::POST).uri("/api/workbenches/1/agent-sessions")
            .header(auth_name.clone(), auth_value.clone())
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(r#"{}"#)).unwrap(),
    ).await.unwrap();
    assert_eq!(create.status(), StatusCode::CREATED);

    let list = app.oneshot(
        Request::builder().uri("/api/workbenches/1/agent-sessions")
            .header(auth_name, auth_value).body(Body::empty()).unwrap(),
    ).await.unwrap();
    assert_eq!(list.status(), StatusCode::OK);
}

#[tokio::test]
async fn unauthenticated_request_is_rejected() { /* assert 401 */ }

#[tokio::test]
async fn get_session_returns_messages_for_owner() { /* assert 200 with messages array */ }
```

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_sessions`
Expected: FAIL.

- [ ] **Step 2: Add model types**

Create `src/agent/models.rs`:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct AgentSessionRow {
    pub id: String,
    pub user_id: i64,
    pub workbench_id: i64,
    pub title: String,
    pub latest_state: Option<String>,
    pub last_message_at: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Serialize, sqlx::FromRow)]
pub struct AgentMessageRow {
    pub id: i64,
    pub session_id: String,
    pub run_id: Option<String>,
    pub role: String,
    pub message_kind: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct AgentSessionDetail {
    pub session: AgentSessionRow,
    pub messages: Vec<AgentMessageRow>,
}
```

- [ ] **Step 3: Implement session store**

Create `src/agent/sessions.rs`:

```rust
use uuid::Uuid;
use crate::{agent::models::{AgentSessionRow, AgentMessageRow}, db::DbPool};

pub async fn create_session(pool: &DbPool, user_id: i64, workbench_id: i64) -> Result<AgentSessionRow, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    sqlx::query_as("INSERT INTO agent_sessions (id, user_id, workbench_id, title, last_message_at, created_at, updated_at) VALUES (?, ?, ?, 'New Session', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) RETURNING id, user_id, workbench_id, title, latest_state, last_message_at, created_at, updated_at")
        .bind(&id).bind(user_id).bind(workbench_id).fetch_one(pool).await
}

pub async fn list_sessions(pool: &DbPool, user_id: i64, workbench_id: i64) -> Result<Vec<AgentSessionRow>, sqlx::Error> { /* WHERE user_id = ? AND workbench_id = ? */ }
pub async fn get_session(pool: &DbPool, user_id: i64, id: &str) -> Result<AgentSessionRow, sqlx::Error> { /* WHERE user_id = ? AND id = ? */ }
pub async fn rename_session(pool: &DbPool, user_id: i64, id: &str, title: &str) -> Result<(), sqlx::Error> { /* UPDATE ... WHERE user_id = ? AND id = ? */ }
pub async fn delete_session(pool: &DbPool, user_id: i64, id: &str) -> Result<(), sqlx::Error> { /* DELETE ... WHERE user_id = ? AND id = ? */ }
pub async fn list_messages(pool: &DbPool, session_id: &str) -> Result<Vec<AgentMessageRow>, sqlx::Error> { /* WHERE session_id = ? ORDER BY created_at, id */ }
```

Every query includes `user_id` for ownership checks.

- [ ] **Step 4: Implement handlers and routes**

Create `src/http/handlers/agent_handler.rs`:

```rust
pub async fn list_sessions(State(state): State<AppState>, session: Session, Path(workbench_id): Path<i64>) -> Result<Json<Vec<AgentSessionRow>>, AppError>
pub async fn create_session(State(state): State<AppState>, session: Session, Path(workbench_id): Path<i64>) -> Result<(StatusCode, Json<AgentSessionRow>), AppError>
pub async fn get_session(State(state): State<AppState>, session: Session, Path((workbench_id, id)): Path<(i64, String)>) -> Result<Json<AgentSessionDetail>, AppError>
pub async fn rename_session(...)
pub async fn delete_session(...)
```

Update `src/http/routes.rs`:

```rust
.route("/api/workbenches/{workbench_id}/agent-sessions",
    get(agent_handler::list_sessions).post(agent_handler::create_session))
.route("/api/workbenches/{workbench_id}/agent-sessions/{id}",
    get(agent_handler::get_session).patch(agent_handler::rename_session).delete(agent_handler::delete_session))
```

Add `patch` import. Update `src/http/handlers/mod.rs`.

- [ ] **Step 5: Run tests and commit**

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_sessions
```

Expected: PASS.

```bash
git add src/agent/models.rs src/agent/sessions.rs src/http/handlers/agent_handler.rs src/http/handlers/mod.rs src/http/routes.rs tests/agent_sessions.rs
git commit -m "feat: add workbench-scoped agent session CRUD"
```

---

### Task 3: AG-UI event types + durable run create/subscribe endpoints

**Files:**
- Create: `src/agent/events.rs`
- Modify: `src/agent/models.rs`
- Create: `src/agent/runs.rs`
- Modify: `src/http/handlers/agent_handler.rs`
- Modify: `src/http/routes.rs`
- Create: `tests/agent_run.rs`

**Produces:**
- `AgUiEvent` enum with `type`-tagged serde
- `RunAgentRequest { thread_id, workbench_id, messages }`
- `CreateRunResponse { run_id, thread_id, status }`
- `POST /api/agent/runs` (durable run creation)
- `GET /api/agent/runs/{run_id}/events` (SSE replay + live)

- [ ] **Step 1: Write failing tests**

Create `tests/agent_run.rs`:

```rust
mod common;
use axum::{body::Body, http::{Method, Request, StatusCode, header}};
use tower::ServiceExt;

#[tokio::test]
async fn run_creation_requires_auth() {
    let app = common::test_app(issueflow::config::Config::for_tests("secret")).await;
    let response = app.oneshot(Request::builder().method(Method::POST).uri("/api/agent/runs")
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(r#"{"threadId":"x","workbenchId":1,"messages":[]}"#)).unwrap()).await.unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_run_returns_durable_metadata() {
    // create session first, then POST /api/agent/runs, assert 200 with { runId, threadId, status }
}

#[tokio::test]
async fn subscribe_events_returns_event_stream() {
    // create run, then GET /api/agent/runs/{runId}/events?after_seq=0, assert text/event-stream
}
```

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_run`
Expected: FAIL.

- [ ] **Step 2: Define AG-UI event types**

Create `src/agent/events.rs`:

```rust
use axum::response::sse::Event;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgUiEvent {
    RunStarted { threadId: String, runId: String },
    RunFinished { threadId: String, runId: String },
    RunError { message: String, code: Option<String> },
    StepStarted { stepName: String },
    StepFinished { stepName: String },
    TextMessageStart { messageId: String, role: String },
    TextMessageContent { messageId: String, delta: String },
    TextMessageEnd { messageId: String },
    ToolCallStart { toolCallId: String, toolCallName: String, parentMessageId: Option<String> },
    ToolCallArgs { toolCallId: String, delta: String },
    ToolCallEnd { toolCallId: String },
    ToolCallResult { messageId: String, toolCallId: String, content: serde_json::Value, role: String },
    StateSnapshot { snapshot: serde_json::Value },
    MessagesSnapshot { messages: Vec<serde_json::Value> },
    Custom { name: String, value: serde_json::Value },
}

pub fn encode_sse(event: &AgUiEvent) -> Event {
    Event::default().json_data(event).unwrap()
}
```

- [ ] **Step 3: Add run and event store functions**

Create `src/agent/runs.rs`:

```rust
use uuid::Uuid;
use crate::{agent::models::AgentRunRow, db::DbPool};

pub async fn create_run(pool: &DbPool, session_id: &str, parent_run_id: Option<&str>, input_payload: &str) -> Result<AgentRunRow, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    sqlx::query_as("INSERT INTO agent_runs (id, session_id, parent_run_id, status, attempt_count, input_payload, started_at) VALUES (?, ?, ?, 'queued', 0, ?, CURRENT_TIMESTAMP) RETURNING ...")
        .bind(&id).bind(session_id).bind(parent_run_id).bind(input_payload).fetch_one(pool).await
}

pub async fn append_event(pool: &DbPool, run_id: &str, seq: i64, event_type: &str, payload: &str) -> Result<(), sqlx::Error> { /* INSERT INTO agent_run_events */ }
pub async fn list_events_after(pool: &DbPool, run_id: &str, after_seq: i64) -> Result<Vec<(i64, String, String)>, sqlx::Error> { /* SELECT seq, event_type, payload WHERE run_id = ? AND seq > ? ORDER BY seq */ }
pub async fn claim_next(pool: &DbPool, worker_id: &str, lease_until: &str) -> Result<Option<AgentRunRow>, sqlx::Error> { /* UPDATE ... SET worker_id=?, leased_until=?, status='running', attempt_count=attempt_count+1 WHERE status='queued' */ }
pub async fn mark_completed(pool: &DbPool, run_id: &str) -> Result<(), sqlx::Error>
pub async fn mark_failed(pool: &DbPool, run_id: &str, error: &str) -> Result<(), sqlx::Error>
pub async fn mark_waiting_input(pool: &DbPool, run_id: &str) -> Result<(), sqlx::Error>
pub async fn reclaim_stale(pool: &DbPool) -> Result<u64, sqlx::Error> { /* UPDATE ... SET worker_id=NULL, status='queued' WHERE status='running' AND leased_until < CURRENT_TIMESTAMP */ }
```

- [ ] **Step 4: Add request DTOs and create-run + subscribe handlers**

Append to `src/agent/models.rs`:

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunAgentRequest {
    pub thread_id: String,
    pub workbench_id: i64,
    pub messages: Vec<serde_json::Value>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateRunResponse {
    pub run_id: String,
    pub thread_id: String,
    pub status: String,
}

#[derive(Debug, Deserialize)]
pub struct RunEventsQuery {
    pub after_seq: Option<i64>,
}
```

Add `create_run` and `subscribe_run_events` to `src/http/handlers/agent_handler.rs`:

```rust
pub async fn create_run(...) -> Result<(StatusCode, Json<CreateRunResponse>), AppError> {
    // validate session ownership, persist user message, create queued run, return metadata
}
pub async fn subscribe_run_events(...) -> Result<Sse<impl Stream<Item = Result<Event, Infallible>>>, AppError> {
    // replay events after after_seq, then fan out live events
}
```

Add routes:

```rust
.route("/api/agent/runs", post(agent_handler::create_run))
.route("/api/agent/runs/{run_id}/events", get(agent_handler::subscribe_run_events))
```

- [ ] **Step 5: Run tests and commit**

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_run
```

Expected: PASS (placeholder handlers return minimal streams).

```bash
git add src/agent/events.rs src/agent/models.rs src/agent/runs.rs src/http/handlers/agent_handler.rs src/http/routes.rs tests/agent_run.rs
git commit -m "feat: add ag-ui events and durable run endpoints"
```

---

### Task 4: GitLab tool definitions and execution layer

**Files:**
- Create: `src/agent/gitlab_tools.rs`
- Create: `src/gitlab/wiki.rs`
- Create: `src/gitlab/repository.rs`
- Modify: `src/gitlab/mod.rs`
- Create: `tests/agent_tools.rs`

**Produces:**
- `tool_definitions() -> Vec<serde_json::Value>` (OpenAI function-calling format)
- `execute_tool(name, arguments, state, session) -> Result<serde_json::Value, AppError>`
- `is_allowed_repo_path(path) -> bool`
- Tools: `create_issue`, `update_issue`, `delete_issue`, `list_issues`, `get_issue`, `list_wiki_pages`, `get_wiki_page`, `get_repo_file`

- [ ] **Step 1: Write failing tools test**

Create `tests/agent_tools.rs`:

```rust
#[test]
fn tool_definitions_expose_expected_names() {
    let defs = issueflow::agent::gitlab_tools::tool_definitions();
    let names: Vec<&str> = defs.iter().map(|v| v["function"]["name"].as_str().unwrap()).collect();
    assert!(names.contains(&"create_issue"));
    assert!(names.contains(&"get_repo_file"));
    assert!(names.contains(&"list_wiki_pages"));
}

#[test]
fn get_repo_file_allowlist_rejects_untrusted_path() {
    assert!(!issueflow::agent::gitlab_tools::is_allowed_repo_path("src/secrets.txt"));
    assert!(issueflow::agent::gitlab_tools::is_allowed_repo_path("AGENTS.md"));
}
```

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_tools`
Expected: FAIL.

- [ ] **Step 2: Add GitLab helper modules**

Create `src/gitlab/wiki.rs` and `src/gitlab/repository.rs` using `reqwest` + bearer token (same pattern as existing `src/gitlab/projects.rs`). Update `src/gitlab/mod.rs`.

- [ ] **Step 3: Implement tool definitions and dispatcher**

Create `src/agent/gitlab_tools.rs`:

```rust
pub fn tool_definitions() -> Vec<serde_json::Value> { /* 8 tools in OpenAI function format */ }
pub fn is_allowed_repo_path(path: &str) -> bool { matches!(path, "AGENTS.md" | "README.md" | "docs/CONFIG.md") }

pub async fn execute_tool(
    name: &str, arguments: serde_json::Value,
    state: &crate::http::routes::AppState, session: &crate::session::Session,
) -> Result<serde_json::Value, crate::error::AppError>
```

Use server `git.token` for issue writes, user `session.access_token` for wiki/repo reads.

- [ ] **Step 4: Run tests and commit**

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_tools
```

Expected: PASS.

```bash
git add src/agent/gitlab_tools.rs src/gitlab/mod.rs src/gitlab/wiki.rs src/gitlab/repository.rs tests/agent_tools.rs
git commit -m "feat: add agent gitlab tools"
```

---

### Task 5: OpenAI client, orchestrator, and worker loop

**Files:**
- Create: `src/agent/openai.rs`
- Create: `src/agent/orchestrator.rs`
- Create: `src/agent/worker.rs`
- Modify: `src/main.rs`
- Modify: `src/agent/sessions.rs`
- Modify: `src/http/handlers/agent_handler.rs`
- Modify: `tests/agent_run.rs`

**Produces:**
- `stream_chat(config, payload) -> impl Stream<Item = Result<ProviderDelta, AppError>>`
- `process_run(state, run, worker_id) -> Result<(), AppError>`
- `run_once(state, worker_id) -> Result<bool, AppError>` (claim + process one run)
- Background worker started from `main.rs`

- [ ] **Step 1: Expand run test with mock model provider**

Add to `tests/agent_run.rs` a test that starts a mock HTTP server at a known URL, configures `AGENT_OPENAI_BASE_URL` to point to it, creates a run, and asserts the event stream contains expected AG-UI frames.

- [ ] **Step 2: Implement OpenAI streaming client**

Create `src/agent/openai.rs`:

```rust
#[derive(Debug, Serialize)]
pub struct OpenAiChatRequest { pub model: String, pub stream: bool, pub messages: Vec<serde_json::Value>, pub tools: Vec<serde_json::Value> }

#[derive(Debug)]
pub enum ProviderDelta { Text(String), ToolStart { id: String, name: String }, ToolArgs { id: String, delta: String }, ToolEnd { id: String }, Done }

pub async fn stream_chat(config: &AgentConfig, payload: OpenAiChatRequest) -> impl Stream<Item = Result<ProviderDelta, AppError>>
```

POST `{base_url}/chat/completions`, read SSE response lines, translate to `ProviderDelta`.

- [ ] **Step 3: Implement orchestrator**

Create `src/agent/orchestrator.rs`:

```rust
pub async fn process_run(state: AppState, run: AgentRunRow, worker_id: &str) -> Result<(), AppError>
```

Behavior: load session + messages + resume_cursor, emit RunStarted + MessagesSnapshot via append_event, call model client, translate deltas into TextMessage*/ToolCall*, persist each event, execute tools, loop up to max_tool_rounds, mark completed/failed/waiting_input.

- [ ] **Step 4: Implement worker loop**

Create `src/agent/worker.rs`:

```rust
pub async fn run_once(state: AppState, worker_id: &str) -> Result<bool, AppError> {
    // reclaim stale, claim next queued, process, return whether work was done
}

pub async fn run_loop(state: AppState) -> Result<(), AppError> {
    let worker_id = Uuid::new_v4().to_string();
    loop {
        let worked = run_once(state.clone(), &worker_id).await?;
        if !worked { tokio::time::sleep(Duration::from_secs(1)).await; }
    }
}
```

Start from `src/main.rs` after server bind:

```rust
let worker_state = app_state.clone();
tokio::spawn(async { crate::agent::worker::run_loop(worker_state).await.unwrap(); });
```

- [ ] **Step 5: Update subscribe handler to serve live events**

In `agent_handler.rs`, `subscribe_run_events` replays from `agent_run_events` then maintains an in-process subscriber list. When `process_run` emits an event, it fans out to all live subscribers.

- [ ] **Step 6: Run tests and commit**

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_run
```

Expected: PASS.

```bash
git add src/agent/openai.rs src/agent/orchestrator.rs src/agent/worker.rs src/main.rs src/agent/sessions.rs src/http/handlers/agent_handler.rs tests/agent_run.rs
git commit -m "feat: add agent worker and orchestrator"
```

---

### Task 6: Frontend — agent page, stream reader, A2UI bridge

**Files:**
- Modify: `web/package.json`
- Modify: `web/src/router/index.ts`
- Modify: `web/src/main.ts`
- Create: `web/src/api/agent.api.ts`
- Create: `web/src/stores/agent.store.ts`
- Create: `web/src/composables/useAgentRun.ts`
- Create: `web/src/composables/useA2UIBridge.ts`
- Create: `web/src/views/AgentView.vue`
- Create: `web/src/components/agent/SessionList.vue`
- Create: `web/src/components/agent/ChatPanel.vue`
- Create: `web/src/components/agent/ChatMessages.vue`
- Create: `web/src/components/agent/ChatInput.vue`
- Create: `web/src/components/agent/ToolCallCard.vue`
- Create: `web/src/components/agent/A2UISurfaceHost.vue`
- Create: `web/src/tests/agent.store.spec.ts`
- Create: `web/src/tests/useAgentRun.spec.ts`

- [ ] **Step 1: Add failing unit tests**

`agent.store.spec.ts`: tests message append, delta assembly, tool call state.
`useAgentRun.spec.ts`: mocked fetch for run creation + SSE reading.

Run: `cd web && npm test -- --run agent.store useAgentRun`
Expected: FAIL.

- [ ] **Step 2: Add a2ui-vue dependency**

```bash
cd web && npm install a2ui-vue@^0.9.3
```

Register in `web/src/main.ts`:

```ts
import { provideA2UI, DEFAULT_CATALOG, defaultTheme } from "a2ui-vue";
import "a2ui-vue/dist/a2ui-vue.css";
provideA2UI({ app, catalog: DEFAULT_CATALOG, theme: defaultTheme });
```

- [ ] **Step 3: Add agent API client**

Create `web/src/api/agent.api.ts`:

```ts
import { apiFetch } from "@/utils/api";
import { authHeaders } from "./helpers";

export async function listSessions(workbenchId: number) { /* GET /api/workbenches/{id}/agent-sessions */ }
export async function createSession(workbenchId: number) { /* POST /api/workbenches/{id}/agent-sessions */ }
export async function getSession(workbenchId: number, id: string) { /* GET /api/workbenches/{id}/agent-sessions/{id} */ }
export async function deleteSession(workbenchId: number, id: string) { /* DELETE */ }
export async function renameSession(workbenchId: number, id: string, title: string) { /* PATCH */ }
export async function createRun(body: unknown) { /* POST /api/agent/runs */ }
export function subscribeRunEvents(runId: string, afterSeq = 0) { /* GET /api/agent/runs/{id}/events, raw fetch for streaming */ }
```

- [ ] **Step 4: Add agent store**

Create `web/src/stores/agent.store.ts` with defineStore containing: sessions list, active session, normalized messages (with TTL-ordered roles and message_kind), active runs tracking, and methods for beginAssistantMessage, appendAssistantDelta, finishAssistantMessage, upsertToolCall, and appendCustomEvent.

- [ ] **Step 5: Implement composables**

`useAgentRun.ts`: POST `/api/agent/runs` to create run, then `subscribeRunEvents` on the returned runId. Read `ReadableStream`, split SSE frames by double newline, dispatch `event:`/`data:` lines to store mutations. On reconnect after refresh, first load session detail from the session detail endpoint, then resubscribe.

`useA2UIBridge.ts`:

```ts
import { useMessageProcessor } from "a2ui-vue";

export function useA2UIBridge() {
  const processor = useMessageProcessor();
  function handleCustom(value: any) {
    if (value?.kind === "a2ui_render") { processor.processMessages([value.payload]); }
  }
  function buildSubmit(surfaceId: string, payload: Record<string, unknown>) {
    return { role: "user", content: { kind: "a2ui_submit", surface_id: surfaceId, payload } };
  }
  return { handleCustom, buildSubmit };
}
```

- [ ] **Step 6: Add route and page shell**

Add to `web/src/router/index.ts`:

```ts
{ path: "/workbench/agent", name: "workbench-agent", component: () => import("@/views/AgentView.vue") }
```

Create `AgentView.vue` with left `SessionList` + right `ChatPanel` layout.
Create `SessionList.vue`, `ChatPanel.vue`, `ChatMessages.vue`, `ChatInput.vue`, `ToolCallCard.vue`, `A2UISurfaceHost.vue`.

- [ ] **Step 7: Run tests and build**

```bash
cd web && npm test -- --run && npm run build
```

Expected: PASS.

```bash
git add web/
git commit -m "feat: add agent session frontend"
```

---

### Task 7: Full verification

**Files:**
- Modify: `AGENTS.md`
- Modify: `docs/local-development.md`
- Modify: `docs/CONFIG.md`
- Modify: `docs/CONFIG_zh.md`

- [ ] **Step 1: Update docs**

Add agent config env vars to `docs/local-development.md` and `docs/CONFIG.md`. Verify `AGENTS.md` architecture rules still correct.

- [ ] **Step 2: Full CI verification**

```bash
PATH="$HOME/.cargo/bin:$PATH" cargo fmt -- --check
PATH="$HOME/.cargo/bin:$PATH" cargo clippy -- -D warnings
PATH="$HOME/.cargo/bin:$PATH" cargo test
cd web && npm test -- --run && npm run build
```

Expected: all PASS.

- [ ] **Step 3: Commit**

```bash
git add AGENTS.md docs/
git commit -m "docs: add agent session development and config notes"
```
