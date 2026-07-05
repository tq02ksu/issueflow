import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { usePrototypeStore } from "@/stores/prototype.store";

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("prototype memory store", () => {
  it("groups memory items by scope", () => {
    const store = usePrototypeStore();

    expect(store.visibleMemoryItems.length).toBeGreaterThan(0);
    expect(store.memoryItemsByScope.loop.length).toBeGreaterThan(0);
    expect(store.memoryItemsByScope.engineering.length).toBeGreaterThan(0);
    expect(store.memoryItemsByScope.governance.length).toBeGreaterThan(0);
  });

  it("memory items have source run references", () => {
    const store = usePrototypeStore();

    for (const item of store.visibleMemoryItems) {
      expect(item.sourceRunIds.length).toBeGreaterThan(0);
      expect(item.summary.length).toBeGreaterThan(0);
    }
  });
});
