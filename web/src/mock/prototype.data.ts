import type {
  PrototypeActivityItem,
  PrototypeIssue,
  PrototypeMemoryScope,
  PrototypeMilestone,
  PrototypeMr,
  PrototypeUserSoul,
  PrototypeSkill,
  PrototypeWorkbench,
} from "./prototype.types";

export const prototypeWorkbenches: PrototypeWorkbench[] = [
  {
    id: "alpha",
    projectId: 102,
    projectPath: "demo/alpha-delivery",
    name: "Alpha Delivery",
    role: {
      name: "Execution Driver",
      personaSummary:
        "Pushes work toward explicit next actions and closes blockers early.",
      waysOfWorking: [
        "Prefer explicit next steps",
        "Escalate blockers quickly",
        "Keep MRs mergeable",
      ],
      goals: [
        "Move issues to execution",
        "Keep review flow short",
        "Protect milestone scope",
      ],
    },
    activeSkillVersionId: "delivery-skill@2.1.0",
  },
  {
    id: "beta",
    projectId: 103,
    projectPath: "demo/aa-workflow-lab",
    name: "AA Workflow Lab",
    role: {
      name: "Product Strategist",
      personaSummary:
        "Keeps acceptance, scope, and value clarity ahead of execution noise.",
      waysOfWorking: [
        "Clarify before handing off",
        "Keep acceptance visible",
        "Prefer explicit risk notes",
      ],
      goals: [
        "Reduce clarifying churn",
        "Improve milestone readiness",
        "Tighten issue acceptance quality",
      ],
    },
    activeSkillVersionId: "delivery-skill@2.2.0",
  },
];

export const prototypeSkills: PrototypeSkill[] = [
  {
    id: "delivery-skill",
    name: "Delivery Skill",
    versions: [
      {
        id: "delivery-skill@2.1.0",
        version: "2.1.0",
        enabled: true,
        uiProfile: {
          tone: "operator",
          density: "compact",
          overviewEmphasis: ["blocked", "ready_for_execution", "in_review"],
          issueFieldPriority: ["state", "blockerSummary", "nextActionSummary"],
          mrFieldPriority: ["state", "reviewSummary", "nextActionSummary"],
          milestoneFieldPriority: ["riskSummary", "nextActionSummary", "goal"],
          defaultExpandedSections: ["state", "blockers", "readiness"],
          recommendedActionOrder: [
            "unblock",
            "start_dev_handoff",
            "resolve_review",
          ],
        },
      },
      {
        id: "delivery-skill@2.2.0",
        version: "2.2.0",
        enabled: true,
        uiProfile: {
          tone: "direct",
          density: "balanced",
          overviewEmphasis: ["clarifying", "planned", "blocked"],
          issueFieldPriority: ["state", "nextActionSummary", "blockerSummary"],
          mrFieldPriority: ["state", "nextActionSummary", "reviewSummary"],
          milestoneFieldPriority: ["goal", "riskSummary", "nextActionSummary"],
          defaultExpandedSections: ["state", "acceptance", "goal"],
          recommendedActionOrder: [
            "clarify_scope",
            "tighten_acceptance",
            "protect_merge_readiness",
          ],
        },
      },
    ],
  },
];

export const prototypeIssues: PrototypeIssue[] = [
  {
    id: "issue-101",
    workbenchId: "alpha",
    iid: 11,
    title: "Define workflow state cards",
    state: "ready_for_execution",
    blockerSummary: "UI profile ordering still missing.",
    nextActionSummary: "Start dev handoff",
    description: "Show issue progression clearly in the cockpit.",
    acceptanceCriteria: [
      "State is visible in list and detail",
      "Next action is always visible",
    ],
    verificationPlan: [
      "Check issue list shows state badge",
      "Confirm detail panel keeps next action pinned",
    ],
    risks: ["Skill emphasis may hide blockers if not constrained."],
  },
  {
    id: "issue-102",
    workbenchId: "alpha",
    iid: 12,
    title: "Expose MR review states",
    state: "clarifying",
    blockerSummary: "Review-state vocabulary not agreed.",
    nextActionSummary: "Clarify review state model",
    description: "Keep MR delivery status visible in the workbench.",
    acceptanceCriteria: [
      "Draft and review states are distinct",
      "Blocked MRs are obvious in overview",
    ],
    verificationPlan: [
      "Review state copy is consistent with issue view",
      "Blocked MRs stay visible in overview summary",
    ],
    risks: ["State model may drift from backend terminology."],
  },
  {
    id: "issue-201",
    workbenchId: "beta",
    iid: 31,
    title: "Tighten milestone acceptance gates",
    state: "planned",
    blockerSummary: "Acceptance checklist not finalized.",
    nextActionSummary: "Refine acceptance criteria",
    description: "Protect milestone quality before execution handoff.",
    acceptanceCriteria: [
      "Milestone detail shows issue and MR state mix",
      "Blocked items are called out first",
    ],
    verificationPlan: [
      "Milestone detail renders both workflow summaries",
      "Next action stays visible for long sessions",
    ],
    risks: ["Too much detail may bury next-step guidance."],
  },
];

export const prototypeMrs: PrototypeMr[] = [
  {
    id: "mr-88",
    workbenchId: "alpha",
    iid: 88,
    title: "Render issue workflow badges",
    state: "in_review",
    reviewSummary: "Two comments remain on workflow naming.",
    nextActionSummary: "Resolve review feedback",
    linkedIssueId: "issue-101",
    readinessChecks: [
      "Workflow state badge visible",
      "Next-step card rendered",
    ],
    verificationNotes: [
      "Lint and build stay green after badge extraction",
      "Review comments close without changing page skeleton",
    ],
    risks: ["Naming drift between issue and MR states."],
  },
  {
    id: "mr-89",
    workbenchId: "alpha",
    iid: 89,
    title: "Add milestone workflow summary card",
    state: "draft",
    reviewSummary: "Draft not ready for design review yet.",
    nextActionSummary: "Complete milestone summary implementation",
    linkedIssueId: "issue-102",
    readinessChecks: ["Milestone risk summary present"],
    verificationNotes: [
      "Milestone detail stays readable on 13-inch screens",
    ],
    risks: ["State aggregation may feel too generic."],
  },
  {
    id: "mr-201",
    workbenchId: "beta",
    iid: 201,
    title: "Clarify milestone acceptance copy",
    state: "changes_requested",
    reviewSummary: "Acceptance language is still ambiguous.",
    nextActionSummary: "Tighten copy and checklist wording",
    linkedIssueId: "issue-201",
    readinessChecks: ["Goal framing updated", "Blocked items visible"],
    verificationNotes: [
      "Acceptance copy is still scannable after edits",
    ],
    risks: ["Copy may remain too abstract for long-session use."],
  },
];

export const prototypeMilestones: PrototypeMilestone[] = [
  {
    id: "ms-q3",
    workbenchId: "alpha",
    title: "Beta launch",
    goal: "Ship a workflow-first prototype that can be reviewed end to end.",
    dueDate: "2026-07-25",
    issueIds: ["issue-101", "issue-102"],
    mrIds: ["mr-88", "mr-89"],
    riskSummary: "Issue 12 is still clarifying and can delay MR readiness.",
    nextActionSummary: "Unblock issue state naming and close review comments.",
  },
  {
    id: "ms-lab",
    workbenchId: "beta",
    title: "Workflow language lab",
    goal: "Test a more product-heavy workflow vocabulary before rollout.",
    dueDate: "2026-08-05",
    issueIds: ["issue-201"],
    mrIds: ["mr-201"],
    riskSummary: "Acceptance quality may stall if copy remains broad.",
    nextActionSummary: "Refine acceptance and review language together.",
  },
];

export const prototypeActivity: PrototypeActivityItem[] = [
  {
    id: "act-1",
    workbenchId: "alpha",
    kind: "issue",
    title: "Issue 11 moved to ready_for_execution",
    summary: "The workflow card requirements are now implementation-ready.",
    timestamp: "2026-07-02T07:20:00Z",
  },
  {
    id: "act-2",
    workbenchId: "alpha",
    kind: "mr",
    title: "MR 88 is in review",
    summary: "Review comments now focus on state naming consistency.",
    timestamp: "2026-07-02T07:22:00Z",
  },
  {
    id: "act-3",
    workbenchId: "beta",
    kind: "milestone",
    title: "Milestone language lab risk flagged",
    summary: "Acceptance wording still needs tightening before rollout.",
    timestamp: "2026-07-02T07:24:00Z",
  },
];

export const prototypeUserSoul: PrototypeUserSoul = {
  name: "TQ",
  personality:
    "Execution-first operator who prefers clear standards, direct feedback, and durable workflow control.",
  waysOfWorking: [
    "Turn ambiguity into explicit next steps",
    "Keep quality bars visible before execution starts",
    "Use skills and memory to reduce repeated explanation",
  ],
  defaultGoal:
    "Keep issues and MRs continuously moving without losing acceptance or test intent.",
};

export const prototypeMemoryScopes: PrototypeMemoryScope[] = [
  {
    scope: "system",
    summary: "Global workflow rules and shared standards are healthy.",
    status: "healthy",
  },
  {
    scope: "workbench",
    summary: "Current workbench memory tracks blockers, role goals, and active skill emphasis.",
    status: "healthy",
  },
  {
    scope: "personal",
    summary: "Personal memory stores your style, preferences, and recurring delivery heuristics.",
    status: "attention",
  },
];
