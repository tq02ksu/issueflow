import { http, HttpResponse } from "msw";

const mockUser = {
  user_id: 7,
  sub: "mock-user-sub",
};

const mockProjects = [
  {
    id: 101,
    name: "Get Started Platform",
    path_with_namespace: "demo/get-started",
    namespace: { id: 1, name: "demo", kind: "group" },
  },
  {
    id: 102,
    name: "Alpha Delivery",
    path_with_namespace: "demo/alpha-delivery",
    namespace: { id: 1, name: "demo", kind: "group" },
  },
  {
    id: 103,
    name: "AA Workflow Lab",
    path_with_namespace: "demo/aa-workflow-lab",
    namespace: { id: 1, name: "demo", kind: "group" },
  },
];

const mockWorkbenches = [
  {
    id: 1,
    user_id: 7,
    project_id: 102,
    project_name: "Alpha Delivery",
    project_path: "demo/alpha-delivery",
    name: "Alpha Delivery",
    created_at: "2026-07-02T07:00:00Z",
    updated_at: "2026-07-02T07:00:00Z",
  },
];

const mockCapabilities = {
  features: ["overview", "issues", "pending_actions", "releases"],
};

const mockMilestonesByProject: Record<number, unknown[]> = {
  102: [
    {
      id: 501,
      iid: 1,
      title: "Beta launch",
      description: "Workflow-centered delivery prototype",
      state: "active",
      due_date: "2026-07-25",
      web_url: "https://gitlab.example.com/demo/alpha-delivery/-/milestones/1",
    },
  ],
};

const mockIssuesByProject: Record<number, unknown[]> = {
  102: [
    {
      id: 9001,
      iid: 11,
      project_id: 102,
      title: "Define workflow state cards",
      description: "Show issue progression clearly in the cockpit.",
      state: "opened",
      web_url: "https://gitlab.example.com/demo/alpha-delivery/-/issues/11",
      milestone: { id: 501, title: "Beta launch" },
      labels: ["workflow", "frontend"],
      created_at: "2026-07-01T08:00:00Z",
      updated_at: "2026-07-02T06:00:00Z",
    },
    {
      id: 9002,
      iid: 12,
      project_id: 102,
      title: "Expose MR review states",
      description: "Keep MR delivery status visible in the workbench.",
      state: "opened",
      web_url: "https://gitlab.example.com/demo/alpha-delivery/-/issues/12",
      milestone: { id: 501, title: "Beta launch" },
      labels: ["mr", "prototype"],
      created_at: "2026-07-01T09:00:00Z",
      updated_at: "2026-07-02T06:30:00Z",
    },
  ],
};

const mockIssueNotesByProjectAndIid: Record<string, unknown[]> = {
  "102:11": [
    {
      id: 301,
      body: "State needs to be first-class in the detail pane.",
      author_name: "Mock PM",
      created_at: "2026-07-02T05:00:00Z",
    },
  ],
  "102:12": [
    {
      id: 302,
      body: "MR actions should lead with review readiness.",
      author_name: "Mock Lead",
      created_at: "2026-07-02T05:30:00Z",
    },
  ],
};

const mockSessionsByWorkbench: Record<number, unknown[]> = {
  1: [
    {
      id: "session-1",
      title: "Workflow cockpit iteration",
      last_message_at: "2026-07-02T06:20:00Z",
      latest_state: "running",
    },
  ],
};

const mockSessionDetails: Record<string, unknown> = {
  "session-1": {
    id: "session-1",
    title: "Workflow cockpit iteration",
    messages: [
      { role: "assistant", content: "Prioritize blocked and ready items first." },
    ],
  },
};

const mockPendingActionsByWorkbench: Record<number, unknown[]> = {
  1: [],
};

function filterProjects(query: string) {
  const normalized = query.trim().toLowerCase();
  if (!normalized) return [];

  return mockProjects.filter(
    (project) =>
      project.name.toLowerCase().includes(normalized) ||
      project.path_with_namespace.toLowerCase().includes(normalized),
  );
}

export const handlers = [
  http.get("/api/auth/me", () => HttpResponse.json(mockUser)),

  http.get("/api/workbenches", () => HttpResponse.json(mockWorkbenches)),

  http.get("/api/workbenches/:id/capabilities", () =>
    HttpResponse.json(mockCapabilities),
  ),

  http.post("/api/workbenches", async ({ request }) => {
    const body = (await request.json()) as {
      project_id: number;
      project_path: string;
      name: string;
    };

    return HttpResponse.json({
      id: 2,
      user_id: 7,
      project_id: body.project_id,
      project_name: body.project_path.split("/").pop() ?? body.project_path,
      project_path: body.project_path,
      name: body.name,
      created_at: "2026-07-02T07:10:00Z",
      updated_at: "2026-07-02T07:10:00Z",
    });
  }),

  http.put("/api/workbenches/:id", async ({ params, request }) => {
    const body = (await request.json()) as {
      project_id: number;
      project_path: string;
      name: string;
    };

    return HttpResponse.json({
      id: Number(params.id),
      user_id: 7,
      project_id: body.project_id,
      project_name: body.project_path.split("/").pop() ?? body.project_path,
      project_path: body.project_path,
      name: body.name,
      created_at: "2026-07-02T07:00:00Z",
      updated_at: "2026-07-02T07:15:00Z",
    });
  }),

  http.get("/api/projects", ({ request }) => {
    const url = new URL(request.url);
    const search = url.searchParams.get("search") ?? "";
    return HttpResponse.json(filterProjects(search));
  }),

  http.get("/api/projects/:projectId/issues", ({ params }) => {
    return HttpResponse.json(
      mockIssuesByProject[Number(params.projectId)] ?? [],
    );
  }),

  http.get("/api/projects/:projectId/milestones", ({ params }) => {
    return HttpResponse.json(
      mockMilestonesByProject[Number(params.projectId)] ?? [],
    );
  }),

  http.get("/api/projects/:projectId/issues/:issueIid/notes", ({ params }) => {
    const key = `${params.projectId}:${params.issueIid}`;
    return HttpResponse.json(mockIssueNotesByProjectAndIid[key] ?? []);
  }),

  http.get("/api/workbenches/:workbenchId/agent-sessions", ({ params }) => {
    return HttpResponse.json(
      mockSessionsByWorkbench[Number(params.workbenchId)] ?? [],
    );
  }),

  http.post("/api/workbenches/:workbenchId/agent-sessions", ({ params }) => {
    return HttpResponse.json({
      id: `session-${params.workbenchId}-new`,
      title: "New Session",
      last_message_at: "2026-07-02T07:20:00Z",
      latest_state: null,
    });
  }),

  http.get(
    "/api/workbenches/:workbenchId/agent-sessions/:sessionId",
    ({ params }) =>
      HttpResponse.json(
        mockSessionDetails[String(params.sessionId)] ?? {
          id: String(params.sessionId),
          title: "Mock Session",
          messages: [],
        },
      ),
  ),

  http.delete(
    "/api/workbenches/:workbenchId/agent-sessions/:sessionId",
    () => new HttpResponse(null, { status: 204 }),
  ),

  http.patch(
    "/api/workbenches/:workbenchId/agent-sessions/:sessionId",
    async ({ params, request }) => {
      const body = (await request.json()) as { title: string };
      return HttpResponse.json({
        id: String(params.sessionId),
        title: body.title,
        messages: [],
      });
    },
  ),

  http.get("/api/workbenches/:workbenchId/pending-actions", ({ params }) => {
    return HttpResponse.json(
      mockPendingActionsByWorkbench[Number(params.workbenchId)] ?? [],
    );
  }),
];
