import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import { describe, expect, it } from "vitest";
import PrototypeMrsView from "@/views/prototype/PrototypeMrsView.vue";

async function renderMrsView() {
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

  await router.push("/workbench/mrs");
  await router.isReady();

  return mount(PrototypeMrsView, {
    global: {
      plugins: [router],
    },
  });
}

describe("mock prototype MRs view", () => {
  it("shows MR workflow state and next action", async () => {
    const wrapper = await renderMrsView();

    expect(wrapper.text()).toContain("in_review");
    expect(wrapper.text()).toContain("Resolve review feedback");
    expect(wrapper.text()).toContain("Readiness checks");
    wrapper.unmount();
  });
});
