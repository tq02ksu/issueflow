# AGENTS.md

## Purpose

`issueflow` is the MVP repository for a GitLab development robot.

## Current State

- `Robot Gateway` is implemented in Rust.
- GitLab CI is the main execution plane for robot jobs.
- `OpenCode Runtime Image` is shared CI infrastructure, not a standalone business service.
- `Agent Workbench` is planned as a Vue 3 + Naive UI frontend.
- Gateway confirmation and status pages should remain lightweight server-rendered pages.

## Repo Layout

- `src/`: Rust Gateway application code.
- `tests/`: Rust integration tests.
- `internal/pages/templates/`: lightweight Gateway HTML templates.
- `scripts/robot/` (planned): GitLab CI robot job scripts.
- `runtime/opencode/` (planned): shared runtime image assets and entrypoints.
- `web/` (planned): Agent Workbench frontend.

## Working Rules

- Prefer the smallest correct change.
- Follow existing patterns before introducing new abstractions.
- Do not opportunistically refactor unrelated code.
- Keep Gateway logic in Rust instead of moving behavior into ad hoc shell scripts or frontend code.
- Keep Gateway lightweight pages separate from the future workbench frontend.
- Distinguish current code from planned structure when editing docs or code.

## Testing

- Use `PATH="$HOME/.cargo/bin:$PATH"` for Rust commands when `cargo` is not on `PATH`.
- Run focused tests for touched areas first, then broader verification when the scope justifies it.
- Current example command: `PATH="$HOME/.cargo/bin:$PATH" cargo test status_route_returns_ok -- --exact`

## Git Hygiene

- Do not commit `target/`.
- Do not overwrite or revert unrelated user changes.
- Avoid destructive git commands unless the user explicitly requests them.
- Keep commits scoped to the work performed.

## Near-Term Priorities

- The repository is still in early bootstrap.
- Near-term focus is the Rust Gateway skeleton and core workflow pieces.
- CI automation, runtime image, and workbench should follow the Gateway foundation rather than expanding prematurely.
