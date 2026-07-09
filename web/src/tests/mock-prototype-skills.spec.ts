import { beforeEach, describe, expect, it } from "vitest";
import { createPinia, setActivePinia } from "pinia";
import { prototypeSkills, prototypeWorkbenches } from "@/mock/prototype.data";
import { usePrototypeStore } from "@/stores/prototype.store";

describe("prototype skills data", () => {
  it("assigns every skill to a real workbench with a business summary", () => {
    for (const skill of prototypeSkills) {
      expect(
        prototypeWorkbenches.some((wb) => wb.id === skill.workbenchId),
      ).toBe(true);
      expect(skill.summary.length).toBeGreaterThan(0);
      for (const version of skill.versions) {
        expect(version.focus.length).toBeGreaterThan(0);
      }
    }
  });

  it("covers all four role workbenches", () => {
    const workbenchIds = new Set(prototypeSkills.map((s) => s.workbenchId));
    for (const id of ["alpha", "beta", "gamma", "delta"]) {
      expect(workbenchIds.has(id)).toBe(true);
    }
  });
});

describe("prototype store skills by role", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
    localStorage.clear();
  });

  it("shows only the active workbench's skills", () => {
    const store = usePrototypeStore();

    store.setActiveRole("developer");
    expect(store.availableSkills.every((s) => s.workbenchId === "alpha")).toBe(
      true,
    );
    expect(store.availableSkills.some((s) => s.id === "delivery-skill")).toBe(
      true,
    );

    store.setActiveRole("manager");
    expect(store.availableSkills.length).toBeGreaterThan(0);
    expect(store.availableSkills.every((s) => s.workbenchId === "gamma")).toBe(
      true,
    );
  });

  it("exposes the active skill for the current workbench", () => {
    const store = usePrototypeStore();
    store.setActiveRole("product");
    expect(store.activeSkill?.workbenchId).toBe("beta");
    const activeVersion = store.currentWorkbench?.activeSkillVersionId;
    expect(
      store.activeSkill?.versions.some((v) => v.id === activeVersion),
    ).toBe(true);
  });
});
