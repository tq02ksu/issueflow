import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import { describe, expect, it } from "vitest";
import PrototypeUserSettingsView from "@/views/prototype/PrototypeUserSettingsView.vue";

async function renderSettingsView() {
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

  await router.push("/settings");
  await router.isReady();

  return mount(PrototypeUserSettingsView, {
    global: {
      plugins: [router],
    },
  });
}

describe("mock prototype settings view", () => {
  it("shows user settings, workbench role, and active skill version", async () => {
    const wrapper = await renderSettingsView();

    expect(wrapper.text()).toContain("User Settings");
    expect(wrapper.text()).toContain("Current Workbench");
    expect(wrapper.text()).toContain("Execution Driver");
    expect(wrapper.text()).toContain("delivery-skill@2.1.0");
    expect(wrapper.text()).toContain("Memory controls");
    wrapper.unmount();
  });
});
