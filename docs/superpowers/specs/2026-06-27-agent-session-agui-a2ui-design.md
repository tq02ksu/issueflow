# Agent Session AG-UI + A2UI Design

## Goal

Add long-lived Agent Sessions to `issueflow` so users can talk with an agent inside the workbench, let the agent inspect GitLab context, and complete issue operations through GitLab APIs.

The design uses:

- `AG-UI` for agent-user communication, run lifecycle, state, and tool execution events
- `A2UI` for agent-authored UI descriptions rendered by the Vue frontend
- an OpenAI-compatible chat completion API for the model backend

## Final Architecture Decision

This design follows three non-negotiable rules.

### 1. Protocol split

- `AG-UI` transports communication and runtime state
- `A2UI` describes UI surfaces and interactive components

They are complementary, not competing protocols.

### 2. Event semantics

- `ToolCallStart` / `ToolCallArgs` / `ToolCallEnd` / `ToolCallResult` are reserved for real logical tool execution only
- `CustomEvent` is the only transport for `A2UI` payloads
- `A2UI` must never be stuffed into `ToolCallArgs` or `ToolCallResult`

### 3. A2UI routing contract

Every `CustomEvent` that carries `A2UI` must include a `kind` field in its payload.

Minimum supported values:

- `a2ui_render`
- `a2ui_submit`

This gives the frontend a stable routing boundary between ordinary AG-UI message handling and A2UI rendering.

## In Scope

- long-lived agent sessions bound to a user and exactly one workbench
- AG-UI run lifecycle over SSE
- A2UI rendering inside the session page through `a2ui-vue`
- OpenAI-compatible model integration
- GitLab issue write actions:
  - create issue
  - update issue
  - delete issue
- GitLab context read actions:
  - list issues
  - get one issue
  - list wiki pages
  - get one wiki page
  - read repository files needed for context, including `AGENTS.md`
- backend persistence for sessions, durable runs, normalized messages, and replayable event logs
- frontend session list and full-screen agent page
- refresh-safe recovery and server-restart run resumption
- focused backend and frontend tests

## Out of Scope

- wiki write actions in the first delivery
- repository file write actions in the first delivery
- merge request automation
- release automation
- multimodal inputs
- sub-agent orchestration
- adopting a full third-party AG-UI server framework

## User Flow

1. The user opens `/workbench/agent`.
2. The frontend loads the user session list and opens one session.
3. The user sends a message.
4. The frontend creates or continues a durable run for the session.
5. A background worker claims the run, loads prior session context, and emits AG-UI events.
6. The agent may:
   - stream text
   - execute GitLab tools
   - emit A2UI UI surfaces through `CustomEvent(kind="a2ui_render")`
7. Every emitted AG-UI event is first appended to durable event storage and then fanned out to subscribers.
8. If the UI asks for structured user input, the run moves to `waiting_input` and the frontend later sends `CustomEvent(kind="a2ui_submit")` to continue work.
9. If the browser refreshes, the frontend reloads the session, replays prior events, and resubscribes to the active run.
10. If the server restarts, the worker scans incomplete runs and resumes eligible work from persisted state.

## Data Model

Persist sessions, durable runs, normalized messages, and runtime AG-UI event logs.

### `agent_sessions`

```sql
CREATE TABLE agent_sessions (
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

CREATE INDEX idx_agent_sessions_user_updated
  ON agent_sessions(user_id, updated_at DESC);
```

Notes:

- `id` is the AG-UI `threadId`
- `latest_state` stores the latest shared AG-UI state snapshot as JSON text
- `workbench_id` is required because every session belongs to exactly one workbench

### `agent_runs`

```sql
CREATE TABLE agent_runs (
  id             TEXT PRIMARY KEY,
  session_id     TEXT NOT NULL,
  parent_run_id  TEXT,
  status         TEXT NOT NULL,
  worker_id      TEXT,
  leased_until   TEXT,
  attempt_count  INTEGER NOT NULL,
  resume_cursor  TEXT,
  input_payload  TEXT,
  error_code     TEXT,
  error_message  TEXT,
  started_at     TEXT NOT NULL,
  finished_at    TEXT,
  FOREIGN KEY (session_id) REFERENCES agent_sessions(id),
  FOREIGN KEY (parent_run_id) REFERENCES agent_runs(id)
);

CREATE INDEX idx_agent_runs_session_started
  ON agent_runs(session_id, started_at DESC);
```

Status values:

- `queued`
- `running`
- `waiting_input`
- `completed`
- `failed`
- `cancelled`

### `agent_messages`

```sql
CREATE TABLE agent_messages (
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

CREATE INDEX idx_agent_messages_session_created
  ON agent_messages(session_id, created_at ASC, id ASC);
```

`message_kind` values:

- `text`
- `tool_call`
- `tool_result`
- `custom`
- `system`

Notes:

- `content` stores normalized JSON, not raw SSE frames
- streamed text chunks are assembled before persistence
- `custom` messages persist the full `A2UI` payload plus `kind`

### `agent_run_events`

```sql
CREATE TABLE agent_run_events (
  id         INTEGER PRIMARY KEY AUTOINCREMENT,
  run_id     TEXT NOT NULL,
  seq        INTEGER NOT NULL,
  event_type TEXT NOT NULL,
  payload    TEXT NOT NULL,
  created_at TEXT NOT NULL,
  FOREIGN KEY (run_id) REFERENCES agent_runs(id),
  UNIQUE (run_id, seq)
);

CREATE INDEX idx_agent_run_events_run_seq
  ON agent_run_events(run_id, seq ASC);
```

Notes:

- every AG-UI event sent to clients is first appended here
- this table enables browser refresh replay and worker resume after restart
- this table is retention-managed, not permanent business history
- keep rows for non-terminal runs
- terminal runs may be cleaned by TTL after their final state is folded into `agent_messages`, `agent_sessions.latest_state`, and `agent_runs`

## AG-UI Event Model

The runtime stream is AG-UI-first. The frontend should react to standard event categories directly.

Every emitted event follows this durability order:

1. append to `agent_run_events`
2. publish to live subscribers
3. fold final state into `agent_messages` and `agent_sessions.latest_state` when appropriate

### Lifecycle events

- `RunStarted`
- `RunFinished`
- `RunError`
- `StepStarted`
- `StepFinished`

These drive the run shell, progress rail, and error state.

### Text events

- `TextMessageStart`
- `TextMessageContent`
- `TextMessageEnd`

These drive streaming assistant chat bubbles.

### Tool events

- `ToolCallStart`
- `ToolCallArgs`
- `ToolCallEnd`
- `ToolCallResult`

These drive the visible tool execution cards and expose what the agent is doing against GitLab.

### State events

- `StateSnapshot`
- `StateDelta`
- `MessagesSnapshot`

Use these to synchronize the current shared session state and to recover message history at run start or reconnect.

### Custom events

`CustomEvent` is reserved here for `A2UI` traffic only.

Required payload shapes:

#### Agent asks frontend to render UI

```json
{
  "kind": "a2ui_render",
  "surface_id": "issue-plan-1",
  "payload": {
    "components": []
  }
}
```

#### Frontend returns user interaction data

```json
{
  "kind": "a2ui_submit",
  "surface_id": "issue-plan-1",
  "payload": {
    "confirmed": true,
    "title": "Fix login layout"
  }
}
```

This is the required closed loop for A2UI interaction. Do not reply to `a2ui_render` with `ToolCallResult`.

## A2UI Integration Rules

`a2ui-vue` is used to render rich, structured UI inside the session page.

### Rendering pipeline

1. The backend emits `CustomEvent(kind="a2ui_render")`.
2. The frontend AG-UI client sees `kind` and routes the payload to `a2ui-vue`.
3. `a2ui-vue` renders one or more surfaces inside the message flow.
4. User actions from those surfaces are collected by the frontend.
5. The frontend packages those actions as `CustomEvent(kind="a2ui_submit")` in the next run input.

### What A2UI is used for

- issue confirmation cards
- structured context panels
- editable forms before write actions
- result cards after GitLab actions

### What A2UI is not used for

- replacing ordinary streamed assistant text
- replacing tool execution lifecycle UI
- bypassing AG-UI state or run semantics

## Backend Architecture

Add a new `src/agent/` module.

### Proposed module shape

```text
src/agent/
  mod.rs
  config.rs
  models.rs
  sessions.rs
  runs.rs
  messages.rs
  openai.rs
  orchestrator.rs
  events.rs
  gitlab_tools.rs
```

### Responsibilities

#### `config.rs`

- parse `agent.openai_base_url`
- parse `agent.openai_api_key`
- parse `agent.model`
- parse `agent.max_tool_rounds`

#### `openai.rs`

- call an OpenAI-compatible chat completion endpoint
- support streaming deltas
- translate provider output into internal run events

#### `orchestrator.rs`

- claim queued runs through a lease
- load prior messages, session state, and resume cursor
- inject tool definitions
- append AG-UI events into `agent_run_events`
- execute GitLab tools when the model requests them
- persist normalized results and latest state
- move runs into `waiting_input`, `completed`, or `failed`

#### `gitlab_tools.rs`

- define the model-visible tool schemas
- execute GitLab read and write actions
- map GitLab responses into structured tool results

#### `sessions.rs`, `runs.rs`, `messages.rs`

- own SQL queries and persistence rules
- keep handlers thin

#### Worker model

- `POST` requests do not own full run execution
- the HTTP layer creates or continues durable runs
- a background worker loop claims eligible runs from the database
- lease expiry allows takeover after process death or restart
- startup scans reclaim `queued` runs and `running` runs whose lease expired

## GitLab Tool Surface

Issue writes are in scope now. Wiki and repository access are read-only in the first delivery.

### Issue write tools

- `create_issue`
- `update_issue`
- `delete_issue`

### Context read tools

- `list_issues`
- `get_issue`
- `list_wiki_pages`
- `get_wiki_page`
- `get_repo_file`

`get_repo_file` must support at least `AGENTS.md`. The implementation may start with a small allowlist for trusted documentation files before broadening to arbitrary repository reads.

### Tool execution rules

- tool calls are visible in the AG-UI stream
- tool results are normalized JSON objects, not raw HTML
- tool output may influence later `a2ui_render` events, but must not itself contain A2UI payloads

### Recovery rules

- first delivery does not require byte-for-byte LLM stream continuation
- first delivery does require deterministic resume from persisted messages, latest state, event log, and `resume_cursor`
- if user interaction is required, close the current run as `waiting_input`
- user submission continues work through a child run linked by `parent_run_id`

## OpenAI-Compatible Model Contract

Use an OpenAI-compatible chat completion API with tool calling.

Required config:

```toml
[agent]
openai_base_url = "https://api.openai.com/v1"
openai_api_key = "sk-..."
model = "gpt-4o"
max_tool_rounds = 10
```

Environment overrides:

- `AGENT_OPENAI_BASE_URL`
- `AGENT_OPENAI_API_KEY`
- `AGENT_MODEL`
- `AGENT_MAX_TOOL_ROUNDS`

The system prompt must explicitly teach the model:

- use GitLab tools for logic
- use `CustomEvent(kind="a2ui_render")` when it wants the frontend to render structured UI
- expect `CustomEvent(kind="a2ui_submit")` as the reply path for those surfaces
- never model A2UI as a tool call

## HTTP and Route Design

Keep workbench-scoped session endpoints plus durable run creation and run event subscription endpoints.

### Session management

| Method | Path | Auth | Purpose |
| --- | --- | --- | --- |
| `GET` | `/api/workbenches/{workbench_id}/agent-sessions` | JWT | list sessions for one workbench |
| `POST` | `/api/workbenches/{workbench_id}/agent-sessions` | JWT | create a session under one workbench and return `threadId` |
| `GET` | `/api/workbenches/{workbench_id}/agent-sessions/{id}` | JWT | load one session metadata and normalized message history |
| `PATCH` | `/api/workbenches/{workbench_id}/agent-sessions/{id}` | JWT | rename session title |
| `DELETE` | `/api/workbenches/{workbench_id}/agent-sessions/{id}` | JWT | delete one session |

### Run endpoints

| Method | Path | Auth | Purpose |
| --- | --- | --- | --- |
| `POST` | `/api/agent/runs` | JWT | append user input, create or continue one durable run, return `runId` |
| `GET` | `/api/agent/runs/{run_id}/events` | JWT | replay persisted events after `after_seq` and keep streaming live events |

### Run creation input shape

The request body should follow AG-UI concepts directly.

```json
{
  "threadId": "session-uuid",
  "workbenchId": 42,
  "messages": [
    { "role": "user", "content": "Please create an issue for the login bug" },
    {
      "role": "user",
      "content": {
        "kind": "a2ui_submit",
        "surface_id": "issue-plan-1",
        "payload": { "confirmed": true }
      }
    }
  ]
}
```

Response example:

```json
{
  "runId": "run-uuid",
  "threadId": "session-uuid",
  "status": "queued"
}
```

### Why this route split

- session CRUD stays scoped to the current workbench
- runtime interaction stays AG-UI-native but is no longer tied to one HTTP request lifetime
- event replay and live streaming share the same durable event log
- session history can be loaded without starting a run
- reconnect and replay still use `MessagesSnapshot` and `StateSnapshot`

## Frontend Architecture

Add a full-screen session page at `/workbench/agent`.

### Component tree

```text
AgentView.vue
  ├── SessionList.vue
  └── ChatPanel.vue
       ├── ChatHeader.vue
       ├── ChatMessages.vue
       │    ├── UserBubble
       │    ├── AssistantBubble
       │    ├── ToolCallCard
       │    ├── ToolResultCard
       │    └── A2UISurfaceHost
       └── ChatInput.vue
```

### Frontend stores and composables

- `agent.store.ts` for session list, active session, and normalized history
- `useAgentRun.ts` for durable run creation + AG-UI SSE subscription
- `useA2UIBridge.ts` for routing `CustomEvent` payloads into `a2ui-vue`

### Event-to-UI mapping

| Event | UI behavior |
| --- | --- |
| `RunStarted` | enter running state |
| `StepStarted` / `StepFinished` | update progress rail |
| `TextMessage*` | stream assistant text |
| `ToolCall*` | show tool execution cards |
| `StateSnapshot` / `StateDelta` | update shared state panel |
| `CustomEvent(kind="a2ui_render")` | render or update A2UI surface |
| `RunFinished` | stop loading and finalize persistence |
| `RunError` | show recoverable error UI |

### Refresh and restart recovery

When the page reloads:

1. load session metadata, normalized messages, and active runs
2. if a run is `queued`, `running`, or `waiting_input`, subscribe to `/api/agent/runs/{run_id}/events?after_seq=0`
3. replay prior events into the store
4. continue listening for live events until terminal status

When the backend restarts:

1. worker startup scans `agent_runs`
2. `queued` runs are claimed immediately
3. `running` runs with expired `leased_until` are reclaimed
4. `waiting_input` runs remain idle until user response arrives

### A2UI action handling

When the user interacts with an A2UI surface:

1. collect the action payload in the frontend
2. build `CustomEvent(kind="a2ui_submit")`
3. include it in the next `POST /api/agent/runs` request

This preserves the A2UI closed loop without abusing tool result semantics.

## Error Handling

### Backend

- invalid `threadId` owned by another user returns `404` or `403`
- malformed A2UI custom payload returns `400`
- model provider failures return `502`
- GitLab failures surface as tool errors and terminate the run only when unrecoverable

### Frontend

- disconnected SSE stream offers retry
- malformed `a2ui_render` event shows a structured fallback card
- failed tool call remains visible in the message flow for debugging

## Security and Permissions

- all session and run endpoints require JWT auth
- GitLab write tools use the server-side configured GitLab token
- GitLab read tools may use the user access token where existing project access semantics must be preserved
- repository file reads should begin with an allowlist, including `AGENTS.md`, before enabling wider file access
- delete issue remains a destructive capability and should be explicitly confirmed by the agent flow before execution
- worker lease ownership must be checked before continuing a recovered run

## Testing Strategy

### Backend integration tests

- session CRUD ownership and validation
- run creation returns durable run metadata
- event subscription endpoint returns `text/event-stream`
- streamed text events arrive in order
- tool lifecycle events are emitted around GitLab calls
- `a2ui_submit` payload is accepted as a custom input event
- malformed custom payload is rejected
- stale leased runs can be reclaimed on worker startup

### Backend unit tests

- OpenAI delta-to-AG-UI event translation
- normalization of streamed chunks into persisted messages
- SQL store behavior for sessions, runs, messages, and event logs

### Frontend tests

- session list and selection behavior
- SSE consumer updates bubbles and tool cards correctly
- `a2ui_render` payload reaches `a2ui-vue`
- submitting an A2UI form produces `a2ui_submit`
- refreshing while a run is active replays prior events and keeps streaming

## Delivery Notes

This design intentionally keeps the protocol boundary clean:

- AG-UI handles runtime communication
- A2UI handles UI description
- A2UI request and response both travel through `CustomEvent`
- tool events remain reserved for real logical tools only
- durable runs and TTL-managed event logs make refresh and restart recovery first-class behavior

That preserves future migration flexibility if AG-UI later standardizes a native A2UI event type.
