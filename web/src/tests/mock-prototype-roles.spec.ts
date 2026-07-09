import { describe, expect, it } from "vitest";
import { prototypeRoleViews } from "@/mock/prototype.data";
import { sortStatesByEmphasis } from "@/mock/prototype.ui-profile";

describe("prototype role views", () => {
  it("defines exactly the four project roles", () => {
    expect(prototypeRoleViews.map((role) => role.key)).toEqual([
      "developer",
      "manager",
      "product",
      "evolution",
    ]);
  });

  it("gives every role non-empty signal cards and quick entries", () => {
    for (const role of prototypeRoleViews) {
      expect(role.signalCards.length).toBeGreaterThan(0);
      expect(role.quickEntries.length).toBeGreaterThan(0);
      expect(role.overviewEmphasis.length).toBeGreaterThan(0);
    }
  });

  it("only routes quick entries to known workbench/system/settings paths", () => {
    const allowed = [
      "/workbench",
      "/workbench/issues",
      "/workbench/mrs",
      "/workbench/milestones",
      "/workbench/turns",
      "/workbench/approvals",
      "/workbench/memory",
      "/skills",
      "/system/gateway",
      "/system/governance",
      "/settings/loop",
      "/settings/integrations",
      "/settings/access",
    ];
    for (const role of prototypeRoleViews) {
      for (const entry of role.quickEntries) {
        expect(allowed).toContain(entry.to);
      }
    }
  });

  it("includes deployment readiness for the manager role", () => {
    const manager = prototypeRoleViews.find((role) => role.key === "manager");
    const ids = manager?.signalCards.map((card) => card.id) ?? [];
    expect(ids).toContain("release-readiness");
  });
});

describe("sortStatesByEmphasis", () => {
  it("orders emphasized states first in the given order", () => {
    const summary = [
      { state: "done", count: 5 },
      { state: "blocked", count: 2 },
      { state: "in_execution", count: 3 },
    ] as const;

    const sorted = sortStatesByEmphasis(
      [...summary],
      ["in_execution", "blocked"],
    );

    expect(sorted.map((item) => item.state)).toEqual([
      "in_execution",
      "blocked",
      "done",
    ]);
  });
});
