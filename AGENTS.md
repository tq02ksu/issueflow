# AGENTS.md

## Purpose

`issueflow` is the MVP repository for the Rust Gateway and control-plane foundation of an agent orchestration platform where `Anthropic SKILLS` are first-class and Git is the storage and history system for both platform-level and project-level skills.

## Current State

- `Robot Gateway` is implemented in Rust.
- The platform direction is a generalized agent orchestration model with a two-layer `skill repo` structure: a platform-level repo for system-wide skills and rules, and project-level repos for individual software systems, their issues, repo maps, and durable project context.
- Code hosting and CI platforms are not permanently limited to a single vendor, but the current primary supported path is `GitLab + OpenCode`.
- GitLab CI is the main execution plane for robot jobs today.
- `OpenCode Runtime Image` is shared CI infrastructure, not a standalone business service.
- `Agent Workbench` is planned as a Vue 3 + Naive UI frontend.
- Gateway confirmation and status pages should remain lightweight server-rendered pages.
- Gateway persistence should target `PostgreSQL` in production and use embedded `SQLite` for default integration-test workflows.

## Repo Layout

- `src/`: Rust Gateway and control-plane application code.
- `tests/`: Rust integration tests.
- `internal/pages/templates/`: lightweight Gateway HTML templates.
- `scripts/robot/integrations/gitlab-ci/`: GitLab CI integration template, job wrapper, and usage docs.
- `scripts/robot/core/` (planned): platform-agnostic robot task entrypoints and shared workflow logic.
- `runtime/opencode/` (planned): shared OpenCode runtime assets and entrypoints used by robot executors.
- `web/` (planned): Agent Workbench frontend.

## Working Rules

- Prefer the smallest correct change.
- Follow existing patterns before introducing new abstractions.
- Do not opportunistically refactor unrelated code.
- Keep Gateway logic in Rust instead of moving behavior into ad hoc shell scripts or frontend code.
- Keep current implementation claims aligned with the actual shipped Rust Gateway scope; describe broader platform direction as planned or evolving unless it already exists in code.
- Keep workflow logic separate from CI-platform-specific adapters when planning `scripts/robot/` code.
- Keep Gateway lightweight pages separate from the future workbench frontend.
- Distinguish current code from planned structure when editing docs or code.
- Prefer `sqlx` over heavier ORM layers for Gateway persistence unless requirements clearly outgrow SQL-first access.
- Keep persistence design compatible with both production `PostgreSQL` and default integration-test `SQLite`; avoid unnecessary database-specific features.

## Testing

- Use `PATH="$HOME/.cargo/bin:$PATH"` for Rust commands when `cargo` is not on `PATH`.
- Run focused tests for touched areas first, then broader verification when the scope justifies it.
- Current example command: `PATH="$HOME/.cargo/bin:$PATH" cargo test status_route_returns_ok -- --exact`
- Prefer integration tests when they can cover the behavior without excessive setup; reserve unit tests for pure logic that does not benefit from database or router wiring.
- Default developer-facing integration tests should run against embedded `SQLite` so they work without a local `PostgreSQL` environment.
- Production remains `PostgreSQL`; keep a small set of `PostgreSQL` checks for migrations or critical queries when features rely on behavior that `SQLite` cannot validate confidently.

## Git Hygiene

- Do not commit `target/`.
- Do not overwrite or revert unrelated user changes.
- Avoid destructive git commands unless the user explicitly requests them.
- Keep commits scoped to the work performed.

## Near-Term Priorities

- The repository is still in early bootstrap.
- Near-term focus is the Rust Gateway foundation, core workflow pieces, and documentation for the platform-level and project-level `skill repo` model.
- CI automation, runtime image, and workbench should follow the Gateway foundation rather than expanding prematurely.
