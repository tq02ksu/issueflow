-- agent_runs stores one execution attempt or queued run inside an agent session.
CREATE TABLE IF NOT EXISTS agent_runs (
    -- Stable text id for the run.
    id             TEXT PRIMARY KEY,
    -- FK to the owning agent session.
    session_id     TEXT NOT NULL,
    -- Optional FK to a parent run when this run is derived from another run.
    parent_run_id  TEXT,
    -- Execution lifecycle status.
    status         TEXT NOT NULL,
    -- Worker identity currently responsible for the run.
    worker_id      TEXT,
    -- Lease expiration timestamp for worker-based execution.
    leased_until   TEXT,
    -- Retry or execution attempt count.
    attempt_count  INTEGER NOT NULL DEFAULT 0,
    -- Serialized resume cursor for streaming or resumable execution.
    resume_cursor  TEXT,
    -- Serialized input payload for the run.
    input_payload  TEXT,
    -- Machine-readable error code for failed runs.
    error_code     TEXT,
    -- Human-readable error message for failed runs.
    error_message  TEXT,
    -- Run start timestamp.
    started_at     TEXT NOT NULL,
    -- Run completion timestamp when available.
    finished_at    TEXT,
    FOREIGN KEY (session_id) REFERENCES agent_sessions(id),
    FOREIGN KEY (parent_run_id) REFERENCES agent_runs(id)
);

-- Supports loading recent runs for one session.
CREATE INDEX IF NOT EXISTS idx_agent_runs_session_started
    ON agent_runs(session_id, started_at DESC);

-- agent_messages stores chat and system-visible messages within a session.
CREATE TABLE IF NOT EXISTS agent_messages (
    -- Stable primary key for the message row.
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    -- FK to the owning session.
    session_id   TEXT NOT NULL,
    -- Optional FK to the run that produced this message.
    run_id       TEXT,
    -- Message role such as user, assistant, or system.
    role         TEXT NOT NULL,
    -- Message kind describing rendering or protocol semantics.
    message_kind TEXT NOT NULL,
    -- Serialized message content.
    content      TEXT NOT NULL,
    -- Message creation timestamp.
    created_at   TEXT NOT NULL,
    FOREIGN KEY (session_id) REFERENCES agent_sessions(id),
    FOREIGN KEY (run_id) REFERENCES agent_runs(id)
);

-- Supports ordered playback of messages within a session.
CREATE INDEX IF NOT EXISTS idx_agent_messages_session_created
    ON agent_messages(session_id, created_at ASC, id ASC);

-- agent_run_events stores ordered runtime events emitted by one run.
CREATE TABLE IF NOT EXISTS agent_run_events (
    -- Stable primary key for the event row.
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    -- FK to the run that emitted the event.
    run_id     TEXT NOT NULL,
    -- Monotonic sequence number within one run.
    seq        INTEGER NOT NULL,
    -- Event type name.
    event_type TEXT NOT NULL,
    -- Serialized event payload.
    payload    TEXT NOT NULL,
    -- Event creation timestamp.
    created_at TEXT NOT NULL,
    FOREIGN KEY (run_id) REFERENCES agent_runs(id),
    -- One event per run and sequence number.
    UNIQUE (run_id, seq)
);

-- Supports streaming ordered events for one run.
CREATE INDEX IF NOT EXISTS idx_agent_run_events_run_seq
    ON agent_run_events(run_id, seq ASC);
