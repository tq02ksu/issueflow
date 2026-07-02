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
    expect(routeNames).toContain("mock-user-settings");
    expect(
      routes.every((route) => route.meta?.public === true),
    ).toBeTruthy();
  }, 10000);

  it("lazy loads heavy workbench routes in mock mode", async () => {
    vi.stubEnv("VITE_APP_MODE", "mock");

    const { routes } = await import("@/router");

    const lazyRouteNames = [
      "mock-workbench-overview",
      "mock-workbench-issues",
      "mock-workbench-mrs",
      "mock-workbench-milestones",
      "mock-user-settings",
    ];

    for (const routeName of lazyRouteNames) {
      const route = routes.find((item) => item.name === routeName);
      expect(typeof route?.component).toBe("function");
    }
  }, 10000);
});
