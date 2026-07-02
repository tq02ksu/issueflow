import { describe, expect, it } from "vitest";
import { search } from "@/api/projects.api";

describe("mock projects api", () => {
  it("returns mock projects without hitting the backend", async () => {
    const projects = await search("aa");

    expect(projects.length).toBeGreaterThan(0);
    expect(projects.some((project) => project.path_with_namespace.includes("aa"))).toBe(true);
  });
});
