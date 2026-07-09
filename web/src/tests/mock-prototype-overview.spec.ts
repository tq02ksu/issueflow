import { beforeEach, describe, expect, it } from "vitest";
import { createPinia, setActivePinia } from "pinia";
import { mount } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import { usePrototypeStore } from "@/stores/prototype.store";
import { i18n, setLocale } from "@/i18n";

describe("prototype store", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("switches workbench and resolves workflow summaries", () => {
    const store = usePrototypeStore();

    store.selectWorkbench("alpha");

    expect(store.currentWorkbench?.id).toBe("alpha");
    expect(store.issueWorkflowSummary.length).toBeGreaterThan(0);
    expect(store.mrWorkflowSummary.length).toBeGreaterThan(0);
  });

  it("updates active skill version and memory actions", () => {
    const store = usePrototypeStore();

    store.setActiveSkillVersion("delivery-skill@2.2.0");
    store.clearWorkbenchMemory();
    expect(store.currentWorkbench?.activeSkillVersionId).toBe(
      "delivery-skill@2.2.0",
    );
    expect(store.lastMemoryAction).toBe("cleared");

    store.rebuildWorkbenchMemory();
    expect(store.lastMemoryAction).toBe("rebuilt");
  });
});

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
