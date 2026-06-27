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

CREATE INDEX IF NOT EXISTS idx_agent_sessions_user_updated
    ON agent_sessions(user_id, updated_at DESC);
