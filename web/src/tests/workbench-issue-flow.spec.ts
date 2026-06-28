import { createPinia, setActivePinia } from "pinia";
import { useSessionStore } from "@/stores/session.store";
import { beforeEach } from "vitest";

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("Workbench store", () => {
  it("setWorkbenches updates the workbench list", () => {
    const store = useSessionStore();
    store.setWorkbenches([
      {
        id: 1,
        project_id: 123,
        project_name: "test",
        project_path: "g/t",
        created_at: "2025",
      },
    ]);
    expect(store.workbenches.length).toBe(1);
    expect(store.workbenches[0].project_path).toBe("g/t");
  });

  it("setCurrentWorkbench updates the current id", () => {
    const store = useSessionStore();
    store.setCurrentWorkbench(42);
    expect(store.currentWorkbenchId.value).toBe(42);
  });
});

describe("Workbench issue flow", () => {
  it("setDraft sets phase to draft", () => {
    const store = useSessionStore();
    store.setDraft({ projectId: 1, title: "t", description: "d" });
    expect(store.phase.value).toBe("draft");
    expect(store.draft.value?.title).toBe("t");
  });

  it("confirmDraft sets phase to confirming", () => {
    const store = useSessionStore();
    store.confirmDraft();
    expect(store.phase.value).toBe("confirming");
  });

  it("setCreated sets phase to created", () => {
    const store = useSessionStore();
    store.setCreated({
      id: 1,
      iid: 2,
      projectId: 3,
      title: "t",
      webUrl: "https://example.com",
    });
    expect(store.phase.value).toBe("created");
    expect(store.created.value?.webUrl).toBe("https://example.com");
  });
});
