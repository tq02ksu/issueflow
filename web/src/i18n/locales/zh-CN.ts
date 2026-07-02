export const zhCN = {
  common: {
    locale: {
      label: "语言切换",
      english: "EN",
      chinese: "中文",
    },
    actions: {
      cancel: "取消",
      save: "保存",
      openPrototype: "打开原型",
      reviewSettings: "查看设置",
      continueToSignIn: "继续登录",
    },
  },
  landing: {
    eyebrow: "工作流驾驶舱",
    title: "我们不管理任务，我们推动任务持续前进。",
    titlePrimary: "我们不管理任务。",
    titleSecondary: "我们推动任务持续前进。",
    lead:
      "一个面向软件交付的 loop engineering 系统，让 issue、MR 和 milestone 持续进入下一步执行动作。",
    impact: "减少等待时间，更早暴露停滞工作，让 readiness 明确可见。",
    loginEyebrow: "Issueflow 网关",
    loginTitle: "面向 issue 驱动交付的受控编排。",
    loginBody:
      "从 Rust Gateway 起步，把 OIDC 和 workflow control 留在服务端，再从稳定的前端基础演进 Agent Workbench。",
    groups: {
      executionObjects: "执行对象",
      controlLayers: "控制层",
    },
    panelsLabel: "首页面板",
    panels: {
      overview: "总览",
      product: "产品",
      engineering: "工程",
    },
    story: {
      eyebrow: "系统内部",
      title: "把业务、产品和工程放进同一个操作界面",
    },
    overview: {
      title: "我们消除的执行瓶颈",
      summary:
        "当需求长期模糊、评审等待过久、上下文只存在于人的脑子里时，项目推进就会明显变慢。",
      requirementClarity: {
        title: "需求清晰度",
        description: "把半成品 issue 变成可执行工作，而不是在团队里来回追问。",
      },
      reviewFlow: {
        title: "评审流动性",
        description: "让 review 压力持续可见，避免 MR 在队列里静默老化。",
      },
      contextContinuity: {
        title: "上下文连续性",
        description: "把标准、阻塞和下一步固化下来，避免每次交接都重新开机。",
      },
    },
    diagram: {
      engine: {
        title: "执行循环引擎",
        label: "澄清 / 推进 / 验证",
        description:
          "循环引擎负责评估状态、写入记忆、排序下一步动作，并决定何时停止或升级。",
      },
      issue: {
        title: "Issue",
        label: "澄清",
        description:
          "Issue 会从模糊状态持续推进到可执行 readiness，并明确验收标准、阻塞项和下一步动作。",
      },
      mr: {
        title: "MR",
        label: "评审",
        description:
          "合并请求始终处于活跃评审循环中，而不是变成被动等待别人注意到的记录。",
      },
      milestone: {
        title: "Milestone",
        label: "压力",
        description:
          "Milestone 会把交付压力反向作用到 issue 和 MR 的状态上，改变下一步最佳动作。",
      },
      role: {
        title: "Role",
        label: "偏置",
        description:
          "一个稳定的 workbench 角色决定系统如何在速度、质量、升级和规划风格之间取舍。",
      },
      memory: {
        title: "Memory",
        label: "上下文",
        description:
          "Workbench memory 保存持续演化的上下文，避免标准和阻塞淹没在聊天记录或人的记忆里。",
      },
      skill: {
        title: "Skill",
        label: "覆盖层",
        description:
          "Skill 可以调节严格度、建议排序和呈现重点，但不会替换系统的骨架结构。",
      },
    },
    product: {
      loopEngine: {
        title: "循环引擎",
        description:
          "每个被系统管理的对象都会进入带有状态、验证和停止条件的推进循环。",
        note: "这是执行系统，不是静态看板。",
      },
      roleWorkbench: {
        title: "角色工作台",
        description:
          "一个 workbench 绑定一个稳定行为模型，用来塑造升级策略、评审严格度和推进节奏。",
        note: "这是行为系统，不是权限系统。",
      },
      memoryLayer: {
        title: "记忆层",
        description:
          "Issue、MR 和项目记忆共同减少上下文漂移，让系统可以随时间重建状态。",
        note: "上下文会变成可操作的持久数据。",
      },
      statePressure: {
        title: "状态压力",
        description:
          "Milestone 衰减、MR 延迟和 issue 停滞会一起影响下一步最佳动作的排序。",
        note: "状态携带压力，而不只是标签。",
      },
    },
    engineering: {
      stateLayer: {
        title: "状态层",
        description:
          "Issue、MR 和 milestone 会被建模成具备显式状态的 work item。",
        note: "这是所有循环作用的状态图。",
      },
      memoryLayer: {
        title: "记忆层",
        description: "结构化记忆和时间记忆共同保存人和 agent 需要的演化上下文。",
        note: "记忆有作用域、可重建，也是产品的一部分。",
      },
      agentLayer: {
        title: "Agent 层",
        description: "角色驱动的 agent 负责评估状态、提出下一步并决定何时升级。",
        note: "Agent 行为受策略和 stop rule 约束。",
      },
      pressureLogic: {
        title: "压力逻辑",
        description:
          "跨对象压力把停滞工作转成工作台中的执行优先级和动作建议。",
        note: "这也是它不是 Jira 加 AI coding 的原因。",
      },
    },
  },
  shell: {
    subtitle: "Agent 工作台",
    workbench: "工作台",
    role: "角色",
    settings: "设置",
    renameWorkbench: "重命名工作台",
    workbenchName: "工作台名称",
    userProfile: "个人设置",
    navigation: {
      overview: "总览",
      issues: "Issues",
      mrs: "MRs",
      milestones: "里程碑",
      pendingActions: "待执行动作",
      releases: "发布",
    },
  },
  workbenchSearch: {
    addTitle: "添加工作台",
    nameTitle: "命名工作台",
    searchPlaceholder: "搜索 GitLab 项目...",
    noProjects: "未找到项目",
    workbenchName: "工作台名称",
    repositoryLabel: "仓库",
    create: "创建",
  },
  issueState: {
    title: "工作项状态",
    evaluate: "评估状态",
    current: "当前",
    next: "下一步",
    missingContext: "缺失上下文",
    roleNotes: "角色说明",
    product: "产品",
    engineering: "工程",
    delivery: "交付",
    empty: "当前还没有共享的 issue 状态。运行一次评估来生成状态。",
    pendingSummary: "状态流转仍在等待确认。",
    noSummary: "暂无摘要。",
  },
  prototype: {
    overview: {
      eyebrow: "总览",
      title: "执行驾驶舱",
      description:
        "让工作台始终围绕明确的下一步动作、可见阻塞和 skill 驱动的重点表达来推进长期交付。",
      currentWorkbench: "当前工作台",
      issueWorkflow: "Issue 流程",
      mrWorkflow: "MR 流程",
      recentActivity: "最近活动",
    },
    issues: {
      eyebrow: "Issues",
      title: "把 issue 质量推进到可执行状态",
      nextAction: "下一步动作",
      acceptanceCriteria: "验收标准",
      verificationPlan: "验证计划",
      risks: "风险",
    },
    mrs: {
      eyebrow: "合并请求",
      title: "让评审状态和合并 readiness 始终可见",
      reviewSummary: "评审摘要",
      readinessChecks: "readiness 检查",
      verificationNotes: "验证说明",
      risks: "风险",
    },
    milestones: {
      eyebrow: "里程碑",
      title: "在延迟发生之前聚合流程压力",
      milestoneLabel: "里程碑",
      issueWorkflow: "Issue 流程",
      mrWorkflow: "MR 流程",
      riskSummary: "风险摘要",
      nextAction: "下一步动作",
    },
    settings: {
      eyebrow: "用户设置",
      title: "塑造操作者，而不是页面骨架",
      description:
        "在不把 workflow 导航挪出 workbench 的前提下，配置 user soul、workbench role、memory controls 和 skill 版本。",
      soulTitle: "用户灵魂",
      soulPersonalityPlaceholder: "这个用户如何做事",
      soulWaysPlaceholder: "每行一条做事原则",
      soulGoalPlaceholder: "默认目标",
      saveSoul: "保存用户灵魂",
      currentWorkbench: "当前工作台",
      roleName: "角色名称",
      roleSummary: "角色摘要",
      roleWays: "每行一条做事方式",
      roleGoals: "每行一个目标",
      saveRole: "保存角色",
      skills: "Skills",
      uploadVersion: "上传版本",
      active: "当前启用",
      makeActive: "设为启用",
      memoryControls: "记忆控制",
      clearMemory: "清空记忆",
      rebuildMemory: "重建记忆",
      lastAction: "最近动作",
      lastActionIdle: "空闲",
      lastActionCleared: "已清空",
      lastActionRebuilt: "已重建",
    },
    recommendedActions: {
      title: "建议的下一步动作",
    },
  },
} as const;
