import { afterEach, describe, expect, it, vi } from "vitest";

describe("mock prototype routing", () => {
  afterEach(() => {
    vi.unstubAllEnvs();
    vi.resetModules();
  });

  it("registers public mock prototype routes in mock mode", async () => {
    vi.stubEnv("VITE_APP_MODE", "mock");

    const { routes } = await import("@/router");

    const routeNames = routes.map((route) => route.name);

    expect(routeNames).toContain("mock-landing");
    expect(routeNames).toContain("mock-workbench-overview");
    expect(routeNames).toContain("mock-workbench-issues");
    expect(routeNames).toContain("mock-workbench-mrs");
    expect(routeNames).toContain("mock-workbench-milestones");
    expect(routeNames).toContain("mock-workbench-turns");
    expect(routeNames).toContain("mock-workbench-approvals");
    expect(routeNames).toContain("mock-workbench-memory");
    expect(routeNames).toContain("mock-settings-loop");
    expect(routeNames).toContain("mock-settings-integrations");
    expect(routeNames).toContain("mock-settings-access");
    expect(routeNames).toContain("mock-skills");
    expect(
      routes.every((route) => route.meta?.public === true),
    ).toBeTruthy();
  }, 10000);

  it("lazy loads heavy workbench routes in mock mode", async () => {
    vi.stubEnv("VITE_APP_MODE", "mock");

    const { routes } = await import("@/router");

    const lazyRouteNames = [
      "mock-workbench-overview",
      "mock-workbench-turns",
      "mock-workbench-agents",
      "mock-workbench-approvals",
      "mock-workbench-memory",
      "mock-workbench-issues",
      "mock-workbench-mrs",
      "mock-workbench-milestones",
      "mock-settings-loop",
      "mock-settings-integrations",
      "mock-settings-access",
      "mock-skills",
      "mock-system-gateway",
      "mock-system-governance",
    ];

    for (const routeName of lazyRouteNames) {
      const route = routes.find((item) => item.name === routeName);
      expect(typeof route?.component).toBe("function");
    }
  }, 10000);
});
