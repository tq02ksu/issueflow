# AGENTS.md Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a repository-level `AGENTS.md` that gives coding agents concise execution rules for working in `issueflow`.

**Architecture:** Keep the file short and repository-scoped. Document only current facts, stable working rules, and clearly marked planned areas so agents can act without confusing roadmap intent for implemented code.

**Tech Stack:** Markdown, Rust repository conventions, Git workflow hygiene

---

## File Structure

- Create: `AGENTS.md`
- Create: `docs/superpowers/plans/2026-05-17-agents-md-implementation.md`
- Reference: `docs/superpowers/specs/2026-05-17-agents-md-design.md`

### Task 1: Add Repository AGENTS.md

**Files:**
- Create: `AGENTS.md`
- Reference: `docs/superpowers/specs/2026-05-17-agents-md-design.md`

- [ ] **Step 1: Write the AGENTS.md content**

```md
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
```

- [ ] **Step 2: Create the file with the approved content**

Use `apply_patch` to add `AGENTS.md` exactly as specified in Step 1.

- [ ] **Step 3: Verify the file contents**

Run: `Read /home/tq02ksu/workspace/tq02ksu/issueflow/AGENTS.md`
Expected: The file includes the seven approved sections and clearly marks planned directories as planned.

- [ ] **Step 4: Commit**

```bash
git add AGENTS.md docs/superpowers/plans/2026-05-17-agents-md-implementation.md
git commit -m "docs: add repository agent instructions"
```
