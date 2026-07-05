import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import { afterEach, describe, expect, it } from "vitest";
import { i18n } from "@/i18n";
import { setLocale } from "@/i18n";
import PrototypeSettingsLoopView from "@/views/prototype/PrototypeSettingsLoopView.vue";

afterEach(() => {
  setLocale("en");
});

async function renderView() {
  setActivePinia(createPinia());

  const router = createRouter({
    history: createMemoryHistory(),
    routes: [
      { path: "/workbench", component: { template: "<div />" } },
      { path: "/settings/loop", component: { template: "<div />" } },
      { path: "/workbench/issues", component: { template: "<div />" } },
      { path: "/workbench/mrs", component: { template: "<div />" } },
      { path: "/workbench/milestones", component: { template: "<div />" } },
    ],
  });

  await router.push("/settings/loop");
  await router.isReady();

  return mount(PrototypeSettingsLoopView, {
    global: {
      plugins: [router, i18n],
    },
  });
}

describe("mock prototype settings loop view", () => {
  it("renders role, SOUL, PRINCIPLE, DESIGN, and SKILL sections", async () => {
    const wrapper = await renderView();

    const text = wrapper.text();
    expect(text).toContain("Role");
    expect(text).toContain("SOUL");
    expect(text).toContain("PRINCIPLE");
    expect(text).toContain("DESIGN");
    expect(text).toContain("SKILL");
    expect(text).toContain("Delivery Skill");
    wrapper.unmount();
  });

  it("renders settings loop chrome in Chinese", async () => {
    setLocale("zh-CN");

    const wrapper = await renderView();

    const text = wrapper.text();
    expect(text).toContain("角色");
    expect(text).toContain("SOUL");
    expect(text).toContain("PRINCIPLE");
    expect(text).toContain("DESIGN");
    expect(text).toContain("SKILL");
    wrapper.unmount();
  });
});
