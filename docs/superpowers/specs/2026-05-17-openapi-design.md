# OpenAPI Design

## Goal

Add a repository-level `openapi.yaml` to `issueflow` that documents the Robot Gateway HTTP contract for handoff.

The file should cover both:

- currently implemented endpoints
- near-term planned endpoints already established in the design documents

The specification must clearly distinguish implemented operations from planned ones.

## Scope

The `openapi.yaml` is a contract and handoff artifact for the Gateway surface.

It should:

- live at the repository root as `openapi.yaml`
- describe the HTTP routes owned by `Robot Gateway`
- reflect current implementation honestly
- include near-term planned endpoints that are already part of the approved design
- give downstream contributors one canonical entry point for API understanding

## Non-Goals

The initial `openapi.yaml` should not:

- describe internal GitLab APIs consumed by the Gateway
- describe GitLab CI job internals
- describe OpenCode runtime image behavior
- invent unapproved endpoints not already grounded in the design
- pretend that server-rendered HTML pages are JSON APIs

## Audience

Primary audience:

- engineers implementing the Rust Gateway
- engineers integrating the future Agent Workbench with the Gateway
- reviewers receiving a handoff of the repository state

Secondary audience:

- operators who need a quick inventory of externally exposed Gateway routes

## File Location

- `openapi.yaml`

The file should live at repository root so it is easy to discover during handoff and does not get buried in implementation docs.

## OpenAPI Version and Metadata

The file should use OpenAPI `3.1.0`.

Top-level metadata should include:

- title: `Issueflow Robot Gateway API`
- version: `0.1.0`
- a short description explaining that the spec mixes implemented and planned Gateway operations and uses a custom status marker

## Status Marking Strategy

Every operation must include a custom extension field:

- `x-issueflow-status: implemented`
- `x-issueflow-status: planned`

This field is required because the user explicitly wants one file containing both present and near-term routes while preserving handoff honesty.

The spec should also mention this convention in the top-level description.

## Endpoint Set

### Implemented Now

#### `GET /status/ping`

Purpose:

- lightweight liveness endpoint for the Rust Gateway bootstrap

Contract:

- response `200`
- content type `text/plain`
- example body: `ok`
- `x-issueflow-status: implemented`

### Planned Near-Term

#### `POST /webhooks/gitlab`

Purpose:

- receive GitLab webhook events

Contract expectations for the initial contract version:

- request body content type `application/json`
- minimal schema should cover `object_kind` and selected `object_attributes` fields used by the Gateway foundation
- response `202` when accepted
- response `401` when webhook secret validation fails
- `x-issueflow-status: planned`

#### `GET /confirm/plan/{token}`

Purpose:

- serve lightweight plan confirmation result or confirmation page

Contract expectations:

- path parameter `token`
- response content type `text/html`
- response `200`
- `x-issueflow-status: planned`

#### `GET /status/session/{session_id}`

Purpose:

- serve lightweight session status page for an issue, MR, or release session

Contract expectations:

- path parameter `session_id`
- response content type `text/html`
- response `200`
- `x-issueflow-status: planned`

#### `GET /confirm/release/{token}`

Purpose:

- reserve the release confirmation page route already implied by the design direction

Contract expectations:

- path parameter `token`
- response content type `text/html`
- response `200`
- `x-issueflow-status: planned`

## Tags

Use simple tags to keep the file readable:

- `Status`
- `Webhooks`
- `Confirmations`

## Schemas

The initial components section should stay minimal.

### Required Schemas

#### `GitlabWebhook`

Minimal JSON object for handoff purposes:

- `object_kind: string`
- `object_attributes: object`

`object_attributes` should include minimal optional fields that match near-term Gateway parsing needs:

- `note: string`
- `noteable_type: string`

This schema is intentionally incomplete relative to the full GitLab event surface.

#### `ErrorResponse`

Simple JSON error payload for machine endpoints:

- `message: string`

This is mainly for `401` or future machine-readable errors on webhook endpoints.

## Content-Type Rules

The `openapi.yaml` must preserve the difference between machine and human endpoints:

- machine endpoints like webhooks use `application/json`
- lightweight Gateway pages use `text/html`
- liveness endpoint uses `text/plain`

This distinction matters because the design explicitly keeps lightweight Gateway pages separate from the future workbench frontend and separate from JSON APIs.

## Style

The file should be concise and practical:

- short summaries
- explicit response codes
- minimal schemas only
- no speculative auth models beyond current webhook secret handling
- no placeholder `TBD` text

## Acceptance Criteria

The handoff-ready `openapi.yaml` is acceptable when:

- it exists at repository root
- it documents `GET /status/ping`
- it includes the planned Gateway routes already established by design
- each operation is clearly marked as `implemented` or `planned`
- it uses honest content types for plain text, HTML, and JSON endpoints
- it does not claim more implementation progress than the repository actually has
