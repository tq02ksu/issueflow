import { flushPromises, mount } from "@vue/test-utils";
import { beforeEach, vi } from "vitest";
import IssueStatePanel from "@/components/issues/IssueStatePanel.vue";

const mockGetIssueState = vi.fn();
const mockEvaluateIssueState = vi.fn();

vi.mock("@/api/workbench.api", async () => {
  const actual = await vi.importActual<typeof import("@/api/workbench.api")>(
    "@/api/workbench.api",
  );

  return {
    ...actual,
    getIssueState: (...args: unknown[]) => mockGetIssueState(...args),
    evaluateIssueState: (...args: unknown[]) => mockEvaluateIssueState(...args),
  };
});

beforeEach(() => {
  vi.resetAllMocks();
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
        '{"current_state":"clarifying","proposed_next_state":"planned","summary":"needs more detail","missing_context":["Acceptance criteria are missing."],"blockers":[],"role_notes":{"product":["Add explicit acceptance criteria before handing off execution."],"engineering":[],"delivery":[]},"heavy_agent":{"required":false,"reason":"","preferred_implementation":null}}',
      created_at: "",
      updated_at: "",
    },
    personalNote: null,
    pendingAction: null,
  });
  mockEvaluateIssueState.mockResolvedValue(null);
});

describe("IssueStatePanel", () => {
  it("loads and renders the current issue state", async () => {
    const wrapper = mount(IssueStatePanel, {
      props: {
        workbenchId: 1,
        issueIid: 77,
      },
      global: {
        stubs: {
          NButton: { template: "<button><slot /></button>" },
          NCard: { template: "<div><slot /></div>" },
          NEmpty: { template: "<div><slot /></div>" },
          NSpin: { template: "<div><slot /></div>" },
          NTag: { template: "<span><slot /></span>" },
          NAlert: { template: "<div><slot /></div>" },
          NThing: { template: "<div><slot /></div>" },
        },
      },
    });

    await flushPromises();

    expect(mockGetIssueState).toHaveBeenCalledWith(1, 77);
    expect(wrapper.text()).toContain("Work Item State");
    expect(wrapper.text()).toContain("clarifying");
    expect(wrapper.text()).toContain("planned");
    expect(wrapper.text()).toContain("Acceptance criteria are missing.");
  });
});
