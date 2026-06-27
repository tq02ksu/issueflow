# Agent Session Data Model Notes

## Purpose

This note explains the first-delivery agent-session data model in plain terms.

Minimum closed loop: 4 tables covering session identity, durable run execution, chat history, and event replay for recovery.

## Four Core Tables

```text
agent_sessions
  1 ─── N agent_runs
  1 ─── N agent_messages

agent_runs
  1 ─── N agent_run_events
  1 ─── N agent_messages
  1 ─── N child agent_runs      (via parent_run_id)

users
  1 ─── N agent_sessions

workbenches
  1 ─── N agent_sessions
```

## 1. `agent_sessions` — Thread identity

Long-lived conversation thread. One per workbench. Stores latest shared state.

| Column | Type | Note |
|---|---|---|
| `id` | TEXT PK | AG-UI `threadId` |
| `user_id` | INTEGER NOT NULL | FK → users.id |
| `workbench_id` | INTEGER NOT NULL | FK → workbenches.id |
| `title` | TEXT NOT NULL | Display name |
| `latest_state` | TEXT | JSON snapshot of current shared state |
| `last_message_at` | TEXT NOT NULL | |
| `created_at` | TEXT NOT NULL | |
| `updated_at` | TEXT NOT NULL | |

## 2. `agent_runs` — Durable workflow task

One agent execution attempt. Not tied to one HTTP request. Survives browser close and server restart.

| Column | Type | Note |
|---|---|---|
| `id` | TEXT PK | AG-UI `runId` |
| `session_id` | TEXT NOT NULL | FK → agent_sessions.id |
| `parent_run_id` | TEXT | FK → agent_runs.id (child continuation) |
| `status` | TEXT NOT NULL | `queued` / `running` / `waiting_input` / `completed` / `failed` / `cancelled` |
| `worker_id` | TEXT | Current owning worker |
| `leased_until` | TEXT | Lease expiry timestamp |
| `attempt_count` | INTEGER NOT NULL DEFAULT 0 | |
| `resume_cursor` | TEXT | JSON: execution breakpoint for recovery |
| `input_payload` | TEXT | Original run trigger input |
| `error_code` | TEXT | |
| `error_message` | TEXT | |
| `started_at` | TEXT NOT NULL | |
| `finished_at` | TEXT | |

## 3. `agent_messages` — Normalized chat history

Final durable conversation record. This is the LLM context source for future runs.

| Column | Type | Note |
|---|---|---|
| `id` | INTEGER PK AUTOINCREMENT | |
| `session_id` | TEXT NOT NULL | FK → agent_sessions.id |
| `run_id` | TEXT | FK → agent_runs.id (nullable) |
| `role` | TEXT NOT NULL | `user` / `assistant` / `tool` / `custom` |
| `message_kind` | TEXT NOT NULL | `text` / `tool_call` / `tool_result` / `custom` / `system` |
| `content` | TEXT NOT NULL | Normalized JSON |
| `created_at` | TEXT NOT NULL | |

## 4. `agent_run_events` — AG-UI event log

Replayable AG-UI event stream. Powers browser refresh replay and live resubscription.

| Column | Type | Note |
|---|---|---|
| `id` | INTEGER PK AUTOINCREMENT | |
| `run_id` | TEXT NOT NULL | FK → agent_runs.id |
| `seq` | INTEGER NOT NULL | Monotonic per run |
| `event_type` | TEXT NOT NULL | AG-UI event name |
| `payload` | TEXT NOT NULL | Full event JSON |
| `created_at` | TEXT NOT NULL | |
| UNIQUE | (run_id, seq) | |

## Deferred Tables

Not needed for first delivery. Add later when requirements grow.

### `agent_session_memories`

Compressed summaries of older conversation segments. Needed when a session has too many messages for the LLM context window.

### `agent_run_controls`

User control requests against active runs: supplement message, interrupt, cancel. Needed when in-run user interaction is required beyond `a2ui_submit`.

### `agent_llm_calls`

Per-provider-request metadata: model, token usage, latency. AG-UI has no native LLM-call event type, so this is a separate observability concern.

## Recovery Model

### Browser refresh

```
1. Load agent_sessions.latest_state
2. Load agent_messages (chat history)
3. Find active agent_runs (status = running / waiting_input)
4. Replay agent_run_events for active run
5. Continue live subscription
```

### Server restart

```
1. Worker scans agent_runs
2. Reclaim: queued runs + running runs with expired leased_until
3. Reconstruct LLM context from agent_messages + resume_cursor
4. Continue execution
```

Context for LLM: load `agent_messages` ordered by `created_at`, trim to fit the model context window. No summary layer yet in the first delivery.

## Event Retention

`agent_run_events` is durable but not permanent.

| Run status | Retention after terminal |
|---|---|
| `completed` | 1–7 days |
| `failed` | 7–14 days |
| `cancelled` | 1 day |

Do not purge until normalized messages and latest_state are persisted.
