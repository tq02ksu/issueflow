# README Positioning Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rewrite the root `README.md` into a short bilingual document that positions `issueflow` as an issue-handling agent guided by repository `skills` with a default fallback workflow.

**Architecture:** Replace the current broad platform narrative in `README.md` with four short bilingual sections that match the approved design. Keep the change limited to documentation, with direct verification by reading the resulting file and checking that the new fallback rules are described correctly.

**Tech Stack:** Markdown, repository documentation conventions

---

### Task 1: Replace the README body with the approved bilingual structure

**Files:**
- Modify: `README.md`
- Reference: `docs/superpowers/specs/2026-06-25-readme-positioning-design.md`

- [ ] **Step 1: Read the approved design before editing**

Read: `docs/superpowers/specs/2026-06-25-readme-positioning-design.md`
Expected: The spec confirms the four target sections, bilingual structure, and the repository-first `skills` fallback model.

- [ ] **Step 2: Replace `README.md` with the new bilingual content**

Write the file content as follows:

```md
# issueflow

## 项目简介 / Overview

`issueflow` 是一个管理和推进 issue 的智能体。它按标准化、业界通用的工作流处理 issue，并通过 Git 仓库中的 `skills` 适配不同业务类型的处理方式。

`issueflow` is an agent for managing and advancing issues. It handles issues through standardized, industry-common workflows and adapts to different business domains through `skills` stored in Git repositories.

## 核心处理模型 / Core Workflow

- 分诊：识别 issue 类型、优先级和当前状态。
- Triage: identify the issue type, priority, and current state.
- 补信息：发现缺失上下文并推动补全。
- Information completion: detect missing context and drive it to completion.
- 方案确认：整理目标、约束和处理方案。
- Solution confirmation: align on goals, constraints, and the handling approach.
- 开发启动：在满足条件后推进实现工作。
- Development kickoff: move implementation forward once entry conditions are met.
- 结果回写：把处理结果、状态和关键结论回写到 issue 流程中。
- Result write-back: record outcomes, status, and key conclusions back into the issue flow.

## Skills 作用方式 / How Skills Guide Handling

`issueflow` 不假设所有 issue 都应该用同一种方式处理。不同业务类型、不同团队约定、不同仓库习惯，可以通过 Git 仓库中的 `skills` 告诉智能体应该如何理解和推进 issue。

`issueflow` does not assume every issue should be handled the same way. Different business domains, team conventions, and repository habits can guide the agent through Git-hosted `skills` that define how issues should be understood and advanced.

issue 所在仓库是处理方式的首选来源。如果该仓库提供了相关 `skills`，智能体优先按这些 `skills` 执行。

The repository that owns the issue is the preferred source of handling guidance. If that repository provides relevant `skills`, the agent follows them first.

## 默认与覆盖关系 / Defaults and Overrides

平台提供默认的通用 issue 处理流程，作为没有业务定制时的基础行为。

The platform provides a default, general-purpose issue handling workflow as the baseline behavior when no business-specific customization is present.

如果 issue 所在仓库没有提供对应的 `skills`，`issueflow` 就回退到这个默认流程；如果仓库内存在合适的 `skills`，这些 `skills` 就覆盖默认方式，定义该类 issue 的具体处理规则。

If the repository that owns the issue does not provide matching `skills`, `issueflow` falls back to the default workflow. When suitable repository-local `skills` exist, they override the default behavior and define the concrete handling rules for that kind of issue.

这种设计让流程骨架保持一致，同时允许不同业务在同一个系统里保留各自的处理方式。

This model keeps the workflow skeleton consistent while allowing different businesses to preserve their own handling styles within the same system.
```

- [ ] **Step 3: Verify the README content by reading it back**

Run: read `README.md`
Expected: The file contains only the short bilingual structure with the four approved sections.

- [ ] **Step 4: Verify that the old broad framing is gone**

Run: search `README.md` for `Agent Definition|platform-skill-repo|project-skill-repo|架构总览|仓库结构`
Expected: No matches.

- [ ] **Step 5: Commit**

```bash
git add README.md docs/superpowers/specs/2026-06-25-readme-positioning-design.md docs/superpowers/plans/2026-06-25-readme-positioning-implementation.md
git commit -m "docs: narrow README positioning"
```

### Task 2: Final verification of the documentation change

**Files:**
- Verify: `README.md`
- Verify: `docs/superpowers/specs/2026-06-25-readme-positioning-design.md`
- Verify: `docs/superpowers/plans/2026-06-25-readme-positioning-implementation.md`

- [ ] **Step 1: Re-read the spec and confirm coverage**

Read: `docs/superpowers/specs/2026-06-25-readme-positioning-design.md`
Expected: Every approved requirement maps to the new `README.md` content.

- [ ] **Step 2: Re-read the plan and confirm no placeholders remain**

Read: `docs/superpowers/plans/2026-06-25-readme-positioning-implementation.md`
Expected: No `TODO`, `TBD`, or vague instructions remain.

- [ ] **Step 3: Run the final README verification search**

Run: search `README.md` for `issueflow is an agent|管理和推进 issue|默认与覆盖关系|Defaults and Overrides`
Expected: Matches confirm the approved bilingual positioning is present.

- [ ] **Step 4: Commit**

```bash
git add README.md docs/superpowers/specs/2026-06-25-readme-positioning-design.md docs/superpowers/plans/2026-06-25-readme-positioning-implementation.md
git commit -m "docs: finalize bilingual README positioning"
```
