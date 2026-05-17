# AGENTS.md Design

## Goal

Create a repository-level `AGENTS.md` for `issueflow` that gives coding agents short, enforceable execution rules for working in this repository.

## Scope

This document is not a full architecture guide and not a product spec. It is a working agreement for agents making code changes in this repository.

The `AGENTS.md` should focus on current facts and stable repository rules:

- project purpose at MVP level
- current technology choices
- repository layout and ownership boundaries
- coding and change-management rules
- test and command expectations
- git hygiene and safety constraints
- near-term implementation priorities

## Non-Goals

The `AGENTS.md` should not:

- restate the full GitLab robot design spec
- describe future modules in excessive detail
- duplicate implementation-plan task lists
- become a long-form onboarding document

## Audience

Primary audience: coding agents operating directly in the `issueflow` repository.

Secondary audience: human contributors who need a fast summary of repository working rules.

## Proposed Structure

The file should contain these sections:

1. `Purpose`
2. `Current State`
3. `Repo Layout`
4. `Working Rules`
5. `Testing`
6. `Git Hygiene`
7. `Near-Term Priorities`

## Content Requirements

### Purpose

State that `issueflow` is the MVP repository for a GitLab development robot.

### Current State

Capture only currently approved architectural facts:

- `Robot Gateway` uses Rust
- GitLab CI is the main execution plane for robot jobs
- `OpenCode Runtime Image` is shared CI infrastructure, not a standalone business service
- `Agent Workbench` is planned as Vue 3 + Naive UI
- Gateway confirmation and status pages remain lightweight server-rendered pages

### Repo Layout

Document present and planned top-level areas with brief responsibility statements:

- `src/` for Rust Gateway code
- `tests/` for Rust integration tests
- `internal/pages/templates/` for lightweight Gateway pages
- `scripts/robot/` for GitLab CI robot scripts
- `runtime/opencode/` for shared runtime image assets
- `web/` for the future workbench frontend

Sections for planned directories should be explicitly marked as planned so agents do not assume they already exist.

### Working Rules

Include concise rules that match current repository expectations:

- prefer minimal correct changes
- follow established patterns before introducing new abstractions
- do not refactor unrelated code opportunistically
- keep Gateway logic in Rust, not in ad hoc shell or frontend code
- keep lightweight Gateway pages separate from the future workbench frontend

### Testing

State current command expectations clearly:

- Rust commands should use `PATH="$HOME/.cargo/bin:$PATH"` when `cargo` is not otherwise available on `PATH`
- run focused tests for touched areas first, then broader verification when relevant

Include the currently valid example command:

`PATH="$HOME/.cargo/bin:$PATH" cargo test status_route_returns_ok -- --exact`

### Git Hygiene

State operational safety rules:

- do not commit `target/`
- do not revert or overwrite unrelated user changes
- avoid destructive git commands unless explicitly requested
- keep commits scoped to the work performed

### Near-Term Priorities

Reflect the current repository phase:

- the repository is still in early bootstrap
- near-term focus is building the Rust Gateway skeleton and core workflow pieces
- CI automation, runtime image, and workbench should follow the Gateway foundation rather than being expanded prematurely

## Style

The `AGENTS.md` should be short, direct, and imperative. It should read like repository operating instructions, not like marketing copy or a design narrative.

## Acceptance Criteria

The resulting `AGENTS.md` is acceptable when:

- an agent can understand the repo's current direction in under two minutes
- the file distinguishes current facts from planned structure
- the file gives concrete command and safety guidance
- the file is short enough to stay maintained as the repo evolves
