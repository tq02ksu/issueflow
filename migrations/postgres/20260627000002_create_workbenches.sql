CREATE TABLE IF NOT EXISTS workbenches (
    id           BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id      BIGINT NOT NULL REFERENCES users(id),
    project_id   BIGINT NOT NULL,
    project_name TEXT NOT NULL,
    project_path TEXT NOT NULL,
    name         TEXT NOT NULL DEFAULT '',
    created_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at   TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, project_id)
);

COMMENT ON TABLE workbenches IS 'user workbench bindings to GitLab projects';
COMMENT ON COLUMN workbenches.id IS 'primary key';
COMMENT ON COLUMN workbenches.user_id IS 'FK to users.id — owner of this workbench';
COMMENT ON COLUMN workbenches.project_id IS 'GitLab project ID';
COMMENT ON COLUMN workbenches.project_name IS 'cached GitLab project display name';
COMMENT ON COLUMN workbenches.project_path IS 'cached GitLab project full path (namespace/project)';
COMMENT ON COLUMN workbenches.name IS 'user-visible workbench name';
COMMENT ON COLUMN workbenches.created_at IS 'binding creation timestamp';
COMMENT ON COLUMN workbenches.updated_at IS 'last update timestamp';
