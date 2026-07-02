export const en = {
  common: {
    locale: {
      label: "Language switcher",
      english: "EN",
      chinese: "中文",
    },
    actions: {
      cancel: "Cancel",
      save: "Save",
      openPrototype: "Open prototype",
      reviewSettings: "Review settings",
      continueToSignIn: "Continue to sign in",
    },
  },
  landing: {
    eyebrow: "Workflow Cockpit",
    title: "We don't manage work. We keep work moving.",
    titlePrimary: "We don't manage work.",
    titleSecondary: "We keep work moving.",
    lead:
      "A loop engineering system for software delivery that keeps issues, MRs, and milestones moving through the next execution step.",
    impact:
      "Lower waiting time. Surface stalled work early. Make readiness explicit.",
    loginEyebrow: "Issueflow Gateway",
    loginTitle: "Controlled orchestration for issue-driven delivery.",
    loginBody:
      "Start with the Rust Gateway, keep OIDC and workflow control server-side, and grow the Agent Workbench from a stable frontend foundation.",
    groups: {
      executionObjects: "Execution objects",
      controlLayers: "Control layers",
    },
    panelsLabel: "Landing panels",
    panels: {
      overview: "Overview",
      product: "Product",
      engineering: "Engineering",
    },
    story: {
      eyebrow: "Inside the system",
      title: "Business, product, and engineering in one surface",
    },
    overview: {
      title: "Execution Bottlenecks We Remove",
      summary:
        "Project work slows down when requirements stay vague, review waits too long, and context lives only in people.",
      requirementClarity: {
        title: "Requirement clarity",
        description:
          "Turn half-ready issues into executable work instead of bouncing questions across the team.",
      },
      reviewFlow: {
        title: "Review flow",
        description:
          "Keep review pressure visible so MRs move instead of aging silently in a queue.",
      },
      contextContinuity: {
        title: "Context continuity",
        description:
          "Persist standards, blockers, and next steps so handoffs do not reset the team every time.",
      },
    },
    diagram: {
      engine: {
        title: "Execution Loop Engine",
        label: "Clarify / Advance / Verify",
        description:
          "The loop engine evaluates state, writes memory, ranks the next action, and decides when to stop or escalate.",
      },
      issue: {
        title: "Issue",
        label: "Clarify",
        description:
          "Issues move from ambiguity to execution readiness with explicit acceptance, blockers, and next actions.",
      },
      mr: {
        title: "MR",
        label: "Review",
        description:
          "Merge requests stay inside an active review loop instead of becoming passive records waiting for someone to notice them.",
      },
      milestone: {
        title: "Milestone",
        label: "Pressure",
        description:
          "Milestones push urgency back into issue and MR state so delivery risk changes the next-best action.",
      },
      role: {
        title: "Role",
        label: "Bias",
        description:
          "One workbench role defines how the loop prioritizes speed, quality, escalation, and planning style.",
      },
      memory: {
        title: "Memory",
        label: "Context",
        description:
          "Workbench memory stores evolving context so standards and blockers do not disappear into chat history or human recall.",
      },
      skill: {
        title: "Skill",
        label: "Overlay",
        description:
          "Skills tune strictness, recommendation ordering, and presentation emphasis without replacing the system skeleton.",
      },
    },
    product: {
      loopEngine: {
        title: "Loop Engine",
        description:
          "Every managed object enters a progression loop with state, verification, and stop conditions.",
        note: "This is an execution system, not a static board.",
      },
      roleWorkbench: {
        title: "Role Workbench",
        description:
          "A workbench binds to one stable behavior model that shapes escalation, review strictness, and pace.",
        note: "Behavior system, not permission system.",
      },
      memoryLayer: {
        title: "Memory Layer",
        description:
          "Issue, MR, and project memory reduce drift and let the system rebuild state over time.",
        note: "Context becomes durable operational data.",
      },
      statePressure: {
        title: "State Pressure",
        description:
          "Milestone decay, MR delay, and issue stagnation feed the next-best-action ranking.",
        note: "State carries pressure, not just labels.",
      },
    },
    engineering: {
      stateLayer: {
        title: "State Layer",
        description:
          "Issue, MR, and milestone objects are modeled as explicit stateful work items.",
        note: "This is the graph the loops operate on.",
      },
      memoryLayer: {
        title: "Memory Layer",
        description:
          "Structured and temporal memory preserve evolving context for people and agents.",
        note: "Memory is scoped, rebuildable, and part of the product.",
      },
      agentLayer: {
        title: "Agent Layer",
        description:
          "Role-driven agents evaluate state, propose the next step, and decide when to escalate.",
        note: "Agent behavior is bounded by policy and stop rules.",
      },
      pressureLogic: {
        title: "Pressure Logic",
        description:
          "Cross-object pressure turns stalled work into ranked execution guidance for the workbench.",
        note: "This is why the system is not Jira plus AI coding.",
      },
    },
  },
  shell: {
    subtitle: "Agent Workbench",
    workbench: "Workbench",
    role: "Role",
    settings: "Settings",
    renameWorkbench: "Rename workbench",
    workbenchName: "Workbench name",
    userProfile: "Profile",
    navigation: {
      overview: "Overview",
      issues: "Issues",
      mrs: "MRs",
      milestones: "Milestones",
      pendingActions: "Pending Actions",
      releases: "Releases",
    },
  },
  workbenchSearch: {
    addTitle: "Add workbench",
    nameTitle: "Name workbench",
    searchPlaceholder: "Search GitLab projects...",
    noProjects: "No projects found",
    workbenchName: "Workbench name",
    repositoryLabel: "Repository",
    create: "Create",
  },
  issueState: {
    title: "Work Item State",
    evaluate: "Evaluate State",
    current: "Current",
    next: "Next",
    missingContext: "Missing context",
    roleNotes: "Role Notes",
    product: "Product",
    engineering: "Engineering",
    delivery: "Delivery",
    empty: "No shared issue state yet. Run an evaluation to capture it.",
    pendingSummary: "State transition pending confirmation.",
    noSummary: "No summary provided.",
  },
  prototype: {
    overview: {
      eyebrow: "Overview",
      title: "Execution cockpit",
      description:
        "Focus the workbench on explicit next actions, visible blockers, and skill-driven emphasis for long-running delivery.",
      currentWorkbench: "Current workbench",
      issueWorkflow: "Issue workflow",
      mrWorkflow: "MR workflow",
      recentActivity: "Recent activity",
    },
    issues: {
      eyebrow: "Issues",
      title: "Drive issue quality into execution",
      nextAction: "Next action",
      acceptanceCriteria: "Acceptance criteria",
      verificationPlan: "Verification plan",
      risks: "Risks",
    },
    mrs: {
      eyebrow: "Merge Requests",
      title: "Keep review and merge readiness visible",
      reviewSummary: "Review summary",
      readinessChecks: "Readiness checks",
      verificationNotes: "Verification notes",
      risks: "Risks",
    },
    milestones: {
      eyebrow: "Milestones",
      title: "Aggregate workflow pressure before it becomes delay",
      milestoneLabel: "Milestone",
      issueWorkflow: "Issue workflow",
      mrWorkflow: "MR workflow",
      riskSummary: "Risk summary",
      nextAction: "Next action",
    },
    settings: {
      eyebrow: "User Settings",
      title: "Shape the operator, not the page skeleton",
      description:
        "Tune user soul, workbench role, memory controls, and skill versions without moving workflow navigation out of the workbench.",
      soulTitle: "User soul",
      soulPersonalityPlaceholder: "How this user works",
      soulWaysPlaceholder: "One operating principle per line",
      soulGoalPlaceholder: "Default goal",
      saveSoul: "Save user soul",
      currentWorkbench: "Current Workbench",
      roleName: "Role name",
      roleSummary: "Role summary",
      roleWays: "One way of working per line",
      roleGoals: "One goal per line",
      saveRole: "Save role",
      skills: "Skills",
      uploadVersion: "Upload version",
      active: "Active",
      makeActive: "Make active",
      memoryControls: "Memory controls",
      clearMemory: "Clear memory",
      rebuildMemory: "Rebuild memory",
      lastAction: "Last action",
      lastActionIdle: "idle",
      lastActionCleared: "cleared",
      lastActionRebuilt: "rebuilt",
    },
    recommendedActions: {
      title: "Recommended next actions",
    },
  },
} as const;
