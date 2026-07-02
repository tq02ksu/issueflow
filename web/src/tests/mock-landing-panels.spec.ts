import { nextTick } from "vue";
import { mount } from "@vue/test-utils";
import { afterEach, describe, expect, it, vi } from "vitest";
import { i18n } from "@/i18n";
import { setLocale } from "@/i18n";

describe("mock landing panels", () => {
  afterEach(() => {
    vi.unstubAllEnvs();
    vi.resetModules();
    i18n.global.locale.value = "en";
    localStorage.clear();
  });

  it("shows overview first and lets the user switch to product and engineering panels", async () => {
    vi.stubEnv("VITE_APP_MODE", "mock");

    const { default: LandingView } = await import("@/views/LandingView.vue");

    const wrapper = mount(LandingView, {
      global: {
        plugins: [i18n],
      },
    });

    expect(wrapper.text()).toContain("Execution Bottlenecks We Remove");
    expect(wrapper.text()).toContain("Execution Loop Engine");
    expect(wrapper.text()).toContain("Issue");
    expect(wrapper.text()).toContain("MR");
    expect(wrapper.text()).toContain("Milestone");
    expect(wrapper.text()).not.toContain("Loop Execution Engine");
    expect(wrapper.findAll("button").some((button) => button.text() === "?")).toBe(
      true,
    );
    expect(wrapper.get('[data-locale="en"]').exists()).toBe(true);
    expect(wrapper.get('[data-locale="zh-CN"]').exists()).toBe(true);

    const buttons = wrapper.findAll("button");
    const productButton = buttons.find((button) => button.text() === "Product");
    const engineeringButton = buttons.find(
      (button) => button.text() === "Engineering",
    );

    expect(productButton).toBeTruthy();
    expect(engineeringButton).toBeTruthy();

    await productButton!.trigger("click");
    expect(wrapper.text()).toContain("Loop Engine");
    expect(wrapper.text()).not.toContain("Execution Bottlenecks We Remove");

    await engineeringButton!.trigger("click");
    expect(wrapper.text()).toContain("Pressure Logic");

    wrapper.unmount();
  });

  it("renders the landing hero in Chinese after locale switch", async () => {
    vi.stubEnv("VITE_APP_MODE", "mock");

    const { default: LandingView } = await import("@/views/LandingView.vue");

    const wrapper = mount(LandingView, {
      global: {
        plugins: [i18n],
      },
    });

    setLocale("zh-CN");
    await nextTick();

    expect(wrapper.text()).toContain("我们不管理任务。");
    expect(wrapper.text()).toContain("我们推动任务持续前进。");
    expect(wrapper.text()).toContain("执行循环引擎");

    wrapper.unmount();
  });

  it("opens only one diagram tooltip at a time", async () => {
    vi.stubEnv("VITE_APP_MODE", "mock");

    const { default: LandingView } = await import("@/views/LandingView.vue");

    const wrapper = mount(LandingView, {
      global: {
        plugins: [i18n],
      },
    });

    const diagram = wrapper.find('[data-testid="landing-diagram"]');
    expect(diagram.exists()).toBe(true);

    const tooltipButtons = wrapper.findAll('[data-testid="diagram-tooltip-trigger"]');
    expect(tooltipButtons.length).toBeGreaterThan(1);

    expect(wrapper.findAll('[data-tooltip-open="true"]').length).toBe(0);

    await tooltipButtons[0]!.trigger("click");
    expect(wrapper.findAll('[data-tooltip-open="true"]').length).toBe(1);

    await tooltipButtons[1]!.trigger("click");
    expect(wrapper.findAll('[data-tooltip-open="true"]').length).toBe(1);

    await tooltipButtons[1]!.trigger("click");
    expect(wrapper.findAll('[data-tooltip-open="true"]').length).toBe(0);

    wrapper.unmount();
  });
});
