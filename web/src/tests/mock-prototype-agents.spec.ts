import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createRouter, createMemoryHistory } from "vue-router";
import { beforeEach, describe, expect, it } from "vitest";
import { i18n } from "@/i18n";

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("agents view", () => {
  it("renders the agents page with loop core, workers, and external agents", async () => {
    const { default: AgentsView } = await import("@/views/AgentsView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/workbench/agents", component: AgentsView },
        { path: "/workbench", component: { template: "<div />" } },
        { path: "/workbench/turns", component: { template: "<div />" } },
        { path: "/workbench/approvals", component: { template: "<div />" } },
        { path: "/workbench/memory", component: { template: "<div />" } },
        { path: "/workbench/issues", component: { template: "<div />" } },
        { path: "/workbench/mrs", component: { template: "<div />" } },
        { path: "/workbench/milestones", component: { template: "<div />" } },
        { path: "/settings", component: { template: "<div />" } },
        { path: "/system/skills", component: { template: "<div />" } },
        { path: "/system/gateway", component: { template: "<div />" } },
        { path: "/system/governance", component: { template: "<div />" } },
      ],
    });

    await router.push("/workbench/agents");
    await router.isReady();

    const wrapper = mount(AgentsView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("Loop Core");
    expect(text).toContain("Worker agents");
    expect(text).toContain("External agents");
    expect(text).toContain("Loop Executor v2");
    expect(text).toContain("Milestone Health Core");
    wrapper.unmount();
  });
});
