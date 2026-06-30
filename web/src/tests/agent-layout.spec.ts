import { createPinia, setActivePinia } from "pinia";
import { mount } from "@vue/test-utils";
import { nextTick } from "vue";
import { beforeEach, vi } from "vitest";
import WorkbenchView from "@/views/WorkbenchView.vue";
import PendingActionsView from "@/views/PendingActionsView.vue";
import ChatPanel from "@/components/agent/ChatPanel.vue";
import { useAgentStore } from "@/stores/agent.store";
import { useSessionStore } from "@/stores/session.store";

const mockListSessions = vi.fn();
const mockCreateSession = vi.fn();
const mockDeleteSession = vi.fn();
const mockGetSession = vi.fn();
const mockCheckAuth = vi.fn();
const mockFetchWorkbenches = vi.fn();
const mockListProjectIssues = vi.fn();
const mockListMilestones = vi.fn();
const mockListPendingActions = vi.fn();
const mockGetPendingAction = vi.fn();
const mockConfirmPendingAction = vi.fn();

vi.mock("@/api/agent.api", () => ({
  listSessions: (...args: unknown[]) => mockListSessions(...args),
  createSession: (...args: unknown[]) => mockCreateSession(...args),
  deleteSession: (...args: unknown[]) => mockDeleteSession(...args),
  getSession: (...args: unknown[]) => mockGetSession(...args),
}));

vi.mock("@/api/issues.api", () => ({
  listProjectIssues: (...args: unknown[]) => mockListProjectIssues(...args),
  listMilestones: (...args: unknown[]) => mockListMilestones(...args),
}));

vi.mock("@/api/workbench.api", async () => {
  const actual = await vi.importActual<typeof import("@/api/workbench.api")>(
    "@/api/workbench.api",
  );

  return {
    ...actual,
    listPendingActions: (...args: unknown[]) => mockListPendingActions(...args),
    getPendingAction: (...args: unknown[]) => mockGetPendingAction(...args),
    confirmPendingAction: (...args: unknown[]) =>
      mockConfirmPendingAction(...args),
  };
});

beforeEach(() => {
  setActivePinia(createPinia());
  vi.resetAllMocks();
  mockCheckAuth.mockResolvedValue(true);
  mockFetchWorkbenches.mockResolvedValue([]);
  mockListProjectIssues.mockResolvedValue([]);
  mockListMilestones.mockResolvedValue([]);
  mockListPendingActions.mockResolvedValue([]);
  mockGetPendingAction.mockResolvedValue(null);
  mockConfirmPendingAction.mockResolvedValue(null);
});

describe("agent layout", () => {
  it("adds an explicit active class to the selected session item", async () => {
    mockListSessions.mockResolvedValueOnce([
      {
        id: "session-1",
        user_id: 1,
        workbench_id: 1,
        title: "Selected Session",
        latest_state: null,
        last_message_at: "",
        created_at: "",
        updated_at: "",
      },
    ]);

    const sessionStore = useSessionStore();
    sessionStore.checkAuth = mockCheckAuth;
    sessionStore.fetchWorkbenches = mockFetchWorkbenches;
    sessionStore.setWorkbenches([
      {
        id: 1,
        project_id: 1,
        project_name: "repo",
        project_path: "group/repo",
        name: "WB",
        created_at: "",
      },
    ]);
    sessionStore.setCurrentWorkbench(1);

    const wrapper = mount(WorkbenchView, {
      global: {
        stubs: {
          AppShell: {
            template: "<div><slot /></div>",
          },
          NButton: {
            template: "<button><slot /></button>",
          },
          NText: {
            template: "<span><slot /></span>",
          },
          NEmpty: { template: "<div><slot /></div>" },
          NCard: { template: "<div><slot /><slot name='header' /></div>" },
          NSpin: { template: "<div><slot /></div>" },
          NStatistic: { template: "<div />" },
          NEllipsis: { template: "<span><slot /></span>" },
          NScrollbar: {
            template: "<div><slot /></div>",
          },
          ChatPanel: {
            template: "<div />",
          },
        },
      },
    });

    await nextTick();
    await nextTick();

    const agentStore = useAgentStore();
    agentStore.setActiveSession("session-1");
    await nextTick();

    expect(wrapper.find(".workbench-session-item--active").exists()).toBe(true);
  });

  it("uses a dedicated scroll container class in the chat panel", async () => {
    mockGetSession.mockResolvedValueOnce({ messages: [] });

    const sessionStore = useSessionStore();
    sessionStore.setCurrentWorkbench(1);

    const wrapper = mount(ChatPanel, {
      props: {
        sessionId: "session-1",
      },
      global: {
        stubs: {
          NScrollbar: {
            template: "<div class='stub-scrollbar'><slot /></div>",
          },
          ChatMessages: {
            template: "<div />",
          },
          ChatInput: {
            template: "<div />",
          },
        },
      },
    });

    await nextTick();

    expect(wrapper.find(".agent-chat-scroll").exists()).toBe(true);
  });

  it("keeps pending actions out of the overview page", async () => {
    mockListSessions.mockResolvedValueOnce([]);
    mockListPendingActions.mockResolvedValueOnce([
      {
        id: "action-1",
        workbench_id: 1,
        project_id: 1,
        artifact_type: "issue",
        artifact_id: "77",
        action_type: "update_gitlab_issue",
        status: "pending",
        payload: "{}",
      },
    ]);
    mockGetPendingAction.mockResolvedValueOnce({
      action: {
        id: "action-1",
        workbench_id: 1,
        project_id: 1,
        artifact_type: "issue",
        artifact_id: "77",
        action_type: "update_gitlab_issue",
        status: "pending",
        payload: "{}",
      },
      preview: {
        kind: "gitlab_issue_description",
        title: "Replace issue #77 description",
        body: "preview body",
      },
    });

    const sessionStore = useSessionStore();
    sessionStore.checkAuth = mockCheckAuth;
    sessionStore.fetchWorkbenches = mockFetchWorkbenches;
    sessionStore.setWorkbenches([
      {
        id: 1,
        project_id: 1,
        project_name: "repo",
        project_path: "group/repo",
        name: "WB",
        created_at: "",
      },
    ]);
    sessionStore.setCurrentWorkbench(1);

    const wrapper = mount(WorkbenchView, {
      global: {
        stubs: {
          AppShell: {
            template: "<div><slot /></div>",
          },
          NButton: {
            template: "<button><slot /></button>",
          },
          NText: {
            template: "<span><slot /></span>",
          },
          NEmpty: { template: "<div><slot /></div>" },
          NCard: { template: "<div><slot /><slot name='header' /></div>" },
          NSpin: { template: "<div><slot /></div>" },
          NStatistic: { template: "<div />" },
          NEllipsis: { template: "<span><slot /></span>" },
          NScrollbar: {
            template: "<div><slot /></div>",
          },
          ChatPanel: {
            template: "<div />",
          },
        },
      },
    });

    await nextTick();
    await nextTick();

    expect(wrapper.text()).not.toContain("Pending Actions");
    expect(wrapper.text()).not.toContain("Replace issue #77 description");
    expect(wrapper.text()).not.toContain("preview body");
  });

  it("renders pending action preview content on the standalone page", async () => {
    mockListPendingActions.mockResolvedValueOnce([
      {
        id: "action-1",
        workbench_id: 1,
        project_id: 1,
        artifact_type: "issue",
        artifact_id: "77",
        action_type: "update_gitlab_issue",
        status: "pending",
        payload: "{}",
      },
    ]);
    mockGetPendingAction.mockResolvedValueOnce({
      action: {
        id: "action-1",
        workbench_id: 1,
        project_id: 1,
        artifact_type: "issue",
        artifact_id: "77",
        action_type: "update_gitlab_issue",
        status: "pending",
        payload: "{}",
      },
      preview: {
        kind: "gitlab_issue_description",
        title: "Replace issue #77 description",
        body: "preview body",
      },
    });

    const sessionStore = useSessionStore();
    sessionStore.checkAuth = mockCheckAuth;
    sessionStore.fetchWorkbenches = mockFetchWorkbenches;
    sessionStore.setWorkbenches([
      {
        id: 1,
        project_id: 1,
        project_name: "repo",
        project_path: "group/repo",
        name: "WB",
        created_at: "",
      },
    ]);
    sessionStore.setCurrentWorkbench(1);

    const wrapper = mount(PendingActionsView, {
      global: {
        stubs: {
          AppShell: {
            template: "<div><slot /></div>",
          },
          NButton: {
            template: "<button><slot /></button>",
          },
          NText: {
            template: "<span><slot /></span>",
          },
          NEmpty: { template: "<div><slot /></div>" },
          NCard: {
            template:
              "<div><slot name='header' /><slot /><slot name='footer' /></div>",
          },
          NH3: { template: "<h3><slot /></h3>" },
          NSpin: { template: "<div><slot /></div>" },
        },
      },
    });

    await nextTick();
    await nextTick();

    expect(wrapper.text()).toContain("Pending Actions");
    expect(wrapper.text()).toContain("Replace issue #77 description");
    expect(wrapper.text()).toContain("preview body");
  });
});
