import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import { afterEach, describe, expect, it } from "vitest";
import { i18n } from "@/i18n";
import { setLocale } from "@/i18n";
import PrototypeUserSettingsView from "@/views/prototype/PrototypeUserSettingsView.vue";

afterEach(() => {
  setLocale("en");
});

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
      plugins: [router, i18n],
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

  it("renders settings chrome in Chinese", async () => {
    setLocale("zh-CN");

    const wrapper = await renderSettingsView();

    expect(wrapper.text()).toContain("用户设置");
    expect(wrapper.text()).toContain("当前工作台");
    expect(wrapper.text()).toContain("记忆控制");
    wrapper.unmount();
  });
});
