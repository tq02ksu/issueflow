CREATE TABLE IF NOT EXISTS engineering_memory (
    id TEXT PRIMARY KEY,
    artifact_type TEXT NOT NULL,
    artifact_id TEXT NOT NULL,
    scope_type TEXT NOT NULL,
    scope_key TEXT NOT NULL,
    scope_project_id BIGINT,
    scope_workbench_id BIGINT,
    scope_user_id BIGINT,
    memory_kind TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',
    revision INTEGER NOT NULL DEFAULT 1,
    updated_by_user_id BIGINT,
    input_text TEXT NOT NULL,
    input_context TEXT NOT NULL,
    source_snapshot TEXT,
    spec TEXT NOT NULL,
    validation_suggestions TEXT NOT NULL,
    risk_notes TEXT NOT NULL,
    evaluation_summary TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(scope_key, artifact_type, artifact_id, memory_kind)
);

COMMENT ON TABLE engineering_memory IS 'Latest structured memory snapshot for one artifact under a system, project, workbench, or personal scope';
COMMENT ON COLUMN engineering_memory.id IS 'primary key';
COMMENT ON COLUMN engineering_memory.artifact_type IS 'artifact category such as issue, spec, decision, or evaluation';
COMMENT ON COLUMN engineering_memory.artifact_id IS 'artifact identity within its category, such as a GitLab issue iid';
COMMENT ON COLUMN engineering_memory.scope_type IS 'memory scope layer: system, project, workbench, or personal';
COMMENT ON COLUMN engineering_memory.scope_key IS 'canonical scope identity used for uniqueness and upsert matching';
COMMENT ON COLUMN engineering_memory.scope_project_id IS 'project id when the memory belongs to a project-related scope';
COMMENT ON COLUMN engineering_memory.scope_workbench_id IS 'workbench id when the memory belongs to a workbench-local scope';
COMMENT ON COLUMN engineering_memory.scope_user_id IS 'user id when the memory belongs to a personal scope';
COMMENT ON COLUMN engineering_memory.memory_kind IS 'functional purpose of the memory such as issue_readiness, issue_note, issue_context, or policy_note';
COMMENT ON COLUMN engineering_memory.status IS 'lifecycle status for the memory row';
COMMENT ON COLUMN engineering_memory.revision IS 'monotonic revision incremented on latest-snapshot updates';
COMMENT ON COLUMN engineering_memory.updated_by_user_id IS 'user who most recently updated the memory';
COMMENT ON COLUMN engineering_memory.input_text IS 'raw or condensed input text used to derive this memory';
COMMENT ON COLUMN engineering_memory.input_context IS 'structured input context JSON used to derive this memory';
COMMENT ON COLUMN engineering_memory.source_snapshot IS 'source-system snapshot JSON, typically captured from GitLab at read time';
COMMENT ON COLUMN engineering_memory.spec IS 'main structured summary or spec JSON for the artifact';
COMMENT ON COLUMN engineering_memory.validation_suggestions IS 'structured validation and testing suggestion JSON';
COMMENT ON COLUMN engineering_memory.risk_notes IS 'structured risk note JSON';
COMMENT ON COLUMN engineering_memory.evaluation_summary IS 'top-level evaluation JSON including readiness outcome summaries';
COMMENT ON COLUMN engineering_memory.created_at IS 'creation timestamp';
COMMENT ON COLUMN engineering_memory.updated_at IS 'last update timestamp';
