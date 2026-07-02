# Loop Engineering Model for issueflow

## Goal

Define how `issueflow` uses `loop engineering` as its product model:

- what a loop means inside `issueflow`
- which parts of the system are fixed platform behavior
- which parts are configurable by role, skill, and policy
- which work is appropriate to enter the system
- how this boundary differs from generic task tracking

This document is a product-boundary spec.
It is intentionally smaller than a full architecture design.

## Core Product Statement

`issueflow` is not a generic work tracker.
It is a loop engineering system for software delivery.

The core claim is:

> `issueflow` does not merely store work. It defines and runs the loops that keep work moving toward execution, review, and completion.

This means the system should be designed around repeatable advancement loops rather than around static ticket storage.

## What "Loop Engineering" Means Here

For `issueflow`, a `loop` is a bounded advancement mechanism around a work object.

Each loop must define:

- a `trigger`
- a `target object`
- a `current state`
- a `goal state`
- a `verification rule`
- a `memory write policy`
- a `stop rule`
- a `handoff rule`

The loop is not just "run AI again."
It is a controlled progression unit that decides whether the object should:

- advance
- stay in place
- request more information
- escalate to a human
- hand off to an external coding agent

## Product Boundary

`issueflow` should only manage work that is intended to enter an execution loop.

This is a hard product principle.

### Appropriate Work

Good system inputs:

- issues intended to enter implementation
- requirements that need clarification before execution
- MRs that need active review progression
- milestones that need pressure and prioritization across linked work

These objects benefit from:

- state progression
- verification rules
- explicit next actions
- durable workbench memory

### Inappropriate Work

Work that should usually stay outside the system in V1:

- raw brainstorming
- very early product discovery
- broad strategic exploration
- open-ended idea capture with no execution target

This work is still valuable, but it is not yet suitable for an execution loop.

If everything enters the system too early, the product collapses back into a generic tracker.

## System Design Rule

The platform should fix the loop skeleton, not freeze every team's exact workflow vocabulary.

This means:

- the existence of loops is built into the system
- the advancement logic shape is built into the system
- the exact policy thresholds are configurable

So `issueflow` should not create a separate loop engine for every individual user or issue.
Instead, it should provide stable loop families with configurable policy layers.

## Fixed vs Configurable

## Fixed Platform Behavior

These should be platform-level invariants:

- every managed object belongs to a loop family
- every loop has explicit states
- every loop produces a next action or a stop decision
- every loop writes durable memory
- every loop can be blocked, escalated, or paused
- every loop has a human-review boundary for sensitive transitions

These rules create product consistency.

## Configurable Behavior

These should remain configurable:

- what counts as "ready" for execution
- how strict acceptance and verification must be
- what review threshold an MR must satisfy
- how aggressively milestone pressure reprioritizes linked work
- what style of recommendation a role should produce
- what behavior bias a skill version should introduce

This lets teams adapt behavior without losing the shared loop model.

## Why Roles and Skills Exist

Roles and skills do not replace the loop.
They shape how the loop behaves.

### Role

A role defines:

- decision bias
- escalation style
- working style
- goal framing

Examples:

- an execution-first role pushes work forward quickly
- a quality-first role blocks sooner on missing acceptance or weak verification
- a planning-first role prefers clarification before execution handoff

### Skill

A skill modifies execution behavior inside a stable system skeleton.

Examples:

- stricter readiness checks
- more aggressive review follow-up
- different recommendation ordering
- different UI emphasis

Skill should tune behavior, not replace the platform loop structure.

## Core Loop Families

The first version of `issueflow` should define a small number of built-in loop families.

## 1. Issue Clarification Loop

Purpose:

- move an issue from ambiguity to execution readiness

Typical states:

- `new`
- `clarifying`
- `planned`
- `ready_for_execution`

Typical trigger:

- new issue enters a workbench
- issue changes materially
- milestone pressure or MR dependency reveals missing context

Verification focus:

- required description exists
- acceptance intent is explicit
- major missing context is surfaced

Stop rule:

- ready for execution
- blocked on external decision
- explicitly deferred by human

## 2. Issue Execution Loop

Purpose:

- keep a ready issue moving through implementation and completion

Typical states:

- `ready_for_execution`
- `in_execution`
- `blocked`
- `done`

Verification focus:

- next action is explicit
- blocking reason is visible
- coding handoff, test intent, or execution status is current

Stop rule:

- issue reaches done
- issue becomes blocked and requires escalation
- work is intentionally re-scoped or returned to clarification

## 3. MR Review Loop

Purpose:

- actively move code change work from draft to merge

Typical states:

- `draft`
- `in_review`
- `changes_requested`
- `ready_to_merge`
- `merged`
- `blocked`

Verification focus:

- review status is current
- unresolved feedback is visible
- merge readiness is explicit

Stop rule:

- merged
- blocked on review or external dependency
- returned to issue execution

## 4. Milestone Pressure Loop

Purpose:

- propagate delivery pressure back onto issues and MRs

This is a major differentiator for the product.

A milestone is not only a reporting bucket.
It is a pressure object that should influence:

- issue priority
- MR urgency
- escalation timing
- next-best-action ranking

Verification focus:

- blocked items are visible
- dominant stalled states are visible
- risk and slippage signals are explicit

Stop rule:

- milestone completes
- milestone is re-scoped
- human intervention changes milestone policy

## PM and Requirement Boundary

A product manager can absolutely use `issueflow`, but not every PM activity belongs inside it.

The rule is:

> PM work should enter `issueflow` when it is intended to move into an execution loop.

So the product manager may do discovery outside the system first.
Then, once a requirement needs structured clarification, execution readiness, review, or milestone pressure, it should enter `issueflow`.

This preserves system quality.

The system is not meant to replace all product thinking.
It is meant to operationalize the work that is now expected to advance.

## Why This Is Not Jira

Jira-style systems primarily model stored work plus workflow labels.

`issueflow` should model:

- active advancement logic
- explicit stop rules
- memory-backed progression
- cross-object pressure
- role-shaped decision bias

The difference is not that `issueflow` has issues and states.
The difference is that state is supposed to trigger controlled movement rather than passive display.

## First-Order Product Principle

This is the key product rule to carry into README, landing page, and future architecture:

> `issueflow` only manages work that is intended to enter an execution loop.

That principle protects the product from becoming:

- a generic backlog bucket
- a static planning board
- a weak wrapper around AI coding tools

## Implications for Future Pages

### Landing Page

Landing should explain:

- this is a loop engineering system
- it keeps work moving
- it is distinct from coding agents and static trackers

### Product Page

Product should explain:

- loop families
- role behavior
- memory
- milestone pressure
- skill overlays

### Engineering Page

Engineering should explain:

- state layer
- memory layer
- agent layer
- verification and stop rules
- cross-object pressure logic

## Summary

`issueflow` should not try to manage every kind of work.
It should manage the work that is expected to advance.

The platform owns the loop skeleton.
Roles, skills, and policy shape how that loop behaves.

That is the cleanest way to turn `loop engineering` into a product model rather than into a vague slogan.
