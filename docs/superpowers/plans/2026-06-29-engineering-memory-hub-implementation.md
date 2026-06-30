# Engineering Memory Hub Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the MVP engineering memory loop for GitLab issues: read issue context, persist engineering memory, create pending actions, confirm through the existing confirmation flow, and update the GitLab issue description with the confirmed result.

**Architecture:** Add two new persistence units, `engineering_memory` and `pending_actions`, as project-level domain storage. Reuse the existing workbench, agent session, and agent run infrastructure for interaction and execution, while keeping GitLab write operations bound to explicit user confirmation and current user permissions.

**Tech Stack:** Rust 2024, Axum, SQLx AnyPool with SQLite/Postgres migrations, reqwest GitLab REST client, existing issueflow session/auth stack, existing agent run/event infrastructure.

---

## File Map

### Existing files to modify

- `migrations/sqlite/`
- `migrations/postgres/`
- `src/db/mod.rs`
- `src/gitlab/issues.rs`
- `src/gitlab/mod.rs`
- `src/http/routes.rs`
- `src/http/handlers/mod.rs`
- `src/http/handlers/confirm_handler.rs`
- `src/http/handlers/agent_handler.rs`
- `src/agent/models.rs`
- `src/agent/runs.rs`
- `src/agent/orchestrator.rs`
- `src/lib.rs`
- `tests/common/mod.rs`
- `tests/webhook_handler.rs`

### New files to create

- `src/memory/mod.rs`
- `src/memory/models.rs`
- `src/memory/store.rs`
- `src/actions/mod.rs`
- `src/actions/models.rs`
- `src/actions/store.rs`
- `src/http/handlers/memory_handler.rs`
- `tests/engineering_memory_handler.rs`
- `tests/pending_actions_handler.rs`
- `tests/gitlab_issue_update.rs`

### Responsibility split

- `src/memory/*`: engineering memory row types, JSON payload types, persistence helpers
- `src/actions/*`: pending action row types, state updates, persistence helpers
- `src/gitlab/issues.rs`: GitLab issue update capability
- `src/http/handlers/memory_handler.rs`: HTTP endpoints for memory refresh and pending action confirmation
- `src/agent/*`: enqueue and execute pending actions using existing run infrastructure
- `tests/*`: integration tests for HTTP and GitLab behavior

### Task 1: Add the failing persistence tests for engineering memory

**Files:**
- Create: `tests/engineering_memory_handler.rs`
- Modify: `tests/common/mod.rs`
- Test: `tests/engineering_memory_handler.rs`

- [ ] **Step 1: Write the failing integration test for creating or replacing an engineering memory snapshot**

```rust
#[tokio::test]
async fn engineering_memory_refresh_upserts_latest_snapshot() {
    let pool = common::test_pool().await;
    let app = common::test_app_with_pool(Config::for_tests("expected-token"), pool.clone()).await;
    let token = common::auth_token("test-jwt-secret", 7, "user-sub", "user-token");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches/1/memory/refresh")
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "projectId": 123,
                        "artifactType": "issue",
                        "artifactId": "77",
                        "inputText": "Need an export button",
                        "inputContext": {"source":"agent-session"},
                        "spec": {"summary":"Add export","acceptance_criteria":[],"success_conditions":[],"boundary_conditions":[],"open_questions":[]},
                        "validationSuggestions": {"happy_path":[],"failure_path":[],"edge_cases":[],"non_goals":[]},
                        "riskNotes": [],
                        "evaluationSummary": {"status":"unknown","summary":"","coverage_notes":[],"missing_cases":[]}
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

- [ ] **Step 2: Add a reusable authenticated test token helper**

```rust
pub fn auth_token(jwt_secret: &str, user_id: i64, sub: &str, access_token: &str) -> String {
    let claims = issueflow::session::build_claims(user_id, sub, access_token);
    issueflow::session::sign_token(&claims, jwt_secret).unwrap()
}
```

- [ ] **Step 3: Run the test to verify it fails because the route does not exist**

Run: `cargo test engineering_memory_refresh_upserts_latest_snapshot -- --exact`

Expected: FAIL with `404 Not Found` or missing route assertions.

- [ ] **Step 4: Commit the failing test**

```bash
git add tests/common/mod.rs tests/engineering_memory_handler.rs
git commit -m "test: add failing engineering memory refresh flow"
```

### Task 2: Add engineering memory schema and storage

**Files:**
- Create: `src/memory/mod.rs`
- Create: `src/memory/models.rs`
- Create: `src/memory/store.rs`
- Modify: `migrations/sqlite/20260627000005_create_engineering_memory.sql`
- Modify: `migrations/postgres/20260627000005_create_engineering_memory.sql`
- Modify: `src/lib.rs`
- Test: `tests/engineering_memory_handler.rs`

- [ ] **Step 1: Add the SQLite migration**

```sql
CREATE TABLE IF NOT EXISTS engineering_memory (
    id TEXT PRIMARY KEY,
    project_id INTEGER NOT NULL,
    artifact_type TEXT NOT NULL,
    artifact_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    revision INTEGER NOT NULL DEFAULT 1,
    updated_by_user_id INTEGER,
    input_text TEXT NOT NULL,
    input_context TEXT NOT NULL,
    spec TEXT NOT NULL,
    validation_suggestions TEXT NOT NULL,
    risk_notes TEXT NOT NULL,
    evaluation_summary TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id, artifact_type, artifact_id)
);
```

- [ ] **Step 2: Add the Postgres migration**

```sql
CREATE TABLE IF NOT EXISTS engineering_memory (
    id TEXT PRIMARY KEY,
    project_id BIGINT NOT NULL,
    artifact_type TEXT NOT NULL,
    artifact_id TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    revision INTEGER NOT NULL DEFAULT 1,
    updated_by_user_id BIGINT,
    input_text TEXT NOT NULL,
    input_context TEXT NOT NULL,
    spec TEXT NOT NULL,
    validation_suggestions TEXT NOT NULL,
    risk_notes TEXT NOT NULL,
    evaluation_summary TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(project_id, artifact_type, artifact_id)
);
```

- [ ] **Step 3: Add memory row and payload models**

```rust
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct EngineeringMemoryRow {
    pub id: String,
    pub project_id: i64,
    pub artifact_type: String,
    pub artifact_id: String,
    pub status: String,
    pub revision: i64,
    pub updated_by_user_id: Option<i64>,
    pub input_text: String,
    pub input_context: String,
    pub spec: String,
    pub validation_suggestions: String,
    pub risk_notes: String,
    pub evaluation_summary: String,
    pub created_at: String,
    pub updated_at: String,
}
```

- [ ] **Step 4: Add an upsert helper that increments revision**

```rust
pub async fn upsert_engineering_memory(
    pool: &DbPool,
    input: &UpsertEngineeringMemoryInput,
) -> Result<EngineeringMemoryRow, sqlx::Error> {
    sqlx::query_as(
        "INSERT INTO engineering_memory (
            id, project_id, artifact_type, artifact_id, status, revision, updated_by_user_id,
            input_text, input_context, spec, validation_suggestions, risk_notes, evaluation_summary
         ) VALUES (?, ?, ?, ?, ?, 1, ?, ?, ?, ?, ?, ?, ?)
         ON CONFLICT(project_id, artifact_type, artifact_id)
         DO UPDATE SET
            status = excluded.status,
            revision = engineering_memory.revision + 1,
            updated_by_user_id = excluded.updated_by_user_id,
            input_text = excluded.input_text,
            input_context = excluded.input_context,
            spec = excluded.spec,
            validation_suggestions = excluded.validation_suggestions,
            risk_notes = excluded.risk_notes,
            evaluation_summary = excluded.evaluation_summary,
            updated_at = CURRENT_TIMESTAMP
         RETURNING id, project_id, artifact_type, artifact_id, status, revision, updated_by_user_id,
            input_text, input_context, spec, validation_suggestions, risk_notes, evaluation_summary,
            created_at, updated_at",
    )
    .bind(&input.id)
    .bind(input.project_id)
    .bind(&input.artifact_type)
    .bind(&input.artifact_id)
    .bind(&input.status)
    .bind(input.updated_by_user_id)
    .bind(&input.input_text)
    .bind(&input.input_context)
    .bind(&input.spec)
    .bind(&input.validation_suggestions)
    .bind(&input.risk_notes)
    .bind(&input.evaluation_summary)
    .fetch_one(pool)
    .await
}
```

- [ ] **Step 5: Export the new module**

```rust
pub mod memory;
```

- [ ] **Step 6: Run the migration-backed test to verify schema wiring still fails only at the missing handler**

Run: `cargo test engineering_memory_refresh_upserts_latest_snapshot -- --exact`

Expected: FAIL with a handler or routing failure, not a migration failure.

- [ ] **Step 7: Commit the storage layer**

```bash
git add migrations/sqlite/20260627000005_create_engineering_memory.sql migrations/postgres/20260627000005_create_engineering_memory.sql src/lib.rs src/memory
git commit -m "feat: add engineering memory storage"
```

### Task 3: Expose engineering memory refresh over HTTP

**Files:**
- Create: `src/http/handlers/memory_handler.rs`
- Modify: `src/http/handlers/mod.rs`
- Modify: `src/http/routes.rs`
- Test: `tests/engineering_memory_handler.rs`

- [ ] **Step 1: Add the failing authorization test**

```rust
#[tokio::test]
async fn engineering_memory_refresh_requires_authentication() {
    let app = common::test_app(Config::for_tests("expected-token")).await;

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches/1/memory/refresh")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
```

- [ ] **Step 2: Add the refresh request/response handler**

```rust
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshEngineeringMemoryRequest {
    pub project_id: i64,
    pub artifact_type: String,
    pub artifact_id: String,
    pub input_text: String,
    pub input_context: serde_json::Value,
    pub spec: serde_json::Value,
    pub validation_suggestions: serde_json::Value,
    pub risk_notes: serde_json::Value,
    pub evaluation_summary: serde_json::Value,
}

pub async fn refresh_engineering_memory(
    State(state): State<AppState>,
    session: Session,
    Path(workbench_id): Path<i64>,
    Json(payload): Json<RefreshEngineeringMemoryRequest>,
) -> Result<Json<EngineeringMemoryRow>, AppError> {
    let _ = workbench_id;
    let row = memory::store::upsert_engineering_memory(
        &state.pool,
        &memory::models::UpsertEngineeringMemoryInput::from_request(session.user_id, payload),
    )
    .await?;

    Ok(Json(row))
}
```

- [ ] **Step 3: Wire the route**

```rust
.route(
    "/api/workbenches/{workbench_id}/memory/refresh",
    post(memory_handler::refresh_engineering_memory),
)
```

- [ ] **Step 4: Run the tests to verify they now pass**

Run: `cargo test engineering_memory_refresh -- --nocapture`

Expected: PASS for both refresh success and auth rejection tests.

- [ ] **Step 5: Commit the handler**

```bash
git add src/http/handlers/memory_handler.rs src/http/handlers/mod.rs src/http/routes.rs tests/engineering_memory_handler.rs
git commit -m "feat: add engineering memory refresh endpoint"
```

### Task 4: Add failing tests and storage for pending actions

**Files:**
- Create: `src/actions/mod.rs`
- Create: `src/actions/models.rs`
- Create: `src/actions/store.rs`
- Create: `tests/pending_actions_handler.rs`
- Modify: `migrations/sqlite/20260627000006_create_pending_actions.sql`
- Modify: `migrations/postgres/20260627000006_create_pending_actions.sql`
- Modify: `src/lib.rs`
- Test: `tests/pending_actions_handler.rs`

- [ ] **Step 1: Write the failing test for creating a pending action**

```rust
#[tokio::test]
async fn create_pending_action_persists_issue_update_draft() {
    let pool = common::test_pool().await;
    let app = common::test_app_with_pool(Config::for_tests("expected-token"), pool).await;
    let token = common::auth_token("test-jwt-secret", 7, "user-sub", "user-token");

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/workbenches/1/pending-actions")
                .header("authorization", format!("Bearer {token}"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "projectId": 123,
                        "artifactType": "issue",
                        "artifactId": "77",
                        "actionType": "update_gitlab_issue",
                        "payload": {
                            "targetField": "description",
                            "updateMode": "replace_full",
                            "renderedContent": "new issue body"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}
```

- [ ] **Step 2: Add the SQLite migration**

```sql
CREATE TABLE IF NOT EXISTS pending_actions (
    id TEXT PRIMARY KEY,
    workbench_id INTEGER NOT NULL,
    project_id INTEGER NOT NULL,
    artifact_type TEXT NOT NULL,
    artifact_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    source_session_id TEXT,
    source_run_id TEXT,
    created_by_user_id INTEGER,
    assigned_user_id INTEGER,
    confirmed_by_user_id INTEGER,
    executed_run_id TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

- [ ] **Step 3: Add the Postgres migration**

```sql
CREATE TABLE IF NOT EXISTS pending_actions (
    id TEXT PRIMARY KEY,
    workbench_id BIGINT NOT NULL,
    project_id BIGINT NOT NULL,
    artifact_type TEXT NOT NULL,
    artifact_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    source_session_id TEXT,
    source_run_id TEXT,
    created_by_user_id BIGINT,
    assigned_user_id BIGINT,
    confirmed_by_user_id BIGINT,
    executed_run_id TEXT,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

- [ ] **Step 4: Add the pending action row model and create helper**

```rust
#[derive(Clone, Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct PendingActionRow {
    pub id: String,
    pub workbench_id: i64,
    pub project_id: i64,
    pub artifact_type: String,
    pub artifact_id: String,
    pub action_type: String,
    pub status: String,
    pub payload: String,
    pub source_session_id: Option<String>,
    pub source_run_id: Option<String>,
    pub created_by_user_id: Option<i64>,
    pub assigned_user_id: Option<i64>,
    pub confirmed_by_user_id: Option<i64>,
    pub executed_run_id: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
```

- [ ] **Step 5: Run the failing test**

Run: `cargo test create_pending_action_persists_issue_update_draft -- --exact`

Expected: FAIL with missing route or handler.

- [ ] **Step 6: Commit the failing action test and storage layer**

```bash
git add tests/pending_actions_handler.rs migrations/sqlite/20260627000006_create_pending_actions.sql migrations/postgres/20260627000006_create_pending_actions.sql src/lib.rs src/actions
git commit -m "feat: add pending action storage scaffolding"
```

### Task 5: Add pending action HTTP endpoints and confirmation state changes

**Files:**
- Create: `tests/pending_actions_handler.rs`
- Modify: `src/http/handlers/mod.rs`
- Modify: `src/http/routes.rs`
- Modify: `src/http/handlers/confirm_handler.rs`
- Modify: `src/http/handlers/memory_handler.rs`
- Test: `tests/pending_actions_handler.rs`

- [ ] **Step 1: Add the failing test for confirming a pending action**

```rust
#[tokio::test]
async fn confirm_pending_action_marks_action_confirmed() {
    let pool = common::test_pool().await;
    let app = common::test_app_with_pool(Config::for_tests("expected-token"), pool.clone()).await;

    let action_id = issueflow::actions::store::insert_pending_action(
        &pool,
        &issueflow::actions::models::CreatePendingActionInput {
            workbench_id: 1,
            project_id: 123,
            artifact_type: "issue".into(),
            artifact_id: "77".into(),
            action_type: "update_gitlab_issue".into(),
            payload: r#"{"renderedContent":"new body"}"#.into(),
            created_by_user_id: Some(7),
            source_session_id: None,
            source_run_id: None,
        },
    )
    .await
    .unwrap()
    .id;

    let token = common::auth_token("test-jwt-secret", 7, "user-sub", "user-token");
    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(&format!("/api/pending-actions/{action_id}/confirm"))
                .header("authorization", format!("Bearer {token}"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
```

- [ ] **Step 2: Add create/list/confirm handlers**

```rust
pub async fn create_pending_action(...) -> Result<(StatusCode, Json<PendingActionRow>), AppError> { ... }

pub async fn list_pending_actions(...) -> Result<Json<Vec<PendingActionRow>>, AppError> { ... }

pub async fn confirm_pending_action(...) -> Result<Json<PendingActionRow>, AppError> { ... }
```

- [ ] **Step 3: Reuse the existing confirm redirect semantics**

```rust
pub async fn confirm_plan(Path(token): Path<String>) -> Redirect {
    Redirect::to(&format!("/workbench?confirm={token}"))
}
```

Implementation note:

- keep the redirect route untouched
- make `pending_actions` confirmation use the same workbench confirmation entry point pattern
- store `confirmed_by_user_id` and transition `status` from `pending` to `confirmed`

- [ ] **Step 4: Wire routes**

```rust
.route(
    "/api/workbenches/{workbench_id}/pending-actions",
    get(memory_handler::list_pending_actions).post(memory_handler::create_pending_action),
)
.route(
    "/api/pending-actions/{id}/confirm",
    post(memory_handler::confirm_pending_action),
)
```

- [ ] **Step 5: Run tests**

Run: `cargo test pending_action -- --nocapture`

Expected: PASS for create, auth rejection, and confirm transition tests.

- [ ] **Step 6: Commit the action handlers**

```bash
git add src/http/handlers/memory_handler.rs src/http/handlers/mod.rs src/http/routes.rs src/http/handlers/confirm_handler.rs tests/pending_actions_handler.rs
git commit -m "feat: add pending action confirmation flow"
```

### Task 6: Add GitLab issue description update support

**Files:**
- Modify: `src/gitlab/issues.rs`
- Create: `tests/gitlab_issue_update.rs`
- Test: `tests/gitlab_issue_update.rs`

- [ ] **Step 1: Add the failing test for issue update payload encoding**

```rust
#[test]
fn update_issue_request_encodes_description_field() {
    let body = issueflow::gitlab::issues::encode_update_issue_body(Some("new body"));
    assert_eq!(body, serde_json::json!({ "description": "new body" }));
}
```

- [ ] **Step 2: Add an update body and API helper**

```rust
#[derive(Serialize)]
struct UpdateIssueBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'a str>,
}

pub async fn update_issue(
    base_url: &str,
    access_token: &str,
    project_id: u64,
    issue_iid: u64,
    description: &str,
) -> Result<GitlabIssue, String> {
    let client = client::build_client(base_url, access_token)?;
    let path = format!("projects/{project_id}/issues/{issue_iid}");
    let body = UpdateIssueBody {
        description: Some(description),
    };

    client.put_json(&path, &body).await
}
```

- [ ] **Step 3: Add a generic `put_json` client helper if missing**

```rust
pub async fn put_json<T, B>(&self, path: &str, body: &B) -> Result<T, String>
where
    T: DeserializeOwned,
    B: Serialize + ?Sized,
{
    let url = self.api_url(path)?;
    self.request_json(Method::PUT, url, Some(body)).await
}
```

- [ ] **Step 4: Run the focused tests**

Run: `cargo test update_issue_request_encodes_description_field -- --exact`

Expected: PASS.

- [ ] **Step 5: Commit the GitLab issue update support**

```bash
git add src/gitlab/issues.rs src/gitlab/client.rs tests/gitlab_issue_update.rs
git commit -m "feat: add gitlab issue description update"
```

### Task 7: Execute confirmed pending actions through the existing agent run path

**Files:**
- Modify: `src/agent/models.rs`
- Modify: `src/agent/runs.rs`
- Modify: `src/agent/orchestrator.rs`
- Modify: `src/actions/store.rs`
- Test: `tests/pending_actions_handler.rs`

- [ ] **Step 1: Add action execution input shape**

```rust
#[derive(Debug, Deserialize, Serialize)]
pub struct ExecutePendingActionInput {
    pub pending_action_id: String,
    pub access_token: String,
}
```

- [ ] **Step 2: Add store helpers for status transitions**

```rust
pub async fn mark_running(pool: &DbPool, id: &str, run_id: &str) -> Result<(), sqlx::Error> { ... }
pub async fn mark_completed(pool: &DbPool, id: &str, run_id: &str) -> Result<(), sqlx::Error> { ... }
pub async fn mark_failed(pool: &DbPool, id: &str, run_id: &str) -> Result<(), sqlx::Error> { ... }
```

- [ ] **Step 3: Extend orchestrator execution branch**

```rust
if let Ok(action_input) = serde_json::from_str::<ExecutePendingActionInput>(payload) {
    execute_pending_action(state, run, action_input).await?;
    return Ok(());
}
```

- [ ] **Step 4: Implement the `update_gitlab_issue` execution branch**

```rust
match action.action_type.as_str() {
    "update_gitlab_issue" => {
        let payload: UpdateGitlabIssuePayload = serde_json::from_str(&action.payload)?;
        issues::update_issue(
            base_url,
            &input.access_token,
            action.project_id as u64,
            payload.issue_iid,
            &payload.rendered_content,
        )
        .await?;
    }
    "refresh_memory" => { /* no-op or future hook */ }
    other => return Err(AppError::BadRequest(format!("unsupported action type: {other}").into())),
}
```

- [ ] **Step 5: Run focused tests**

Run: `cargo test confirm_pending_action_marks_action_confirmed -- --exact`

Expected: PASS, plus no compilation errors in the agent path.

- [ ] **Step 6: Commit the executor integration**

```bash
git add src/agent/models.rs src/agent/runs.rs src/agent/orchestrator.rs src/actions/store.rs
git commit -m "feat: execute confirmed pending actions via agent runs"
```

### Task 8: Add end-to-end fallback rules and regression tests

**Files:**
- Modify: `tests/pending_actions_handler.rs`
- Modify: `tests/gitlab_issue_update.rs`
- Modify: `tests/webhook_handler.rs`
- Test: `tests/pending_actions_handler.rs`

- [ ] **Step 1: Add the failing fallback test case**

```rust
#[tokio::test]
async fn pending_action_can_fallback_to_comment_when_issue_update_is_not_allowed() {
    // mock GitLab update permission failure
    // assert action remains explicit and fallback branch is selected
}
```

- [ ] **Step 2: Implement the minimal fallback rule**

```rust
if update_issue_result_is_permission_denied(&error) {
    actions::store::replace_action_type(pool, &action.id, "publish_gitlab_comment").await?;
}
```

- [ ] **Step 3: Add regression coverage for webhook preparation-only behavior**

```rust
#[tokio::test]
async fn webhook_route_accepts_valid_issue_hook_without_writing_gitlab() {
    let payload = r#"{"object_kind":"issue","object_attributes":{"noteable_type":"Issue"}}"#;
    // assert ACCEPTED and no auth-required write path triggered
}
```

- [ ] **Step 4: Run the broader targeted backend suite**

Run: `cargo test engineering_memory_handler pending_actions_handler gitlab_issue_update webhook_handler -- --nocapture`

Expected: PASS for new flows and no regression in webhook behavior.

- [ ] **Step 5: Commit the end-to-end guards**

```bash
git add tests/pending_actions_handler.rs tests/gitlab_issue_update.rs tests/webhook_handler.rs
git commit -m "test: cover fallback and preparation-only behavior"
```

### Task 9: Run the backend quality gate and summarize follow-up UI work

**Files:**
- Modify: `docs/superpowers/specs/2026-06-29-engineering-memory-hub-design.md`
- Modify: `docs/superpowers/plans/2026-06-29-engineering-memory-hub-implementation.md`

- [ ] **Step 1: Run backend formatting and tests**

Run: `cargo fmt -- --check && cargo test`

Expected: PASS.

- [ ] **Step 2: Run backend linting**

Run: `cargo clippy -- -D warnings`

Expected: PASS.

- [ ] **Step 3: Record any UI follow-up explicitly in docs instead of extending MVP scope**

```md
## Deferred UI Work

- show engineering memory snapshot in workbench
- show pending action preview cards
- bind confirm tokens to richer workbench state recovery
```

- [ ] **Step 4: Commit the verified backend MVP**

```bash
git add docs/superpowers/specs/2026-06-29-engineering-memory-hub-design.md docs/superpowers/plans/2026-06-29-engineering-memory-hub-implementation.md
git commit -m "docs: record engineering memory hub follow-up work"
```

## Self-Review

Spec coverage check:

- engineering memory storage: covered in Tasks 1-3
- pending actions and confirmation: covered in Tasks 4-5
- direct full issue description update with confirmation: covered in Tasks 5-7
- fallback to comment on insufficient edit permission: covered in Task 8
- preparation-only webhook behavior: covered in Task 8
- workbench remains current schema while acting as work domain: reflected throughout and does not require immediate migration

Placeholder scan:

- no `TODO`, `TBD`, or deferred implementation markers inside executable tasks
- deferred work is isolated to Task 9 documentation, not hidden in implementation tasks

Type consistency check:

- `engineering_memory` uses `validation_suggestions`
- pending action type uses `update_gitlab_issue`
- execution branch names align with action types and route payloads

## Execution Handoff

Plan complete and saved to `docs/superpowers/plans/2026-06-29-engineering-memory-hub-implementation.md`. Two execution options:

**1. Subagent-Driven (recommended)** - I dispatch a fresh subagent per task, review between tasks, fast iteration

**2. Inline Execution** - Execute tasks in this session using executing-plans, batch execution with checkpoints

Which approach?
