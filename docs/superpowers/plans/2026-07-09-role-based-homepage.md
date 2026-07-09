# Role-Based Homepage & Role-Adaptive Dashboard Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the prototype homepage's single entry with four role entries (Developer, R&D Manager & Architect, Product Designer, System Evolution Expert) and make the Workbench Overview adapt its signal cards, quick entries, and emphasis to the selected role.

**Architecture:** Frontend mock-only. A new `PrototypeRoleView` concept (keyed `developer|manager|product|evolution`) lives in `prototype.data.ts`, is held in `prototype.store.ts` as `activeRoleKey` (persisted to `localStorage`), and drives (1) the homepage role picker, (2) the shell role indicator/switcher, and (3) the Overview dashboard signal strip + quick entries + issue/MR emphasis. One shell, one interaction model (per `PROTOTYPE.md §10.12`).

**Tech Stack:** Vue 3 (`<script setup>`), Pinia, Naive UI, vue-i18n, Vitest + @vue/test-utils.

---

## File Structure

- `web/src/mock/prototype.types.ts` — add role-view types.
- `web/src/mock/prototype.data.ts` — add `prototypeRoleViews`.
- `web/src/mock/prototype.ui-profile.ts` — add generic `sortStatesByEmphasis`.
- `web/src/stores/prototype.store.ts` — role state, `setActiveRole`, `activeRoleView`, role-emphasis overlay on summaries.
- `web/src/components/prototype/RoleEntryGrid.vue` — new, homepage 4-role picker.
- `web/src/components/prototype/RoleSignalStrip.vue` — new, dashboard signal cards + quick entries.
- `web/src/views/LandingView.vue` — swap single button for `RoleEntryGrid`.
- `web/src/components/layout/AppShell.vue` — role indicator/switcher dropdown.
- `web/src/views/prototype/PrototypeWorkbenchOverview.vue` + `web/src/components/prototype/OverviewCards.vue` — mount signal strip, role mission in hero.
- `web/src/i18n/locales/zh-CN.ts` + `web/src/i18n/locales/en.ts` — `prototype.roles.*` + `shell.roleSwitch`.
- `web/src/tests/mock-prototype-roles.spec.ts` — new.
- `web/src/tests/mock-prototype-overview.spec.ts` — extend.
- `docs/PROTOTYPE.md` — short pointer note.

Convention notes (match existing prototype code):
- Role identity text (name/tagline/mission) and picker/shell chrome go through i18n.
- Signal card `label`/`value`/`hint` are literal English strings in `prototype.data.ts` (same convention as existing `role.personaSummary`/`goals`).
- Quick entries reference existing `shell.navigation.*` i18n keys via a `labelKey` field, so they render translated for free.

---

## Task 1: Role-view types

**Files:**
- Modify: `web/src/mock/prototype.types.ts` (append after `PrototypeRole`, around line 23)

- [ ] **Step 1: Add types**

Append to `web/src/mock/prototype.types.ts`:

```ts
export type PrototypeRoleKey =
  | "developer"
  | "manager"
  | "product"
  | "evolution";

export interface PrototypeSignalCard {
  id: string;
  label: string;
  value: string;
  hint: string;
  tone: "neutral" | "attention" | "positive";
}

export interface PrototypeQuickEntry {
  id: string;
  labelKey: string;
  to: string;
}

export interface PrototypeRoleView {
  key: PrototypeRoleKey;
  sequence: "A" | "B" | "C" | "D";
  signalCards: PrototypeSignalCard[];
  quickEntries: PrototypeQuickEntry[];
  overviewEmphasis: string[];
}
```

- [ ] **Step 2: Typecheck**

Run: `npm --prefix web run build`
Expected: PASS (no type errors from the new declarations).

- [ ] **Step 3: Commit**

```bash
git add web/src/mock/prototype.types.ts
git commit -m "feat(proto): add role-view types"
```

---

## Task 2: Role-view mock data

**Files:**
- Modify: `web/src/mock/prototype.data.ts` (add import + new export near the top, after `prototypeWorkbenches`)
- Test: `web/src/tests/mock-prototype-roles.spec.ts` (create)

- [ ] **Step 1: Write the failing test**

Create `web/src/tests/mock-prototype-roles.spec.ts`:

```ts
import { describe, expect, it } from "vitest";
import { prototypeRoleViews } from "@/mock/prototype.data";

describe("prototype role views", () => {
  it("defines exactly the four project roles", () => {
    expect(prototypeRoleViews.map((role) => role.key)).toEqual([
      "developer",
      "manager",
      "product",
      "evolution",
    ]);
  });

  it("gives every role non-empty signal cards and quick entries", () => {
    for (const role of prototypeRoleViews) {
      expect(role.signalCards.length).toBeGreaterThan(0);
      expect(role.quickEntries.length).toBeGreaterThan(0);
      expect(role.overviewEmphasis.length).toBeGreaterThan(0);
    }
  });

  it("only routes quick entries to known workbench/system/settings paths", () => {
    const allowed = [
      "/workbench",
      "/workbench/issues",
      "/workbench/mrs",
      "/workbench/milestones",
      "/workbench/turns",
      "/workbench/approvals",
      "/workbench/memory",
      "/skills",
      "/system/gateway",
      "/system/governance",
      "/settings/loop",
      "/settings/integrations",
      "/settings/access",
    ];
    for (const role of prototypeRoleViews) {
      for (const entry of role.quickEntries) {
        expect(allowed).toContain(entry.to);
      }
    }
  });

  it("includes deployment readiness for the manager role", () => {
    const manager = prototypeRoleViews.find((role) => role.key === "manager");
    const ids = manager?.signalCards.map((card) => card.id) ?? [];
    expect(ids).toContain("release-readiness");
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npm --prefix web test -- --run mock-prototype-roles`
Expected: FAIL — `prototypeRoleViews` is not exported.

- [ ] **Step 3: Add the mock data**

In `web/src/mock/prototype.data.ts`, add `PrototypeRoleView` to the type import block (top of file), then add this export immediately after the `prototypeWorkbenches` array (after line 61):

```ts
export const prototypeRoleViews: PrototypeRoleView[] = [
  {
    key: "developer",
    sequence: "A",
    overviewEmphasis: ["in_execution", "changes_requested", "blocked"],
    signalCards: [
      {
        id: "my-execution",
        label: "My in-execution work",
        value: "3",
        hint: "Issues you are actively driving",
        tone: "neutral",
      },
      {
        id: "my-pending-actions",
        label: "My pending actions",
        value: "2",
        hint: "Writes waiting for your confirmation",
        tone: "attention",
      },
      {
        id: "review-queue",
        label: "Review queue",
        value: "1 changes requested",
        hint: "MRs needing another pass",
        tone: "attention",
      },
    ],
    quickEntries: [
      { id: "issues", labelKey: "shell.navigation.issues", to: "/workbench/issues" },
      { id: "mrs", labelKey: "shell.navigation.mrs", to: "/workbench/mrs" },
      { id: "turns", labelKey: "shell.navigation.turns", to: "/workbench/turns" },
      { id: "approvals", labelKey: "shell.navigation.approvals", to: "/workbench/approvals" },
    ],
  },
  {
    key: "manager",
    sequence: "A",
    overviewEmphasis: ["blocked", "ready_for_execution", "in_review"],
    signalCards: [
      {
        id: "milestone-pressure",
        label: "Milestone pressure",
        value: "Q3 at risk",
        hint: "Blocked items threatening delivery",
        tone: "attention",
      },
      {
        id: "verification-debt",
        label: "Verification debt",
        value: "4 unverified",
        hint: "Objects lacking independent verification",
        tone: "attention",
      },
      {
        id: "release-readiness",
        label: "Release / deployment readiness",
        value: "1 gray rollout",
        hint: "Deploy plan, gray release, rollback status",
        tone: "neutral",
      },
      {
        id: "risk-alerts",
        label: "Risk alerts",
        value: "2",
        hint: "High-risk actions and repeated failures",
        tone: "attention",
      },
    ],
    quickEntries: [
      { id: "milestones", labelKey: "shell.navigation.milestones", to: "/workbench/milestones" },
      { id: "governance", labelKey: "shell.navigation.governance", to: "/system/governance" },
      { id: "integrations", labelKey: "shell.navigation.settingsIntegrations", to: "/settings/integrations" },
      { id: "turns", labelKey: "shell.navigation.turns", to: "/workbench/turns" },
    ],
  },
  {
    key: "product",
    sequence: "B",
    overviewEmphasis: ["new", "clarifying", "planned"],
    signalCards: [
      {
        id: "clarification-queue",
        label: "Clarification queue",
        value: "3",
        hint: "Issues awaiting requirement clarity",
        tone: "attention",
      },
      {
        id: "requirement-readiness",
        label: "Requirement readiness",
        value: "72%",
        hint: "Issues with complete acceptance criteria",
        tone: "positive",
      },
      {
        id: "evolution-proposals",
        label: "Collaborative-evolution proposals",
        value: "1",
        hint: "SKILL change suggestions awaiting approval",
        tone: "neutral",
      },
    ],
    quickEntries: [
      { id: "issues", labelKey: "shell.navigation.issues", to: "/workbench/issues" },
      { id: "approvals", labelKey: "shell.navigation.approvals", to: "/workbench/approvals" },
      { id: "skills", labelKey: "shell.navigation.skills", to: "/skills" },
      { id: "memory", labelKey: "shell.navigation.memory", to: "/workbench/memory" },
    ],
  },
  {
    key: "evolution",
    sequence: "C",
    overviewEmphasis: ["blocked", "in_execution", "done"],
    signalCards: [
      {
        id: "evolution-proposals",
        label: "System-evolution proposals",
        value: "2",
        hint: "skill_evolution_proposal awaiting review",
        tone: "neutral",
      },
      {
        id: "loop-health",
        label: "Loop health",
        value: "88% success",
        hint: "Success rate, reject rate, budget overrun",
        tone: "positive",
      },
      {
        id: "governance-signals",
        label: "Governance signals",
        value: "1 comprehension rot",
        hint: "Approve-without-reading and drift signals",
        tone: "attention",
      },
    ],
    quickEntries: [
      { id: "governance", labelKey: "shell.navigation.governance", to: "/system/governance" },
      { id: "skills", labelKey: "shell.navigation.skills", to: "/skills" },
      { id: "gateway", labelKey: "shell.navigation.gateway", to: "/system/gateway" },
      { id: "turns", labelKey: "shell.navigation.turns", to: "/workbench/turns" },
    ],
  },
];
```

- [ ] **Step 4: Run test to verify it passes**

Run: `npm --prefix web test -- --run mock-prototype-roles`
Expected: PASS (all 4 tests).

- [ ] **Step 5: Commit**

```bash
git add web/src/mock/prototype.data.ts web/src/tests/mock-prototype-roles.spec.ts
git commit -m "feat(proto): add role-view mock data with role signal cards"
```

---

## Task 3: Emphasis sort helper

**Files:**
- Modify: `web/src/mock/prototype.ui-profile.ts` (add generic helper after `summarizeStates`, line 50)

- [ ] **Step 1: Write the failing test**

Append to `web/src/tests/mock-prototype-roles.spec.ts`:

```ts
import { sortStatesByEmphasis } from "@/mock/prototype.ui-profile";

describe("sortStatesByEmphasis", () => {
  it("orders emphasized states first in the given order", () => {
    const summary = [
      { state: "done", count: 5 },
      { state: "blocked", count: 2 },
      { state: "in_execution", count: 3 },
    ] as const;

    const sorted = sortStatesByEmphasis([...summary], [
      "in_execution",
      "blocked",
    ]);

    expect(sorted.map((item) => item.state)).toEqual([
      "in_execution",
      "blocked",
      "done",
    ]);
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npm --prefix web test -- --run mock-prototype-roles`
Expected: FAIL — `sortStatesByEmphasis` is not exported.

- [ ] **Step 3: Add the helper**

In `web/src/mock/prototype.ui-profile.ts`, add after `summarizeStates` (after line 50):

```ts
export function sortStatesByEmphasis<TState extends string>(
  summary: WorkflowSummaryItem<TState>[],
  emphasis: readonly string[],
): WorkflowSummaryItem<TState>[] {
  return [...summary].sort((left, right) => {
    const leftIndex = emphasis.indexOf(left.state);
    const rightIndex = emphasis.indexOf(right.state);
    const normalizedLeft =
      leftIndex === -1 ? Number.MAX_SAFE_INTEGER : leftIndex;
    const normalizedRight =
      rightIndex === -1 ? Number.MAX_SAFE_INTEGER : rightIndex;
    return normalizedLeft - normalizedRight;
  });
}
```

- [ ] **Step 4: Run test to verify it passes**

Run: `npm --prefix web test -- --run mock-prototype-roles`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add web/src/mock/prototype.ui-profile.ts web/src/tests/mock-prototype-roles.spec.ts
git commit -m "feat(proto): add generic emphasis sort helper"
```

---

## Task 4: Store role state + role emphasis overlay

**Files:**
- Modify: `web/src/stores/prototype.store.ts`

- [ ] **Step 1: Write the failing test**

Append to `web/src/tests/mock-prototype-roles.spec.ts`:

```ts
import { beforeEach, vi } from "vitest";
import { createPinia, setActivePinia } from "pinia";
import { usePrototypeStore } from "@/stores/prototype.store";

describe("prototype store role state", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
  });

  it("defaults to the developer role", () => {
    const store = usePrototypeStore();
    expect(store.activeRoleKey).toBe("developer");
    expect(store.activeRoleView?.key).toBe("developer");
  });

  it("switches role, persists it, and exposes its signal cards", () => {
    const store = usePrototypeStore();
    store.setActiveRole("manager");
    expect(store.activeRoleView?.key).toBe("manager");
    expect(store.activeRoleView?.signalCards.length).toBeGreaterThan(0);
    expect(localStorage.getItem("issueflow_prototype_role")).toBe("manager");
  });

  it("puts the active role's emphasis first in the issue summary", () => {
    const store = usePrototypeStore();
    store.setActiveRole("product");
    const first = store.issueWorkflowSummary[0]?.state;
    const emphasis = store.activeRoleView?.overviewEmphasis ?? [];
    if (first && emphasis.length) {
      const present = store.issueWorkflowSummary
        .map((item) => item.state)
        .filter((state) => emphasis.includes(state));
      const expectedOrder = emphasis.filter((state) =>
        present.includes(state),
      );
      expect(present).toEqual(expectedOrder);
    }
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npm --prefix web test -- --run mock-prototype-roles`
Expected: FAIL — `activeRoleKey` / `setActiveRole` do not exist.

- [ ] **Step 3: Wire role state into the store**

In `web/src/stores/prototype.store.ts`:

3a. Add `prototypeRoleViews` to the `@/mock/prototype.data` import block:

```ts
  prototypeRoleViews,
```

3b. Add `PrototypeRoleKey` to the `@/mock/prototype.types` import block:

```ts
import type {
  PrototypeRecommendedAction,
  PrototypeRole,
  PrototypeRoleKey,
} from "@/mock/prototype.types";
```

3c. Change the ui-profile import to include the new helper:

```ts
import {
  getSkillUiProfile,
  sortStatesByEmphasis,
  summarizeStates,
} from "@/mock/prototype.ui-profile";
```

(Remove `sortIssueStatesByProfile` and `sortMrStatesByProfile` from this import — they are replaced below.)

3d. Add state near the other `ref`s (after line 45, `lastMemoryAction`):

```ts
  const roleViews = ref(structuredClone(prototypeRoleViews));
  const storedRole =
    typeof localStorage !== "undefined"
      ? (localStorage.getItem("issueflow_prototype_role") as PrototypeRoleKey | null)
      : null;
  const activeRoleKey = ref<PrototypeRoleKey>(storedRole ?? "developer");
```

3e. Add computed after `activeUiProfile` (after line 61):

```ts
  const activeRoleView = computed(
    () =>
      roleViews.value.find((role) => role.key === activeRoleKey.value) ??
      roleViews.value[0] ??
      null,
  );

  const activeEmphasis = computed(() => [
    ...(activeRoleView.value?.overviewEmphasis ?? []),
    ...activeUiProfile.value.overviewEmphasis,
  ]);
```

3f. Replace the `issueWorkflowSummary` computed (lines 108-113) with:

```ts
  const issueWorkflowSummary = computed(() =>
    sortStatesByEmphasis(
      summarizeStates(visibleIssues.value.map((issue) => issue.state)),
      activeEmphasis.value,
    ),
  );
```

3g. Replace the `mrWorkflowSummary` computed (lines 115-120) with:

```ts
  const mrWorkflowSummary = computed(() =>
    sortStatesByEmphasis(
      summarizeStates(visibleMrs.value.map((mr) => mr.state)),
      activeEmphasis.value,
    ),
  );
```

3h. Add the action (after `selectLoop`, before the `return`):

```ts
  function setActiveRole(key: PrototypeRoleKey) {
    activeRoleKey.value = key;
    if (typeof localStorage !== "undefined") {
      localStorage.setItem("issueflow_prototype_role", key);
    }
  }
```

3i. Add to the `return { ... }` object:

```ts
    roleViews,
    activeRoleKey,
    activeRoleView,
    setActiveRole,
```

- [ ] **Step 4: Run test to verify it passes**

Run: `npm --prefix web test -- --run mock-prototype-roles`
Expected: PASS. Also run `npm --prefix web test -- --run mock-prototype-overview` to confirm the summary-sort change did not break existing store tests.

- [ ] **Step 5: Commit**

```bash
git add web/src/stores/prototype.store.ts web/src/tests/mock-prototype-roles.spec.ts
git commit -m "feat(proto): add active role state and role emphasis overlay to store"
```

---

## Task 5: i18n keys for roles

**Files:**
- Modify: `web/src/i18n/locales/zh-CN.ts`
- Modify: `web/src/i18n/locales/en.ts`

- [ ] **Step 1: Add English keys**

In `web/src/i18n/locales/en.ts`, add a `roleSwitch: "Switch role",` entry inside the existing `shell` object (next to `role`), and add a `roles` block inside the existing `prototype` object (sibling of `overview`):

```ts
      roles: {
        pickerEyebrow: "Choose your view",
        pickerTitle: "Enter by role",
        pickerDescription:
          "Same workbench, one interaction model. Your role tunes which signals and entries surface first.",
        developer: {
          name: "Developer",
          tagline: "Drive assigned work to done",
          mission:
            "Focus on in-execution issues, MRs, your pending actions, and the review queue.",
        },
        manager: {
          name: "R&D Manager & Architect",
          tagline: "Delivery, architecture & deployment",
          mission:
            "Watch milestone pressure, verification debt, release readiness, and architecture risk.",
        },
        product: {
          name: "Product Designer",
          tagline: "Clarify scope, shape requirements",
          mission:
            "Keep clarification, acceptance quality, and collaborative-evolution proposals in view.",
        },
        evolution: {
          name: "System Evolution Expert",
          tagline: "Improve the loop itself",
          mission:
            "Review system-evolution proposals, loop health, and governance signals.",
        },
      },
```

- [ ] **Step 2: Add Chinese keys**

In `web/src/i18n/locales/zh-CN.ts`, add `roleSwitch: "切换角色",` inside the `shell` object (next to `role`), and add inside the `prototype` object (sibling of `overview`):

```ts
      roles: {
        pickerEyebrow: "选择你的视角",
        pickerTitle: "按角色进入",
        pickerDescription:
          "同一个工作台，一套交互模型。你的角色只决定哪些信号和入口优先呈现。",
        developer: {
          name: "研发人员",
          tagline: "把分配到的工作推进到完成",
          mission:
            "聚焦执行中的 Issue、MR、待你确认的动作，以及 review 队列。",
        },
        manager: {
          name: "研发经理 & 架构设计师",
          tagline: "交付、架构与部署方案",
          mission:
            "关注里程碑压力、验证债务、发布/部署就绪度与架构风险。",
        },
        product: {
          name: "产品设计师",
          tagline: "澄清范围、结构化需求",
          mission:
            "持续关注澄清队列、验收质量与协作进化的 SKILL 建议。",
        },
        evolution: {
          name: "系统进化专家",
          tagline: "改进 Loop 本身",
          mission:
            "审阅系统进化提案、Loop 健康度与治理信号。",
        },
      },
```

- [ ] **Step 3: Verify the locales still parse**

Run: `npm --prefix web run build`
Expected: PASS (no syntax/type errors). Both locale objects must keep matching shapes.

- [ ] **Step 4: Commit**

```bash
git add web/src/i18n/locales/en.ts web/src/i18n/locales/zh-CN.ts
git commit -m "feat(proto): add role picker and role identity i18n keys"
```

---

## Task 6: RoleEntryGrid component + homepage integration

**Files:**
- Create: `web/src/components/prototype/RoleEntryGrid.vue`
- Modify: `web/src/views/LandingView.vue`
- Modify: `web/src/tests/mock-landing-panels.spec.ts` (extend) OR the new roles spec

- [ ] **Step 1: Write the failing test**

Append to `web/src/tests/mock-prototype-roles.spec.ts`:

```ts
import { mount } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import { i18n, setLocale } from "@/i18n";

describe("role entry homepage", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
    vi.stubEnv("VITE_APP_MODE", "mock");
  });

  it("renders four role cards and selecting one sets role + navigates", async () => {
    setLocale("en");
    vi.resetModules();
    const { default: LandingView } = await import("@/views/LandingView.vue");
    const { usePrototypeStore } = await import("@/stores/prototype.store");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/", component: { template: "<div />" } },
        { path: "/workbench", component: { template: "<div />" } },
      ],
    });
    await router.push("/");
    await router.isReady();

    const wrapper = mount(LandingView, {
      global: { plugins: [router, i18n] },
    });

    const cards = wrapper.findAll("[data-role-key]");
    expect(cards.length).toBe(4);

    await wrapper.find('[data-role-key="manager"]').trigger("click");
    await router.isReady();

    const store = usePrototypeStore();
    expect(store.activeRoleKey).toBe("manager");
    expect(router.currentRoute.value.path).toBe("/workbench");
    wrapper.unmount();
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npm --prefix web test -- --run mock-prototype-roles`
Expected: FAIL — no `[data-role-key]` elements yet.

- [ ] **Step 3: Create the component**

Create `web/src/components/prototype/RoleEntryGrid.vue`:

```vue
<template>
  <section class="role-grid">
    <div class="role-grid__header">
      <div class="role-grid__eyebrow">{{ t("prototype.roles.pickerEyebrow") }}</div>
      <h2>{{ t("prototype.roles.pickerTitle") }}</h2>
      <p>{{ t("prototype.roles.pickerDescription") }}</p>
    </div>
    <div class="role-grid__cards">
      <button
        v-for="role in store.roleViews"
        :key="role.key"
        :data-role-key="role.key"
        type="button"
        class="role-card"
        @click="enter(role.key)"
      >
        <div class="role-card__name">
          {{ t(`prototype.roles.${role.key}.name`) }}
        </div>
        <div class="role-card__tagline">
          {{ t(`prototype.roles.${role.key}.tagline`) }}
        </div>
        <ul class="role-card__signals">
          <li v-for="card in role.signalCards.slice(0, 3)" :key="card.id">
            {{ card.label }}
          </li>
        </ul>
      </button>
    </div>
  </section>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { usePrototypeStore } from "@/stores/prototype.store";
import type { PrototypeRoleKey } from "@/mock/prototype.types";

const store = usePrototypeStore();
const router = useRouter();
const { t } = useI18n();

function enter(key: PrototypeRoleKey) {
  store.setActiveRole(key);
  router.push("/workbench");
}
</script>

<style scoped>
.role-grid {
  display: grid;
  gap: 20px;
}
.role-grid__eyebrow {
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-bottom: 8px;
}
.role-grid__header h1,
.role-grid__header h2 {
  margin: 0 0 8px;
}
.role-grid__header p {
  max-width: 640px;
  margin: 0;
  color: var(--if-color-muted);
}
.role-grid__cards {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 16px;
}
.role-card {
  display: grid;
  gap: 10px;
  align-content: start;
  text-align: left;
  padding: 20px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
  cursor: pointer;
  transition:
    transform 0.12s ease,
    border-color 0.12s ease,
    box-shadow 0.12s ease;
}
.role-card:hover {
  transform: translateY(-2px);
  border-color: var(--if-color-accent);
  box-shadow: var(--if-shadow-panel);
}
.role-card__name {
  font-size: 16px;
  font-weight: 800;
}
.role-card__tagline {
  color: var(--if-color-accent-strong);
  font-size: 13px;
  font-weight: 600;
}
.role-card__signals {
  margin: 4px 0 0;
  padding-left: 16px;
  color: var(--if-color-muted);
  font-size: 12px;
  line-height: 1.6;
}
@media (max-width: 960px) {
  .role-grid__cards {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
@media (max-width: 560px) {
  .role-grid__cards {
    grid-template-columns: 1fr;
  }
}
</style>
```

- [ ] **Step 4: Integrate into the homepage**

In `web/src/views/LandingView.vue`:

4a. Replace the `landing__actions` block (lines 28-32) with:

```html
        <RoleEntryGrid />
```

4b. Add the import in `<script setup>` (after the `LanguageSwitcher` import, line 164):

```ts
import RoleEntryGrid from "@/components/prototype/RoleEntryGrid.vue";
```

4c. Remove the now-unused `NButton` from the `naive-ui` import on line 165 only if it is not used elsewhere in the file. Check: the `v-else` login branch still uses `<n-button>` (line 154), so keep `NButton` in the import. No change needed to the import.

- [ ] **Step 5: Run test to verify it passes**

Run: `npm --prefix web test -- --run mock-prototype-roles`
Expected: PASS. Also run `npm --prefix web test -- --run mock-landing-panels` to confirm the hero still renders.

- [ ] **Step 6: Commit**

```bash
git add web/src/components/prototype/RoleEntryGrid.vue web/src/views/LandingView.vue web/src/tests/mock-prototype-roles.spec.ts
git commit -m "feat(proto): add role entry grid to prototype homepage"
```

---

## Task 7: Shell role indicator / switcher

**Files:**
- Modify: `web/src/components/layout/AppShell.vue`
- Test: `web/src/tests/mock-prototype-shell.spec.ts` (extend)

- [ ] **Step 1: Write the failing test**

Append a new `it` inside the existing `describe("prototype shell", ...)` in `web/src/tests/mock-prototype-shell.spec.ts`:

```ts
  it("shows the active role name and a role switch control in the shell", async () => {
    setActivePinia(createPinia());
    setLocale("en");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: "/workbench", component: { template: "<div />" } }],
    });
    await router.push("/workbench");
    await router.isReady();

    const wrapper = mount(AppShell, {
      props: { activeKey: "dashboard", prototypeMode: true },
      global: { plugins: [router, i18n] },
      slots: { default: "<div>content</div>" },
    });

    expect(wrapper.text()).toContain("Developer");
    expect(wrapper.text()).toContain("Switch role");
    wrapper.unmount();
  });
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npm --prefix web test -- --run mock-prototype-shell`
Expected: FAIL — shell shows the workbench persona ("Execution Driver"), not "Developer"/"Switch role".

- [ ] **Step 3: Add the role switcher**

In `web/src/components/layout/AppShell.vue`:

3a. Replace the `shell__chip` span (lines 28-30) with a role dropdown:

```html
          <n-dropdown
            trigger="click"
            :options="roleDropdownOptions"
            @select="onRoleSelect"
          >
            <span class="shell__chip" style="cursor: pointer">
              {{ activeRoleName }} · {{ t("shell.roleSwitch") }}
            </span>
          </n-dropdown>
```

3b. Add to `<script setup>` computed section (after the existing `menuOptions`/`profileDropdownOptions`, near line 329):

```ts
const activeRoleName = computed(() => {
  const key = prototypeStore.activeRoleView?.key;
  return key ? t(`prototype.roles.${key}.name`) : "";
});

const roleDropdownOptions = computed(() =>
  prototypeStore.roleViews.map((role) => ({
    key: role.key,
    label: t(`prototype.roles.${role.key}.name`),
  })),
);

function onRoleSelect(key: string) {
  prototypeStore.setActiveRole(key as PrototypeRoleKey);
}
```

3c. Add the type import at the top of `<script setup>` (with the other `import type` lines, near line 132):

```ts
import type { PrototypeRoleKey } from "@/mock/prototype.types";
```

3d. Update the left sider summary block (lines 51-55) to reflect the active role:

```html
            <div class="prototype-sider__summary">
              <span class="shell__section-label">{{ t("shell.role") }}</span>
              <strong>{{ activeRoleName }}</strong>
              <p>{{ activeRoleMission }}</p>
            </div>
```

3e. Add the mission computed alongside `activeRoleName`:

```ts
const activeRoleMission = computed(() => {
  const key = prototypeStore.activeRoleView?.key;
  return key ? t(`prototype.roles.${key}.mission`) : "";
});
```

- [ ] **Step 4: Run test to verify it passes**

Run: `npm --prefix web test -- --run mock-prototype-shell`
Expected: PASS (both existing tests and the new one).

- [ ] **Step 5: Commit**

```bash
git add web/src/components/layout/AppShell.vue web/src/tests/mock-prototype-shell.spec.ts
git commit -m "feat(proto): add role indicator and switcher to prototype shell"
```

---

## Task 8: RoleSignalStrip + dashboard integration

**Files:**
- Create: `web/src/components/prototype/RoleSignalStrip.vue`
- Modify: `web/src/views/prototype/PrototypeWorkbenchOverview.vue`
- Test: `web/src/tests/mock-prototype-overview.spec.ts` (extend)

- [ ] **Step 1: Write the failing test**

Append to `web/src/tests/mock-prototype-overview.spec.ts` a component test. Add these imports at the top of the file:

```ts
import { mount } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import { i18n, setLocale } from "@/i18n";
```

Then add a new `describe`:

```ts
describe("prototype overview signal strip", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
    setLocale("en");
  });

  it("renders the active role's signal cards and quick entries", async () => {
    const store = usePrototypeStore();
    store.setActiveRole("manager");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/workbench", component: { template: "<div />" } },
        { path: "/workbench/turns", component: { template: "<div />" } },
        { path: "/workbench/milestones", component: { template: "<div />" } },
        { path: "/system/governance", component: { template: "<div />" } },
        { path: "/settings/integrations", component: { template: "<div />" } },
      ],
    });
    await router.push("/workbench");
    await router.isReady();

    const { default: PrototypeWorkbenchOverview } = await import(
      "@/views/prototype/PrototypeWorkbenchOverview.vue"
    );

    const wrapper = mount(PrototypeWorkbenchOverview, {
      global: { plugins: [router, i18n] },
    });

    expect(wrapper.text()).toContain("Release / deployment readiness");
    expect(wrapper.text()).toContain("Milestone pressure");
    wrapper.unmount();
  });
});
```

- [ ] **Step 2: Run test to verify it fails**

Run: `npm --prefix web test -- --run mock-prototype-overview`
Expected: FAIL — signal card labels not rendered anywhere.

- [ ] **Step 3: Create RoleSignalStrip**

Create `web/src/components/prototype/RoleSignalStrip.vue`:

```vue
<template>
  <div v-if="role" class="signal-strip">
    <div class="signal-strip__cards">
      <article
        v-for="card in role.signalCards"
        :key="card.id"
        class="signal-card"
        :class="`signal-card--${card.tone}`"
      >
        <div class="signal-card__label">{{ card.label }}</div>
        <div class="signal-card__value">{{ card.value }}</div>
        <div class="signal-card__hint">{{ card.hint }}</div>
      </article>
    </div>
    <nav class="signal-strip__entries">
      <RouterLink
        v-for="entry in role.quickEntries"
        :key="entry.id"
        :to="entry.to"
        class="signal-entry"
      >
        {{ t(entry.labelKey) }}
      </RouterLink>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";
import { useI18n } from "vue-i18n";
import type { PrototypeRoleView } from "@/mock/prototype.types";

defineProps<{ role: PrototypeRoleView | null }>();

const { t } = useI18n();
</script>

<style scoped>
.signal-strip {
  display: grid;
  gap: 16px;
}
.signal-strip__cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}
.signal-card {
  padding: 16px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}
.signal-card--attention {
  border-color: rgba(251, 191, 36, 0.5);
  background: rgba(251, 191, 36, 0.08);
}
.signal-card--positive {
  border-color: rgba(15, 118, 110, 0.35);
  background: rgba(15, 118, 110, 0.08);
}
.signal-card__label {
  color: var(--if-color-muted);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.04em;
}
.signal-card__value {
  margin: 8px 0 4px;
  font-size: 22px;
  font-weight: 800;
}
.signal-card__hint {
  color: var(--if-color-muted);
  font-size: 12px;
  line-height: 1.5;
}
.signal-strip__entries {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}
.signal-entry {
  display: inline-flex;
  padding: 8px 14px;
  border-radius: 999px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  background: rgba(255, 255, 255, 0.7);
  color: var(--if-color-text);
  font-size: 13px;
  font-weight: 600;
  text-decoration: none;
}
.signal-entry:hover {
  border-color: var(--if-color-accent);
  color: var(--if-color-accent-strong);
}
</style>
```

- [ ] **Step 4: Mount it in the overview + show role mission**

In `web/src/views/prototype/PrototypeWorkbenchOverview.vue`:

4a. Insert the signal strip between the header and `OverviewCards` (after line 12, before line 14):

```html
      <RoleSignalStrip :role="store.activeRoleView" />
```

4b. Add the import (after the `OverviewCards` import, line 28):

```ts
import RoleSignalStrip from "@/components/prototype/RoleSignalStrip.vue";
```

- [ ] **Step 5: Run test to verify it passes**

Run: `npm --prefix web test -- --run mock-prototype-overview`
Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add web/src/components/prototype/RoleSignalStrip.vue web/src/views/prototype/PrototypeWorkbenchOverview.vue web/src/tests/mock-prototype-overview.spec.ts
git commit -m "feat(proto): add role signal strip to workbench overview"
```

---

## Task 9: Docs note + full quality gate

**Files:**
- Modify: `docs/PROTOTYPE.md`

- [ ] **Step 1: Add a pointer note**

In `docs/PROTOTYPE.md`, at the end of §10.12 (after line 970's table), add:

```markdown
> 实现说明：首页已提供 4 个角色入口（研发人员、研发经理 & 架构设计师、产品设计师、系统进化专家），进入同一 Workbench 后由角色视角驱动 Dashboard 的信号卡片、快捷入口与 issue/MR 强调顺序。详见 `docs/superpowers/specs/2026-07-09-role-based-homepage-design.md`。
```

- [ ] **Step 2: Run the full quality gate**

Run:

```bash
npm --prefix web run lint && npm --prefix web run format:check && npm --prefix web run build && npm --prefix web test -- --run
```

Expected: all PASS. If `format:check` fails, run `npm --prefix web run format` and re-stage.

- [ ] **Step 3: Commit**

```bash
git add docs/PROTOTYPE.md
git commit -m "docs: note role-based homepage entries in PROTOTYPE"
```

---

## Self-Review

- **Spec coverage:** homepage 4 entries (Task 6), same-workbench role injection via store + localStorage (Task 4), shell role indicator/switcher (Task 7), dashboard signal strip + quick entries + emphasis (Tasks 3/4/8), deployment concerns for manager (Task 2 `release-readiness`), i18n zh+en (Task 5), tests (Tasks 2/4/6/7/8), doc note (Task 9). All spec sections covered.
- **Placeholder scan:** none — every code step contains full code.
- **Type consistency:** `PrototypeRoleKey`, `PrototypeRoleView`, `PrototypeSignalCard`, `PrototypeQuickEntry` defined in Task 1 and used consistently; store exposes `roleViews`, `activeRoleKey`, `activeRoleView`, `setActiveRole` (Task 4) consumed identically in Tasks 6/7/8; `sortStatesByEmphasis` defined Task 3, used Task 4.
