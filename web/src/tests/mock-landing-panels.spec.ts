import { mount } from "@vue/test-utils";
import { createRouter, createMemoryHistory } from "vue-router";
import { createPinia, setActivePinia } from "pinia";
import { afterEach, describe, expect, it, vi } from "vitest";
import { i18n, setLocale } from "@/i18n";

afterEach(() => {
  setLocale("en");
  vi.unstubAllEnvs();
  vi.resetModules();
});

describe("mock landing", () => {
  it("renders the system diagram hero with loop engine ring", async () => {
    vi.stubEnv("VITE_APP_MODE", "mock");
    setActivePinia(createPinia());

    const { default: LandingView } = await import("@/views/LandingView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: "/workbench", component: { template: "<div />" } }],
    });

    const wrapper = mount(LandingView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("Loop engineering.");
    expect(text).toContain("for software delivery.");
    expect(text).toContain("Design the loop, run the turns");
    expect(text).toContain("LOOP");
    expect(text).toContain("Discover");
    expect(text).toContain("Handoff");
    expect(text).toContain("Verify");
    expect(text).toContain("Persist");
    expect(text).toContain("Schedule");
    expect(text).toContain("Decision Loop");
    wrapper.unmount();
  });

  it("renders the landing hero in Chinese", async () => {
    vi.stubEnv("VITE_APP_MODE", "mock");
    setActivePinia(createPinia());
    setLocale("zh-CN");

    const { default: LandingView } = await import("@/views/LandingView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: "/workbench", component: { template: "<div />" } }],
    });

    const wrapper = mount(LandingView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("发现");
    expect(text).toContain("交付");
    expect(text).toContain("验证");
    expect(text).toContain("决策闭环");
    expect(text).toContain("不编造");
    wrapper.unmount();
  });
});
