import { createPinia, setActivePinia } from "pinia";
import { beforeEach, describe, expect, it } from "vitest";
import { usePrototypeStore } from "@/stores/prototype.store";

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("prototype store approvals & runs", () => {
  it("resolves pending and history approvals for the active workbench", () => {
    const store = usePrototypeStore();
    store.selectWorkbench("alpha");

    expect(store.pendingApprovals.length).toBeGreaterThan(0);
    expect(store.approvalHistory.length).toBeGreaterThan(0);
  });

  it("selects an approval and updates its status", () => {
    const store = usePrototypeStore();
    store.selectWorkbench("alpha");

    const pendingCount = store.pendingApprovals.length;
    const firstApproval = store.pendingApprovals[0];

    store.selectApproval(firstApproval.id);
    expect(store.selectedApproval?.id).toBe(firstApproval.id);

    store.updateApprovalStatus(firstApproval.id, "approved");
    expect(store.pendingApprovals.length).toBe(pendingCount - 1);
  });

  it("resolves runs for the active workbench", () => {
    const store = usePrototypeStore();
    store.selectWorkbench("alpha");

    expect(store.visibleTurns.length).toBeGreaterThan(0);
  });

  it("selects a run and resolves events", () => {
    const store = usePrototypeStore();
    store.selectWorkbench("alpha");

    const firstRun = store.visibleTurns[0];
    store.selectTurn(firstRun.id);

    expect(store.selectedTurn?.id).toBe(firstRun.id);
    expect(store.selectedTurn?.events.length).toBeGreaterThan(0);
  });
});
