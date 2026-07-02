import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import { describe, expect, it } from "vitest";
import PrototypeMilestonesView from "@/views/prototype/PrototypeMilestonesView.vue";

async function renderMilestonesView() {
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

  await router.push("/workbench/milestones");
  await router.isReady();

  return mount(PrototypeMilestonesView, {
    global: {
      plugins: [router],
    },
  });
}

describe("mock prototype milestones view", () => {
  it("shows milestone workflow summaries", async () => {
    const wrapper = await renderMilestonesView();

    expect(wrapper.text()).toContain("Issue workflow");
    expect(wrapper.text()).toContain("MR workflow");
    expect(wrapper.text()).toContain("Beta launch");
    wrapper.unmount();
  });
});
