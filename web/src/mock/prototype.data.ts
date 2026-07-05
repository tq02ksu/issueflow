import type {
  PrototypeActivityItem,
  PrototypeApproval,
  PrototypeIssue,
  PrototypeLoop,
  PrototypeMemoryItem,
  PrototypeMemoryScope,
  PrototypeMilestone,
  PrototypeMr,
  PrototypeTurn,
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

export const prototypeLoops: PrototypeLoop[] = [
  {
    id: "loop-1",
    workbenchId: "alpha",
    name: "Issue workflow scan",
    type: "issue",
    enabled: true,
    status: "healthy",
    boundObject: "Issue #11 — Define workflow state cards",
    boundObjectId: "issue-101",
    goal: "Keep issue state progression visible and enforce acceptance quality.",
    schedulePolicy: "Every 4 hours",
    stateMachinePolicy:
      "Issue lifecycle: new → clarifying → planned → ready_for_execution → in_execution → done",
    skillRefs: ["delivery-skill"],
    verificationPolicy:
      "Check acceptance criteria completeness before state transition.",
    budgetPolicy: "Max 5000 tokens per run",
    notificationPolicy: "Notify on blocked → waiting_approval transitions",
    nextRunAt: "2026-07-05T14:00:00Z",
    lastRunAt: "2026-07-04T10:14:30Z",
  },
  {
    id: "loop-2",
    workbenchId: "alpha",
    name: "MR review scan",
    type: "mr",
    enabled: true,
    status: "waiting_approval",
    boundObject: "MR !88 — Render issue workflow badges",
    boundObjectId: "mr-88",
    goal: "Keep review flow moving and detect stale MRs.",
    schedulePolicy: "On review update event",
    stateMachinePolicy:
      "MR lifecycle: draft → in_review → changes_requested → ready_to_merge → merged",
    skillRefs: ["delivery-skill"],
    verificationPolicy:
      "Confirm review comments are resolved before merge recommendation.",
    budgetPolicy: "Max 3000 tokens per run",
    notificationPolicy:
      "Notify on changes_requested → ready_to_merge transitions",
    nextRunAt: null,
    lastRunAt: "2026-07-04T11:29:00Z",
  },
  {
    id: "loop-3",
    workbenchId: "alpha",
    name: "Milestone health scan",
    type: "milestone",
    enabled: true,
    status: "healthy",
    boundObject: "Beta launch milestone",
    boundObjectId: "ms-q3",
    goal: "Aggregate issue and MR states to surface delivery risk.",
    schedulePolicy: "Daily at 08:00 UTC",
    stateMachinePolicy:
      "Pressure-based: flag risk when >30% issues still in clarifying phase",
    skillRefs: ["delivery-skill"],
    verificationPolicy:
      "Cross-check milestone scope completeness before risk flagging.",
    budgetPolicy: "Max 2000 tokens per run",
    notificationPolicy: "Notify on high-risk or critical flags",
    nextRunAt: "2026-07-06T08:00:00Z",
    lastRunAt: "2026-07-03T08:58:30Z",
  },
  {
    id: "loop-4",
    workbenchId: "beta",
    name: "Acceptance quality scan",
    type: "milestone",
    enabled: true,
    status: "blocked",
    boundObject: "Workflow language lab",
    boundObjectId: "ms-lab",
    goal: "Tighten acceptance language precision before rollout.",
    schedulePolicy: "Weekly on Monday",
    stateMachinePolicy:
      "Quality gate: flag if acceptance < 80% precision score",
    skillRefs: ["delivery-skill"],
    verificationPolicy: "Precision scoring via governance engine.",
    budgetPolicy: "Max 8000 tokens per run",
    notificationPolicy: "Notify on precision score drop",
    nextRunAt: "2026-07-07T08:00:00Z",
    lastRunAt: "2026-07-02T08:25:00Z",
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
    verificationNotes: ["Milestone detail stays readable on 13-inch screens"],
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
    verificationNotes: ["Acceptance copy is still scannable after edits"],
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

export const prototypeApprovals: PrototypeApproval[] = [
  {
    id: "approval-1",
    workbenchId: "alpha",
    actionType: "issue_comment",
    sourceLoop: "Issue workflow progression",
    sourceTurnId: "turn-101",
    riskLevel: "low",
    targetObject: "issue-101",
    targetUrl: "/workbench/issues",
    draftContent:
      "This issue is now ready_for_execution. All acceptance criteria are clear and the verification plan is defined. Proceeding to development handoff.",
    generationBasis: "Evaluator confirmed acceptance completeness.",
    memoryRelation:
      "Issue state transition from clarifying to ready_for_execution.",
    status: "pending",
    createdAt: "2026-07-04T10:15:00Z",
  },
  {
    id: "approval-2",
    workbenchId: "alpha",
    actionType: "mr_comment",
    sourceLoop: "MR review progression",
    sourceTurnId: "turn-102",
    riskLevel: "medium",
    targetObject: "mr-88",
    targetUrl: "/workbench/mrs",
    draftContent:
      "Review is complete with two minor naming comments. Merging is safe after naming alignment.",
    generationBasis: "Reviewer feedback summary from latest run.",
    memoryRelation: "MR blockages from issue state naming drift.",
    status: "pending",
    createdAt: "2026-07-04T11:30:00Z",
  },
  {
    id: "approval-3",
    workbenchId: "alpha",
    actionType: "state_transition",
    sourceLoop: "Milestone pressure check",
    sourceTurnId: "turn-103",
    riskLevel: "high",
    targetObject: "ms-q3",
    targetUrl: "/workbench/milestones",
    draftContent:
      "Beta launch milestone has one issue still in clarifying state. Recommend flagging as at-risk before proceeding.",
    generationBasis: "Milestone health evaluation from latest run.",
    memoryRelation: "Known risk: clarifying churn may delay milestone scope.",
    status: "approved",
    createdAt: "2026-07-03T09:00:00Z",
  },
  {
    id: "approval-4",
    workbenchId: "alpha",
    actionType: "issue_comment",
    sourceLoop: "Issue state evaluation",
    sourceTurnId: "turn-104",
    riskLevel: "low",
    targetObject: "issue-102",
    targetUrl: "/workbench/issues",
    draftContent: "Review-state vocabulary agreed. Moving issue to planned.",
    generationBasis: "Evaluator confirmed vocabulary consensus.",
    memoryRelation: "Previous run flagged vocabulary drift.",
    status: "approved",
    createdAt: "2026-07-02T14:00:00Z",
  },
  {
    id: "approval-5",
    workbenchId: "beta",
    actionType: "milestone_update",
    sourceLoop: "Acceptance quality check",
    sourceTurnId: "turn-201",
    riskLevel: "critical",
    targetObject: "ms-lab",
    targetUrl: "/workbench/milestones",
    draftContent:
      "Acceptance language remains too broad. Recommend rejecting until acceptance checklist is tightened.",
    generationBasis: "Governance engine flagged comprehension risk.",
    memoryRelation:
      "Recurring pattern: broad acceptance language leads to rework.",
    status: "rejected",
    createdAt: "2026-07-02T08:30:00Z",
  },
  {
    id: "approval-6",
    workbenchId: "beta",
    actionType: "mr_comment",
    sourceLoop: "MR quality gate",
    sourceTurnId: "turn-202",
    riskLevel: "medium",
    targetObject: "mr-201",
    targetUrl: "/workbench/mrs",
    draftContent: "Copy tightened per reviewer feedback. Ready for re-review.",
    generationBasis: "Previous rejection feedback incorporated.",
    memoryRelation: "Acceptance copy evolution tracked across two runs.",
    status: "execution_failed",
    createdAt: "2026-07-01T16:00:00Z",
  },
];

export const prototypeTurns: PrototypeTurn[] = [
  {
    id: "turn-101",
    workbenchId: "alpha",
    loopName: "Alpha Delivery",
    targets: [
      {
        objectType: "issue",
        objectId: "issue-101",
        actions: ["Evaluate readiness", "Generate next-step draft"],
        result: "ready_for_execution — acceptance criteria complete",
      },
      {
        objectType: "mr",
        objectId: "mr-88",
        actions: ["Scan review comments", "Evaluate merge readiness"],
        result: "in_review — 2 naming comments remain",
      },
    ],
    status: "waiting_approval",
    triggerSource: "schedule",
    startTime: "2026-07-04T10:10:00Z",
    endTime: "2026-07-04T10:14:30Z",
    durationSecs: 270,
    summary:
      "Scanned issue-101 and mr-88. Issue ready for execution, MR needs naming alignment.",
    conclusion:
      "Issue-101 can advance to ready_for_execution. MR-88 requires review comment resolution before merge.",
    events: [
      {
        timestamp: "2026-07-04T10:10:00Z",
        kind: "created",
        message: "Turn triggered by schedule.",
      },
      {
        timestamp: "2026-07-04T10:10:05Z",
        kind: "fetching_objects",
        message: "Fetched 2 objects (1 issue, 1 MR).",
      },
      {
        timestamp: "2026-07-04T10:12:00Z",
        kind: "memory_loaded",
        message: "Loop memory loaded: previous state context restored.",
      },
      {
        timestamp: "2026-07-04T10:13:00Z",
        kind: "executor_invoked",
        message: "Executor evaluated issue-101 acceptance criteria.",
        targetId: "issue-101",
        agentId: "loop-executor-v2",
      },
      {
        timestamp: "2026-07-04T10:13:30Z",
        kind: "executor_invoked",
        message: "Executor scanned MR-88 review comments.",
        targetId: "mr-88",
        agentId: "mr-progression-core",
      },
      {
        timestamp: "2026-07-04T10:14:00Z",
        kind: "evaluator_confirmed",
        message:
          "Evaluator confirmed issue-101 readiness and MR-88 naming concern.",
      },
      {
        timestamp: "2026-07-04T10:14:20Z",
        kind: "conclusion_generated",
        message: "Conclusion drafted for both objects.",
      },
      {
        timestamp: "2026-07-04T10:14:30Z",
        kind: "approval_requested",
        message: "Approval requested for 2 draft actions.",
      },
    ],
    agents: [
      {
        agentId: "loop-executor-v2",
        agentName: "Loop Executor v2",
        role: "executor",
        model: "balanced",
        status: "done",
        tokensUsed: 2100,
        cost: 0.021,
        retries: 0,
        responsibleFor: ["issue-101"],
      },
      {
        agentId: "mr-progression-core",
        agentName: "MR Progression Core",
        role: "executor",
        model: "balanced",
        status: "done",
        tokensUsed: 1320,
        cost: 0.013,
        retries: 0,
        responsibleFor: ["mr-88"],
      },
      {
        agentId: "evaluator-core",
        agentName: "Evaluator Core",
        role: "evaluator",
        model: "cheap-fast",
        status: "done",
        tokensUsed: 450,
        cost: 0.0002,
        retries: 0,
        responsibleFor: ["issue-101", "mr-88"],
      },
    ],
    draftActions: [
      {
        targetId: "issue-101",
        actionType: "issue_comment",
        draftContent:
          "Issue is ready_for_execution. All acceptance criteria are clear.",
        riskLevel: "low",
      },
      {
        targetId: "mr-88",
        actionType: "mr_comment",
        draftContent: "Two naming comments remain on workflow badge rendering.",
        riskLevel: "medium",
      },
    ],
    recommendations: [
      "Start dev handoff for issue-101",
      "Resolve naming comments on MR-88",
    ],
    memoryRead: ["loop:issue-101", "loop:mr-88"],
    memoryWritten: "issue-101 readiness confirmed; mr-88 naming drift flagged",
    totalTokens: 3870,
    totalCost: 0.0342,
  },
  {
    id: "turn-102",
    workbenchId: "alpha",
    loopName: "Alpha Delivery",
    targets: [
      {
        objectType: "milestone",
        objectId: "ms-q3",
        actions: ["Aggregate issue/MR states", "Evaluate delivery risk"],
        result: "at-risk — issue-102 still in clarifying phase",
      },
    ],
    status: "completed",
    triggerSource: "schedule",
    startTime: "2026-07-03T08:55:00Z",
    endTime: "2026-07-03T08:58:30Z",
    durationSecs: 210,
    summary:
      "Milestone Beta launch health check. One issue still clarifying, flagged as at-risk.",
    conclusion:
      "Beta launch milestone has delivery risk. Issue-102 in clarifying state delays downstream MRs.",
    events: [
      {
        timestamp: "2026-07-03T08:55:00Z",
        kind: "created",
        message: "Turn triggered by daily schedule.",
      },
      {
        timestamp: "2026-07-03T08:55:02Z",
        kind: "fetching_objects",
        message: "Fetched milestone and linked items.",
      },
      {
        timestamp: "2026-07-03T08:56:00Z",
        kind: "memory_loaded",
        message: "Loop memory loaded: issue-102 clarifying state persisted.",
      },
      {
        timestamp: "2026-07-03T08:57:00Z",
        kind: "executor_invoked",
        message: "Executor aggregated workflow states.",
        targetId: "ms-q3",
        agentId: "milestone-health-core",
      },
      {
        timestamp: "2026-07-03T08:57:30Z",
        kind: "evaluator_confirmed",
        message: "Evaluator confirmed at-risk flag.",
      },
      {
        timestamp: "2026-07-03T08:58:00Z",
        kind: "conclusion_generated",
        message: "Conclusion: flag as at-risk, recommend pushing issue-102.",
      },
      {
        timestamp: "2026-07-03T08:58:30Z",
        kind: "completed",
        message: "Turn completed. Milestone flagged with risk.",
      },
    ],
    agents: [
      {
        agentId: "milestone-health-core",
        agentName: "Milestone Health Core",
        role: "executor",
        model: "cheap-fast",
        status: "done",
        tokensUsed: 580,
        cost: 0.0003,
        retries: 0,
        responsibleFor: ["ms-q3"],
      },
      {
        agentId: "evaluator-core",
        agentName: "Evaluator Core",
        role: "evaluator",
        model: "cheap-fast",
        status: "done",
        tokensUsed: 400,
        cost: 0.0002,
        retries: 0,
        responsibleFor: ["ms-q3"],
      },
    ],
    draftActions: [],
    recommendations: [
      "Push issue-102 to planned",
      "Re-evaluate milestone in 2 days",
    ],
    memoryRead: ["loop:issue-101", "loop:issue-102", "engineering:alpha"],
    memoryWritten: "ms-q3 flagged at-risk due to issue-102 clarifying state",
    totalTokens: 980,
    totalCost: 0.0005,
  },
  {
    id: "turn-103",
    workbenchId: "alpha",
    loopName: "Alpha Delivery",
    targets: [
      {
        objectType: "issue",
        objectId: "issue-102",
        actions: ["Clarify review-state vocabulary"],
        result: "planned — vocabulary consensus reached",
      },
    ],
    status: "completed",
    triggerSource: "manual",
    startTime: "2026-07-02T13:50:00Z",
    endTime: "2026-07-02T13:53:00Z",
    durationSecs: 180,
    summary:
      "Manual evaluation triggered for issue-102. Vocabulary consensus confirmed.",
    conclusion:
      "Review-state vocabulary now agreed. Issue-102 can move to planned.",
    events: [
      {
        timestamp: "2026-07-02T13:50:00Z",
        kind: "created",
        message: "Turn triggered manually.",
      },
      {
        timestamp: "2026-07-02T13:50:02Z",
        kind: "fetching_objects",
        message: "Issue data and discussion fetched.",
      },
      {
        timestamp: "2026-07-02T13:52:00Z",
        kind: "executor_invoked",
        message: "Executor evaluated vocabulary alignment.",
        targetId: "issue-102",
        agentId: "loop-executor-v2",
      },
      {
        timestamp: "2026-07-02T13:52:30Z",
        kind: "evaluator_confirmed",
        message: "Evaluator confirmed vocabulary consensus.",
      },
      {
        timestamp: "2026-07-02T13:53:00Z",
        kind: "completed",
        message: "Turn completed. State advanced to planned.",
      },
    ],
    agents: [
      {
        agentId: "loop-executor-v2",
        agentName: "Loop Executor v2",
        role: "executor",
        model: "high-reasoning",
        status: "done",
        tokensUsed: 4800,
        cost: 0.072,
        retries: 0,
        responsibleFor: ["issue-102"],
      },
      {
        agentId: "evaluator-core",
        agentName: "Evaluator Core",
        role: "evaluator",
        model: "balanced",
        status: "done",
        tokensUsed: 880,
        cost: 0.0088,
        retries: 0,
        responsibleFor: ["issue-102"],
      },
    ],
    draftActions: [
      {
        targetId: "issue-102",
        actionType: "issue_comment",
        draftContent: "Review-state vocabulary agreed. Moving to planned.",
        riskLevel: "low",
      },
    ],
    recommendations: ["Proceed to planning for issue-102"],
    memoryRead: ["loop:issue-102"],
    memoryWritten:
      "issue-102 vocabulary consensus reached; state transitioned to planned",
    totalTokens: 5680,
    totalCost: 0.0808,
  },
  {
    id: "turn-104",
    workbenchId: "alpha",
    loopName: "Alpha Delivery",
    targets: [
      {
        objectType: "issue",
        objectId: "issue-101",
        actions: ["Re-evaluate after failed turn-105"],
        result: "failed — memory desync during re-evaluation",
      },
    ],
    status: "failed",
    triggerSource: "schedule",
    startTime: "2026-07-02T10:10:00Z",
    endTime: "2026-07-02T10:13:00Z",
    durationSecs: 180,
    summary:
      "Scheduled turn failed. Memory layer returned stale data after retry exhaustion.",
    conclusion:
      "Turn failed due to memory desync. Requires manual intervention or memory rebuild.",
    events: [
      {
        timestamp: "2026-07-02T10:10:00Z",
        kind: "created",
        message: "Turn triggered by schedule.",
      },
      {
        timestamp: "2026-07-02T10:10:03Z",
        kind: "fetching_objects",
        message: "Issue data fetched.",
      },
      {
        timestamp: "2026-07-02T10:12:00Z",
        kind: "executor_invoked",
        message: "Executor attempted evaluation with stale memory.",
        targetId: "issue-101",
        agentId: "loop-executor-v2",
      },
      {
        timestamp: "2026-07-02T10:13:00Z",
        kind: "failed",
        message: "Turn failed: memory desync detected after 2 retries.",
      },
    ],
    agents: [
      {
        agentId: "loop-executor-v2",
        agentName: "Loop Executor v2",
        role: "executor",
        model: "balanced",
        status: "failed",
        tokensUsed: 1200,
        cost: 0.012,
        retries: 2,
        responsibleFor: ["issue-101"],
      },
    ],
    draftActions: [],
    recommendations: ["Rebuild workbench memory", "Retry turn manually"],
    memoryRead: ["loop:issue-101"],
    memoryWritten: "turn-104 failed: memory desync on issue-101",
    totalTokens: 1200,
    totalCost: 0.012,
  },
  {
    id: "turn-201",
    workbenchId: "beta",
    loopName: "AA Workflow Lab",
    targets: [
      {
        objectType: "milestone",
        objectId: "ms-lab",
        actions: [
          "Evaluate acceptance language precision",
          "Cross-check asset quality",
        ],
        result: "blocked — acceptance language too broad",
      },
      {
        objectType: "mr",
        objectId: "mr-201",
        actions: ["Scan review state", "Evaluate copy quality"],
        result: "changes_requested — copy still ambiguous",
      },
    ],
    status: "completed",
    triggerSource: "schedule",
    startTime: "2026-07-02T08:20:00Z",
    endTime: "2026-07-02T08:25:00Z",
    durationSecs: 300,
    summary:
      "Acceptance quality evaluation. Both milestone and MR flagged for language precision issues.",
    conclusion:
      "Acceptance language remains too broad across both objects. Governance risk flagged.",
    events: [
      {
        timestamp: "2026-07-02T08:20:00Z",
        kind: "created",
        message: "Turn triggered by schedule.",
      },
      {
        timestamp: "2026-07-02T08:20:02Z",
        kind: "fetching_objects",
        message: "Fetched milestone, MR, and linked items.",
      },
      {
        timestamp: "2026-07-02T08:23:00Z",
        kind: "memory_loaded",
        message: "Loop memory loaded: recurring acceptance quality concerns.",
      },
      {
        timestamp: "2026-07-02T08:24:00Z",
        kind: "executor_invoked",
        message: "Executor evaluated acceptance language precision.",
        targetId: "ms-lab",
        agentId: "quality-gate-core",
      },
      {
        timestamp: "2026-07-02T08:24:20Z",
        kind: "executor_invoked",
        message: "Executor scanned MR copy quality.",
        targetId: "mr-201",
        agentId: "mr-progression-core",
      },
      {
        timestamp: "2026-07-02T08:24:40Z",
        kind: "evaluator_confirmed",
        message:
          "Evaluator: acceptance language remains too broad across both objects.",
      },
      {
        timestamp: "2026-07-02T08:25:00Z",
        kind: "completed",
        message: "Turn completed. Objects flagged for governance review.",
      },
    ],
    agents: [
      {
        agentId: "quality-gate-core",
        agentName: "Quality Gate Core",
        role: "executor",
        model: "high-reasoning",
        status: "done",
        tokensUsed: 5200,
        cost: 0.078,
        retries: 0,
        responsibleFor: ["ms-lab"],
      },
      {
        agentId: "mr-progression-core",
        agentName: "MR Progression Core",
        role: "executor",
        model: "balanced",
        status: "done",
        tokensUsed: 1600,
        cost: 0.016,
        retries: 0,
        responsibleFor: ["mr-201"],
      },
      {
        agentId: "evaluator-core",
        agentName: "Evaluator Core",
        role: "evaluator",
        model: "high-reasoning",
        status: "done",
        tokensUsed: 1430,
        cost: 0.021,
        retries: 0,
        responsibleFor: ["ms-lab", "mr-201"],
      },
    ],
    draftActions: [
      {
        targetId: "ms-lab",
        actionType: "milestone_update",
        draftContent:
          "Acceptance language too broad. Recommend reject until tightened.",
        riskLevel: "critical",
      },
      {
        targetId: "mr-201",
        actionType: "mr_comment",
        draftContent:
          "Copy still ambiguous per reviewer feedback. Tighten before re-review.",
        riskLevel: "medium",
      },
    ],
    recommendations: [
      "Tighten acceptance criteria templates",
      "Review all object copy for precision",
    ],
    memoryRead: ["loop:ms-lab", "loop:mr-201", "governance:governance"],
    memoryWritten:
      "acceptance quality concern detected across ms-lab and mr-201",
    totalTokens: 8230,
    totalCost: 0.115,
  },
  {
    id: "turn-202",
    workbenchId: "beta",
    loopName: "AA Workflow Lab",
    targets: [
      {
        objectType: "mr",
        objectId: "mr-201",
        actions: ["Generate and submit review comment"],
        result: "failed — comment submission returned 403",
      },
    ],
    status: "failed",
    triggerSource: "event",
    startTime: "2026-07-01T15:55:00Z",
    endTime: "2026-07-01T15:58:00Z",
    durationSecs: 180,
    summary:
      "Quality gate turn for MR comment posting. Execution failed during comment submission.",
    conclusion:
      "Comment submission failed with 403. Likely permission issue — requires manual investigation.",
    events: [
      {
        timestamp: "2026-07-01T15:55:00Z",
        kind: "created",
        message: "Turn triggered by MR update event.",
      },
      {
        timestamp: "2026-07-01T15:55:02Z",
        kind: "fetching_objects",
        message: "MR data and review state fetched.",
      },
      {
        timestamp: "2026-07-01T15:57:00Z",
        kind: "executor_invoked",
        message: "Executor generated comment draft.",
        targetId: "mr-201",
        agentId: "mr-progression-core",
      },
      {
        timestamp: "2026-07-01T15:57:40Z",
        kind: "evaluator_confirmed",
        message: "Evaluator approved draft content.",
      },
      {
        timestamp: "2026-07-01T15:58:00Z",
        kind: "failed",
        message: "Turn failed: comment submission returned 403.",
      },
    ],
    agents: [
      {
        agentId: "mr-progression-core",
        agentName: "MR Progression Core",
        role: "executor",
        model: "balanced",
        status: "failed",
        tokensUsed: 1600,
        cost: 0.016,
        retries: 1,
        responsibleFor: ["mr-201"],
      },
    ],
    draftActions: [
      {
        targetId: "mr-201",
        actionType: "mr_comment",
        draftContent:
          "Copy tightened per reviewer feedback. Ready for re-review.",
        riskLevel: "medium",
      },
    ],
    recommendations: ["Check GitLab PAT permissions", "Retry after auth fix"],
    memoryRead: ["loop:mr-201"],
    memoryWritten: "turn-202 failed: 403 on mr-201 comment submission",
    totalTokens: 1600,
    totalCost: 0.016,
  },
];

export const prototypeMemoryItems: PrototypeMemoryItem[] = [
  {
    id: "mem-loop-101",
    scope: "loop",
    objectType: "issue",
    objectId: "issue-101",
    summary:
      "Issue is ready for execution. Acceptance criteria and verification plan are defined. UI profile ordering dependency still unresolved.",
    knownRisks: ["Skill emphasis may hide blockers if not constrained."],
    knownBlockers: ["UI profile ordering missing from implementation."],
    suggestedNextSteps: [
      "Start dev handoff",
      "Resolve UI profile ordering first",
    ],
    lastUpdatedAt: "2026-07-04T10:14:30Z",
    sourceTurnIds: ["turn-101", "turn-105"],
  },
  {
    id: "mem-loop-102",
    scope: "loop",
    objectType: "issue",
    objectId: "issue-102",
    summary:
      "Issue vocabulary is still in clarifying phase. Review-state naming not yet agreed across team.",
    knownRisks: ["State model may drift from backend terminology."],
    knownBlockers: ["Review-state vocabulary not agreed."],
    suggestedNextSteps: [
      "Clarify review state model",
      "Align with backend terminology",
    ],
    lastUpdatedAt: "2026-07-02T13:53:00Z",
    sourceTurnIds: ["turn-104"],
  },
  {
    id: "mem-eng-alpha",
    scope: "engineering",
    objectType: "project",
    objectId: "alpha",
    summary:
      "Alpha Delivery project is stable. Workflow state model is in active refinement. Review-state naming is the current open concern.",
    knownRisks: ["Naming drift between issue and MR states."],
    knownBlockers: [],
    suggestedNextSteps: ["Align review-state vocabulary across all artifacts"],
    lastUpdatedAt: "2026-07-04T10:14:30Z",
    sourceTurnIds: ["turn-101", "turn-102", "turn-103", "turn-104"],
  },
  {
    id: "mem-eng-beta",
    scope: "engineering",
    objectType: "project",
    objectId: "beta",
    summary:
      "AA Workflow Lab project has acceptance quality concerns. Language precision needs improvement before rollout.",
    knownRisks: ["Acceptance language too abstract for long-session use."],
    knownBlockers: ["Acceptance checklist not finalized."],
    suggestedNextSteps: [
      "Tighten acceptance criteria",
      "Test copy readability on smaller screens",
    ],
    lastUpdatedAt: "2026-07-02T08:25:00Z",
    sourceTurnIds: ["turn-201"],
  },
  {
    id: "mem-gov-1",
    scope: "governance",
    objectType: "system",
    objectId: "governance",
    summary:
      "Governance engine detected one high-risk recurring pattern: broad acceptance language leading to rework across workbenches.",
    knownRisks: [
      "Recurring acceptance quality issues",
      "One execution failure in past week",
    ],
    knownBlockers: [],
    suggestedNextSteps: [
      "Review acceptance templates",
      "Consider tightening auto-evaluation thresholds",
    ],
    lastUpdatedAt: "2026-07-04T10:14:30Z",
    sourceTurnIds: ["turn-201", "turn-202"],
  },
];

export const prototypeMemoryScopes: PrototypeMemoryScope[] = [
  {
    scope: "system",
    summary: "Global workflow rules and shared standards are healthy.",
    status: "healthy",
  },
  {
    scope: "workbench",
    summary:
      "Current workbench memory tracks blockers, role goals, and active skill emphasis.",
    status: "healthy",
  },
  {
    scope: "personal",
    summary:
      "Personal memory stores your style, preferences, and recurring delivery heuristics.",
    status: "attention",
  },
];
