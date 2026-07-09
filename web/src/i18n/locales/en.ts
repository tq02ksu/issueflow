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
    eyebrow: "Loop Control Plane",
    title: "Loop engineering.",
    titlePrimary: "Loop engineering.",
    titleSecondary: "for software delivery.",
    lead: "A turn-based orchestration system that treats issues, MRs, and milestones as pluggable fact modules — managed by controlled state machines, executed by multi-agent runtimes, gated by explicit human approvals.",
    impact:
      "No silent writes. Every state transition goes through turn → evaluate → approve.",
    tagline:
      "Design the loop, run the turns — the system tightens itself over time.",
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
      title: "How Issueflow Works",
      summary:
        "Fact modules (issues, MRs, milestones) are managed by controlled state machines. Each turn dispatches agents to evaluate, plan, and draft actions. Nothing writes without approval.",
      requirementClarity: {
        title: "State-driven facts",
        description:
          "Every issue and MR is a pluggable fact module with explicit workflow states managed by a state machine — not free-form chat.",
      },
      reviewFlow: {
        title: "Turn-based orchestration",
        description:
          "Each loop turn fetches objects, dispatches agents, evaluates results, and gates writes through the approval layer.",
      },
      contextContinuity: {
        title: "Evolving memory",
        description:
          "Loop memory persists understanding across turns. Engineering memory tracks project patterns. Governance memory flags systemic risks.",
      },
    },
    diagram: {
      engine: {
        title: "Loop Orchestrator",
        label: "Turn / Evaluate / Approve",
        description:
          "The Loop Agent is an orchestrator — it plans turns, dispatches agents, monitors execution, evaluates results, and gates writes through approvals.",
      },
      soul: "Mission, long-term goals, value priorities",
      ruleNoFabricate: "No fabrication",
      ruleApproveWrites: "Approve writes",
      loopHub: "LOOP\nAgent",
      discover: "Discover",
      handoff: "Handoff",
      verify: "Verify",
      persist: "Persist",
      schedule: "Schedule",
      mem0: "mem0",
      otel: "OTel",
      gateway: "Gateway",
      decisionLoop: "Decision Loop",
      evaluator: "Evaluator",
      humanApproval: "Human Approval",
      steerEvolve: "Steer & Evolve",
      issue: {
        title: "Issue",
        label: "Pluggable Fact",
        description:
          "Issues are pluggable fact modules with controlled state machines.",
      },
      mr: {
        title: "MR",
        label: "Pluggable Fact",
        description:
          "MRs are managed as fact modules with state machine progression.",
      },
      milestone: {
        title: "Milestone",
        label: "Aggregate View",
        description:
          "Milestones aggregate linked fact states to surface delivery pressure.",
      },
      role: {
        title: "SOUL + PRINCIPLE",
        label: "Loop Constitution",
        description:
          "Immutable soul and principle shape planning and decisions.",
      },
      memory: {
        title: "Memory",
        label: "Evolving Context",
        description:
          "Persisted understanding across turns — not chat transcript.",
      },
      skill: {
        title: "Skill",
        label: "Capability Overlay",
        description: "Versioned capability objects. Upgrades require approval.",
      },
    },
    product: {
      loopEngine: {
        title: "Loop Orchestrator",
        description:
          "Plans turns, dispatches agent runtimes, monitors execution, and controls the approval gate for every write operation.",
        note: "An orchestrator, not a chat bot.",
      },
      roleWorkbench: {
        title: "State Machine Layer",
        description:
          "Each fact module has a controlled state machine. Transitions require evaluation, and writes require confirmation.",
        note: "State is the product, not a feature.",
      },
      memoryLayer: {
        title: "Multi-layer Memory",
        description:
          "Loop, engineering, and governance memory persist context across turns so understanding evolves rather than resets.",
        note: "Context becomes durable operational data.",
      },
      statePressure: {
        title: "Human Checkpoints",
        description:
          "Draft actions sit in the approval queue until explicitly confirmed. No silent writes — full audit trail.",
        note: "Deliberate execution, not automation blind spots.",
      },
    },
    engineering: {
      stateLayer: {
        title: "Pluggable Facts",
        description:
          "Issues, MRs, and milestones are external fact modules. Their schema is defined, their state machine is controlled.",
        note: "Facts plug in; the loop orchestrates them.",
      },
      memoryLayer: {
        title: "mem0 + otel",
        description:
          "Persistent memory via mem0, structured observability via otel-compatible telemetry. Replaceable without breaking the loop.",
        note: "Memory is an interface, not a vendor lock.",
      },
      agentLayer: {
        title: "Plugin Agent Runtime",
        description:
          "Different tasks use different runtimes (OpenCode for code, Hermes for general tasks). Loop agent orchestrates them all.",
        note: "Runtimes are plugins, not monoliths.",
      },
      pressureLogic: {
        title: "State Machine Governance",
        description:
          "Fact module state transitions are gated by evaluation and approval. Governance monitors drift, debt, and degradation over time.",
        note: "State machines enforce discipline at system scale.",
      },
    },
    concepts: {
      stateMachine: {
        label: "State Machines",
        desc: "Issues, MRs, and milestones are managed as fact modules with controlled state machines.",
      },
      turn: {
        label: "Turn-based",
        desc: "Each loop turn orchestrates agents, evaluates results, and gates writes through approvals.",
      },
      approval: {
        label: "Approval Gates",
        desc: "No silent writes — every state transition waits for explicit human confirmation.",
      },
      memory: {
        label: "Evolving Memory",
        desc: "Loop, engineering, and governance memory persist understanding across turns.",
      },
    },
    flow: {
      define: {
        title: "Define the loop",
        desc: "Set SOUL (mission) and PRINCIPLE (behavior rules). Bind fact modules to the loop's state machine.",
      },
      execute: {
        title: "Turn executes",
        desc: "The orchestrator plans a turn, dispatches agents, evaluates results, and drafts actions.",
      },
      confirm: {
        title: "Confirm and advance",
        desc: "Review draft actions. Approve, reject, or steer before the next turn begins.",
      },
    },
    storyCards: {
      stateMachine: {
        title: "Controlled state machines",
        desc: "Every fact module has a state machine. Transitions are gated — no state change happens without evaluation and approval.",
      },
      orchestrator: {
        title: "Orchestrator, not chatbot",
        desc: "The loop agent discovers, hands off, verifies, persists, and schedules — an execution engine with explicit decision loops.",
      },
      governance: {
        title: "Evolving governance",
        desc: "Evaluator feedback, human approvals, and self-evolution converge into a runtime governance loop that tightens over turns.",
      },
    },
  },
  shell: {
    subtitle: "Agent Workbench",
    workbench: "Workbench",
    role: "Role",
    roleSwitch: "Switch role",
    settings: "Settings",
    renameWorkbench: "Rename workbench",
    workbenchName: "Workbench name",
    userProfile: "Profile",
    preferences: "Preferences",
    signOut: "Sign out",
    navigation: {
      dashboard: "Dashboard",
      turns: "Turns",
      agents: "Agents",
      approvals: "Approvals",
      memory: "Memory",
      skills: "Skills",
      factModules: "Fact Modules",
      settings: "Settings",
      settingsLoop: "Loop Configuration",
      settingsIntegrations: "Integrations",
      settingsAccess: "Access",
      system: "System",
      gateway: "AI Gateway",
      governance: "Governance",
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
    roles: {
      developer: {
        name: "Developer",
        tagline: "Drive assigned work to done",
        mission:
          "Focus on in-execution issues, MRs, your pending actions, and the review queue.",
      },
      manager: {
        name: "R&D Manager & Architect",
        tagline: "Delivery, architecture & deployment",
        mission:
          "Watch milestone pressure, verification debt, release readiness, and architecture risk.",
      },
      product: {
        name: "Product Designer",
        tagline: "Clarify scope, shape requirements",
        mission:
          "Keep clarification, acceptance quality, and collaborative-evolution proposals in view.",
      },
      evolution: {
        name: "System Evolution Expert",
        tagline: "Improve the loop itself",
        mission:
          "Review system-evolution proposals, loop health, and governance signals.",
      },
    },
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
      eyebrow: "Settings",
      title: "Configure loop behaviour, integrations, and access",
      loopEyebrow: "Settings",
      loopTitle: "Loop Configuration",
      loopDescription:
        "Define how the loop thinks and acts: Role, SOUL, PRINCIPLE, DESIGN, and SKILL.",
      role: "Role",
      roleName: "Role name",
      username: "Username",
      rolePlaceholder: "e.g. Execution Driver",
      defaultGoal: "Default goal",
      soul: "SOUL",
      soulPlaceholder:
        "Write the loop's soul here — why it exists, its long-term goals, and value priorities. Free-form, no required fields.",
      soulHintTitle: "What goes here",
      soulHint1: "Mission — what this loop exists to accomplish",
      soulHint2: "Long-term goals — what success looks like over time",
      soulHint3: "Value priorities — what matters most when trade-offs arise",
      principle: "PRINCIPLE",
      principlePlaceholder:
        "Write the loop's behaviour constitution here — rules that govern how it operates. Free-form, no required fields.",
      principleHintTitle: "What goes here",
      principleHint1: "Accuracy vs speed preference",
      principleHint2: "Write threshold — when human approval is required",
      principleHint3:
        "Core constraints — never fabricate, state uncertainty, etc.",
      design: "DESIGN",
      designPlaceholder:
        "Write the execution strategy here — how the loop plans and acts. Free-form, no required fields.",
      designHintTitle: "What goes here",
      designHint1: "Execution flow — plan → act → observe, step limits",
      designHint2: "Knowledge context — memory window, external inputs",
      designHint3: "Output rules — format, length, schema requirements",
      resources: "Resources",
      resourcesNote:
        "Uploaded documents the loop can reference during execution.",
      resourcesHintTitle: "What goes here",
      resourcesHint1: "CSV tables — state rules, priority matrices",
      resourcesHint2: "Markdown guides — checklists, conventions",
      resourcesHint3: "TOML/YAML configs — budget, schedule",
      uploadResource: "Upload resource",
      skill: "SKILL",
      skillNote: "Versioned capability modules referenced by the loop.",
      skillHintTitle: "What goes here",
      skillHint1: "Skill name + active version the loop is bound to",
      skillHint2: "Skills define UI profiles, emphasis, and field priorities",
      skillHint3: "Manage versions on the Skills page",
      skills: "Skills",
      uploadVersion: "Upload version",
      active: "Active",
      makeActive: "Make active",
      integrationsEyebrow: "Settings",
      integrationsTitle: "Integrations",
      integrationsDescription:
        "GitLab project binding and external service connections.",
      projects: "Project binding",
      projectBinding: "Project path",
      projectPathPlaceholder: "namespace/project-name",
      projectId: "Project ID",
      defaultBranch: "Default branch",
      accessEyebrow: "Settings",
      accessTitle: "Access",
      accessDescription:
        "Secrets and credentials used by the loop, encrypted at rest.",
      secretsList: "Managed secrets",
      secretName: "Name",
      secretScope: "Scope",
      secretLastAudit: "Last audit",
      addSecret: "Add secret",
      clearMemory: "Clear memory",
      rebuildMemory: "Rebuild memory",
      lastAction: "Last action",
      lastActionIdle: "idle",
      lastActionCleared: "cleared",
      lastActionRebuilt: "rebuilt",
    },
    preferences: {
      personality: "Personality",
      personalityPlaceholder: "How you work",
      waysOfWorking: "Ways of working",
      waysOfWorkingPlaceholder: "One operating principle per line",
      goalPlaceholder: "Your default goal",
    },
    recommendedActions: {
      title: "Recommended next actions",
    },
    approvals: {
      eyebrow: "Approvals",
      title: "Stop before writing, not after",
      description:
        "Review, approve, or reject actions before the system writes to GitLab. Every approval carries context and reasoning.",
      pending: "Pending",
      history: "History",
      noPending: "No pending approvals.",
      noHistory: "No approval history.",
      actionDetail: "Action detail",
      whatWillHappen: "What will be written / executed",
      whyThisAction: "Why this action exists",
      memoryRelation: "Related memory",
      sourceLoop: "Source loop",
      sourceRun: "Source run",
      createdAt: "Created at",
      approve: "Approve",
      reject: "Reject",
    },
    turns: {
      eyebrow: "Turns",
      title: "Every execution leaves a trace",
      description:
        "Inspect turn history, timelines, costs, and outcomes. See what the system did, not just what it says.",
      turnDetail: "Turn detail",
      loopName: "Loop",
      trigger: "Trigger",
      duration: "Duration",
      totalTokens: "Total tokens",
      totalCost: "Total cost",
      objectsProcessed: "Objects processed",
      agents: "Agents",
      targets: "Targets",
      summary: "Summary",
      conclusion: "Conclusion",
      timeline: "Timeline",
      stopTurn: "Stop this turn",
      stopLoop: "Stop loop",
    },
    memory: {
      eyebrow: "Memory",
      title: "What the system remembers",
      description:
        "Loop memory, engineering memory, and governance memory evolve over time. This is not transcript — it is current understanding.",
      loopMemory: "Loop memory",
      engineeringMemory: "Engineering memory",
      governanceMemory: "Governance memory",
      systemStatus: "System memory status",
      updatedAt: "Updated at",
      sourcesFrom: "Sources from",
      suggestedNextSteps: "Suggested next steps",
    },
    agents: {
      eyebrow: "System",
      title: "Agent runtime status",
      description:
        "Monitor Loop Core, worker agents, and external agent connections.",
      loopCore: "Loop Core",
      loopCoreChatHint:
        "Chat with the Loop Agent orchestrating this workbench.",
      chatPlaceholder: "Ask the Loop Agent…",
      chatSend: "Send",
      serviceStatus: "Service status",
      activeLoops: "Active loops",
      queueLength: "Queue length",
      workerAgents: "Worker agents",
      externalAgents: "External agents",
      boundTo: "Bound to",
    },
    skillsPage: {
      eyebrow: "System",
      title: "Skill registry",
      description: "Browse installed skills, versions, and loop bindings.",
      bindings: "Loop bindings",
    },
    gateway: {
      eyebrow: "System",
      title: "AI Gateway",
      description:
        "Model routing, budget consumption, provider configuration, and usage logs.",
      modelRouting: "Model routing",
      budget: "Budget",
      currentRunBudget: "Current run budget",
      loopDailyBudget: "Loop daily budget",
      monthlyBudget: "Monthly budget",
      providers: "Providers",
      fallback: "Fallback",
      usageLogs: "Usage logs",
    },
    governance: {
      eyebrow: "System",
      title: "Governance",
      description:
        "Verification debt, risk alerts, comprehension rot, and improvement proposals.",
      verificationDebt: "Verification debt",
      riskAlerts: "Risk alerts",
      comprehensionRot: "Comprehension rot",
      improvementProposals: "Improvement proposals",
      lastVerified: "Last verified",
    },
  },
} as const;
