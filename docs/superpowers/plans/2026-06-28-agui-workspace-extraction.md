# AG-UI Workspace Extraction Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extract the AG-UI protocol, runtime, and Axum transport code from the monolithic `issueflow` crate into reusable workspace crates without changing current product behavior.

**Architecture:** Convert the repository into a Cargo workspace with three internal SDK-style crates: `agui-protocol` for pure protocol definitions, `agui-runtime` for provider-driven orchestration, and `agui-axum` for HTTP/SSE glue. Keep database persistence, GitLab integration, session/auth, and business routing in the root `issueflow` application, which will consume the new crates through explicit adapters.

**Tech Stack:** Rust 2024, Cargo workspace, tokio, axum, async-openai, serde, sqlx

---

## File Map

- Create: `crates/agui-protocol/Cargo.toml`
- Create: `crates/agui-protocol/src/lib.rs`
- Create: `crates/agui-protocol/src/events.rs`
- Create: `crates/agui-runtime/Cargo.toml`
- Create: `crates/agui-runtime/src/lib.rs`
- Create: `crates/agui-runtime/src/provider.rs`
- Create: `crates/agui-runtime/src/openai.rs`
- Create: `crates/agui-runtime/src/engine.rs`
- Create: `crates/agui-axum/Cargo.toml`
- Create: `crates/agui-axum/src/lib.rs`
- Create: `crates/agui-axum/src/sse.rs`
- Modify: `Cargo.toml`
- Modify: `src/lib.rs`
- Modify: `src/agent/events.rs`
- Modify: `src/agent/openai.rs`
- Modify: `src/agent/orchestrator.rs`
- Modify: `src/http/handlers/agent_handler.rs`
- Modify: `src/error.rs`
- Modify: `tests/agent_run.rs`
- Modify: `tests/agent_prompt.rs`
- Create: `tests/agui_protocol.rs`
- Create: `tests/agui_runtime.rs`

## Task 1: Convert the repository into a Cargo workspace

**Files:**
- Create: `crates/agui-protocol/Cargo.toml`
- Create: `crates/agui-runtime/Cargo.toml`
- Create: `crates/agui-axum/Cargo.toml`
- Modify: `Cargo.toml`

- [ ] **Step 1: Write the failing workspace structure test**

```rust
// tests/agui_protocol.rs
use std::path::Path;

#[test]
fn workspace_contains_agui_crates() {
    assert!(Path::new("crates/agui-protocol/Cargo.toml").exists());
    assert!(Path::new("crates/agui-runtime/Cargo.toml").exists());
    assert!(Path::new("crates/agui-axum/Cargo.toml").exists());
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agui_protocol workspace_contains_agui_crates -- --exact`

Expected: FAIL because the crate manifests do not exist yet.

- [ ] **Step 3: Add workspace members and minimal crate manifests**

```toml
# Cargo.toml
[workspace]
members = [
  ".",
  "crates/agui-protocol",
  "crates/agui-runtime",
  "crates/agui-axum",
]
resolver = "2"

[workspace.dependencies]
async-openai = { version = "0.41.1", features = ["byot", "chat-completion"] }
axum = "0.8"
futures = "0.3"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["macros", "rt-multi-thread", "net", "sync"] }
tokio-stream = "0.1"
thiserror = "2"
uuid = { version = "1", features = ["v4", "serde"] }
```

```toml
# crates/agui-protocol/Cargo.toml
[package]
name = "agui-protocol"
version = "0.1.0"
edition = "2024"

[dependencies]
serde.workspace = true
serde_json.workspace = true
```

```toml
# crates/agui-runtime/Cargo.toml
[package]
name = "agui-runtime"
version = "0.1.0"
edition = "2024"

[dependencies]
agui-protocol = { path = "../agui-protocol" }
async-openai.workspace = true
futures.workspace = true
serde.workspace = true
serde_json.workspace = true
tokio.workspace = true
tokio-stream.workspace = true
uuid.workspace = true
```

```toml
# crates/agui-axum/Cargo.toml
[package]
name = "agui-axum"
version = "0.1.0"
edition = "2024"

[dependencies]
agui-protocol = { path = "../agui-protocol" }
axum.workspace = true
serde_json.workspace = true
```

- [ ] **Step 4: Re-run test to verify it passes**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agui_protocol workspace_contains_agui_crates -- --exact`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/agui-protocol/Cargo.toml crates/agui-runtime/Cargo.toml crates/agui-axum/Cargo.toml tests/agui_protocol.rs
git commit -m "refactor: add agui workspace skeleton"
```

## Task 2: Extract AG-UI protocol events into `agui-protocol`

**Files:**
- Create: `crates/agui-protocol/src/lib.rs`
- Create: `crates/agui-protocol/src/events.rs`
- Modify: `src/agent/events.rs`
- Modify: `src/lib.rs`
- Test: `tests/agui_protocol.rs`

- [ ] **Step 1: Write the failing protocol round-trip test**

```rust
// tests/agui_protocol.rs
use agui_protocol::events::AgUiEvent;

#[test]
fn protocol_event_serializes_with_expected_tag_names() {
    let event = AgUiEvent::RunStarted {
        thread_id: "thread-1".into(),
        run_id: "run-1".into(),
    };

    let json = serde_json::to_value(&event).unwrap();

    assert_eq!(json["type"], "RUN_STARTED");
    assert_eq!(json["threadId"], "thread-1");
    assert_eq!(json["runId"], "run-1");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agui_protocol protocol_event_serializes_with_expected_tag_names -- --exact`

Expected: FAIL because `agui_protocol` does not yet export the event type.

- [ ] **Step 3: Move the AG-UI event enum into the new crate and re-export it**

```rust
// crates/agui-protocol/src/lib.rs
pub mod events;
```

```rust
// crates/agui-protocol/src/events.rs
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AgUiEvent {
    RunStarted {
        #[serde(rename = "threadId")]
        thread_id: String,
        #[serde(rename = "runId")]
        run_id: String,
    },
    RunFinished {
        #[serde(rename = "threadId")]
        thread_id: String,
        #[serde(rename = "runId")]
        run_id: String,
    },
    RunError {
        message: String,
        code: Option<String>,
    },
    TextMessageStart {
        #[serde(rename = "messageId")]
        message_id: String,
        role: String,
    },
    TextMessageContent {
        #[serde(rename = "messageId")]
        message_id: String,
        delta: String,
    },
    TextMessageEnd {
        #[serde(rename = "messageId")]
        message_id: String,
    },
    ToolCallStart {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        #[serde(rename = "toolCallName")]
        tool_call_name: String,
        #[serde(rename = "parentMessageId")]
        parent_message_id: Option<String>,
    },
    ToolCallArgs {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        delta: String,
    },
    ToolCallEnd {
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
    },
    ToolCallResult {
        #[serde(rename = "messageId")]
        message_id: String,
        #[serde(rename = "toolCallId")]
        tool_call_id: String,
        content: serde_json::Value,
        role: String,
    },
    Custom {
        name: String,
        value: serde_json::Value,
    },
}
```

```rust
// src/agent/events.rs
pub use agui_protocol::events::AgUiEvent;
```

- [ ] **Step 4: Re-run test to verify it passes**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agui_protocol protocol_event_serializes_with_expected_tag_names -- --exact`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/agui-protocol/src/lib.rs crates/agui-protocol/src/events.rs src/agent/events.rs src/lib.rs tests/agui_protocol.rs
git commit -m "refactor: extract agui protocol crate"
```

## Task 3: Extract provider delta and OpenAI transport into `agui-runtime`

**Files:**
- Create: `crates/agui-runtime/src/lib.rs`
- Create: `crates/agui-runtime/src/provider.rs`
- Create: `crates/agui-runtime/src/openai.rs`
- Modify: `src/agent/openai.rs`
- Modify: `src/error.rs`
- Test: `tests/agui_runtime.rs`

- [ ] **Step 1: Write the failing runtime delta test**

```rust
// tests/agui_runtime.rs
use agui_runtime::provider::ProviderDelta;

#[test]
fn provider_delta_supports_tool_roundtrip_markers() {
    let deltas = vec![
        ProviderDelta::ToolStart {
            id: "call_1".into(),
            name: "list_issues".into(),
        },
        ProviderDelta::ToolArgs {
            id: "call_1".into(),
            delta: "{\"project_id\":1}".into(),
        },
        ProviderDelta::ToolEnd {
            id: "call_1".into(),
        },
    ];

    assert_eq!(deltas.len(), 3);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agui_runtime provider_delta_supports_tool_roundtrip_markers -- --exact`

Expected: FAIL because `agui_runtime` does not yet export `ProviderDelta`.

- [ ] **Step 3: Move provider delta and OpenAI client into `agui-runtime`**

```rust
// crates/agui-runtime/src/lib.rs
pub mod openai;
pub mod provider;
```

```rust
// crates/agui-runtime/src/provider.rs
#[derive(Debug)]
pub enum ProviderDelta {
    Text(String),
    ToolStart { id: String, name: String },
    ToolArgs { id: String, delta: String },
    ToolEnd { id: String },
    Done,
}
```

```rust
// src/agent/openai.rs
pub use agui_runtime::openai::OpenAiClient;
pub use agui_runtime::provider::ProviderDelta;
```

- [ ] **Step 4: Re-run test to verify it passes**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agui_runtime provider_delta_supports_tool_roundtrip_markers -- --exact`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/agui-runtime/src/lib.rs crates/agui-runtime/src/provider.rs crates/agui-runtime/src/openai.rs src/agent/openai.rs src/error.rs tests/agui_runtime.rs
git commit -m "refactor: extract agui runtime provider layer"
```

## Task 4: Move the AG-UI run engine into `agui-runtime`

**Files:**
- Create: `crates/agui-runtime/src/engine.rs`
- Modify: `crates/agui-runtime/src/lib.rs`
- Modify: `src/agent/orchestrator.rs`
- Modify: `src/http/handlers/agent_handler.rs`
- Test: `tests/agent_run.rs`

- [ ] **Step 1: Write the failing orchestrator regression test**

```rust
// tests/agent_run.rs
#[tokio::test]
async fn create_run_persists_input_payload_for_runtime_execution() {
    // existing test setup
    // assert persisted input payload remains present after runtime extraction
    assert!(json.get("runId").is_some());
}
```

- [ ] **Step 2: Run test to verify it fails in the middle of extraction**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_run create_run_returns_durable_metadata -- --exact`

Expected: FAIL once orchestrator code is removed before the adapter is wired.

- [ ] **Step 3: Introduce a runtime engine API and adapt `issueflow` to call it**

```rust
// crates/agui-runtime/src/engine.rs
pub struct RunEngine;

impl RunEngine {
    pub async fn process_rounds(/* runtime-neutral args */) -> Result<(), String> {
        todo!("move provider loop and tool round orchestration here")
    }
}
```

```rust
// src/agent/orchestrator.rs
use agui_runtime::engine::RunEngine;

pub async fn process_run(state: AppState, run: &AgentRunRow) -> Result<(), AppError> {
    // keep DB/session/tool lookup in issueflow
    // delegate provider/event orchestration to RunEngine
    RunEngine::process_rounds(/* adapted args */)
        .await
        .map_err(|e| AppError::Internal(e.into()))
}
```

- [ ] **Step 4: Re-run test to verify it passes**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_run create_run_returns_durable_metadata -- --exact`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/agui-runtime/src/engine.rs crates/agui-runtime/src/lib.rs src/agent/orchestrator.rs src/http/handlers/agent_handler.rs tests/agent_run.rs
git commit -m "refactor: move agui run engine into runtime crate"
```

## Task 5: Extract SSE/Axum glue into `agui-axum`

**Files:**
- Create: `crates/agui-axum/src/lib.rs`
- Create: `crates/agui-axum/src/sse.rs`
- Modify: `src/http/handlers/agent_handler.rs`
- Test: `tests/agent_run.rs`

- [ ] **Step 1: Write the failing SSE bridge test**

```rust
// tests/agent_run.rs
#[tokio::test]
async fn subscribe_events_returns_event_stream() {
    // keep existing route test and assert content-type remains text/event-stream
    assert_eq!(
        response.headers()[axum::http::header::CONTENT_TYPE],
        "text/event-stream"
    );
}
```

- [ ] **Step 2: Run test to verify it fails during transport extraction**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_run subscribe_events_returns_event_stream -- --exact`

Expected: FAIL if SSE glue is removed before the new helper is wired.

- [ ] **Step 3: Move SSE/event conversion into `agui-axum`**

```rust
// crates/agui-axum/src/lib.rs
pub mod sse;
```

```rust
// crates/agui-axum/src/sse.rs
use agui_protocol::events::AgUiEvent;
use axum::response::sse::Event;

pub fn encode_event(event: &AgUiEvent) -> Event {
    Event::default().json_data(event).unwrap()
}

pub fn build_event_from_payload(event_type: &str, payload: &str) -> Event {
    match event_type {
        "CUSTOM" => {
            if let Ok(value) = serde_json::from_str::<serde_json::Value>(payload) {
                Event::default().event("custom").json_data(value).unwrap()
            } else {
                Event::default().data(payload)
            }
        }
        _ => Event::default().event(event_type).data(payload),
    }
}
```

- [ ] **Step 4: Re-run test to verify it passes**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agent_run subscribe_events_returns_event_stream -- --exact`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/agui-axum/src/lib.rs crates/agui-axum/src/sse.rs src/http/handlers/agent_handler.rs tests/agent_run.rs
git commit -m "refactor: extract agui axum transport helpers"
```

## Task 6: Full integration cleanup and verification

**Files:**
- Modify: any touched files from previous tasks
- Test: `tests/agui_protocol.rs`
- Test: `tests/agui_runtime.rs`
- Test: `tests/agent_run.rs`
- Test: `tests/agent_prompt.rs`

- [ ] **Step 1: Run targeted integration tests**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test --test agui_protocol --test agui_runtime --test agent_run --test agent_prompt`

Expected: PASS.

- [ ] **Step 2: Run full test suite**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo test`

Expected: PASS with 0 failures.

- [ ] **Step 3: Run formatting check**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo fmt -- --check`

Expected: PASS.

- [ ] **Step 4: Run lint gate**

Run: `PATH="$HOME/.cargo/bin:$PATH" cargo clippy -- -D warnings`

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml crates/agui-protocol crates/agui-runtime crates/agui-axum src tests
git commit -m "refactor: split agui implementation into workspace crates"
```
