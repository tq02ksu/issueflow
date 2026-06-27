import { describe, it, expect, beforeEach } from "vitest";
import { setActivePinia, createPinia } from "pinia";
import { useSessionStore } from "@/stores/session";

describe("workbench sidebar", () => {
  beforeEach(() => {
    setActivePinia(createPinia());
  });

  it("store initializes capabilities as empty", () => {
    const store = useSessionStore();
    expect(store.capabilities.features).toEqual([]);
  });

  it("setCurrentWorkbench updates currentWorkbenchId", () => {
    const store = useSessionStore();
    store.setWorkbenches([
      {
        id: 1,
        project_id: 42,
        project_name: "repo",
        project_path: "org/repo",
        name: "My WB",
        created_at: "",
      },
    ]);

    store.setCurrentWorkbench(1);

    expect(store.currentWorkbenchId.value).toBe(1);
  });

  it("Workbench interface includes name field", () => {
    const store = useSessionStore();
    store.setWorkbenches([
      {
        id: 1,
        project_id: 1,
        project_name: "x",
        project_path: "a/b",
        name: "display-name",
        created_at: "",
      },
    ]);

    expect(store.workbenches[0].name).toBe("display-name");
  });

  it("setCurrentWorkbench with null resets capabilities", () => {
    const store = useSessionStore();
    (store.capabilities as any).features = ["issues"];
    store.setCurrentWorkbench(null);
    expect(store.capabilities.features).toEqual([]);
  });
});
