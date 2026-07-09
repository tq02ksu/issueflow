import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import { afterEach, describe, expect, it } from "vitest";
import AppShell from "@/components/layout/AppShell.vue";
import { i18n } from "@/i18n";
import { setLocale } from "@/i18n";

afterEach(() => {
  setLocale("en");
});

describe("prototype shell", () => {
  it("shows dashboard, turns, agents, approvals, memory, fact modules, and settings in mock mode", async () => {
    setActivePinia(createPinia());

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/workbench", component: { template: "<div />" } },
        { path: "/workbench/turns", component: { template: "<div />" } },
        { path: "/workbench/agents", component: { template: "<div />" } },
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

    await router.push("/workbench");
    await router.isReady();

    const wrapper = mount(AppShell, {
      props: {
        activeKey: "dashboard",
        prototypeMode: true,
      },
      global: {
        plugins: [router, i18n],
      },
      slots: {
        default: "<div>content</div>",
      },
    });

    expect(wrapper.text()).toContain("Dashboard");
    expect(wrapper.text()).toContain("Turns");
    expect(wrapper.text()).toContain("Agents");
    expect(wrapper.text()).toContain("Approvals");
    expect(wrapper.text()).toContain("Memory");
    expect(wrapper.text()).toContain("Fact Modules");
    expect(wrapper.text()).toContain("Settings");
    expect(wrapper.text()).toContain("System");
    wrapper.unmount();
  });

  it("renders shell navigation in Chinese", async () => {
    setActivePinia(createPinia());
    setLocale("zh-CN");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/workbench", component: { template: "<div />" } },
        { path: "/workbench/turns", component: { template: "<div />" } },
        { path: "/workbench/agents", component: { template: "<div />" } },
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

    await router.push("/workbench");
    await router.isReady();

    const wrapper = mount(AppShell, {
      props: {
        activeKey: "dashboard",
        prototypeMode: true,
      },
      global: {
        plugins: [router, i18n],
      },
      slots: {
        default: "<div>content</div>",
      },
    });

    expect(wrapper.text()).toContain("总览");
    expect(wrapper.text()).toContain("执行记录");
    expect(wrapper.text()).toContain("Agents");
    expect(wrapper.text()).toContain("待确认");
    expect(wrapper.text()).toContain("系统记忆");
    expect(wrapper.text()).toContain("事实模块");
    expect(wrapper.text()).toContain("设置");
    expect(wrapper.text()).toContain("系统");
    wrapper.unmount();
  });

  it("shows the active role name and a role switch control in the shell", async () => {
    setActivePinia(createPinia());
    setLocale("en");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [{ path: "/workbench", component: { template: "<div />" } }],
    });
    await router.push("/workbench");
    await router.isReady();

    const wrapper = mount(AppShell, {
      props: { activeKey: "dashboard", prototypeMode: true },
      global: { plugins: [router, i18n] },
      slots: { default: "<div>content</div>" },
    });

    expect(wrapper.text()).toContain("Developer");
    expect(wrapper.text()).toContain("Switch role");
    wrapper.unmount();
  });
});
