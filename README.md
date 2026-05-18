# issueflow

`issueflow` is a development automation project focused on the workflow from `Issue` to `PR/MR`.

The project does not hard-code a single code hosting platform or CI platform as a permanent constraint. The current primary supported path is `GitLab + OpenCode`, with `GitLab CI` as the main execution plane today.

## Workflow Focus

The repository is centered on making `issue -> PR/MR` delivery more structured, automatable, and observable.

The intended workflow includes stages such as:

- issue intake and validation
- explicit start of development work
- plan generation and confirmation
- implementation and verification
- PR/MR status tracking and follow-up

The exact platform integrations can evolve over time, but the workflow model is the stable core.

## Current Support Position

- Code hosting and CI integrations are not treated as hard product limits.
- The main supported combination right now is `GitLab + OpenCode`.
- `GitLab CI` is the current primary robot execution plane.
- The repository should avoid implying that other platform integrations already exist unless they are implemented.

## Repository Layout

Current directories:

- `src/`: Rust Gateway application code.
- `tests/`: Rust integration tests.
- `internal/pages/templates/`: lightweight Gateway HTML templates.

Planned directories:

- `scripts/robot/core/`: platform-agnostic robot task entrypoints and shared workflow logic.
- `scripts/robot/platforms/gitlab-ci/`: GitLab CI adapters, job wrappers, and pipeline-facing integration.
- `runtime/opencode/`: shared OpenCode runtime assets and entrypoints used by robot executors.
- `web/`: planned Agent Workbench frontend.

## Current Implementation Status

- `Robot Gateway` is implemented in Rust.
- Gateway confirmation and status pages remain lightweight server-rendered pages.
- Gateway persistence targets `PostgreSQL` in production and embedded `SQLite` for default integration-test workflows.
- `Agent Workbench` is still planned rather than implemented.

## Near-Term Direction

- Keep the Gateway foundation lightweight and reliable.
- Keep workflow logic separate from CI-platform-specific adapters.
- Expand automation around the `issue -> PR/MR` flow before broadening platform coverage.
