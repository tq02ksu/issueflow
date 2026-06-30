# Engineering Memory Hub Design

## Goal

Build a project control and engineering memory layer that observes real project work without replacing GitLab or changing the core development workflow.

Primary goal:

- help projects succeed by recording, understanding, and evaluating what happened during delivery

Secondary goal:

- derive AI-assisted acceptance, validation, risk, and quality insights from the recorded process

This system is explicitly not a test management platform, not a project management replacement, and not a generic knowledge base.

## Product Boundary

The real workflow still starts in GitLab, Jira, or another issue system.

`issueflow` adds a sidecar control layer that:

- structures incoming requirements and work context
- records key decisions and process signals
- persists current engineering understanding as internal memory
- optionally writes controlled updates back to GitLab through explicit user-triggered actions

The system must not use service-side GitLab API tokens.

Default behavior:

- webhook, scheduled job, tool call, and agent session may all trigger preparation work
- without an authenticated user session, the system may only prepare drafts and pending actions
- GitLab write operations require an explicit user action and must execute with that user's current permissions
- when the system is going to replace an issue description, it must show the final rendered description to the user and route execution through the existing confirmation to-do mechanism before execution

Future optional behavior:

- a user may opt into background automation with a personal PAT, subject to encryption at rest, revocation, explicit scope binding, and allowlisted operations

## Core Model

The system has four separate concerns:

1. `workbench`
   User working context. A workbench has one primary project and may later relate to multiple supporting projects.
2. `engineering_memory`
   Current project-level engineering understanding for a specific artifact.
3. `pending_actions`
   Explicit work items that are prepared now and executed later after confirmation or background authorization.
4. `agent_sessions` / `agent_runs`
   Interaction history and execution process. These remain separate from memory and pending actions.

These objects must stay related but not collapsed into one storage model.

## Workbench Direction

Current schema binds a workbench to a single project. That is acceptable for v1 implementation work, but the long-term meaning of `workbench` changes here:

- a workbench is a user work domain
- it has one primary project
- it may later include multiple related projects
- agent sessions continue to belong to a workbench

This means:

- `engineering_memory` should not be keyed by `workbench_id`
- `pending_actions` should carry both `workbench_id` and `project_id`

## Engineering Memory

### Purpose

`engineering_memory` stores the current engineering understanding of an artifact. It is not a transcript, not an audit log, and not a version history table.

V1 uses latest-snapshot semantics:

- one row per `project_id + artifact_type + artifact_id`
- updates overwrite the current snapshot
- `revision` increments on change
- historical reconstruction is deferred to event streams or future history mechanisms

### Artifact Scope

V1 supports a small fixed artifact set:

- `issue`
- `spec`
- `decision`
- `evaluation`

### V1 Fields

- `id`
- `project_id`
- `artifact_type`
- `artifact_id`
- `status`
- `revision`
- `updated_by_user_id`
- `created_at`
- `updated_at`
- `input_text`
- `input_context`
- `spec`
- `validation_suggestions`
- `risk_notes`
- `evaluation_summary`

### Content Semantics

Flat metadata fields:

- `id`
- `project_id`
- `artifact_type`
- `artifact_id`
- `status`
- `revision`
- `updated_by_user_id`
- `created_at`
- `updated_at`

Raw input:

- `input_text`

Structured JSON content:

- `input_context`
- `spec`
- `validation_suggestions`
- `risk_notes`
- `evaluation_summary`

### JSON Shapes

`spec`

```json
{
  "summary": "",
  "acceptance_criteria": [],
  "success_conditions": [],
  "boundary_conditions": [],
  "open_questions": []
}
```

`validation_suggestions`

```json
{
  "happy_path": [],
  "failure_path": [],
  "edge_cases": [],
  "non_goals": []
}
```

`risk_notes`

```json
[
  {
    "title": "",
    "severity": "low",
    "description": "",
    "mitigation": ""
  }
]
```

`evaluation_summary`

```json
{
  "status": "unknown",
  "summary": "",
  "coverage_notes": [],
  "missing_cases": []
}
```

### Constraints

- unique key on `project_id + artifact_type + artifact_id`
- `revision >= 1`
- no `external_refs`
- no extra future-proof reference field in v1

## Pending Actions

### Purpose

`pending_actions` stores prepared but not yet fully executed work. It is the bridge between passive observation and explicit controlled writeback.

It is not the same thing as an agent run.

An action may be created by a webhook, scheduled job, or agent session, then later confirmed and executed through an `agent_run`.

### V1 Fields

- `id`
- `workbench_id`
- `project_id`
- `artifact_type`
- `artifact_id`
- `action_type`
- `status`
- `payload`
- `source_session_id`
- `source_run_id`
- `created_by_user_id`
- `assigned_user_id`
- `confirmed_by_user_id`
- `executed_run_id`
- `created_at`
- `updated_at`

### V1 Action Types

- `update_gitlab_issue`
- `publish_gitlab_comment`
- `refresh_memory`

### V1 Statuses

- `pending`
- `confirmed`
- `running`
- `completed`
- `cancelled`
- `failed`

### Payload

`payload` is JSON and contains action-specific data such as:

- target issue information
- prepared full issue description
- target field information
- execution arguments
- confirmation context

## Relationship Model

- `workbench` is the user-facing work domain
- `engineering_memory` is the current internal project memory
- `pending_actions` is the queued or awaiting-confirmation operation layer
- `agent_sessions` and `agent_runs` remain the interaction and execution layer

Typical flow:

1. A webhook, schedule, tool call, or agent session triggers preparation work.
2. The system reads source context and updates `engineering_memory`.
3. The system creates a `pending_action` if a follow-up operation is needed.
4. A user confirms the action through the existing confirmation to-do mechanism, typically surfaced through the workbench or an agent session entry point.
5. The system creates or reuses an `agent_run` to execute the action.
6. Results are written back to GitLab and action status is updated.

Default writeback path:

1. If the current user has permission to edit the target issue, the system prepares a full issue description update.
2. The user reviews the final rendered description and explicitly confirms it through the existing confirmation to-do flow.
3. The system executes `update_gitlab_issue`.
4. If the user lacks edit permission, the system may fall back to `publish_gitlab_comment`.

## Error Handling

V1 error handling should stay conservative:

- failed GitLab writes do not invalidate memory
- failed memory refresh does not auto-publish anything
- pending actions remain explicit objects and may be cancelled or retried
- permission failure must be surfaced as a user-context problem, not retried with broader credentials
- full issue description replacement must remain previewable before execution

## Testing Direction

For HTTP-facing features, follow the project rule:

- write failing integration tests first in `tests/`
- use `common::test_app(config)` and `.oneshot(request).await`
- cover success, authorization rejection, and invalid input

For pure storage and mapping logic:

- add unit tests around serialization, transitions, and row mapping

## Out of Scope for V1

- service-side GitLab tokens
- `external_refs`
- memory version history tables
- generic artifact graph modeling
- test case management
- CI or test execution orchestration
- automatic background GitLab writeback without explicit user authorization
- full multi-project workbench schema migration
- restricting issue updates to an agent-managed subsection only

## Recommendation

Start implementation with the narrowest stable slice:

1. add `engineering_memory`
2. add `pending_actions`
3. connect action creation to existing agent session and run flow
4. keep workbench schema mostly unchanged for now, while treating it conceptually as a work domain with one primary project
