# README Positioning Design

## Goal

Rewrite the root `README.md` so the project is described as a focused issue-handling agent system rather than a broad platform narrative.

## Confirmed Direction

- The README should describe `issueflow` as an agent for managing and advancing issues.
- The README should say that issues are handled through standardized, industry-common workflows.
- Different business domains should be guided by `skills` stored in Git repositories.
- The repository that contains the issue determines the preferred handling approach.
- If that repository does not provide relevant `skills`, `issueflow` falls back to platform-provided default handling.
- The README should be bilingual, with Chinese and English presented together.
- The README should stay short and should not include the previous large platform framing.
- Sections about current implementation scope and repository structure are intentionally excluded.

## Target README Structure

1. `# issueflow`
2. `项目简介 / Overview`
3. `核心处理模型 / Core Workflow`
4. `Skills 作用方式 / How Skills Guide Handling`
5. `默认与覆盖关系 / Defaults and Overrides`

## Content Rules

- Each section should present Chinese first and English immediately after it.
- The English text should match the Chinese meaning closely, but does not need to be a rigid sentence-by-sentence translation.
- Keep terminology stable across both languages, especially `issue`, `skills`, `workflow`, `default`, and `override`.
- Remove or avoid concepts that make the scope sound larger than intended, including:
  - broad platform positioning
  - dual skill repo model
  - platform-level versus project-level repository architecture
  - large system context diagrams
  - extended product strategy discussion
  - `Agent Definition` framing

## Section Intent

### 项目简介 / Overview

State in one short bilingual paragraph that `issueflow` is an issue management and issue progression agent, and that it uses standardized workflows plus Git-hosted `skills` to adapt handling to different business types.

### 核心处理模型 / Core Workflow

List the main workflow stages in bilingual bullet form:

- triage
- information completion
- solution confirmation
- development kickoff
- result write-back

### Skills 作用方式 / How Skills Guide Handling

Explain that the issue's repository is the first source of handling guidance. Repository-local `skills` tell the agent how to process issues for that business context.

### 默认与覆盖关系 / Defaults and Overrides

Explain the fallback model clearly:

- use repository `skills` first
- if absent, use the platform default workflow
- defaults provide consistency
- repository `skills` provide business-specific behavior

## Non-Goals

- No attempt to document the full architecture.
- No attempt to describe every current subsystem.
- No expansion into a product manifesto.
- No code or behavior changes in this task.

## Verification

Verification for this task is a direct read of the updated `README.md` confirming:

- bilingual structure is present
- the four approved sections are present
- large platform framing is removed
- fallback behavior is described correctly
