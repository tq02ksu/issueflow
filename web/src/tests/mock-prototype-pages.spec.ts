import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createRouter, createMemoryHistory } from "vue-router";
import { beforeEach, describe, expect, it } from "vitest";
import { i18n } from "@/i18n";

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("turns view", () => {
  it("renders the turns list with multi-target turn data", async () => {
    const { default: TurnsView } = await import("@/views/TurnsView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/workbench/turns", component: TurnsView },
        { path: "/workbench", component: { template: "<div />" } },
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

    await router.push("/workbench/turns");
    await router.isReady();

    const wrapper = mount(TurnsView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("Turns");
    expect(text).toContain("turn-101");
    expect(text).toContain("waiting_approval");
    expect(text).toContain("turn-102");
    expect(text).toContain("completed");
    expect(text).toContain("turn-104");
    expect(text).toContain("failed");
    wrapper.unmount();
  });

  it("selects a turn and shows detail with agents and targets", async () => {
    const { default: TurnsView } = await import("@/views/TurnsView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/workbench/turns", component: TurnsView },
        { path: "/workbench", component: { template: "<div />" } },
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

    await router.push("/workbench/turns");
    await router.isReady();

    const wrapper = mount(TurnsView, {
      global: { plugins: [router, i18n] },
    });

    // The first turn should be auto-selected
    const text = wrapper.text();
    expect(text).toContain("MR Progression Core");
    expect(text).toContain("Evaluator Core");
    wrapper.unmount();
  });
});

describe("approvals view", () => {
  it("renders pending approvals for the active workbench", async () => {
    const { default: ApprovalsView } =
      await import("@/views/ApprovalsView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/workbench/approvals", component: ApprovalsView },
        { path: "/workbench", component: { template: "<div />" } },
        { path: "/workbench/turns", component: { template: "<div />" } },
        { path: "/workbench/agents", component: { template: "<div />" } },
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

    await router.push("/workbench/approvals");
    await router.isReady();

    const wrapper = mount(ApprovalsView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("Pending");
    expect(text).toContain("History");
    expect(text).toContain("issue_comment");
    expect(text).toContain("low");
    expect(text).toContain("medium");
    wrapper.unmount();
  });
});

describe("memory view", () => {
  it("renders memory scopes with loop, engineering, and governance", async () => {
    const { default: MemoryView } = await import("@/views/MemoryView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/workbench/memory", component: MemoryView },
        { path: "/workbench", component: { template: "<div />" } },
        { path: "/workbench/turns", component: { template: "<div />" } },
        { path: "/workbench/agents", component: { template: "<div />" } },
        { path: "/workbench/approvals", component: { template: "<div />" } },
        { path: "/workbench/issues", component: { template: "<div />" } },
        { path: "/workbench/mrs", component: { template: "<div />" } },
        { path: "/workbench/milestones", component: { template: "<div />" } },
        { path: "/settings", component: { template: "<div />" } },
        { path: "/system/skills", component: { template: "<div />" } },
        { path: "/system/gateway", component: { template: "<div />" } },
        { path: "/system/governance", component: { template: "<div />" } },
      ],
    });

    await router.push("/workbench/memory");
    await router.isReady();

    const wrapper = mount(MemoryView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("Loop memory");
    expect(text).toContain("Engineering memory");
    expect(text).toContain("Governance memory");
    expect(text).toContain("healthy");
    expect(text).toContain("Clear memory");
    wrapper.unmount();
  });
});

describe("gateway view", () => {
  it("renders model routing, budget, providers, and usage logs", async () => {
    const { default: GatewayView } = await import("@/views/GatewayView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/system/gateway", component: GatewayView },
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
        { path: "/system/governance", component: { template: "<div />" } },
      ],
    });

    await router.push("/system/gateway");
    await router.isReady();

    const wrapper = mount(GatewayView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("Model routing");
    expect(text).toContain("Budget");
    expect(text).toContain("Providers");
    expect(text).toContain("Usage logs");
    wrapper.unmount();
  });
});

describe("governance view", () => {
  it("renders verification debt, risk alerts, comprehension rot, and proposals", async () => {
    const { default: GovernanceView } =
      await import("@/views/GovernanceView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/system/governance", component: GovernanceView },
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
      ],
    });

    await router.push("/system/governance");
    await router.isReady();

    const wrapper = mount(GovernanceView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("Verification debt");
    expect(text).toContain("Risk alerts");
    expect(text).toContain("Comprehension rot");
    expect(text).toContain("Improvement proposals");
    wrapper.unmount();
  });
});

describe("skills page view", () => {
  it("renders skill list and loop bindings", async () => {
    const { default: SkillsPageView } =
      await import("@/views/SkillsPageView.vue");

    const router = createRouter({
      history: createMemoryHistory(),
      routes: [
        { path: "/system/skills", component: SkillsPageView },
        { path: "/workbench", component: { template: "<div />" } },
        { path: "/workbench/turns", component: { template: "<div />" } },
        { path: "/workbench/agents", component: { template: "<div />" } },
        { path: "/workbench/approvals", component: { template: "<div />" } },
        { path: "/workbench/memory", component: { template: "<div />" } },
        { path: "/workbench/issues", component: { template: "<div />" } },
        { path: "/workbench/mrs", component: { template: "<div />" } },
        { path: "/workbench/milestones", component: { template: "<div />" } },
        { path: "/settings", component: { template: "<div />" } },
        { path: "/system/gateway", component: { template: "<div />" } },
        { path: "/system/governance", component: { template: "<div />" } },
      ],
    });

    await router.push("/system/skills");
    await router.isReady();

    const wrapper = mount(SkillsPageView, {
      global: { plugins: [router, i18n] },
    });

    const text = wrapper.text();
    expect(text).toContain("delivery-skill");
    expect(text).toContain("Loop bindings");
    wrapper.unmount();
  });
});
