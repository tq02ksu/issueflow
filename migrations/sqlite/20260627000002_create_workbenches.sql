-- workbenches binds one user-facing work domain to one primary GitLab project.
CREATE TABLE IF NOT EXISTS workbenches (
    -- Stable primary key for the workbench.
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    -- FK to users.id for the owner of this workbench.
    user_id      INTEGER NOT NULL,
    -- GitLab project ID bound to this workbench.
    project_id   INTEGER NOT NULL,
    -- Cached GitLab project display name.
    project_name TEXT NOT NULL,
    -- Cached GitLab project full path (namespace/project).
    project_path TEXT NOT NULL,
    -- User-visible workbench name.
    name         TEXT NOT NULL DEFAULT '',
    -- Timestamp when the workbench binding was created.
    created_at   TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Timestamp when the workbench was last updated.
    updated_at   TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- One workbench per user and GitLab project.
    UNIQUE(user_id, project_id)
);
