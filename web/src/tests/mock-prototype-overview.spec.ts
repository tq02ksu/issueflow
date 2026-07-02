import { beforeEach, describe, expect, it } from "vitest";
import { createPinia, setActivePinia } from "pinia";
import { usePrototypeStore } from "@/stores/prototype.store";

describe("prototype store", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("switches workbench and resolves workflow summaries", () => {
    const store = usePrototypeStore();

    store.selectWorkbench("alpha");

    expect(store.currentWorkbench?.id).toBe("alpha");
    expect(store.issueWorkflowSummary.length).toBeGreaterThan(0);
    expect(store.mrWorkflowSummary.length).toBeGreaterThan(0);
  });

  it("updates active skill version and memory actions", () => {
    const store = usePrototypeStore();

    store.setActiveSkillVersion("delivery-skill@2.2.0");
    store.clearWorkbenchMemory();
    expect(store.currentWorkbench?.activeSkillVersionId).toBe(
      "delivery-skill@2.2.0",
    );
    expect(store.lastMemoryAction).toBe("cleared");

    store.rebuildWorkbenchMemory();
    expect(store.lastMemoryAction).toBe("rebuilt");
  });
});
