CREATE TABLE IF NOT EXISTS agent_sessions (
    id              TEXT PRIMARY KEY,
    user_id         INTEGER NOT NULL REFERENCES users(id),
    workbench_id    INTEGER NOT NULL REFERENCES workbenches(id),
    title           TEXT NOT NULL,
    latest_state    TEXT,
    last_message_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at      TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_agent_sessions_user_updated
    ON agent_sessions(user_id, updated_at DESC);

COMMENT ON TABLE agent_sessions IS 'Long-lived conversation sessions within a workbench';
COMMENT ON COLUMN agent_sessions.id IS 'primary key';
COMMENT ON COLUMN agent_sessions.user_id IS 'FK to users.id for the session owner';
COMMENT ON COLUMN agent_sessions.workbench_id IS 'FK to workbenches.id for the work domain';
COMMENT ON COLUMN agent_sessions.title IS 'user-visible session title';
COMMENT ON COLUMN agent_sessions.latest_state IS 'serialized latest frontend/runtime state snapshot';
COMMENT ON COLUMN agent_sessions.last_message_at IS 'timestamp of the latest message in the session';
COMMENT ON COLUMN agent_sessions.created_at IS 'session creation timestamp';
COMMENT ON COLUMN agent_sessions.updated_at IS 'last update timestamp';
