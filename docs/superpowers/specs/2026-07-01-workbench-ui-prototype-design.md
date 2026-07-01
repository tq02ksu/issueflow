# Workbench UI Prototype Design

## Goal

Build a frontend-only, mock-driven prototype for `issueflow` that demonstrates how the system helps users advance a GitLab workbench through:

- `issues`
- `merge requests`
- `milestones`
- one workbench-bound role
- low-frequency user and workbench settings
- `skill` selection with version management
- skill-driven UI emphasis without changing page skeletons

The prototype should be operational enough to click through and understand the product, but it does not need real backend integration.

## Problem

The product needs a frontend that explains and demonstrates the updated `issueflow` positioning:

- this is a project advancement system for AI coding workflows
- the high-frequency user loop is workbench execution, not configuration
- configuration such as role, goals, and skills should exist, but should not dominate the primary workflow
- different skills should shape how the system presents priorities, but users should not lose orientation because the UI structure keeps shifting

So the prototype should not behave like a generic admin dashboard.
It should feel like a stable execution cockpit for long-session use by programmers and product managers.

## Core Product Claim

The prototype should communicate this product claim:

> `issueflow` helps users move a workbench forward by keeping issues, merge requests, and milestones in view, while role configuration and skills tune how the system prioritizes and presents execution.

## Product Boundary

This prototype is not:

- a real GitLab-integrated frontend
- a real upload pipeline for skills
- a real memory inspection tool
- a chat-first shell
- a fully dynamic layout engine

This prototype is:

- a clickable product prototype
- a mock-backed workbench cockpit
- a demonstration of workbench-level object management
- a demonstration of role and skill configuration
- a demonstration of skill-driven UI emphasis with fixed page skeletons

## Development Mode Boundary

The prototype should not hijack the existing frontend development flow.

Required development split:

- `npm run dev` remains the normal frontend development entry for the current application
- `npm run dev:mock` should start the mock-driven prototype experience

This keeps the prototype isolated and avoids forcing the mock workbench UI into the default development path before the team intentionally adopts it.

## Hard Constraints

The following rules are fixed:

- all main object pages are current-`workbench` scoped
- left navigation contains `Overview`, `Issues`, `MRs`, and `Milestones`
- `Settings` must not appear alongside those main workbench navigation items
- `Settings` is entered from a separate personal entry point, such as the header avatar/menu
- one workbench can only have one configured role
- `memory` is a system-internal runtime mechanism and is not normally visible
- users may perform low-frequency memory control actions such as clearing or rebuilding memory
- page skeletons must stay fixed
- skills may influence emphasis, density, tone, defaults, and recommendation ordering, but may not replace or reorder whole-page structure

## Information Architecture

The prototype should have two levels of experience:

1. a public landing page
2. an authenticated workbench application shell

### Public Landing

The landing page should explain:

- what `issueflow` is
- which workbench objects it manages
- how role and skills affect project advancement
- how the user works with the system in a short 3-step flow

The landing page is explanatory, not marketing-heavy.
It should orient users quickly and hand them into the workbench prototype.

### Authenticated Application Shell

The shell should include:

- a stable header with brand, current workbench context, and personal entry point
- a left navigation for workbench objects
- a main content area

Main navigation:

- `Overview`
- `Issues`
- `MRs`
- `Milestones`

Personal entry:

- `User Settings`
- memory control actions
- session actions such as sign out

## Page Design

## Overview

Purpose:

- show the current workbench state at a glance
- summarize role, goals, and active skill influence
- suggest the next best actions

Fixed skeleton:

- workbench summary header
- role summary card
- current goals card
- issue, MR, and milestone statistics
- recommended next steps
- recent activity

What can vary by skill:

- card density
- copy tone
- which fields are visually emphasized inside a card
- which recommendation appears first
- which sections are expanded by default

What cannot vary:

- the presence of the above overview sections
- their overall page-level skeleton

## Issues

Purpose:

- manage workbench issues in a high-frequency execution loop

Fixed skeleton:

- top filter and search bar
- list pane
- detail pane
- recommendation/action area inside the detail pane

Supported prototype interactions:

- filter by state
- search by title or keyword
- select an issue from the list
- switch among mock recommended actions
- view the currently emphasized fields under the active skill

Skill influence is limited to:

- field priority in list rows
- field priority inside issue detail cards
- tone of action labels and explanatory copy
- default expanded sections
- recommendation ordering

## MRs

Purpose:

- manage workbench merge requests as delivery objects, not just passive records

Fixed skeleton:

- top filter and search bar
- list pane
- detail pane
- recommendation/action area inside the detail pane

Supported prototype interactions:

- filter by review state or merge state
- select an MR
- inspect linked issue and delivery context
- inspect readiness and review checklist summaries
- switch among mock recommended actions

Skill influence is limited to:

- whether review risk or delivery progress gets stronger emphasis
- default focus inside readiness summaries
- copy tone
- recommendation ordering

## Milestones

Purpose:

- make milestone-level planning visible in the same workbench loop

Fixed skeleton:

- milestone list
- milestone detail view
- related issues and MRs summary
- risks and next-step recommendation area

Supported prototype interactions:

- filter milestones by state or timing
- select a milestone
- inspect linked issues and MRs
- inspect goal, scope, risk, and next-step summaries

Skill influence is limited to:

- whether timeline pressure, scope, or execution blockers are emphasized first
- density and wording
- recommendation ordering

## User Settings

Purpose:

- provide low-frequency configuration without interrupting the main execution workflow

The page should have two layers:

1. user-level settings
2. a smaller optional section for the current workbench

### User-Level Settings

Include:

- display identity
- default collaboration preferences
- high-level system preferences relevant to prototype UX

### Current Workbench Configuration Area

Include:

- current role selection and description
- ways of working
- primary goals
- active skills
- skill version selection
- enable or disable a skill version
- memory control actions such as clear or rebuild

This area is intentionally secondary to the main workbench object pages.

## Role Model

Each workbench has exactly one role configuration.

The role model should include:

- role name
- role persona summary
- ways of working
- explicit goals

This role shapes how recommendations and emphasis are framed across the workbench, but it does not create additional navigation or multi-role switching in V1 of the prototype.

## Skill Model

The prototype should support skill management as a user-visible concept.

Visible capabilities:

- list installed skills
- show multiple versions for a skill
- enable or disable a version
- choose the active version for the current workbench
- mock an upload flow

The prototype does not need to implement real package upload or parsing.
It only needs a believable UI flow with mock persistence in frontend state.

## Skill-Driven UI Model

The UI must support skill-driven UX adaptation under a strict rule:

> skills may influence emphasis, not page skeletons

So the frontend should model a lightweight `ui profile` for the active skill version.

Recommended fields:

- `tone`
- `density`
- `overview_emphasis`
- `issue_field_priority`
- `mr_field_priority`
- `milestone_field_priority`
- `default_expanded_sections`
- `recommended_action_order`

The profile should change:

- emphasis labels
- field order within cards
- default section expansion state
- recommendation ordering
- wording style

The profile must not change:

- route structure
- page skeletons
- navigation model
- core split-pane arrangement of object pages

## Memory Control Model

`memory` remains an internal mechanism.
The prototype should not expose raw memory records as a normal page.

Visible user actions:

- clear current workbench memory
- rebuild current workbench memory

These actions should include lightweight confirmation and warning language because they are low-frequency control actions.

## State and Data Model

All data is mock-only and frontend-local.

The prototype should include mock models for:

- users
- workbenches
- role profiles
- role goals
- skill definitions
- skill versions
- skill UI profiles
- issues
- merge requests
- milestones
- activity items

The data model should allow switching current workbench and immediately reflecting:

- different role config
- different active skill version
- different object lists
- different emphasis rules

## Interaction Boundaries

This prototype should support:

- route navigation
- workbench switching
- selecting issues, MRs, and milestones
- opening user settings
- editing role, work style, and goals in UI forms
- switching active skill version
- mock enabling and disabling skill versions
- mock upload entry
- confirming memory clear or rebuild

This prototype should not support:

- real network persistence
- real authentication dependence for the prototype flow
- real GitLab mutations
- real file upload
- real backend-side skill execution
- real memory inspection

## Visual Direction

The visual direction should optimize for long-session use:

- clear hierarchy over decorative effects
- information density without clutter
- strong scanability
- stable navigation
- comfortable contrast
- compact but readable cards and tables

This should feel like a deliberate workbench for execution, not a marketing page stretched into an app.

## Components

The implementation should stay modular and focused:

- app shell and header actions
- workbench navigation
- overview summary cards
- split-pane object views
- settings panels
- skill version selector and state controls
- confirmation dialogs for memory control

Each unit should have one clear purpose so the mock-driven prototype is easy to evolve later.

## Data Flow

At a high level:

1. user selects or switches workbench
2. frontend resolves mock workbench data
3. frontend resolves current role and active skill version
4. frontend resolves the active skill `ui profile`
5. current route renders a fixed page skeleton
6. page content applies skill-driven emphasis rules inside that skeleton

This keeps the system stable while still proving that skills can shape the experience.

## Error Handling

Because this is a prototype, error handling should focus on believable UI states:

- empty state when no issues, MRs, or milestones exist
- disabled or warning state when no skill version is active
- confirmation flows for destructive memory actions
- clear non-blocking inline feedback for mock save, enable, disable, and upload actions

The UI should never feel broken just because the backend is intentionally absent.

## Testing

Frontend testing should cover the prototype's important behaviors:

- route rendering for all main pages
- workbench switch updates visible context
- object page selection updates detail panes
- settings entry opens correctly outside main navigation
- skill version switching changes visible emphasis without changing page skeleton
- memory actions require confirmation

The test strategy should stay aligned with the mock-driven nature of the prototype.

## Delivery Strategy

Implementation should prioritize visible product coherence over completeness:

1. stabilize app shell and navigation
2. build mock data source and workbench switching
3. implement `Overview`
4. implement `Issues`, `MRs`, and `Milestones`
5. implement `User Settings`
6. implement skill-driven emphasis rules
7. refine copy, density, and empty states

## Delivery Notes

The implementation should include a separate frontend development command for the prototype:

- `npm run dev:mock`

This command should make it obvious that the user is running the prototype path rather than the main application path.

The result should be a believable product prototype that communicates the system clearly and supports basic click-through evaluation.
