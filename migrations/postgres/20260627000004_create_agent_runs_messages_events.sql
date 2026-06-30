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

COMMENT ON TABLE agent_runs IS 'Execution attempts or queued runs inside an agent session';
COMMENT ON COLUMN agent_runs.id IS 'primary key';
COMMENT ON COLUMN agent_runs.session_id IS 'FK to agent_sessions.id';
COMMENT ON COLUMN agent_runs.parent_run_id IS 'optional FK to a parent run';
COMMENT ON COLUMN agent_runs.status IS 'execution lifecycle status';
COMMENT ON COLUMN agent_runs.worker_id IS 'worker identity currently responsible for the run';
COMMENT ON COLUMN agent_runs.leased_until IS 'lease expiration timestamp for worker-based execution';
COMMENT ON COLUMN agent_runs.attempt_count IS 'retry or execution attempt count';
COMMENT ON COLUMN agent_runs.resume_cursor IS 'serialized resume cursor for resumable execution';
COMMENT ON COLUMN agent_runs.input_payload IS 'serialized input payload for the run';
COMMENT ON COLUMN agent_runs.error_code IS 'machine-readable error code for failed runs';
COMMENT ON COLUMN agent_runs.error_message IS 'human-readable error message for failed runs';
COMMENT ON COLUMN agent_runs.started_at IS 'run start timestamp';
COMMENT ON COLUMN agent_runs.finished_at IS 'run completion timestamp';

COMMENT ON TABLE agent_messages IS 'Chat and system-visible messages stored within an agent session';
COMMENT ON COLUMN agent_messages.id IS 'primary key';
COMMENT ON COLUMN agent_messages.session_id IS 'FK to agent_sessions.id';
COMMENT ON COLUMN agent_messages.run_id IS 'optional FK to the run that produced the message';
COMMENT ON COLUMN agent_messages.role IS 'message role such as user, assistant, or system';
COMMENT ON COLUMN agent_messages.message_kind IS 'message kind describing rendering or protocol semantics';
COMMENT ON COLUMN agent_messages.content IS 'serialized message content';
COMMENT ON COLUMN agent_messages.created_at IS 'message creation timestamp';

COMMENT ON TABLE agent_run_events IS 'Ordered runtime events emitted by one agent run';
COMMENT ON COLUMN agent_run_events.id IS 'primary key';
COMMENT ON COLUMN agent_run_events.run_id IS 'FK to agent_runs.id';
COMMENT ON COLUMN agent_run_events.seq IS 'monotonic sequence number within one run';
COMMENT ON COLUMN agent_run_events.event_type IS 'event type name';
COMMENT ON COLUMN agent_run_events.payload IS 'serialized event payload';
COMMENT ON COLUMN agent_run_events.created_at IS 'event creation timestamp';
