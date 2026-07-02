import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import { describe, expect, it } from "vitest";
import { i18n } from "@/i18n";
import PrototypeIssuesView from "@/views/prototype/PrototypeIssuesView.vue";

async function renderIssuesView() {
  setActivePinia(createPinia());

  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: "/workbench", component: { template: "<div />" } },
      { path: "/workbench/issues", component: { template: "<div />" } },
      { path: "/workbench/mrs", component: { template: "<div />" } },
      { path: "/workbench/milestones", component: { template: "<div />" } },
      { path: "/settings", component: { template: "<div />" } },
    ],
  });

  await router.push("/workbench/issues");
  await router.isReady();

  return mount(PrototypeIssuesView, {
    global: {
      plugins: [router, i18n],
    },
  });
}

describe("mock prototype issues view", () => {
  it("shows workflow state and recommended next action", async () => {
    const wrapper = await renderIssuesView();

    expect(wrapper.text()).toContain("ready_for_execution");
    expect(wrapper.text()).toContain("Start dev handoff");
    expect(wrapper.text()).toContain("Acceptance criteria");
    wrapper.unmount();
  });
});
