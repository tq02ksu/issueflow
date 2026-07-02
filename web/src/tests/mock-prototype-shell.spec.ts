import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import { describe, expect, it } from "vitest";
import AppShell from "@/components/layout/AppShell.vue";

describe("prototype shell", () => {
  it("shows overview, issues, MRs, milestones, and settings entry in mock mode", async () => {
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

    await router.push("/workbench");
    await router.isReady();

    const wrapper = mount(AppShell, {
      props: {
        activeKey: "overview",
        prototypeMode: true,
      },
      global: {
        plugins: [router],
      },
      slots: {
        default: "<div>content</div>",
      },
    });

    expect(wrapper.text()).toContain("Overview");
    expect(wrapper.text()).toContain("Issues");
    expect(wrapper.text()).toContain("MRs");
    expect(wrapper.text()).toContain("Milestones");
    expect(wrapper.text()).toContain("Settings");
    wrapper.unmount();
  });
});
