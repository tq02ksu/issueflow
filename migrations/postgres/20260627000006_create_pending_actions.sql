CREATE TABLE IF NOT EXISTS pending_actions (
    id TEXT PRIMARY KEY,
    workbench_id BIGINT NOT NULL REFERENCES workbenches(id),
    project_id BIGINT NOT NULL,
    artifact_type TEXT NOT NULL,
    artifact_id TEXT NOT NULL,
    action_type TEXT NOT NULL,
    status TEXT NOT NULL,
    payload TEXT NOT NULL,
    source_session_id TEXT REFERENCES agent_sessions(id),
    source_run_id TEXT REFERENCES agent_runs(id),
    created_by_user_id BIGINT,
    assigned_user_id BIGINT,
    confirmed_by_user_id BIGINT,
    executed_run_id TEXT REFERENCES agent_runs(id),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_pending_actions_workbench_created
    ON pending_actions(workbench_id, created_at DESC);

COMMENT ON TABLE pending_actions IS 'Prepared work items that await confirmation or execution';
COMMENT ON COLUMN pending_actions.id IS 'primary key';
COMMENT ON COLUMN pending_actions.workbench_id IS 'FK to workbenches.id where the action is surfaced';
COMMENT ON COLUMN pending_actions.project_id IS 'GitLab project ID targeted by the action';
COMMENT ON COLUMN pending_actions.artifact_type IS 'artifact category targeted by the action';
COMMENT ON COLUMN pending_actions.artifact_id IS 'artifact identity within its category';
COMMENT ON COLUMN pending_actions.action_type IS 'action type such as update_gitlab_issue or apply_issue_readiness';
COMMENT ON COLUMN pending_actions.status IS 'action lifecycle status';
COMMENT ON COLUMN pending_actions.payload IS 'serialized action payload used for preview and execution';
COMMENT ON COLUMN pending_actions.source_session_id IS 'optional source agent session that prepared the action';
COMMENT ON COLUMN pending_actions.source_run_id IS 'optional source agent run that prepared the action';
COMMENT ON COLUMN pending_actions.created_by_user_id IS 'user who created the action';
COMMENT ON COLUMN pending_actions.assigned_user_id IS 'user currently assigned to review or execute the action';
COMMENT ON COLUMN pending_actions.confirmed_by_user_id IS 'user who confirmed the action';
COMMENT ON COLUMN pending_actions.executed_run_id IS 'agent run that executed the action after confirmation';
COMMENT ON COLUMN pending_actions.created_at IS 'action creation timestamp';
COMMENT ON COLUMN pending_actions.updated_at IS 'last update timestamp';
