# Workbench Sidebar And Naming Design

## Summary

Refine workbench identity and navigation so that:

- each workbench has an editable display `name`
- each workbench keeps a synchronized GitLab `project_id` and `project_path`
- GitLab-backed behavior uses `project_id` as the primary key
- the workbench switcher moves into the sidebar in a compact form
- sidebar feature menus change based on the selected workbench's capabilities

## Goals

1. Separate display naming from repository identity.
2. Keep the workbench control area visually small.
3. Make sidebar actions change when the current workbench changes.
4. Ensure GitLab-backed operations query by `project_id`, not `project_path`.
5. Keep project rebinding controlled by GitLab search/sync, not free-text path edits.

## Non-Goals

- Implement full issue/release/agent business behavior in this change.
- Build a large workbench settings screen.
- Introduce free-form repository editing.

## Recommended UX

Use the compact `A-lite` sidebar layout:

- top of sidebar contains a one-line workbench selector
- selector shows primary label = workbench `name`
- selector shows secondary text = `project_path`
- selector dropdown contains:
  - switch to another workbench
  - add workbench
  - rename current workbench
  - rebind current workbench to another GitLab project
- feature menu sits directly below the selector and changes with the selected workbench

This keeps the workbench control area small while preserving the causal relationship between selected workbench and available features.

## Data Model

### Workbench Fields

Each workbench should carry three distinct concepts:

- `project_id`: GitLab project primary key for all GitLab-backed operations
- `project_path`: synchronized repository full path for display, traceability, and rebinding context
- `name`: editable display name for the workbench itself

### Semantics

- `project_id` is the business key used by future issue queries, issue creation, and other GitLab operations.
- `project_path` is not a business key; it is a synchronized configuration/display field.
- `name` is a pure display field and may differ from both GitLab project title and path.

## Backend Design

### Create Workbench

Frontend submits all three fields:

- `project_id`
- `project_path`
- `name`

Frontend autofills `name` with the last segment of `project_path` if the user does not override it.

Backend still treats all fields as required for creation.

### Update Workbench

Allow updating:

- `name` independently
- `project_id` and `project_path` together when rebinding to a different GitLab project

Rules:

- `name` may be edited directly
- `project_id` and `project_path` must always be updated as a pair
- `project_id` and `project_path` must come from a GitLab project selection/sync flow, not arbitrary user text entry

### Capability Model

Return a capability list per workbench so the frontend can build menus dynamically.

Initial shape can be a simple list of keys such as:

- `overview`
- `issues`
- `agents`
- `releases`
- `settings`

This should be lightweight now and extensible later to include disabled reasons or policy metadata.

## Frontend Design

### Sidebar Structure

Sidebar should become:

1. compact workbench selector
2. dynamic feature menu for current workbench

The selector remains visually compact. Rename/settings actions should live inside selector-triggered UI rather than a dedicated sidebar panel.

### Add Workbench Flow

The add-workbench dialog should:

1. search GitLab projects
2. let the user choose one project
3. prefill `name` with the final segment of `project_path`
4. let the user override `name`
5. submit `name + project_id + project_path`

### Rename Flow

Renaming should be a lightweight action from the selector dropdown or a small inline modal.

### Rebind Flow

Rebinding should:

1. open GitLab project search
2. select a new project
3. submit new `project_id + project_path`
4. preserve existing `name` unless the user explicitly resets it

### Dynamic Menu Behavior

When the selected workbench changes:

1. current workbench context updates immediately
2. frontend reloads capability/menu data for that workbench
3. main content routes adjust to the new capability set

Menu handling rules:

- allowed feature: visible and enabled
- visible but not allowed: disabled with reason
- not applicable: hidden entirely

If the active route becomes invalid after switching workbench, frontend should redirect to a safe default feature such as `overview`.

## Error Handling

### Backend

- invalid create/update payload: `400` problem details
- mismatched or incomplete `project_id + project_path` update: `400`
- unauthorized project binding: `403` or `400` with clear detail
- uniqueness conflicts: `409`

### Frontend

- create/rename/rebind failures should render clear inline error feedback
- no silent failure for dialog actions
- invalid post-switch feature route should auto-fallback

## Testing

### HTTP Integration Tests

Add `tests/` coverage for:

- create workbench with `name + project_id + project_path`
- update name only
- rebind with paired `project_id + project_path`
- reject invalid partial rebind updates
- capability-bearing workbench responses used by the sidebar

### Frontend Tests

Add/extend tests for:

- selector shows `name` as primary label
- selector shows `project_path` as secondary text
- create dialog defaults `name` from the last path segment
- switching current workbench updates capability-driven menu state

## Implementation Notes

- prefer minimal schema and API changes that clearly separate `name` from repository identity
- keep permission/capability modeling small at first; the important boundary is that the frontend stops hardcoding the sidebar
- all future project-bound queries should use `project_id`
