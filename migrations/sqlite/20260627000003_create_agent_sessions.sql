-- agent_sessions stores long-lived conversation threads inside a workbench.
CREATE TABLE IF NOT EXISTS agent_sessions (
    -- Stable text id for the session.
    id              TEXT PRIMARY KEY,
    -- FK to users.id for the session owner.
    user_id         INTEGER NOT NULL,
    -- FK to workbenches.id for the work domain.
    workbench_id    INTEGER NOT NULL,
    -- User-visible session title.
    title           TEXT NOT NULL,
    -- Serialized latest frontend/runtime state snapshot.
    latest_state    TEXT,
    -- Timestamp of the latest message seen in the session.
    last_message_at TEXT NOT NULL,
    -- Session creation timestamp.
    created_at      TEXT NOT NULL,
    -- Session last update timestamp.
    updated_at      TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id),
    FOREIGN KEY (workbench_id) REFERENCES workbenches(id)
);

-- Supports listing recent sessions for one user.
CREATE INDEX IF NOT EXISTS idx_agent_sessions_user_updated
    ON agent_sessions(user_id, updated_at DESC);
