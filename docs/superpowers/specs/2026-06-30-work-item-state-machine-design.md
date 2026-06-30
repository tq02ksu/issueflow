# Work Item State Machine Design

## Goal

Define the first core execution mechanism for the updated `issueflow` positioning:

- use `issue` as the first managed work item
- drive project advancement through an explicit work-item state machine
- let a light in-system agent plus `skills` control most non-coding work
- delegate heavy execution work such as code writing and test execution to external heavy agents like `OpenCode` or `Codex`

This defines a state-transition-centric design.

## Problem

The main project problem is not that AI cannot write code.
The real problem is that project work items do not move through a controlled execution model.

Typical failure modes:

- issues enter the system with weak context
- teams do not share a clear standard for what each stage means
- project state is implicit in chat, not explicit in work items
- there is no stable boundary between lightweight coordination work and heavy implementation work
- AI coding tools can write code, but nobody controls when an issue is actually ready to hand off

So the system should not optimize for generating one more issue report.
It should optimize for moving work items through explicit states.

## Core Product Claim

`issueflow` is not primarily a better issue form.
It is a work-item advancement system.

For V1, the first managed work item is `issue`.

The platform should answer:

> What state is this issue currently in, what is allowed next, and who or what should advance it?

## Product Boundary

This feature is not:

- a generic issue CRUD replacement
- a code generation engine
- a full autonomous delivery bot
- a test execution platform
- a milestone planner for V1

This feature is:

- a controlled issue state machine
- a light-agent orchestration layer
- a `skill`-driven decision layer
- a bridge to optional heavy agents when execution must leave the control plane

## Managed Work Item

V1 manages only `issue` as a work item.

The model must stay extensible so future work can apply the same architecture to:

- `merge request`
- `milestone`
- other project work items

This means work-item identity must stay generic, even though V1 only exercises `issue`.

## Work Item States

The state set is fixed as a platform enum:

- `new`
- `clarifying`
- `planned`
- `ready_for_execution`
- `in_execution`
- `blocked`
- `done`

These states are not redefined by `skills`.
`skills` control how items move through the state machine, not what states exist.

### State Meaning

#### `new`

The issue has entered the system but has not yet been actively interpreted.

#### `clarifying`

The issue needs more context, structure, decisions, or role input before planning or execution.

#### `planned`

The issue has a coherent plan or execution framing, but it is not yet approved for implementation handoff.

#### `ready_for_execution`

The issue is ready to be handed to an execution path.
This does not mean code has started yet.

#### `in_execution`

Execution is actively happening, whether by humans, light-agent managed steps, or an external heavy agent.

#### `blocked`

The issue cannot progress because of dependency, missing input, environment problems, or an explicit stop condition.

#### `done`

The issue's intended execution lifecycle is complete for the current workflow.

## State Machine Principle

The stable system mechanism is not a checklist of issue rules.
The stable mechanism is the state machine itself:

- current state
- allowed transitions
- transition reasons
- transition requirements
- transition side effects

The exact judgment for a transition should be controlled by the light agent plus `skills`, while the state enum itself stays fixed in the platform.

## Agent Model

## Light Agent

The light agent is the primary actor inside `issueflow`.

It should do almost everything except:

- writing code
- running heavyweight tests

Responsibilities include:

- read and organize issue context
- read issue notes, labels, milestones, and related project context
- load and apply `skills`
- evaluate current issue state
- propose or perform state transitions
- write structured memory
- prepare pending actions
- coordinate role viewpoints
- write non-heavy GitLab updates when allowed
- decide whether heavy execution is needed
- decide which heavy agent implementation is appropriate

The light agent is not a toy helper.
It is the main orchestration intelligence inside the system.

## Heavy Agent

Heavy agents are external execution engines.

Examples:

- `OpenCode`
- `Codex`
- future implementation-specific agents

Responsibilities include:

- heavy repository understanding
- code generation or code modification
- heavyweight execution work
- heavyweight testing work when needed

Heavy agents are optional and selected by need.
They are not the system core.

## Boundary Between Light and Heavy Agents

The key rule is:

- `issueflow` owns work-item progression
- heavy agents own heavy execution

This means the handoff boundary is explicit:

- the light agent decides whether an issue is ready to hand off
- the light agent decides why
- the light agent records the reasoning and state transition
- the light agent prepares or confirms the delegation action
- a heavy agent only enters once the system reaches the proper state

## Skill Model

`skills` should control advancement method, not just text generation style.

For V1, skills should govern:

1. what context is required in each issue state
2. what the light agent should do in each state
3. what conditions allow transition to the next state
4. what conditions require escalation to a heavy agent
5. what different stakeholder roles care about during advancement

Examples:

- a product-oriented skill can define what must be clarified before moving from `clarifying` to `planned`
- an engineering-oriented skill can define what must exist before moving from `planned` to `ready_for_execution`
- a delivery-oriented skill can define when an issue should move to `blocked`

So:

- the state machine gives hard boundaries
- `skills` give transition policy and execution method

## Memory Model

`engineering_memory` should store current structured understanding for one work item under one scope.

It must support four scopes:

- `system`
- `project`
- `workbench`
- `personal`

### Why Scopes Matter

The same issue can have:

- a project-shared current state understanding
- a workbench-local temporary orchestration view
- a personal note or follow-up
- future platform-level default policy memory

So memory must not be only project-level and must not be only personal.

## Work Item Identity

Memory must separate:

- which work item this record describes
- which scope owns this memory
- what kind of memory this is

Core concepts:

- `work_item_type`
  Which work-item category this memory describes.
  V1 uses `issue`.
- `work_item_id`
  Which specific work item inside that category is being described.
  For V1 this should map to GitLab issue identity used by the system.
- `scope_type` and `scope_key`
  Which scope owns the memory.
- `memory_kind`
  What kind of memory this is.

## Memory Kinds

V1 recommended memory kinds:

- `issue_state`
- `issue_context`
- `issue_note`
- `policy_note`

### V1 Intended Usage

- `project + issue_state`
  Shared current issue state and transition understanding
- `workbench + issue_context`
  Workbench-local structured context assembled by the light agent
- `personal + issue_note`
  User-specific notes or concerns
- `system + policy_note`
  Future default transition policy

## Current Issue State Memory

The structure should stay state-centered in memory and API design.

For `project + issue_state`, the memory should capture:

- current issue state
- proposed next state
- why the agent thinks that next state is appropriate
- what is missing
- whether heavy-agent escalation is needed
- recommended pending action

Recommended JSON shape:

```json
{
  "current_state": "clarifying",
  "proposed_next_state": "planned",
  "summary": "The issue has enough clarified product intent to move into planning.",
  "missing_context": [],
  "blockers": [],
  "role_notes": {
    "product": [],
    "engineering": [],
    "delivery": []
  },
  "heavy_agent": {
    "required": false,
    "reason": "",
    "preferred_implementation": null
  }
}
```

## Pending Action Model

`pending_actions` remains the write-control and transition-control bridge.

V1 should use it for actions such as:

- apply issue state update back to GitLab
- publish a clarification request comment
- publish a planning summary
- enqueue or authorize heavy-agent delegation

The important part is that work-item state advancement remains controlled and auditable.

## Primary Workflow

V1 flow:

1. A user opens a workbench and selects an issue.
2. The light agent reads current issue context and related project context.
3. The light agent loads relevant `skills`.
4. The light agent evaluates the current issue state and proposed next state.
5. The result is persisted as scoped memory.
6. The system prepares a pending action if a transition or write-back needs confirmation.
7. The user reviews and confirms when required.
8. If the next step requires heavy execution, the system records or triggers delegation to the selected heavy agent.

## API Direction

The API should be state-machine-oriented.

Recommended V1 shape:

- `POST /api/workbenches/{id}/issues/{issue_iid}/state/evaluate`
  Evaluate current state and next transition
- `GET /api/workbenches/{id}/issues/{issue_iid}/state`
  Read current shared state memory plus workbench/personal overlays
- existing pending action confirmation endpoints remain in use

Any transitional route prototypes should be removed before final implementation continues.

## UI Direction

The UI should show:

- current issue state
- proposed next state
- why
- what is missing or blocking
- whether heavy-agent escalation is recommended
- what confirmation action is waiting

This is more important than a static report.

## What Should Stay Hardcoded

Only stable platform boundaries should be hardcoded:

- the state enum
- allowed transition graph shape
- memory scope model
- pending action execution rules
- write-back confirmation boundaries

What should not be permanently hardcoded:

- the detailed semantic judgment for issue advancement
- the exact meaning of “enough context” for every project
- the exact heavy-agent escalation threshold

Those should be controlled by the light agent plus `skills`.

## Non-Goals

V1 should not attempt:

- full MR state machine support
- milestone state machine support
- full autonomous background delivery
- embedded heavy code execution
- replacing `OpenCode` or `Codex`

## Summary

The correct core for this feature is:

- explicit work-item state machine
- light agent as the primary in-system actor
- `skills` as transition policy and execution method
- optional heavy agents for implementation work
- scoped memory for shared and personal understanding
- pending actions for controlled transitions and write-back

This is the mechanism users will actually pay attention to, because it controls work-item advancement rather than merely scoring issue quality.
