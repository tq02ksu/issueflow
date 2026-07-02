# Frontend I18n Foundation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add English and Simplified Chinese internationalization to the Vue frontend with an explicit language switcher, browser-aware default selection, persisted locale choice, and translated landing plus prototype workbench UI.

**Architecture:** Use `vue-i18n` as the single locale runtime and keep locale resolution logic in a small `web/src/i18n/` module. Add one reusable language switcher component, then migrate visible landing, shell, and prototype UI copy from inline strings to namespaced message catalogs without changing page structure or routing.

**Tech Stack:** Vue 3, TypeScript, Vue I18n, Pinia, Vue Router, Naive UI, Vitest, Vue Test Utils

---

### Task 1: Add i18n runtime and locale resolution foundation

**Files:**
- Modify: `web/package.json`
- Create: `web/src/i18n/locale.ts`
- Create: `web/src/i18n/index.ts`
- Create: `web/src/i18n/locales/en.ts`
- Create: `web/src/i18n/locales/zh-CN.ts`
- Modify: `web/src/main.ts`
- Create: `web/src/tests/i18n-locale.spec.ts`

- [ ] **Step 1: Add the failing locale-resolution test**

```ts
import { beforeEach, describe, expect, it, vi } from "vitest";

describe("frontend locale resolution", () => {
  beforeEach(() => {
    localStorage.clear();
    vi.restoreAllMocks();
  });

  it("prefers a persisted locale over browser language", async () => {
    localStorage.setItem("issueflow.locale", "zh-CN");
    vi.stubGlobal("navigator", { language: "en-US" });

    const { resolveInitialLocale } = await import("@/i18n/locale");
    expect(resolveInitialLocale()).toBe("zh-CN");
  });

  it("uses browser Chinese on first run", async () => {
    vi.stubGlobal("navigator", { language: "zh-CN" });

    const { resolveInitialLocale } = await import("@/i18n/locale");
    expect(resolveInitialLocale()).toBe("zh-CN");
  });

  it("falls back to English when browser language is unsupported", async () => {
    vi.stubGlobal("navigator", { language: "fr-FR" });

    const { resolveInitialLocale } = await import("@/i18n/locale");
    expect(resolveInitialLocale()).toBe("en");
  });
});
```

- [ ] **Step 2: Run the locale test and confirm it fails**

Run: `npm --prefix web test -- --run src/tests/i18n-locale.spec.ts`

Expected: FAIL because `@/i18n/locale` does not exist yet.

- [ ] **Step 3: Install the i18n dependency**

Run: `npm --prefix web install vue-i18n`

Expected: `package.json` and lockfile update with `vue-i18n` added under dependencies.

- [ ] **Step 4: Implement locale helpers and the i18n bootstrap**

`web/src/i18n/locale.ts`

```ts
export const SUPPORTED_LOCALES = ["en", "zh-CN"] as const;

export type AppLocale = (typeof SUPPORTED_LOCALES)[number];

export const DEFAULT_LOCALE: AppLocale = "en";
export const LOCALE_STORAGE_KEY = "issueflow.locale";

export function isSupportedLocale(value: string | null): value is AppLocale {
  return value === "en" || value === "zh-CN";
}

export function normalizeBrowserLocale(value: string | undefined): AppLocale {
  if (!value) {
    return DEFAULT_LOCALE;
  }

  const normalized = value.toLowerCase();
  if (normalized.startsWith("zh")) {
    return "zh-CN";
  }

  return "en";
}

export function resolveInitialLocale(): AppLocale {
  const stored = localStorage.getItem(LOCALE_STORAGE_KEY);
  if (isSupportedLocale(stored)) {
    return stored;
  }

  return normalizeBrowserLocale(globalThis.navigator?.language);
}

export function persistLocale(locale: AppLocale) {
  localStorage.setItem(LOCALE_STORAGE_KEY, locale);
}
```

`web/src/i18n/locales/en.ts`

```ts
export const en = {
  common: {
    locale: {
      label: "Language switcher",
      english: "EN",
      chinese: "中文",
    },
    actions: {
      cancel: "Cancel",
      save: "Save",
      openPrototype: "Open prototype",
      reviewSettings: "Review settings",
      continueToSignIn: "Continue to sign in",
    },
  },
} as const;
```

`web/src/i18n/locales/zh-CN.ts`

```ts
export const zhCN = {
  common: {
    locale: {
      label: "语言切换",
      english: "EN",
      chinese: "中文",
    },
    actions: {
      cancel: "取消",
      save: "保存",
      openPrototype: "打开原型",
      reviewSettings: "查看设置",
      continueToSignIn: "继续登录",
    },
  },
} as const;
```

`web/src/i18n/index.ts`

```ts
import { createI18n } from "vue-i18n";
import { en } from "./locales/en";
import { zhCN } from "./locales/zh-CN";
import { persistLocale, resolveInitialLocale, type AppLocale } from "./locale";

export type { AppLocale } from "./locale";

export const messages = {
  en,
  "zh-CN": zhCN,
} as const;

export const i18n = createI18n({
  legacy: false,
  locale: resolveInitialLocale(),
  fallbackLocale: "en",
  messages,
});

export function setLocale(locale: AppLocale) {
  i18n.global.locale.value = locale;
  persistLocale(locale);
}
```

`web/src/main.ts`

```ts
import { i18n } from "./i18n";

// ...
app.use(router);
app.use(i18n);
provideA2UI({ app, catalog: DEFAULT_CATALOG, theme: defaultTheme });
```

- [ ] **Step 5: Run the locale test and type-aware build check**

Run: `npm --prefix web test -- --run src/tests/i18n-locale.spec.ts`

Expected: PASS

Run: `npm --prefix web run typecheck`

Expected: PASS

- [ ] **Step 6: Commit the foundation**

```bash
git add web/package.json web/package-lock.json web/src/main.ts web/src/i18n web/src/tests/i18n-locale.spec.ts
git commit -m "feat: add frontend i18n foundation"
```

### Task 2: Add a reusable language switcher and wire it into the landing and shell

**Files:**
- Create: `web/src/components/i18n/LanguageSwitcher.vue`
- Modify: `web/src/views/LandingView.vue`
- Modify: `web/src/components/layout/AppShell.vue`
- Create: `web/src/tests/language-switcher.spec.ts`
- Modify: `web/src/tests/mock-prototype-shell.spec.ts`
- Modify: `web/src/tests/mock-landing-panels.spec.ts`

- [ ] **Step 1: Add the failing switcher interaction test**

```ts
import { mount } from "@vue/test-utils";
import { describe, expect, it } from "vitest";
import { i18n } from "@/i18n";
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";

describe("LanguageSwitcher", () => {
  it("switches the active locale and persists it", async () => {
    localStorage.clear();
    i18n.global.locale.value = "en";

    const wrapper = mount(LanguageSwitcher, {
      global: { plugins: [i18n] },
    });

    await wrapper.get('[data-locale="zh-CN"]').trigger("click");

    expect(i18n.global.locale.value).toBe("zh-CN");
    expect(localStorage.getItem("issueflow.locale")).toBe("zh-CN");
  });
});
```

- [ ] **Step 2: Run the switcher test and confirm it fails**

Run: `npm --prefix web test -- --run src/tests/language-switcher.spec.ts`

Expected: FAIL because the switcher component does not exist yet.

- [ ] **Step 3: Implement the reusable switcher**

`web/src/components/i18n/LanguageSwitcher.vue`

```vue
<template>
  <div class="language-switcher" role="group" :aria-label="t('common.locale.label')">
    <button
      v-for="option in options"
      :key="option.value"
      class="language-switcher__button"
      :class="{ 'language-switcher__button--active': locale === option.value }"
      type="button"
      :data-locale="option.value"
      @click="onSelect(option.value)"
    >
      {{ option.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { setLocale, type AppLocale } from "@/i18n";

const { t, locale } = useI18n();

const options = computed(() => [
  { value: "en" as const, label: t("common.locale.english") },
  { value: "zh-CN" as const, label: t("common.locale.chinese") },
]);

function onSelect(value: AppLocale) {
  setLocale(value);
}
</script>
```

- [ ] **Step 4: Place the switcher into the landing hero and shell header**

`web/src/views/LandingView.vue`

```vue
<script setup lang="ts">
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";
import { useI18n } from "vue-i18n";

const { t } = useI18n();
</script>

<template>
  <section class="landing__hero">
    <div class="landing__hero-toolbar">
      <LanguageSwitcher />
    </div>
    <!-- existing hero layout stays unchanged -->
  </section>
</template>
```

`web/src/components/layout/AppShell.vue`

```vue
<script setup lang="ts">
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";
</script>

<template>
  <n-layout-header bordered class="shell__header">
    <div class="shell__brand">...</div>
    <div class="shell__header-tools">
      <LanguageSwitcher />
      <!-- existing workbench selector / chips / settings / user menu remain -->
    </div>
  </n-layout-header>
</template>
```

- [ ] **Step 5: Verify switcher behavior in isolation and in mounted pages**

Run: `npm --prefix web test -- --run src/tests/language-switcher.spec.ts src/tests/mock-landing-panels.spec.ts src/tests/mock-prototype-shell.spec.ts`

Expected: PASS after landing and shell mount the switcher correctly.

- [ ] **Step 6: Commit the switcher integration**

```bash
git add web/src/components/i18n/LanguageSwitcher.vue web/src/views/LandingView.vue web/src/components/layout/AppShell.vue web/src/tests/language-switcher.spec.ts web/src/tests/mock-landing-panels.spec.ts web/src/tests/mock-prototype-shell.spec.ts
git commit -m "feat: add frontend language switcher"
```

### Task 3: Translate landing page copy and help-content dictionaries

**Files:**
- Modify: `web/src/i18n/locales/en.ts`
- Modify: `web/src/i18n/locales/zh-CN.ts`
- Modify: `web/src/views/LandingView.vue`
- Modify: `web/src/tests/mock-landing-panels.spec.ts`

- [ ] **Step 1: Add a failing landing locale test**

```ts
it("renders the landing hero in Chinese after locale switch", async () => {
  const wrapper = mount(LandingView, {
    global: { plugins: [i18n] },
  });

  setLocale("zh-CN");
  await nextTick();

  expect(wrapper.text()).toContain("我们不管理任务，我们推动任务持续前进。");
  expect(wrapper.text()).toContain("执行循环引擎");
});
```

- [ ] **Step 2: Run the landing test and confirm it fails**

Run: `npm --prefix web test -- --run src/tests/mock-landing-panels.spec.ts`

Expected: FAIL because landing copy is still hard-coded in English.

- [ ] **Step 3: Move landing strings into message catalogs**

`web/src/i18n/locales/en.ts`

```ts
export const en = {
  common: { /* existing common keys */ },
  landing: {
    eyebrow: "Workflow Cockpit",
    title: "We don't manage work. We keep work moving.",
    lead:
      "A loop engineering system for software delivery that keeps issues, MRs, and milestones moving through the next execution step.",
    impact:
      "Lower waiting time. Surface stalled work early. Make readiness explicit.",
    panels: {
      overview: "Overview",
      product: "Product",
      engineering: "Engineering",
    },
    diagram: {
      engine: {
        title: "Execution Loop Engine",
        label: "Clarify, advance, verify",
        description:
          "The loop engine evaluates state, writes memory, ranks the next action, and decides when to stop or escalate.",
      },
    },
  },
} as const;
```

`web/src/i18n/locales/zh-CN.ts`

```ts
export const zhCN = {
  common: { /* existing common keys */ },
  landing: {
    eyebrow: "工作流驾驶舱",
    title: "我们不管理任务，我们推动任务持续前进。",
    lead:
      "一个面向软件交付的 loop engineering 系统，让 issue、MR 和 milestone 持续进入下一步执行动作。",
    impact:
      "减少等待时间，更早暴露停滞工作，让 readiness 明确可见。",
    panels: {
      overview: "总览",
      product: "产品",
      engineering: "工程",
    },
    diagram: {
      engine: {
        title: "执行循环引擎",
        label: "澄清、推进、验证",
        description:
          "循环引擎负责评估状态、写入记忆、排序下一步动作，并决定何时停止或升级。",
      },
    },
  },
} as const;
```

`web/src/views/LandingView.vue`

```ts
const { t } = useI18n();

const panels = computed(() => [
  { id: "overview", label: t("landing.panels.overview") },
  { id: "product", label: t("landing.panels.product") },
  { id: "engineering", label: t("landing.panels.engineering") },
]);
```

```vue
<div class="landing__eyebrow">{{ t("landing.eyebrow") }}</div>
<h1>{{ t("landing.title") }}</h1>
<p class="landing__lead">{{ t("landing.lead") }}</p>
<p class="landing__impact">{{ t("landing.impact") }}</p>
```

- [ ] **Step 4: Translate the remaining landing cards, tooltip labels, and non-mock login copy**

Replace all remaining inline strings in `LandingView.vue` with `t(...)` calls or computed arrays backed by `landing.*` dictionaries. Keep the diagram structure and click-to-toggle behavior unchanged.

- [ ] **Step 5: Verify landing behavior in both locales**

Run: `npm --prefix web test -- --run src/tests/mock-landing-panels.spec.ts`

Expected: PASS with assertions for English defaults and Chinese post-switch rendering.

- [ ] **Step 6: Commit the landing migration**

```bash
git add web/src/i18n/locales/en.ts web/src/i18n/locales/zh-CN.ts web/src/views/LandingView.vue web/src/tests/mock-landing-panels.spec.ts
git commit -m "feat: localize landing page"
```

### Task 4: Translate shell and prototype workbench UI

**Files:**
- Modify: `web/src/components/layout/AppShell.vue`
- Modify: `web/src/views/prototype/PrototypeWorkbenchOverview.vue`
- Modify: `web/src/views/prototype/PrototypeIssuesView.vue`
- Modify: `web/src/views/prototype/PrototypeMrsView.vue`
- Modify: `web/src/views/prototype/PrototypeMilestonesView.vue`
- Modify: `web/src/views/prototype/PrototypeUserSettingsView.vue`
- Modify: `web/src/components/prototype/OverviewCards.vue`
- Modify: `web/src/components/prototype/RecommendedActionsCard.vue`
- Modify: `web/src/components/prototype/SkillVersionPanel.vue`
- Modify: `web/src/components/prototype/WorkbenchRolePanel.vue`
- Modify: `web/src/components/prototype/MemoryControlPanel.vue`
- Modify: `web/src/components/prototype/UserMenu.vue`
- Modify: `web/src/components/prototype/WorkflowStateBadge.vue`
- Modify: `web/src/components/issues/IssueStatePanel.vue`
- Modify: `web/src/components/workbench/WorkbenchSearchDialog.vue`
- Modify: `web/src/i18n/locales/en.ts`
- Modify: `web/src/i18n/locales/zh-CN.ts`
- Modify: `web/src/tests/mock-prototype-overview.spec.ts`
- Modify: `web/src/tests/mock-prototype-issues.spec.ts`
- Modify: `web/src/tests/mock-prototype-mrs.spec.ts`
- Modify: `web/src/tests/mock-prototype-milestones.spec.ts`
- Modify: `web/src/tests/mock-prototype-settings.spec.ts`
- Modify: `web/src/tests/mock-prototype-shell.spec.ts`
- Modify: `web/src/tests/issue-state-panel.spec.ts`

- [ ] **Step 1: Add a failing prototype Chinese-render test**

```ts
it("renders prototype navigation and settings in Chinese", async () => {
  setLocale("zh-CN");

  const wrapper = mount(AppShell, {
    props: { activeKey: "overview", prototypeMode: true },
    global: { plugins: [pinia, router, i18n] },
  });

  await nextTick();

  expect(wrapper.text()).toContain("总览");
  expect(wrapper.text()).toContain("里程碑");
  expect(wrapper.text()).toContain("设置");
});
```

- [ ] **Step 2: Run the relevant prototype tests and confirm they fail**

Run: `npm --prefix web test -- --run src/tests/mock-prototype-shell.spec.ts src/tests/mock-prototype-overview.spec.ts src/tests/mock-prototype-issues.spec.ts src/tests/mock-prototype-mrs.spec.ts src/tests/mock-prototype-milestones.spec.ts src/tests/mock-prototype-settings.spec.ts src/tests/issue-state-panel.spec.ts`

Expected: FAIL while shell and prototype views still render hard-coded English strings.

- [ ] **Step 3: Add shell and prototype namespaces to the message catalogs**

`web/src/i18n/locales/en.ts`

```ts
shell: {
  subtitle: "Agent Workbench",
  workbench: "Workbench",
  role: "Role",
  settings: "Settings",
  renameWorkbench: "Rename workbench",
  workbenchName: "Workbench name",
  navigation: {
    overview: "Overview",
    issues: "Issues",
    mrs: "MRs",
    milestones: "Milestones",
    pendingActions: "Pending Actions",
  },
},
prototype: {
  settings: {
    eyebrow: "User Settings",
    currentWorkbench: "Current Workbench",
    memoryControls: "Memory controls",
  },
},
```

`web/src/i18n/locales/zh-CN.ts`

```ts
shell: {
  subtitle: "Agent 工作台",
  workbench: "工作台",
  role: "角色",
  settings: "设置",
  renameWorkbench: "重命名工作台",
  workbenchName: "工作台名称",
  navigation: {
    overview: "总览",
    issues: "事项",
    mrs: "合并请求",
    milestones: "里程碑",
    pendingActions: "待执行动作",
  },
},
prototype: {
  settings: {
    eyebrow: "用户设置",
    currentWorkbench: "当前工作台",
    memoryControls: "记忆控制",
  },
},
```

- [ ] **Step 4: Replace visible inline strings with `t(...)` calls**

Representative conversions:

`web/src/components/layout/AppShell.vue`

```ts
const { t } = useI18n();

label: () =>
  h(RouterLink, { to: "/workbench" }, { default: () => t("shell.navigation.overview") }),
```

```vue
<div class="shell__subtitle">{{ t("shell.subtitle") }}</div>
<span class="shell__section-label">{{ t("shell.workbench") }}</span>
<n-button quaternary tag="a" href="/settings">{{ t("shell.settings") }}</n-button>
```

`web/src/components/prototype/MemoryControlPanel.vue`

```vue
<template #header>{{ t("prototype.settings.memoryControls") }}</template>
```

`web/src/views/prototype/PrototypeMilestonesView.vue`

```vue
<div class="prototype-page__eyebrow">{{ t("shell.navigation.milestones") }}</div>
```

- [ ] **Step 5: Verify translated shell and prototype rendering**

Run: `npm --prefix web test -- --run src/tests/mock-prototype-shell.spec.ts src/tests/mock-prototype-overview.spec.ts src/tests/mock-prototype-issues.spec.ts src/tests/mock-prototype-mrs.spec.ts src/tests/mock-prototype-milestones.spec.ts src/tests/mock-prototype-settings.spec.ts src/tests/issue-state-panel.spec.ts`

Expected: PASS

- [ ] **Step 6: Commit the prototype migration**

```bash
git add web/src/components/layout/AppShell.vue web/src/views/prototype web/src/components/prototype web/src/components/issues/IssueStatePanel.vue web/src/components/workbench/WorkbenchSearchDialog.vue web/src/i18n/locales/en.ts web/src/i18n/locales/zh-CN.ts web/src/tests/mock-prototype-shell.spec.ts web/src/tests/mock-prototype-overview.spec.ts web/src/tests/mock-prototype-issues.spec.ts web/src/tests/mock-prototype-mrs.spec.ts web/src/tests/mock-prototype-milestones.spec.ts web/src/tests/mock-prototype-settings.spec.ts web/src/tests/issue-state-panel.spec.ts
git commit -m "feat: localize prototype workbench ui"
```

### Task 5: Run the frontend quality gate and verify no regressions in mock mode

**Files:**
- Modify: any files required to fix test, lint, or type issues found during verification

- [ ] **Step 1: Run the focused frontend test suite**

Run: `npm --prefix web test -- --run src/tests/i18n-locale.spec.ts src/tests/language-switcher.spec.ts src/tests/mock-landing-panels.spec.ts src/tests/mock-prototype-shell.spec.ts src/tests/mock-prototype-overview.spec.ts src/tests/mock-prototype-issues.spec.ts src/tests/mock-prototype-mrs.spec.ts src/tests/mock-prototype-milestones.spec.ts src/tests/mock-prototype-settings.spec.ts src/tests/issue-state-panel.spec.ts src/tests/app.spec.ts src/tests/mock-prototype-routing.spec.ts`

Expected: PASS

- [ ] **Step 2: Run lint, typecheck, and build**

Run: `npm --prefix web run lint`
Expected: PASS

Run: `npm --prefix web run typecheck`
Expected: PASS

Run: `npm --prefix web run build`
Expected: PASS

- [ ] **Step 3: Smoke-check mock mode startup**

Run: `npm --prefix web run dev:mock`

Expected: local dev server starts, the landing page loads in mock mode, and language switching works without proxying to the backend for mock-only paths.

- [ ] **Step 4: Commit any final fixes from verification**

```bash
git add web
git commit -m "test: verify frontend i18n prototype flow"
```

## Spec Coverage Check

- Global i18n framework: covered in Task 1.
- Explicit switcher with `EN` / `中文`: covered in Task 2.
- Locale precedence and persistence: covered in Task 1 and Task 2.
- Landing page translation: covered in Task 3.
- App shell and prototype translation: covered in Task 4.
- Test coverage for selection and switching: covered in Tasks 1, 2, 3, and 4.
- Guardrails against route or skeleton changes: preserved throughout Tasks 2 to 4.

## Placeholder Scan

No `TODO`, `TBD`, or deferred implementation placeholders remain in the task steps. Any `/* existing ... */` fragments shown in snippets are context markers only; implementation work must expand them into the actual surrounding file content when editing.

## Type Consistency Check

- Shared locale type is `AppLocale`.
- Shared storage key is `issueflow.locale`.
- Shared switch API is `setLocale(...)`.
- Message namespaces are `common`, `landing`, `shell`, and `prototype.*`.

These names must remain consistent across helpers, components, and tests.
