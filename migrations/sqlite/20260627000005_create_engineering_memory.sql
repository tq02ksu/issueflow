-- engineering_memory stores the latest structured memory snapshot for one artifact
-- under a specific scope. It is used for shared project memory, workbench-local
-- context, personal notes, and future system-level policy memory.
CREATE TABLE IF NOT EXISTS engineering_memory (
    -- Stable row id for this memory record.
    id TEXT PRIMARY KEY,
    -- The artifact category this memory describes, such as issue/spec/decision.
    artifact_type TEXT NOT NULL,
    -- The artifact identity within its type, such as GitLab issue iid "77".
    artifact_id TEXT NOT NULL,
    -- The scope layer that owns this memory: system/project/workbench/personal.
    scope_type TEXT NOT NULL,
    -- Canonical scope identity used for uniqueness and upsert matching.
    scope_key TEXT NOT NULL,
    -- Project id when the memory belongs to a project or a project-related scope.
    scope_project_id INTEGER,
    -- Workbench id when the memory belongs to a workbench-local execution context.
    scope_workbench_id INTEGER,
    -- User id when the memory belongs to one specific user.
    scope_user_id INTEGER,
    -- The functional role of the memory, such as issue_readiness or issue_note.
    memory_kind TEXT NOT NULL,
    -- Lifecycle status for the memory row.
    status TEXT NOT NULL DEFAULT 'draft',
    -- Monotonic revision for latest-snapshot updates.
    revision INTEGER NOT NULL DEFAULT 1,
    -- User who most recently updated the memory.
    updated_by_user_id INTEGER,
    -- Raw or condensed input text used to derive this memory.
    input_text TEXT NOT NULL,
    -- Structured input context JSON captured for this memory.
    input_context TEXT NOT NULL,
    -- Source-system snapshot JSON, typically the GitLab issue context at read time.
    source_snapshot TEXT,
    -- Main structured summary/spec JSON for the artifact.
    spec TEXT NOT NULL,
    -- Structured validation or testing suggestions JSON.
    validation_suggestions TEXT NOT NULL,
    -- Structured risk note JSON.
    risk_notes TEXT NOT NULL,
    -- Top-level evaluation JSON, including readiness outcome summaries.
    evaluation_summary TEXT NOT NULL,
    -- Creation timestamp for this memory record.
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- Last update timestamp for this memory record.
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    -- One latest-snapshot memory row per scope/artifact/kind combination.
    UNIQUE(scope_key, artifact_type, artifact_id, memory_kind)
);
