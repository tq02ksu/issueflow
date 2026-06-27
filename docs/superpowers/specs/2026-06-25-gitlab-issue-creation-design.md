# GitLab Issue Creation Design

## Goal

Implement the first delivery phase of the updated `issueflow` positioning: a chat-driven agent flow that prepares an issue draft, gets user confirmation, and then creates the final issue in GitLab through the Gateway.

## Context

The repository already has:

- a Rust Gateway entrypoint
- HTTP routing and handlers
- OIDC login and callback handling
- GitLab webhook intake
- a frontend skeleton
- Docker and GitLab CI artifacts in partial form

The requested work spans multiple sub-projects, but the first phase should stay focused. This phase will only implement chat-driven GitLab issue creation.

## Phase Breakdown

### Phase 1: Chat-Driven GitLab Issue Creation

Implement now.

### Phase 2: Web and Gateway Frame Tightening

Later phase. Refine the basic web and gateway project framework around the narrower issue-agent positioning.

### Phase 3: GitHub Actions CI and Release

Later phase. Add `.github/workflows` for CI and release.

### Phase 4: Docker Runtime and Release Flow Tightening

Later phase. Reconcile the root `Dockerfile` and runtime packaging with the delivery path.

## Phase 1 Scope

### In Scope

- Use chat as the issue creation entrypoint.
- Use `ag-ui` / `a2ui` as the interaction channel between the page and the agent-facing flow.
- Let the agent produce a structured issue draft from conversation context.
- Require explicit user confirmation before the final GitLab issue is created.
- Add GitLab API configuration for Gateway-side authenticated calls.
- Use the Rust `gitlab` crate instead of writing a custom HTTP client.
- Add the minimal Gateway endpoints needed to support draft confirmation and final issue creation.
- Accept only the minimum required issue fields for phase 1:
  - `project_id`
  - `title`
  - `description`
- Return basic created issue data to the caller after successful GitLab creation.
- Add focused tests for configuration loading and the new Gateway behavior.
- Update local development documentation with the extra GitLab token configuration required for this flow.

### Out of Scope

- page-driven direct CRUD issue forms
- GitLab milestone creation
- GitLab wiki creation
- issue editing or closing
- labels, assignees, milestones, attachments, or advanced issue fields
- full agent orchestration across multiple workflows
- permission model refinement beyond current Gateway ownership of credentials
- GitHub Actions changes
- Docker packaging changes

## User Flow

Phase 1 should follow this flow:

1. The user interacts with the page through chat.
2. The page and agent exchange structured messages through `ag-ui` / `a2ui`.
3. The agent decides that a GitLab issue should be created.
4. The agent produces a structured issue draft containing `project_id`, `title`, and `description`.
5. The page shows the draft to the user for explicit confirmation.
6. After confirmation, the Gateway creates the issue in GitLab.
7. The system returns the created issue identity and URL back to the page.

This keeps issue creation agent-driven, but avoids uncontrolled write actions.

## Architecture

The Gateway remains the only component that holds the GitLab API token. The page does not create issues directly. The agent does not talk to GitLab directly. Instead, the agent produces a draft, the user confirms it, and the Gateway performs the final GitLab API write.

This approach matches the current control-plane direction better than direct page CRUD or unconstrained agent writes.

## Configuration Changes

Extend the existing `git` configuration shape with:

- `git.base_url`
- `git.token`

Keep the existing field:

- `git.webhook_secret`

Expected mapping:

| TOML | Environment Variable | Required for GitLab creation |
| --- | --- | --- |
| `git.base_url` | `GIT_BASE_URL` | yes |
| `git.token` | `GIT_TOKEN` | yes |
| `git.webhook_secret` | `GIT_WEBHOOK_SECRET` | yes for current server startup |

Notes:

- `git.webhook_secret` remains required because the existing Gateway startup already depends on it.
- `git.base_url` should support self-hosted GitLab, not only `gitlab.com`.

## Phase 1 API Shape

The implementation should stay minimal, but it still needs a controlled Gateway write endpoint after confirmation.

Recommended write endpoint:

```text
POST /api/issues
```

Request body:

```json
{
  "project_id": 123,
  "title": "Draft onboarding issue",
  "description": "Created by issueflow agent"
}
```

Response body should stay intentionally small and return the basic created issue identity, for example:

```json
{
  "id": 456,
  "iid": 12,
  "project_id": 123,
  "title": "Draft onboarding issue",
  "web_url": "https://gitlab.example.com/group/project/-/issues/12"
}
```

This endpoint is not meant to represent page CRUD. It is the final controlled write step used after the chat-driven draft/confirmation flow.

Status handling:

- `201 Created` for success
- `400 Bad Request` for invalid JSON or missing/empty required fields
- `500 Internal Server Error` for local configuration errors
- `502 Bad Gateway` for upstream GitLab API failures

## Web and Agent Interaction Shape

Phase 1 does not require a large frontend implementation, but it does require the framework boundary to be explicit.

### Web Role

- host the chat experience
- render structured draft content returned by the agent flow
- present the confirmation action
- display the created GitLab issue result

### Agent Role

- interpret user intent from chat
- decide when the conversation is ready to become an issue
- generate the structured issue draft
- wait for user confirmation before triggering final creation

### Gateway Role

- hold GitLab credentials
- validate final payload
- create the issue in GitLab
- return the created issue metadata

## Code Shape

Keep the implementation minimal and close to the current structure.

### Configuration

Update:

- `src/config/raw.rs`
- `src/config/sources.rs`
- `src/config.rs`

to support the additional GitLab API settings.

### GitLab Integration

Extend `src/gitlab/` with a minimal issue creation module that:

- initializes the `gitlab` crate client from Gateway config
- exposes a small function for creating one issue
- hides SDK details from HTTP handlers as much as practical without over-abstracting

### HTTP Layer

Add:

- a request/response type for final issue creation
- a new handler under `src/http/handlers/`
- a new route in `src/http/routes.rs`

### Web Layer

Add only the minimum framework needed to represent the flow boundary:

- a chat-driven issue draft state
- a confirmation state
- a created-result state

The phase does not require a polished full UI. It only requires the interaction model to exist.

## Validation Rules

First version validation should stay minimal:

- `project_id` must be present and numeric
- `title` must be non-empty after trimming
- `description` may be empty but must still be accepted as a string field

No additional business rules are required in phase 1.

## Testing

Add focused tests for the smallest useful surface.

### Configuration Tests

Verify the new GitLab fields load from environment and/or TOML according to existing config precedence.

### HTTP Tests

Verify at least:

- request with empty title is rejected
- request with valid payload reaches the GitLab issue creation path
- configuration errors are surfaced correctly

### Web Flow Tests

Verify at least:

- a structured draft can be rendered in the web flow
- confirmation transitions to the final create step
- success state can display the created GitLab issue result

The phase does not require a live GitLab integration test. A narrow test seam around the issue creation call is acceptable for this stage.

## Documentation Changes

Update the local development documentation to explain:

- the new `GIT_BASE_URL`
- the new `GIT_TOKEN`
- that GitLab issue creation is chat-driven and finalized through the Gateway

README does not need a large expansion for this phase.

## Risks

- The `gitlab` crate API may impose slightly different request builder patterns than expected.
- The exact `ag-ui` / `a2ui` integration shape may need a minimal adapter layer in the web app.
- Current config requires `GIT_WEBHOOK_SECRET` even when only testing issue creation; this is acceptable for phase 1 but may be worth separating later.
- Without a broader auth model, the endpoint is Gateway-trusted rather than user-scoped. That is acceptable for the first controlled agent path.

## Success Criteria

Phase 1 is complete when:

- the web flow can represent chat-generated issue drafts
- the user can explicitly confirm the draft
- the Gateway starts with GitLab API configuration present
- the final confirmation path can create a GitLab issue through the `gitlab` crate
- invalid payloads fail with clear HTTP responses
- local development docs mention the extra config needed for issue creation
