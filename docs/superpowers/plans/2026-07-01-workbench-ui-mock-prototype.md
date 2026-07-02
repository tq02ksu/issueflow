# Workbench UI Mock Prototype Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a frontend-only, mock-driven workbench prototype that demonstrates workflow-centered `Issues`, `MRs`, and `Milestones`, plus isolated user settings with workbench role and skill controls.

**Architecture:** Keep the existing frontend intact for the normal application path and add a separate mock-prototype path behind `npm run dev:mock`. Use local mock modules plus a lightweight UI-profile layer so skills can change emphasis, tone, and defaults without changing page skeletons.

**Tech Stack:** Vue 3, Vue Router, Pinia, Naive UI, Vite, Vitest.

---

## File Map

### Existing files to modify

- `web/package.json`
- `web/vite.config.ts`
- `web/src/main.ts`
- `web/src/App.vue`
- `web/src/router/index.ts`
- `web/src/components/layout/AppShell.vue`
- `web/src/styles/tokens.css`
- `web/src/styles/main.css`
- `web/src/views/LandingView.vue`
- `web/src/views/WorkbenchView.vue`
- `web/src/views/IssuesView.vue`
- `web/src/tests/app.spec.ts`
- `web/src/tests/issues-view-list-state.spec.ts`
- `web/src/tests/workbench-sidebar.spec.ts`

### New files to create

- `web/src/mock/prototype.data.ts`
- `web/src/mock/prototype.types.ts`
- `web/src/mock/prototype.ui-profile.ts`
- `web/src/stores/prototype.store.ts`
- `web/src/views/MrsView.vue`
- `web/src/views/MilestonesView.vue`
- `web/src/views/UserSettingsView.vue`
- `web/src/components/prototype/OverviewCards.vue`
- `web/src/components/prototype/ObjectSplitView.vue`
- `web/src/components/prototype/WorkflowStateBadge.vue`
- `web/src/components/prototype/RecommendedActionsCard.vue`
- `web/src/components/prototype/UserMenu.vue`
- `web/src/components/prototype/WorkbenchRolePanel.vue`
- `web/src/components/prototype/SkillVersionPanel.vue`
- `web/src/components/prototype/MemoryControlPanel.vue`
- `web/src/tests/mock-prototype-routing.spec.ts`
- `web/src/tests/mock-prototype-overview.spec.ts`
- `web/src/tests/mock-prototype-issues.spec.ts`
- `web/src/tests/mock-prototype-mrs.spec.ts`
- `web/src/tests/mock-prototype-milestones.spec.ts`
- `web/src/tests/mock-prototype-settings.spec.ts`

### Responsibility split

- `web/src/mock/*`: mock domain data, workflow states, and skill-driven UI profiles
- `web/src/stores/prototype.store.ts`: local state for current workbench, selected object rows, active skill version, and settings mutations
- `web/src/components/prototype/*`: reusable prototype-only UI blocks
- `web/src/views/*View.vue`: fixed page skeletons for landing, overview, issues, MRs, milestones, and settings
- `web/src/router/index.ts`: route gating between the normal app and mock prototype mode
- `web/package.json` + `web/vite.config.ts`: separate prototype dev entry via `npm run dev:mock`

## Task 1: Add isolated mock-prototype mode

**Files:**
- Modify: `web/package.json`
- Modify: `web/vite.config.ts`
- Modify: `web/src/main.ts`
- Modify: `web/src/App.vue`
- Create: `web/src/tests/mock-prototype-routing.spec.ts`
- Test: `web/src/tests/mock-prototype-routing.spec.ts`

- [ ] **Step 1: Write the failing routing-mode test**

```ts
import { describe, expect, it, vi } from "vitest";

vi.stubEnv("VITE_APP_MODE", "mock");

describe("mock prototype mode", () => {
  it("keeps the mock routes public in mock mode", async () => {
    const { router } = await import("@/router/index");
    const names = router.getRoutes().map((route) => route.name);

    expect(names).toContain("mock-landing");
    expect(names).toContain("mock-workbench-overview");
    expect(names).toContain("mock-workbench-issues");
    expect(names).toContain("mock-workbench-mrs");
    expect(names).toContain("mock-workbench-milestones");
    expect(names).toContain("mock-user-settings");
  });
});
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-routing.spec.ts`
Expected: FAIL because mock routes do not exist yet.

- [ ] **Step 3: Add the mock-mode scripts and Vite env handling**

```json
{
  "scripts": {
    "dev": "vite",
    "dev:mock": "vite --mode mock"
  }
}
```

```ts
const appMode = process.env.VITE_APP_MODE;
export default defineConfig({
  plugins: [vue()],
  define: {
    __APP_MODE__: JSON.stringify(appMode ?? "default"),
  },
});
```

```ts
const appMode = import.meta.env.VITE_APP_MODE ?? "default";
createApp(App, { appMode });
```

- [ ] **Step 4: Add mock route names and public access handling**

```ts
const appMode = import.meta.env.VITE_APP_MODE ?? "default";
const isMockMode = appMode === "mock";

export const routes = isMockMode
  ? [
      { path: "/", name: "mock-landing", component: LandingView, meta: { public: true } },
      { path: "/workbench", name: "mock-workbench-overview", component: WorkbenchView, meta: { public: true } },
      { path: "/workbench/issues", name: "mock-workbench-issues", component: IssuesView, meta: { public: true } },
      { path: "/workbench/mrs", name: "mock-workbench-mrs", component: MrsView, meta: { public: true } },
      { path: "/workbench/milestones", name: "mock-workbench-milestones", component: MilestonesView, meta: { public: true } },
      { path: "/settings", name: "mock-user-settings", component: UserSettingsView, meta: { public: true } },
    ]
  : existingRoutes;
```

- [ ] **Step 5: Run the test to verify it passes**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-routing.spec.ts`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add web/package.json web/vite.config.ts web/src/main.ts web/src/App.vue web/src/router/index.ts web/src/tests/mock-prototype-routing.spec.ts
git commit -m "feat: add isolated mock prototype mode"
```

## Task 2: Add mock workbench data and skill-driven UI profiles

**Files:**
- Create: `web/src/mock/prototype.types.ts`
- Create: `web/src/mock/prototype.data.ts`
- Create: `web/src/mock/prototype.ui-profile.ts`
- Create: `web/src/stores/prototype.store.ts`
- Create: `web/src/tests/mock-prototype-overview.spec.ts`
- Test: `web/src/tests/mock-prototype-overview.spec.ts`

- [ ] **Step 1: Write the failing store test for workbench-scoped workflow data**

```ts
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { usePrototypeStore } from "@/stores/prototype.store";

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("prototype store", () => {
  it("switches workbench and resolves workflow summaries", () => {
    const store = usePrototypeStore();

    store.selectWorkbench("alpha");

    expect(store.currentWorkbench?.id).toBe("alpha");
    expect(store.issueWorkflowSummary.length).toBeGreaterThan(0);
    expect(store.mrWorkflowSummary.length).toBeGreaterThan(0);
  });
});
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-overview.spec.ts`
Expected: FAIL because the prototype store does not exist.

- [ ] **Step 3: Define the prototype domain types**

```ts
export type IssueWorkflowState =
  | "new"
  | "clarifying"
  | "planned"
  | "ready_for_execution"
  | "in_execution"
  | "blocked"
  | "done";

export type MrWorkflowState =
  | "draft"
  | "in_review"
  | "changes_requested"
  | "ready_to_merge"
  | "merged"
  | "blocked";

export interface SkillUiProfile {
  tone: "direct" | "coach" | "operator";
  density: "compact" | "balanced" | "relaxed";
  overviewEmphasis: string[];
  issueFieldPriority: string[];
  mrFieldPriority: string[];
  milestoneFieldPriority: string[];
  defaultExpandedSections: string[];
  recommendedActionOrder: string[];
}
```

- [ ] **Step 4: Add mock data and the prototype store**

```ts
export const mockWorkbenches = [
  {
    id: "alpha",
    name: "Alpha Delivery",
    role: {
      name: "Execution Driver",
      personaSummary: "Pushes work toward clear next actions and closes blockers early.",
      waysOfWorking: ["Prefer explicit next steps", "Escalate blockers quickly"],
      goals: ["Move issues to execution", "Keep MRs mergeable"],
    },
    activeSkillVersionId: "delivery-skill@2.1.0",
  },
];
```

```ts
export const usePrototypeStore = defineStore("prototype", () => {
  const currentWorkbenchId = ref("alpha");
  const selectedIssueId = ref("issue-101");
  const selectedMrId = ref("mr-88");
  const selectedMilestoneId = ref("ms-q3");

  const currentWorkbench = computed(() =>
    mockWorkbenches.find((workbench) => workbench.id === currentWorkbenchId.value) ?? null,
  );

  const activeUiProfile = computed(() => getSkillUiProfile(currentWorkbench.value?.activeSkillVersionId));
  const issueWorkflowSummary = computed(() => summarizeIssueStates(currentWorkbenchId.value));
  const mrWorkflowSummary = computed(() => summarizeMrStates(currentWorkbenchId.value));

  function selectWorkbench(id: string) {
    currentWorkbenchId.value = id;
  }

  return {
    currentWorkbench,
    activeUiProfile,
    issueWorkflowSummary,
    mrWorkflowSummary,
    selectWorkbench,
  };
});
```

- [ ] **Step 5: Run the test to verify it passes**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-overview.spec.ts`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add web/src/mock/prototype.types.ts web/src/mock/prototype.data.ts web/src/mock/prototype.ui-profile.ts web/src/stores/prototype.store.ts web/src/tests/mock-prototype-overview.spec.ts
git commit -m "feat: add mock workbench workflow data"
```

## Task 3: Rework the shell and landing page for the prototype flow

**Files:**
- Modify: `web/src/components/layout/AppShell.vue`
- Modify: `web/src/views/LandingView.vue`
- Create: `web/src/components/prototype/UserMenu.vue`
- Modify: `web/src/tests/workbench-sidebar.spec.ts`
- Modify: `web/src/tests/app.spec.ts`
- Test: `web/src/tests/workbench-sidebar.spec.ts`
- Test: `web/src/tests/app.spec.ts`

- [ ] **Step 1: Write the failing shell test for fixed workbench navigation**

```ts
import { render, screen } from "@testing-library/vue";
import AppShell from "@/components/layout/AppShell.vue";

it("shows Overview, Issues, MRs, and Milestones in mock mode", () => {
  render(AppShell, { props: { activeKey: "overview" } });

  expect(screen.getByText("Overview")).toBeTruthy();
  expect(screen.getByText("Issues")).toBeTruthy();
  expect(screen.getByText("MRs")).toBeTruthy();
  expect(screen.getByText("Milestones")).toBeTruthy();
});
```

- [ ] **Step 2: Run the tests to verify they fail**

Run: `npm --prefix web test -- --run web/src/tests/workbench-sidebar.spec.ts web/src/tests/app.spec.ts`
Expected: FAIL because the shell does not render the prototype navigation and settings entry yet.

- [ ] **Step 3: Add a prototype-only user menu and fixed mock navigation**

```ts
const mockMenuOptions: MenuOption[] = [
  { key: "overview", label: () => h(RouterLink, { to: "/workbench" }, { default: () => "Overview" }) },
  { key: "issues", label: () => h(RouterLink, { to: "/workbench/issues" }, { default: () => "Issues" }) },
  { key: "mrs", label: () => h(RouterLink, { to: "/workbench/mrs" }, { default: () => "MRs" }) },
  { key: "milestones", label: () => h(RouterLink, { to: "/workbench/milestones" }, { default: () => "Milestones" }) },
];
```

```vue
<UserMenu>
  <n-dropdown :options="userMenuOptions" />
</UserMenu>
```

- [ ] **Step 4: Update the landing page to explain the workflow-centered prototype**

```vue
<ul class="landing__steps">
  <li>Select a workbench and review its active role and goals.</li>
  <li>Advance Issues, MRs, and Milestones through explicit workflow states.</li>
  <li>Adjust skills in Settings to change emphasis without changing the page structure.</li>
</ul>
```

- [ ] **Step 5: Run the tests to verify they pass**

Run: `npm --prefix web test -- --run web/src/tests/workbench-sidebar.spec.ts web/src/tests/app.spec.ts`
Expected: PASS

- [ ] **Step 6: Commit**

```bash
git add web/src/components/layout/AppShell.vue web/src/views/LandingView.vue web/src/components/prototype/UserMenu.vue web/src/tests/workbench-sidebar.spec.ts web/src/tests/app.spec.ts
git commit -m "feat: add mock prototype shell and landing"
```

## Task 4: Build workflow-centered Overview, Issues, MRs, and Milestones pages

**Files:**
- Modify: `web/src/views/WorkbenchView.vue`
- Modify: `web/src/views/IssuesView.vue`
- Create: `web/src/views/MrsView.vue`
- Create: `web/src/views/MilestonesView.vue`
- Create: `web/src/components/prototype/OverviewCards.vue`
- Create: `web/src/components/prototype/ObjectSplitView.vue`
- Create: `web/src/components/prototype/WorkflowStateBadge.vue`
- Create: `web/src/components/prototype/RecommendedActionsCard.vue`
- Create: `web/src/tests/mock-prototype-issues.spec.ts`
- Create: `web/src/tests/mock-prototype-mrs.spec.ts`
- Create: `web/src/tests/mock-prototype-milestones.spec.ts`
- Test: `web/src/tests/mock-prototype-issues.spec.ts`
- Test: `web/src/tests/mock-prototype-mrs.spec.ts`
- Test: `web/src/tests/mock-prototype-milestones.spec.ts`

- [ ] **Step 1: Write the failing issue-page test for visible workflow state**

```ts
import { render, screen } from "@testing-library/vue";
import IssuesView from "@/views/IssuesView.vue";

it("shows workflow state and recommended next action in issue rows", () => {
  render(IssuesView);

  expect(screen.getByText("ready_for_execution")).toBeTruthy();
  expect(screen.getByText("Start dev handoff")).toBeTruthy();
});
```

- [ ] **Step 2: Write the failing MR and milestone tests**

```ts
it("shows MR workflow state and next action", () => {
  render(MrsView);

  expect(screen.getByText("in_review")).toBeTruthy();
  expect(screen.getByText("Resolve review feedback")).toBeTruthy();
});
```

```ts
it("shows milestone workflow summaries", () => {
  render(MilestonesView);

  expect(screen.getByText("Issue workflow")).toBeTruthy();
  expect(screen.getByText("MR workflow")).toBeTruthy();
});
```

- [ ] **Step 3: Run the tests to verify they fail**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-issues.spec.ts web/src/tests/mock-prototype-mrs.spec.ts web/src/tests/mock-prototype-milestones.spec.ts`
Expected: FAIL because the prototype pages do not render workflow-first content yet.

- [ ] **Step 4: Build reusable workflow-first prototype components**

```vue
<template>
  <n-tag :type="tagType" size="small">{{ state }}</n-tag>
</template>
```

```vue
<template>
  <div class="prototype-split-view">
    <aside class="prototype-split-view__list"><slot name="list" /></aside>
    <section class="prototype-split-view__detail"><slot name="detail" /></section>
  </div>
</template>
```

- [ ] **Step 5: Implement the four fixed-skeleton pages**

```vue
<OverviewCards
  :workbench="store.currentWorkbench"
  :issue-summary="store.issueWorkflowSummary"
  :mr-summary="store.mrWorkflowSummary"
  :ui-profile="store.activeUiProfile"
/>
```

```vue
<ObjectSplitView>
  <template #list>
    <IssueRow v-for="issue in store.visibleIssues" :key="issue.id" :issue="issue" />
  </template>
  <template #detail>
    <section data-section="state">...</section>
    <section data-section="blockers">...</section>
    <RecommendedActionsCard :actions="store.recommendedIssueActions" />
  </template>
</ObjectSplitView>
```

```vue
<ObjectSplitView>
  <template #detail>
    <section data-section="state">...</section>
    <section data-section="readiness">...</section>
    <RecommendedActionsCard :actions="store.recommendedMrActions" />
  </template>
</ObjectSplitView>
```

```vue
<ObjectSplitView>
  <template #detail>
    <section data-section="goal">...</section>
    <section data-section="issue-workflow">...</section>
    <section data-section="mr-workflow">...</section>
  </template>
</ObjectSplitView>
```

- [ ] **Step 6: Run the tests to verify they pass**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-issues.spec.ts web/src/tests/mock-prototype-mrs.spec.ts web/src/tests/mock-prototype-milestones.spec.ts`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add web/src/views/WorkbenchView.vue web/src/views/IssuesView.vue web/src/views/MrsView.vue web/src/views/MilestonesView.vue web/src/components/prototype/OverviewCards.vue web/src/components/prototype/ObjectSplitView.vue web/src/components/prototype/WorkflowStateBadge.vue web/src/components/prototype/RecommendedActionsCard.vue web/src/tests/mock-prototype-issues.spec.ts web/src/tests/mock-prototype-mrs.spec.ts web/src/tests/mock-prototype-milestones.spec.ts
git commit -m "feat: build workflow-centered prototype pages"
```

## Task 5: Add isolated user settings with workbench role, skill, and memory controls

**Files:**
- Create: `web/src/views/UserSettingsView.vue`
- Create: `web/src/components/prototype/WorkbenchRolePanel.vue`
- Create: `web/src/components/prototype/SkillVersionPanel.vue`
- Create: `web/src/components/prototype/MemoryControlPanel.vue`
- Create: `web/src/tests/mock-prototype-settings.spec.ts`
- Test: `web/src/tests/mock-prototype-settings.spec.ts`

- [ ] **Step 1: Write the failing settings test for workbench role and skill controls**

```ts
import { render, screen } from "@testing-library/vue";
import UserSettingsView from "@/views/UserSettingsView.vue";

it("shows user settings plus a smaller current workbench section", () => {
  render(UserSettingsView);

  expect(screen.getByText("User Settings")).toBeTruthy();
  expect(screen.getByText("Current Workbench")).toBeTruthy();
  expect(screen.getByText("Execution Driver")).toBeTruthy();
  expect(screen.getByText("delivery-skill@2.1.0")).toBeTruthy();
});
```

- [ ] **Step 2: Run the test to verify it fails**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-settings.spec.ts`
Expected: FAIL because the settings page and panels do not exist.

- [ ] **Step 3: Build the settings panels with mock-only interactions**

```vue
<WorkbenchRolePanel
  :role="store.currentWorkbench?.role"
  @update-role="store.updateWorkbenchRole"
/>
<SkillVersionPanel
  :skills="store.availableSkills"
  :active-version-id="store.currentWorkbench?.activeSkillVersionId"
  @set-active-version="store.setActiveSkillVersion"
  @toggle-version="store.toggleSkillVersion"
  @mock-upload="store.mockUploadSkill"
/>
<MemoryControlPanel
  @clear-memory="store.clearWorkbenchMemory"
  @rebuild-memory="store.rebuildWorkbenchMemory"
/>
```

- [ ] **Step 4: Add the store mutations for settings interactions**

```ts
function setActiveSkillVersion(versionId: string) {
  const workbench = requireWorkbench();
  workbench.activeSkillVersionId = versionId;
}

function clearWorkbenchMemory() {
  lastMemoryAction.value = "cleared";
}
```

- [ ] **Step 5: Run the test to verify it passes**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-settings.spec.ts`
Expected: PASS

- [ ] **Step 6: Run focused frontend verification**

Run: `npm --prefix web test -- --run web/src/tests/mock-prototype-routing.spec.ts web/src/tests/mock-prototype-overview.spec.ts web/src/tests/mock-prototype-issues.spec.ts web/src/tests/mock-prototype-mrs.spec.ts web/src/tests/mock-prototype-milestones.spec.ts web/src/tests/mock-prototype-settings.spec.ts`
Expected: PASS

- [ ] **Step 7: Commit**

```bash
git add web/src/views/UserSettingsView.vue web/src/components/prototype/WorkbenchRolePanel.vue web/src/components/prototype/SkillVersionPanel.vue web/src/components/prototype/MemoryControlPanel.vue web/src/stores/prototype.store.ts web/src/tests/mock-prototype-settings.spec.ts
git commit -m "feat: add prototype user settings controls"
```

## Summary

This plan keeps the current app path intact while adding a separate mock-driven prototype that proves the product's main claim:

- workflow-centered `Issues` and `MRs`
- milestone aggregation
- stable page skeletons
- skill-driven UI emphasis
- isolated user settings for role, skill, and memory control
