import { describe, expect, it } from "vitest";
import {
  prototypeRoleViews,
  prototypeWorkbenches,
} from "@/mock/prototype.data";
import { sortStatesByEmphasis } from "@/mock/prototype.ui-profile";
import { beforeEach, vi } from "vitest";
import { createPinia, setActivePinia } from "pinia";
import { usePrototypeStore } from "@/stores/prototype.store";
import { mount } from "@vue/test-utils";
import { flushPromises } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import { i18n, setLocale } from "@/i18n";

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

  it("binds each role to a distinct workbench dataset", () => {
    const boundIds = prototypeRoleViews.map((role) => role.workbenchId);
    expect(new Set(boundIds).size).toBe(prototypeRoleViews.length);
    for (const role of prototypeRoleViews) {
      expect(
        prototypeWorkbenches.some((wb) => wb.id === role.workbenchId),
      ).toBe(true);
    }
  });
});

describe("sortStatesByEmphasis", () => {
  it("orders emphasized states first in the given order", () => {
    const summary = [
      { state: "done", count: 5 },
      { state: "blocked", count: 2 },
      { state: "in_execution", count: 3 },
    ] as const;

    const sorted = sortStatesByEmphasis(
      [...summary],
      ["in_execution", "blocked"],
    );

    expect(sorted.map((item) => item.state)).toEqual([
      "in_execution",
      "blocked",
      "done",
    ]);
  });
});

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

  it("switches the visible workbench dataset when the role changes", () => {
    const store = usePrototypeStore();
    store.setActiveRole("manager");
    expect(store.currentWorkbenchId).toBe(store.activeRoleView?.workbenchId);
    expect(store.currentWorkbench?.id).toBe(store.activeRoleView?.workbenchId);

    store.setActiveRole("evolution");
    expect(store.currentWorkbenchId).toBe(store.activeRoleView?.workbenchId);
  });

  it("puts the active role's emphasis first in the issue summary", () => {
    const store = usePrototypeStore();
    store.setActiveRole("product");
    const emphasis = store.activeRoleView?.overviewEmphasis ?? [];
    const present = store.issueWorkflowSummary
      .map((item) => item.state)
      .filter((state) => emphasis.includes(state));
    const expectedOrder = emphasis.filter((state) => present.includes(state));
    expect(present).toEqual(expectedOrder);
  });
});

describe("role entry homepage", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
    vi.stubEnv("VITE_APP_MODE", "mock");
  });

  it("renders four role cards and selecting one sets role + navigates", async () => {
    setLocale("en");
    const { default: LandingView } = await import("@/views/LandingView.vue");

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
    await flushPromises();

    const store = usePrototypeStore();
    expect(store.activeRoleKey).toBe("manager");
    expect(router.currentRoute.value.path).toBe("/workbench");
    wrapper.unmount();
  });
});
