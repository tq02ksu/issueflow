CREATE TABLE IF NOT EXISTS agent_runs (
    id             TEXT PRIMARY KEY,
    session_id     TEXT NOT NULL REFERENCES agent_sessions(id),
    parent_run_id  TEXT REFERENCES agent_runs(id),
    status         TEXT NOT NULL,
    worker_id      TEXT,
    leased_until   TIMESTAMP,
    attempt_count  INTEGER NOT NULL DEFAULT 0,
    resume_cursor  TEXT,
    input_payload  TEXT,
    error_code     TEXT,
    error_message  TEXT,
    started_at     TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    finished_at    TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_runs_session_started
    ON agent_runs(session_id, started_at DESC);

CREATE TABLE IF NOT EXISTS agent_messages (
    id           BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    session_id   TEXT NOT NULL REFERENCES agent_sessions(id),
    run_id       TEXT REFERENCES agent_runs(id),
    role         TEXT NOT NULL,
    message_kind TEXT NOT NULL,
    content      TEXT NOT NULL,
    created_at   TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_messages_session_created
    ON agent_messages(session_id, created_at ASC, id ASC);

CREATE TABLE IF NOT EXISTS agent_run_events (
    id         BIGINT PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    run_id     TEXT NOT NULL REFERENCES agent_runs(id),
    seq        INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    payload    TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (run_id, seq)
);

CREATE INDEX IF NOT EXISTS idx_agent_run_events_run_seq
    ON agent_run_events(run_id, seq ASC);
