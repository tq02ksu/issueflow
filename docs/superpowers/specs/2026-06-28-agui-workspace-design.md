# AG-UI Workspace Extraction Design

## Goal

Extract the AG-UI implementation from the monolithic `issueflow` crate into a
small Rust workspace structure that supports internal reuse now and future
cross-project reuse later, while keeping the current product behavior intact.

## Scope

This design covers the Rust backend only. It does not split the Vue frontend,
does not introduce a standalone publish pipeline, and does not yet create
separate crates for OpenAI, encoder-only transport, or A2UI toolkit features.

## Target Package Layout

The repository will move from a single Rust package to a Cargo workspace with
four members:

1. `crates/agui-protocol`
2. `crates/agui-runtime`
3. `crates/agui-axum`
4. root application crate `issueflow`

The root application remains the deployable service. The three `agui-*` crates
become internal SDK-style layers with stable boundaries.

## Package Responsibilities

### `agui-protocol`

Purpose: pure AG-UI protocol and surface contract definitions.

Contents:
- AG-UI event enums and payload structs
- protocol DTOs shared across runtime and transport
- capability and metadata types when needed
- serialization-facing helpers that do not depend on a web framework

Rules:
- no `axum`
- no `sqlx`
- no `async-openai`
- no `issueflow` business types
- keep dependencies minimal: `serde`, `serde_json`, and similarly small support crates only

This crate is the most reusable and should be treated as the most stable public
boundary.

### `agui-runtime`

Purpose: provider-driven AG-UI run execution and event orchestration.

Contents:
- provider delta model
- run loop and tool round orchestration
- prompt assembly interfaces
- tool execution interfaces
- conversion from provider output into AG-UI protocol events
- default OpenAI-backed provider implementation using `async-openai`

Rules:
- may depend on `agui-protocol`
- must not depend on `axum`
- must not depend on `sqlx`
- must not depend on `issueflow` GitLab/session/workbench logic
- `async-openai` is allowed internally for now, but must not dominate the public API design

Public API direction:
- define runtime-facing traits and request/response structs in terms of AG-UI behavior, not OpenAI-specific types
- keep OpenAI-specific wiring behind runtime-owned adapters

### `agui-axum`

Purpose: HTTP/SSE transport glue for exposing AG-UI runtime behavior through Axum.

Contents:
- SSE event encoding for AG-UI event streams
- `axum::response::sse::Event` bridge
- handler helpers and response adapters used by AG-UI endpoints
- transport-specific error shaping where required

Rules:
- may depend on `agui-protocol` and `agui-runtime`
- may depend on `axum`
- must not depend on `sqlx`
- must not contain `issueflow` business logic

This crate acts as the current transport/formatting layer. If the project later
needs multiple transport encoders, this is the future split point for an
independent `agui-encoder` crate.

### `issueflow`

Purpose: product-specific business application.

Contents:
- GitLab integration
- session/auth integration
- database persistence
- workbench and workflow rules
- agent tool definitions and business adapters into `agui-runtime`
- Axum route registration that composes `agui-axum` with application handlers

Rules:
- owns all product/business state
- consumes `agui-*` crates instead of defining AG-UI protocol/runtime types locally

## Dependency Boundaries

Allowed dependency direction:

`issueflow` -> `agui-axum` -> `agui-runtime` -> `agui-protocol`

Also allowed:

`issueflow` -> `agui-runtime`
`issueflow` -> `agui-protocol`
`agui-axum` -> `agui-protocol`

Not allowed:

- `agui-protocol` depending on anything above it
- `agui-runtime` depending on `agui-axum`
- any `agui-*` crate depending on `sqlx`
- any `agui-*` crate depending on GitLab/business workflow modules

## Mapping from Current Code

Initial move targets:

- `src/agent/events.rs` -> `agui-protocol`
- AG-UI-facing request/response types that are not DB rows -> `agui-protocol` or `agui-runtime`, depending on whether they are transport-neutral
- `src/agent/openai.rs` -> `agui-runtime`
- AG-UI run loop parts of `src/agent/orchestrator.rs` -> `agui-runtime`
- SSE encoding pieces currently mixed into agent/web handlers -> `agui-axum`

Code that should remain in `issueflow`:

- `src/agent/gitlab_tools.rs`
- `src/agent/runs.rs`
- `src/agent/sessions.rs`
- `src/http/handlers/agent_handler.rs` business/session/db decisions
- `src/gitlab/*`
- `src/session/*`
- `src/workflow/*`

Important distinction:
- database-backed agent session/run persistence is product logic and stays in `issueflow`
- AG-UI event modeling and streaming mechanics move out

## Migration Strategy

The extraction should be incremental and keep the application buildable at each stage.

### Phase 1: Workspace skeleton

- convert the repository to a Cargo workspace
- keep the root `issueflow` package as a workspace member
- add empty `agui-protocol`, `agui-runtime`, and `agui-axum` crates
- move shared dependency declarations to workspace scope where useful

### Phase 2: Protocol extraction

- move AG-UI event types and pure DTOs into `agui-protocol`
- update `issueflow` imports to consume protocol definitions from the new crate
- keep behavior unchanged

### Phase 3: Runtime extraction

- move provider delta handling and run orchestration code into `agui-runtime`
- introduce runtime traits/interfaces so `issueflow` passes in business-specific tool execution and prompt context
- keep `async-openai` inside `agui-runtime` for now

### Phase 4: Axum extraction

- move SSE/event response formatting into `agui-axum`
- refactor `issueflow` handlers to call transport helpers rather than building AG-UI SSE responses directly

### Phase 5: Cleanup

- remove old duplicate modules from the root crate
- tighten public APIs
- rename or split modules if any layer still leaks business concerns

## API Design Constraints

- Design reusable APIs in terms of AG-UI concepts, not `issueflow` naming.
- Do not expose `AppState`, DB pools, GitLab clients, or session structs from `agui-*` crates.
- Avoid embedding `axum::response::sse::Event` into runtime APIs.
- Keep OpenAI-specific request construction behind runtime adapters.
- Prefer small, explicit interfaces over generic context bags.

## Testing Strategy

- `agui-protocol`: unit tests for serialization and event shape
- `agui-runtime`: unit tests for provider delta translation, tool loop behavior, and error propagation
- `agui-axum`: focused tests for SSE formatting and handler/response glue
- `issueflow`: integration tests proving the application still serves agent routes and business flows correctly

The current integration tests around agent runs, session auth, and AG-UI event
streams should remain in `issueflow` and continue passing through the migration.

## Deferred Work

The following are intentionally deferred:

- splitting `async-openai` into a separate `agui-openai` crate
- introducing `agui-encoder` as a standalone crate
- extracting an A2UI toolkit crate
- publishing the new crates independently

These remain future options, but not part of this migration.
