# Work Item State Machine Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first issue-centered work-item state machine MVP with light-agent orchestration, scoped memory, controlled pending actions, and a future-ready heavy-agent delegation boundary.

**Architecture:** Keep the stable platform core in Rust: issue states, scoped memory, pending actions, and controlled write-back. Let the light in-system agent plus `skills` produce state evaluation and next-step proposals, while leaving heavy code execution to external agents such as `OpenCode` or `Codex`.

**Tech Stack:** Rust 2024, Axum, SQLx with SQLite/Postgres migrations, existing GitLab REST client, Vue 3, Naive UI, Vite, existing session/auth and pending action infrastructure.

---

## File Map

### Existing files to modify

- `migrations/sqlite/20260627000005_create_engineering_memory.sql`
- `migrations/postgres/20260627000005_create_engineering_memory.sql`
- `src/memory/models.rs`
- `src/memory/store.rs`
- `src/http/handlers/memory_handler.rs`
- `src/http/routes.rs`
- `src/actions/models.rs`
- `src/actions/store.rs`
- `src/agent/models.rs`
- `src/agent/orchestrator.rs`
- `src/lib.rs`
- `openapi.yaml`
- `tests/common/mod.rs`
- `web/src/api/workbench.api.ts`
- `web/src/views/IssuesView.vue`

### New files to create

- `src/issue_state/mod.rs`
- `src/issue_state/models.rs`
- `src/issue_state/service.rs`
- `tests/issue_state_handler.rs`
- `tests/issue_state_memory_scope.rs`
- `tests/issue_state_pending_action.rs`
- `web/src/components/issues/IssueStatePanel.vue`
- `web/src/tests/issue-state.spec.ts`

### Responsibility split

- `src/issue_state/*`: fixed work-item state enum, evaluation payloads, and light-agent-facing orchestration service
- `src/memory/*`: scoped memory row types and lookups
- `src/http/handlers/memory_handler.rs`: state evaluate/read endpoints and controlled action preparation
- `src/actions/*`: pending action payloads for issue state transition and delegation
- `web/src/components/issues/IssueStatePanel.vue`: issue state UI inside the issue drawer
- `tests/*`: persistence, routing, and pending action verification

## Task 1: Preserve scoped memory as the storage base

**Files:**
- Modify: `src/memory/models.rs`
- Modify: `src/memory/store.rs`
- Modify: `tests/issue_state_memory_scope.rs`
- Create: `tests/issue_state_memory_scope.rs`
- Test: `tests/issue_state_memory_scope.rs`

- [ ] **Step 1: Rename the scoped memory store test to state-oriented intent**

```rust
#[tokio::test]
async fn upsert_engineering_memory_separates_project_and_personal_issue_state_scope() {
    // Same structure as the current scoped-memory test, but memory_kind uses
    // issue_state and issue_note rather than any readiness-specific memory kind.
}
```

- [ ] **Step 2: Add `issue_state` to `MemoryKind`**

```rust
pub enum MemoryKind {
    IssueState,
    IssueNote,
    IssueContext,
    PolicyNote,
}
```

- [ ] **Step 3: Keep `scope_key`-based uniqueness and lookup helpers**

Run: `cargo test upsert_engineering_memory_separates_project_and_personal_issue_state_scope -- --exact`

Expected: PASS

## Task 2: Add issue state domain models

**Files:**
- Create: `src/issue_state/mod.rs`
- Create: `src/issue_state/models.rs`
- Create: `src/issue_state/service.rs`
- Test: `tests/issue_state_handler.rs`

- [ ] **Step 1: Write a failing model-driven test for minimal state evaluation**

```rust
#[test]
fn evaluate_issue_with_description_moves_from_new_to_planned() {
    let state = issueflow::issue_state::service::evaluate_issue_state(
        "new",
        "Export report",
        Some("Users need CSV export"),
        &[],
    );

    assert_eq!(state.current_state, "new");
    assert_eq!(state.proposed_next_state, "planned");
}
```

- [ ] **Step 2: Define the state enum**

```rust
pub enum IssueState {
    New,
    Clarifying,
    Planned,
    ReadyForExecution,
    InExecution,
    Blocked,
    Done,
}
```

- [ ] **Step 3: Define the evaluation payload**

```rust
pub struct IssueStateEvaluation {
    pub current_state: String,
    pub proposed_next_state: String,
    pub summary: String,
    pub missing_context: Vec<String>,
    pub blockers: Vec<String>,
    pub role_notes: RoleNotes,
    pub heavy_agent: HeavyAgentDecision,
}
```

- [ ] **Step 4: Implement a temporary evaluator stub behind the future light-agent boundary**

```rust
pub fn evaluate_issue_state(
    current_state: &str,
    title: &str,
    description: Option<&str>,
    notes: &[IssueNote],
) -> IssueStateEvaluation {
    // Temporary deterministic stub only for bootstrapping the state-machine flow.
}
```

- [ ] **Step 5: Run the model test**

Run: `cargo test evaluate_issue_with_description_moves_from_new_to_planned -- --exact`

Expected: PASS

## Task 3: Add issue state evaluate/read endpoints

**Files:**
- Modify: `src/http/handlers/memory_handler.rs`
- Modify: `src/http/routes.rs`
- Modify: `src/lib.rs`
- Create: `tests/issue_state_handler.rs`
- Test: `tests/issue_state_handler.rs`

- [ ] **Step 1: Write failing HTTP tests for state evaluate and state read**

```rust
#[tokio::test]
async fn evaluate_issue_state_creates_project_state_memory_and_pending_action() {}

#[tokio::test]
async fn get_issue_state_returns_project_state_memory() {}
```

- [ ] **Step 2: Add routes**

```rust
.route(
    "/api/workbenches/{workbench_id}/issues/{issue_iid}/state/evaluate",
    post(memory_handler::evaluate_issue_state),
)
.route(
    "/api/workbenches/{workbench_id}/issues/{issue_iid}/state",
    get(memory_handler::get_issue_state),
)
```

- [ ] **Step 3: Implement evaluate handler**

Responsibilities:

- load issue and notes from GitLab
- build workbench `issue_context` memory
- build project `issue_state` memory
- prepare one pending action for controlled transition/write-back

- [ ] **Step 4: Implement read handler**

Responsibilities:

- return project `issue_state`
- return optional personal `issue_note`
- return latest relevant pending action

- [ ] **Step 5: Run the route tests**

Run: `cargo test issue_state_handler -- --nocapture`

Expected: PASS

## Task 4: Add state-transition pending action payloads

**Files:**
- Modify: `src/actions/models.rs`
- Modify: `src/actions/store.rs`
- Modify: `src/http/handlers/memory_handler.rs`
- Create: `tests/issue_state_pending_action.rs`
- Test: `tests/issue_state_pending_action.rs`

- [ ] **Step 1: Write a failing preview test for `apply_issue_state_transition`**

```rust
#[tokio::test]
async fn issue_state_pending_action_preview_renders_transition_summary() {}
```

- [ ] **Step 2: Add a typed payload**

```rust
pub struct ApplyIssueStateTransitionPayload {
    pub current_state: String,
    pub proposed_next_state: String,
    pub target_issue: TargetIssueRef,
    pub source_memory_id: String,
    pub transition_summary: String,
    pub heavy_agent: HeavyAgentDecision,
    pub write_back: IssueStateWriteBackPlan,
}
```

- [ ] **Step 3: Render pending action preview from the typed payload**

- [ ] **Step 4: Keep execution conservative**

V1 write-back stays limited to GitLab issue description/comment updates.
Do not implement heavy-agent triggering yet.

- [ ] **Step 5: Run the pending action tests**

Run: `cargo test issue_state_pending_action -- --nocapture`

Expected: PASS

## Task 5: Add issue state panel to the issue drawer

**Files:**
- Modify: `web/src/api/workbench.api.ts`
- Create: `web/src/components/issues/IssueStatePanel.vue`
- Modify: `web/src/views/IssuesView.vue`
- Create: `web/src/tests/issue-state.spec.ts`
- Test: `web/src/tests/issue-state.spec.ts`

- [ ] **Step 1: Add state API client types**

```ts
export interface IssueStateDetail {
  projectMemory: EngineeringMemory | null;
  personalNote: EngineeringMemory | null;
  pendingAction: PendingAction | null;
}
```

- [ ] **Step 2: Add `evaluateIssueState()` and `getIssueState()` API helpers**

- [ ] **Step 3: Build `IssueStatePanel.vue`**

Display:

- current state
- proposed next state
- summary
- missing context
- blockers
- heavy-agent recommendation
- pending confirmation action

- [ ] **Step 4: Mount the panel in the issue detail drawer**

- [ ] **Step 5: Run the frontend test**

Run: `npm --prefix web test -- --run issue-state.spec.ts`

Expected: PASS

## Task 6: Document the new API and verify the slice

**Files:**
- Modify: `openapi.yaml`
- Modify: `tests/issue_state_handler.rs`
- Test: `tests/issue_state_handler.rs`

- [ ] **Step 1: Replace old route docs with issue state docs**

- [ ] **Step 2: Add one integration test for confirmation-driven state write-back**

- [ ] **Step 3: Run focused backend verification**

Run: `cargo test issue_state_handler issue_state_memory_scope issue_state_pending_action -- --nocapture`

Expected: PASS

- [ ] **Step 4: Run focused frontend verification**

Run: `npm --prefix web test -- --run issue-state.spec.ts`

Expected: PASS

## Summary

This plan deliberately shifts implementation toward the real platform core:

- explicit issue states
- scoped memory
- light-agent orchestration
- skill-controlled advancement method
- controlled delegation boundary for heavy agents
