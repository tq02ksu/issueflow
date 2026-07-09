# Role-Based Homepage & Role-Adaptive Dashboard Design

## Goal

Replace the prototype homepage's single "Open Prototype" entry with **four role entries**, and make the Workbench Overview (Dashboard) adapt its signal cards, quick entries, and emphasis to the selected role.

The four roles are:

1. 研发人员 (Developer)
2. 研发经理 & 架构设计师 (R&D Manager & Architect)
3. 产品设计师 (Product Designer)
4. 系统进化专家 (System Evolution Expert)

The change stays inside the frontend mock prototype (`isMockMode`). No backend work.

## Problem

The current homepage (`LandingView.vue`, mock branch) shows a hero diagram and one button into `/workbench`. It does not communicate that different roles use the same system for different purposes, and it does not match how a real project team actually splits concerns (execution vs. delivery/architecture vs. requirements vs. platform evolution).

`PROTOTYPE.md §10.12` establishes a hard constraint we must respect:

> **交互模型只有一套，不是多套。** All roles work in the same Workbench shell and walk the same Dashboard → Object/Turn Detail → Pending Action → approve path.

So role differentiation must be expressed as **content emphasis and entry points**, not as separate pages or separate interaction models. This maps directly to the doc's four unified sequences A/B/C/D (§10.12).

## Design Principles

- One interaction model, one shell. Roles only change emphasis, signal cards, and quick entries.
- The homepage becomes a role picker; picking a role sets an active role context and routes into `/workbench`.
- The role context is visible and switchable from the shell top bar (so the user is never trapped in one role).
- All role-specific content is mock-driven and lives in `prototype.data.ts` / a new role-profile module, following the existing `SkillUiProfile` pattern.
- Scope is bounded to: homepage + Workbench Overview Dashboard + shell role indicator. Other pages (Turns, Approvals, Memory, etc.) are unchanged in this iteration.

## Role → Content Mapping

Each role maps to a `PrototypeRoleView` with: mission, the sequence it drives, a set of signal cards, quick entries, and default workflow-state emphasis.

### 1. Developer (研发人员) — Project advancement loop, execution view (Sequence A)

- Cares about: issues/MRs assigned to me, pending actions I must confirm, in-flight turns, review queue, items I'm blocking.
- Signal cards: my in-execution work items · review queue (`changes_requested`) · my pending actions · blocked items · recent turns.
- Quick entries: Issues · MRs · Turns · Approvals.
- Emphasis: `in_execution`, `changes_requested`, `blocked`.

### 2. R&D Manager & Architect (研发经理 & 架构设计师) — Milestone pressure + governance + architecture + deployment (Sequence A + §10.6)

- Cares about: milestone pressure map, blocked items, verification debt, risk alerts, overall progress health, architecture consistency, **deployment/release readiness, gray-release and rollback plans, environment configuration (Environment Profiles), release risk**.
- Signal cards: milestone pressure · verification debt · risk alerts · **release/deployment readiness** · **environment health** · budget/health overview.
- Quick entries: Milestones · Governance · Environment Profiles (Settings) · Turns.
- Emphasis: `blocked` · milestone risk · release readiness.

### 3. Product Designer (产品设计师) — Clarification + requirement structuring + collaborative evolution (Sequence B + D)

- Cares about: issues awaiting clarification, acceptance-criteria quality, requirement readiness, collaborative-evolution SKILL change proposals, clarification-correction-rate trend.
- Signal cards: clarification queue · acceptance quality · requirement readiness · collaborative-evolution SKILL proposals · correction-rate trend.
- Quick entries: Issues · Approvals · Skills · Memory.
- Emphasis: `clarifying`, `new`, clarification debt.

### 4. System Evolution Expert (系统进化专家) — Evolution loop, system-evolution direction (Sequence C)

- Cares about: system-evolution proposals (`skill_evolution_proposal`), governance reports, loop success rate / reject rate / budget overrun / comprehension rot, Skill Registry, Policy.
- Signal cards: system-evolution proposals · loop health metrics · governance signals · Skill Registry status · policy config.
- Quick entries: Governance · Skills · Gateway · Turns.
- Emphasis: evolution proposals · loop health · governance.

## Architecture

### Naming

The existing per-workbench `PrototypeWorkbench.role` (`PrototypeRole`) is a workbench persona ("Execution Driver", "Product Strategist") and is unrelated. To avoid collision, the new concept is `PrototypeRoleView` with a stable `key`:

```ts
type PrototypeRoleKey =
  | "developer"
  | "manager"
  | "product"
  | "evolution";

interface PrototypeSignalCard {
  id: string;
  label: string;        // i18n key
  value: string;        // mock metric, e.g. "3", "2 blocked"
  tone: "neutral" | "attention" | "positive";
  hint: string;         // i18n key, short context line
}

interface PrototypeQuickEntry {
  id: string;
  label: string;        // i18n key
  to: string;           // route path
}

interface PrototypeRoleView {
  key: PrototypeRoleKey;
  name: string;             // i18n key
  tagline: string;          // i18n key, one line for homepage card
  mission: string;          // i18n key, dashboard hero description
  sequence: "A" | "B" | "D" | "C";
  signalCards: PrototypeSignalCard[];
  quickEntries: PrototypeQuickEntry[];
  overviewEmphasis: string[]; // workflow-state emphasis order
}
```

### Data

- Add `prototypeRoleViews: PrototypeRoleView[]` (4 entries) to `prototype.data.ts`.
- All labels/hints/taglines/missions are i18n keys under a new `prototype.roles.*` namespace in both `zh-CN.ts` and `en.ts`.
- Signal card values are static mock strings (consistent with the prototype's mock-only nature).

### Store (`prototype.store.ts`)

- Add `roleViews` (from mock) and `activeRoleKey` state (default `"developer"`).
- Add `activeRoleView` computed.
- Add `setActiveRole(key)` action; persist to `localStorage` (`issueflow_prototype_role`) so a refresh keeps the role, and so the homepage selection survives navigation into `/workbench`.
- Add `activeRoleEmphasis` used to reorder the issue/MR workflow summaries (reusing the existing `sort*StatesByProfile` mechanism by overlaying role emphasis on top of the skill UI profile emphasis — role emphasis takes precedence when present).

### Homepage (`LandingView.vue`, mock branch)

- Keep the brand top bar and the hero diagram (it communicates the loop engine well).
- Replace the single-button `landing__actions` area with a **role entry grid**: 4 cards, each with role name, tagline, an icon, and the 2–3 headline concerns.
- Clicking a card calls `store.setActiveRole(key)` and routes to `/workbench`.
- Cards use existing design tokens (`--if-*`) and match the prototype's warm card style.

### Shell (`AppShell.vue`, prototype mode)

- Replace/augment the existing `shell__chip` (currently shows workbench persona) with an **active-role indicator** that is a dropdown to switch roles without returning to the homepage.
- The left sider role summary block reflects the active role view's name + mission.

### Dashboard (`PrototypeWorkbenchOverview.vue` + `OverviewCards.vue`)

- The hero card's eyebrow/description reflect the active role's mission.
- Prepend a **role signal strip**: a row rendering `activeRoleView.signalCards` (compact metric cards with tone coloring).
- Add a **quick entries** row rendering `activeRoleView.quickEntries` as router links.
- Keep issue/MR workflow summary + recent activity cards, but reorder issue/MR summaries using role emphasis.
- No new routes; all quick entries point at existing prototype routes.

## Components

New:

- `web/src/components/prototype/RoleEntryGrid.vue` — homepage 4-role picker.
- `web/src/components/prototype/RoleSignalStrip.vue` — dashboard signal cards + quick entries.

Changed:

- `LandingView.vue`, `AppShell.vue`, `PrototypeWorkbenchOverview.vue`, `OverviewCards.vue`.
- `prototype.types.ts`, `prototype.data.ts`, `prototype.store.ts`, `prototype.ui-profile.ts` (role-emphasis overlay helper).
- `i18n/locales/zh-CN.ts`, `i18n/locales/en.ts`.

## Testing

Follow the existing `web/src/tests/mock-prototype-*.spec.ts` style (Vitest + component/store assertions):

- `mock-prototype-roles.spec.ts` (new):
  - homepage renders 4 role entry cards
  - clicking a card sets `activeRoleKey` and navigates to `/workbench`
  - store `setActiveRole` updates `activeRoleView` and persists to localStorage
  - each role view exposes non-empty `signalCards` and `quickEntries`
  - quick-entry routes all exist in the mock router
- `mock-prototype-overview.spec.ts` (extend): dashboard renders the active role's signal strip and mission; switching role changes the signal cards and issue/MR emphasis order.

Run: `npm --prefix web test -- --run` and the full quality gate before completion.

## Out of Scope

- Backend/role-based auth or permissions.
- Role-specific pages with bespoke layouts (Turns/Approvals/etc. reuse the same components).
- Default landing sub-page / menu-highlight per role (homepage + Dashboard only).

## Update: role-bound workbench datasets

A follow-up refinement makes each role bind to its own workbench dataset so the data seen after entering differs per role. `PrototypeRoleView` gains a `workbenchId`; `setActiveRole` also switches `currentWorkbenchId` to that workbench, reusing the existing workbench-scoped filtering. Four workbenches now exist: `alpha` (developer), `gamma` (manager), `beta` (product), `delta` (evolution), each with its own issues, MRs, milestones, activity, approvals, turns, and loops. The homepage entries are simplified to four plain buttons (no card detail).

## Doc Update

`docs/PROTOTYPE.md §10.12` currently states the four sequences and role-adaptive signal cards conceptually. This design realizes the homepage role-picker aspect; a short note will be added there pointing to this spec.
