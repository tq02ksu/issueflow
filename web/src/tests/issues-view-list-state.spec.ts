import { flushPromises, mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { beforeEach, vi } from "vitest";
import IssuesView from "@/views/IssuesView.vue";
import { useSessionStore } from "@/stores/session.store";

const mockListProjectIssues = vi.fn();
const mockListMilestones = vi.fn();
const mockListIssueNotes = vi.fn();
const mockGetIssueState = vi.fn();

vi.mock("@/api/issues.api", () => ({
  listProjectIssues: (...args: unknown[]) => mockListProjectIssues(...args),
  listMilestones: (...args: unknown[]) => mockListMilestones(...args),
  listIssueNotes: (...args: unknown[]) => mockListIssueNotes(...args),
}));

vi.mock("@/api/workbench.api", async () => {
  const actual = await vi.importActual<typeof import("@/api/workbench.api")>(
    "@/api/workbench.api",
  );

  return {
    ...actual,
    getIssueState: (...args: unknown[]) => mockGetIssueState(...args),
  };
});

beforeEach(() => {
  setActivePinia(createPinia());
  vi.resetAllMocks();
  mockListProjectIssues.mockResolvedValue([
    {
      id: 1,
      iid: 77,
      project_id: 123,
      title: "Export report",
      description: "Users need CSV export",
      state: "opened",
      web_url: "https://gitlab.example.com/group/project/-/issues/77",
      milestone: null,
      labels: ["backend"],
      created_at: null,
      updated_at: null,
    },
  ]);
  mockListMilestones.mockResolvedValue([]);
  mockListIssueNotes.mockResolvedValue([]);
  mockGetIssueState.mockResolvedValue({
    projectMemory: {
      id: "state-1",
      artifact_type: "issue",
      artifact_id: "77",
      scope_type: "project",
      scope_key: "project:123",
      scope_project_id: 123,
      scope_workbench_id: null,
      scope_user_id: null,
      memory_kind: "issue_state",
      status: "active",
      revision: 1,
      updated_by_user_id: 1,
      input_text: "Export report",
      input_context: "{}",
      source_snapshot: null,
      spec: "{}",
      validation_suggestions: "{}",
      risk_notes: "[]",
      evaluation_summary:
        '{"current_state":"clarifying","proposed_next_state":"planned","summary":"needs more detail","missing_context":[],"blockers":[],"role_notes":{"product":[],"engineering":[],"delivery":[]},"heavy_agent":{"required":false,"reason":"","preferred_implementation":null}}',
      created_at: "",
      updated_at: "",
    },
    personalNote: null,
    pendingAction: null,
  });
});

describe("IssuesView list state badge", () => {
  it("renders the current work item state in the issue list", async () => {
    const sessionStore = useSessionStore();
    const workbenches = [
      {
        id: 1,
        project_id: 123,
        project_name: "project",
        project_path: "group/project",
        name: "Workbench",
      },
    ];
    sessionStore.checkAuth = vi.fn().mockResolvedValue(true);
    sessionStore.fetchWorkbenches = vi.fn().mockImplementation(async () => {
      sessionStore.setWorkbenches(workbenches);
      return workbenches;
    });
    sessionStore.setWorkbenches(workbenches);
    sessionStore.setCurrentWorkbench(1);

    const wrapper = mount(IssuesView, {
      global: {
        stubs: {
          AppShell: { template: "<div><slot /></div>" },
          IssueStatePanel: { template: "<div />" },
          NSpin: { template: "<div><slot /></div>" },
          NCard: { template: "<div><slot /></div>" },
          NList: { template: "<div><slot /></div>" },
          NListItem: { template: "<div><slot /><slot name='prefix' /></div>" },
          NTag: { template: "<span><slot /></span>" },
          NH3: { template: "<h3><slot /></h3>" },
          NRadioGroup: { template: "<div><slot /></div>" },
          NRadioButton: { template: "<button><slot /></button>" },
          NEmpty: { template: "<div><slot /></div>" },
          NDrawer: { template: "<div><slot /></div>" },
          NDrawerContent: { template: "<div><slot /></div>" },
          NDivider: { template: "<hr />" },
        },
      },
    });

    await flushPromises();

    expect(mockGetIssueState).toHaveBeenCalledWith(1, 77);
    expect(wrapper.text()).toContain("clarifying");
  });
});
