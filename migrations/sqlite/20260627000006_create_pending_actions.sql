-- pending_actions stores prepared work that awaits confirmation or execution.
CREATE TABLE IF NOT EXISTS pending_actions (
    -- Stable text id for the pending action.
    id TEXT PRIMARY KEY,
    -- FK to the workbench where the action is surfaced.
    workbench_id INTEGER NOT NULL,
    -- GitLab project ID targeted by the action.
    project_id INTEGER NOT NULL,
    -- Artifact category targeted by the action, such as issue.
    artifact_type TEXT NOT NULL,
    -- Artifact identity within its category.
    artifact_id TEXT NOT NULL,
    -- Action type such as update_gitlab_issue or apply_issue_readiness.
    action_type TEXT NOT NULL,
    -- Action lifecycle status.
    status TEXT NOT NULL,
    -- Serialized action payload used for preview and execution.
    payload TEXT NOT NULL,
    -- Optional source agent session that prepared the action.
    source_session_id TEXT,
    -- Optional source agent run that prepared the action.
    source_run_id TEXT,
    -- User who created the action.
    created_by_user_id INTEGER,
    -- User currently assigned to review or execute the action.
    assigned_user_id INTEGER,
    -- User who confirmed the action.
    confirmed_by_user_id INTEGER,
    -- Agent run that executed the action after confirmation.
    executed_run_id TEXT,
    -- Action creation timestamp.
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Action last update timestamp.
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (workbench_id) REFERENCES workbenches(id),
    FOREIGN KEY (source_session_id) REFERENCES agent_sessions(id),
    FOREIGN KEY (source_run_id) REFERENCES agent_runs(id),
    FOREIGN KEY (executed_run_id) REFERENCES agent_runs(id)
);

-- Supports listing newest pending actions inside one workbench.
CREATE INDEX IF NOT EXISTS idx_pending_actions_workbench_created
    ON pending_actions(workbench_id, created_at DESC);
