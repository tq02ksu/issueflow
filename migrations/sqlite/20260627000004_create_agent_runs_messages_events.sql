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

CREATE INDEX IF NOT EXISTS idx_agent_runs_session_started
    ON agent_runs(session_id, started_at DESC);

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

CREATE INDEX IF NOT EXISTS idx_agent_messages_session_created
    ON agent_messages(session_id, created_at ASC, id ASC);

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

CREATE INDEX IF NOT EXISTS idx_agent_run_events_run_seq
    ON agent_run_events(run_id, seq ASC);
