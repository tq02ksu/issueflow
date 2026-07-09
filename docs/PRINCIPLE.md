

## 决策: 一个 Workbench = 一个 Loop

系统内一个 workbench 绑定一个 loop。loop 的所有配置（调度策略、状态机、skill 引用、预算策略、通知策略）就是 workbench 的配置，统一在 Settings 中管理。

这意味着：
- 不需要单独的 "Loops 管理" 页面 —— loop 的配置和启停都在 workbench Settings 中进行
- Dashboard 即为当前 loop 的运行总览
- 多 workbench 切换 = 多 loop 切换

## Turn = Loop 的一次执行轮

Turn 是 loop 的一次完整执行周期，对应论文中的 turn 概念。每次 turn 包含 execute → evaluate → conclude 三个阶段，产生一条 timeline 和至少一次 memory 写入。起于 trigger（schedule / event / manual），终于 completed / failed / waiting_approval。

## LOOP 定义

- 固定的状态
- SOUL（方向层）
- PRINCIPLE（约束层）
- EXECUTION（执行层：包含 DESIGN + KNOWLEDGE + SHORT-TERM GOALS + RULE）

🧠 各层含义
1) SOUL（不变）

回答：为什么做

任务使命
长期目标
价值优先级

👉 特点：稳定、不随任务变

2) PRINCIPLE（不变）

回答：什么是对的做法

不编造
不确定要说明
优先准确性 vs 速度
是否允许主动澄清

👉 特点：行为宪法

补充原则：
- 对象优先于对话
- 写操作默认进入待确认状态
- memory 接口稳定, 不绑定具体实现
- 高配置信息后置, 高频观察信息前置

3) EXECUTION（变化层 ⭐）

回答：现在怎么做

这是你原来 3 个东西的合并：

包含内容：
A. SHORT-TERM GOALS（短期目标）✔
当前要完成什么
子目标拆解
任务阶段
B. DESIGN（执行流程）✔
plan → act → observe
是否多步
是否需要重规划
C. KNOWLEDGE（信息）✔
用户输入
工具结果
记忆上下文
D. RULE（输出约束）✔（轻量保留）
输出格式
长度限制
JSON / Markdown 等

## 对象原则

系统的主对象不是对话, 而是：

**系统主对象（核心事实模块，state machine 管理流转与变更）：**

- LOOP
- Turn
- Agent
- MEMORY
- APPROVAL / PENDING ACTION
- SKILL

**可插拔的事实模块（外部工作对象，以受控状态机方式管理流转，状态变更需经 turn 评估和 approval 确认）：**

- Issue
- Merge Request
- Milestone

聊天和消息流只是交互方式, 不是系统主模型。

## 写操作原则

默认读多写少。

任何外部写操作默认都不直接执行, 而是先形成待确认动作：

- GitLab comment
- GitLab issue / MR 更新
- 环境写入
- 外部系统写入

确认主体可以是：

- 人工确认
- 更高层 LOOP 确认
- 明确授权的系统级策略确认

对于 LOOP 内的执行结果, 更合理的分工是：

- executor 负责执行
- evaluator 负责确认
- evaluator 产出结论性消息, 再提交给上层 LOOP 使用

## Memory 原则

memory 是系统能力, 不是某个具体产品能力。

- 当前采用 mem0 作为通用记忆系统
- 后端通过稳定接口与 memory 系统交互
- 未来如替换 memory 实现, 只要接口不变, 后端逻辑不需要跟着改

重点不是 memory 内部怎么做, 而是：

- 什么时候写
- 写什么结构
- 怎么被 LOOP 读取和使用

## SKILL 原则

SKILL 是 LOOP 的能力引用对象, 也是 Agent Runtime 的执行对象。

也就是说：

- LOOP 决定使用什么 SKILL
- Agent Runtime 决定怎样执行这个 SKILL
- SKILL 本身应支持版本管理和升级

因此 SKILL 既不应完全写死在 LOOP 里, 也不应完全散落在 Runtime 实现里。

SKILL 升级本身应作为人工确认动作处理。

升级内容不仅可能是 SKILL 版本本身, 也可能是对 LOOP 定义中以下层的修改：

- SOUL
- PRINCIPLE
- DESIGN

## 交互优先级原则

系统应该优先暴露高频观察信息, 再暴露低频配置能力。

高频信息：

- LOOP 当前状态
- Turn 当前结果
- 运行中的子 Agent 及状态
- 最新事件流
- 待确认动作
- 当前 memory 摘要

低频信息：

- provider 配置
- secret 管理
- skill 版本管理
- 环境配置

这条原则不仅影响 UI, 也影响模块入口组织方式。

## 人工介入原则

人工可以随时介入各 agent 的执行。

系统设计时必须原生考虑不同强度的介入方式, 不能只支持最终审批：

- 停止一个工具调用
- 发送 steering 消息
- 停止本次 run
- 停止整个 loop

因此运行时架构、消息队列、状态机和 agent 工作流都必须兼容这种中途介入能力。

## 进化
与其评估提示词, 评估SKILL, 评估LOOP, 系统复杂度越来越高, 个性化太强, 评估成本越来越高, 不如让LOOP自我进化。

## 决策: LOOP指导LOOP进化
采用系统级LOOP指导LOOP 进化，形成自我进化的闭环。
系统预置LOOP进化原则，LOOP在执行过程中自我评估，发现问题后自我修正，形成自我进化的闭环。

**依赖子项**
- **记忆系统**: 采用mem0做为通用记忆系统, 记录LOOP执行过程中的问题和改进建议, 形成自我进化的闭环。
- **可观测平台**: otel 兼容的可观测平台, 统一的元数据定义方便 LOOP评估系统反思. ~~langfuse~~

## Agent Runtime
不同类型LOOP的执行环境需要采用不同的Agent Runtime, 这个是Harness, Context工程领域的工作和差异性, 本系统采用支持插件的方式来集成而不是自己开发。

## LOOP 控制Agent职责

LOOP 控制Agent 的职责不是直接完成所有重型执行, 而是负责：

- 根据预算决定执行范围和强度
- 规划任务
- 调起 Agent
- 跟进 Agent 执行过程中遇到的问题
- 整理执行结果
- 做评估
- 做优化
- 实时更新任务状态
- 实时更新总结信息

因此 LOOP 控制Agent 更接近 orchestrator / manager, 而不是单一执行器。

LOOP 可以根据需要调起子 Agent 进入工作。

在执行时, LOOP 会组合使用：

- 不同的 SKILL
- 不同的 Agent Runtime
- 不同类型的子 Agent

来完成一个具体工作。

在整个过程中, LOOP 控制Agent 需要持续监督并管理子 Agent 的：

- 工作状态
- 生命周期
- 当前阶段
- 当前问题

并支持与其进行运行时交互：

- 打断
- steering
- stop
- 其他必要的人工介入

不同类型任务
- **编程任务**: OpenCode, codex, copilot cli
- **通用任务**: Hermes, OpenClaw



## 角色关注点与系统进化

以下从 `DESIGN.md`、`MVP.md`、`Work Item State Machine Design`、`Engineering Memory Hub Design`、`Loop Engineering Model Design` 和 `Workbench UI Prototype Design` 中提取各角色在 issueflow 中的关注点、输入和输出。

角色分两层：

- **系统用户**（产品经理、研发人员、测试人员、架构设计师）：使用 issueflow 推进自己的项目协作，产出是他们开发的系统
- **平台维护者**（issueflow 系统开发人员）：使用 issueflow 进化 issueflow 本身，产出是版本化的 skill 和 loop 定义

两条线之上，还有一个**系统/平台协作进化层**：issueflow 在帮助系统用户推进项目的过程中，发现可复用的协作模式，沉淀为版本化的 SKILL 知识。

---

### 产品经理

**关注点**

- 需求是否足够清晰，能否进入 execution 阶段
- Milestone 是否在轨道上，哪些 item 阻塞交付
- 哪些 issue 需要进一步澄清（clarifying）
- 当前系统对需求的"理解"是否正确（memory 中存储的 spec / acceptance criteria）

**输入**

- Issue 描述、需求文档
- Acceptance criteria
- Milestone 目标与时间线
- 对 Light Agent 输出的 steering 修正
- 对 Pending Action 的 approve / reject / revise

**输出**（进入 engineering_memory + pending_actions）

- Clarification Loop 状态：当前 issue 在哪个状态（new / clarifying / planned / ready_for_execution）
- Milestone Pressure 报告：被阻塞项、主导停滞状态、风险与滑移信号
- Issue readiness 评估：缺什么上下文、是否有 open question
- 草稿评论 / Issue 更新建议（以 Pending Action 形式）
- **SKILL 变更反馈**：系统从各项目使用中积累的协作模式知识，以 skill 版本变更建议的形式回到产品经理的 Dashboard，作为知识积累的显式反馈（来源：系统/平台协作进化层）

---

### 研发人员

**关注点**

- 哪些 issue 已 ready_for_execution，可以开始写代码
- MR 是否进入 review、谁在 review、是否需要修改
- Heavy Agent（OpenCode / Codex）委派状态和执行结果
- 自己负责的工作项当前状态和 next action

**输入**

- Code（通过 MR 或 Heavy Agent 产出）
- MR 描述与 diff
- 对执行结果的确认或修正
- 对 Heavy Agent 委派的确认

**输出**（进入 engineering_memory + pending_actions）

- Execution-ready issues（状态 = ready_for_execution）
- MR Review Loop 状态（draft → in_review → changes_requested → ready_to_merge）
- Heavy Agent 执行结果与结论
- Pending Action：委派确认、代码合并确认

---

### 测试人员

**关注点**

- 哪些 issue / MR 需要验证
- Acceptance criteria 是什么，是否有建议的验证路径
- 当前 engineering memory 中的 evaluation summary（覆盖了哪些 case、缺什么）
- 回归风险

**输入**

- 测试结果、验证发现
- Bug 报告
- 对 validation_suggestions 的补充或修正
- 对 evaluation 结论的确认或驳回

**输出**（进入 engineering_memory + pending_actions）

- Validation suggestions（happy path / failure path / edge cases / non-goals）
- Evaluation summary（coverage notes、missing cases）
- Risk notes（title、severity、mitigation）
- 验证债务信号（当大量输出未经独立 evaluator 检查时触发）

---

### 架构设计师

架构设计师是系统用户中关注结构质量的角色。与产品经理关注需求、研发关注实现、测试关注验证不同，架构设计师关注的是系统层面的决策正确性和技术债务的显式化。

**关注点**

- 跨模块的架构决策是否有显式记录，后续执行是否偏离了设计意图
- 技术债务是否被系统感知并标记为风险（engineering_memory 中的 risk_notes）
- 外部 Heavy Agent 产出的代码是否引入了非预期的架构耦合
- 多项 issue / MR 之间的设计一致性

**输入**

- 架构决策记录（ADR 类文档、设计 spec）
- 系统分解描述（模块边界、接口契约）
- 对 Light Agent 产出的 spec / decision artifact 的审查和修正
- 对架构级 risk note 的确认或驳回

**输出**（进入 engineering_memory，artifact_type = `decision`）

- Decision records：本次评审的架构决策，持久化为 engineering_memory
- Architectural risk notes：与产品级 risk 不同，聚焦模块耦合、接口漂移、技术选型偏差
- Cross-module consistency check：多项工作项之间的设计一致性信号
- Pending Action：架构变更确认、技术债务处置建议

---

### 系统用户交互形式总结

四种系统用户角色在 issueflow 中的交互形式 **高度一致**，差异仅在内容层面上由 role 和 skill 驱动：

**一致的交互形式：**

| 维度 | 一致点 |
|------|--------|
| 页面骨架 | 同一套 Workbench shell：Dashboard / Issues / MRs / Milestones |
| 工作流入口 | 都在 Dashboard 看到待处理信号 → 进入对应对象详情 → 看到 Light Agent 输出 → 执行确认/修正 |
| Pending Action 机制 | 所有写回和状态变更都走统一的 approve / reject / revise |
| AG-UI Agent Session | 所有角色都可以在 `/workbench/agent` 中与 Agent 实时对话 |
| Memory 读写 | 都在统一的多 scope 多 kind 模型下读写（project / workbench / personal） |
| 人工介入 | 共享同一套 Steer / Stop / Takeover 控制面 |

**差异仅在内容层：**

- 同一 Issue 详情页，产品经理看到 clarification 状态和 open question 优先展开，研发人员看到 execution readiness 和 Heavy Agent 委派状态优先展开
- 同一 Dashboard，不同角色看到不同优先级的信号卡片（clarification debt vs review queue vs verification debt）
- 这些差异由 role profile 和 skill UI profile 控制（`tone`、`density`、`default_expanded_sections`、`recommended_action_order`），**不改页面骨架**

结论：**交互形式是同构的，不需要为不同角色做不同的交互模型。**

---

### 交互模型 vs 实现方式的区分

以上分析引出一个关键区分：

| | 交互模型 | 实现方式 |
|------|---------|---------|
| **定义** | 人如何与系统互动（shell、页面骨架、操作路径、确认流程） | Loop 内部如何工作（数据源、处理逻辑、写回目标） |
| **是否统一** | **一套**：所有角色共用 Workbench shell + Dashboard + Pending Action + Agent Session | **可不同**：不同 Loop 类型读取不同数据、产出不同内容、写入不同目标 |
| **谁可见** | 对用户完全可见，是用户体验 | 对用户透明，是 Loop 的内部配置 |
| **差异来源** | role profile + skill UI profile 驱动 emphasis / defaults | Loop 类型 + evolution_policy + 数据源绑定 |

实例对照：

- 产品经理在 Workbench 中查看 Issue clarification 状态 → approve 一个 comment draft → GitLab 写回
- issueflow 开发人员在 Workbench 中查看 Evolution Turn 结果 → approve 一个 skill_evolution_proposal → Skill Registry 发布

人看到的是同一个 Dashboard、同一个 Pending Action 列表、同一个 approve 按钮。区别在于 Loop 内部读的是 GitLab issue 还是 OTel metrics，写入目标是 GitLab 还是 Skill Registry——这些差异被封装在 Loop 的配置层里，不反映在人机交互界面上。

因此系统设计的关键约束是：**新类型的 Loop（如 Evolution Observation Loop）不需要新的交互模型，只需要定义新的数据源绑定和产出物类型。** 这保持了 Workbench shell 的稳定性，同时允许 Loop 家族持续扩展。

---


### 系统/平台协作进化：SKILL 作为知识沉淀

issueflow 不仅帮助系统用户推进项目，还从运行中持续提取可复用的协作模式，沉淀为版本化的 SKILL 知识。

交互模型与系统用户完全一致：Loop 读取观测数据（OTel traces / metrics / events）与审计数据（agent_run_events + Pending Action 记录 + steering/takeover 日志），识别跨项目的协作模式，产出的 SKILL 变更建议以 Pending Action 形式进入对应角色（产品经理、研发等）的 Workbench Dashboard。

**进化逻辑：**

```
观测数据（OTel traces / metrics / events）
  +
审计数据（agent_run_events / Pending Action 记录 / steering + takeover 日志 / engineering_memory 变更记录）
  │
  ▼
Evolution Observation Loop（绑定在 Evolution Workbench 上，产出物以 Pending Action 形式反馈给产品经理等角色）
  │
  ├── 识别可复用的协作模式
  │     ├── 某个项目反复出现的 clarification 模式 → 可沉淀为 clarification skill
  │     ├── 某个团队的 review 节奏和标准 → 可沉淀为 review skill
  │     ├── 某种架构决策的验证方式 → 可沉淀为 architecture review skill
  │     └── 某种测试策略的覆盖模式 → 可沉淀为 validation skill
  │
  ├── 生成 SKILL 沉淀建议（以 Pending Action 形式出现在 Dashboard 中）
  │     ├── skill 的 input_contract / output_contract
  │     ├── skill 的 risk_level 和 evaluation_requirements
  │     └── skill 的适用范围（scope：system / project / workbench / personal）
  │
  └── 进入 Skill Registry
        ├── 产品经理等角色在各自 Workbench 中 approve → 版本化发布
        ├── 关联到源项目和新项目
        └── 持续观察 adoption 和退化信号
```

**沉淀产物：**

| 产物 | 内容 | 来源 |
|------|------|------|
| 新 SKILL 版本 | 从协作模式中提取的 skill 定义，含 input/output contract、risk level、evaluation requirements | 观测数据 + 审计数据（跨项目聚合） |
| SKILL 升级建议 | 已有 skill 的参数调整、阈值修订、约束增减 | 观测数据 + 审计数据（同一 skill 在不同项目中的驳回率和修正模式） |
| SKILL 退化告警 | skill 的 adoption 下降、驳回率上升或长期未使用 | 观测数据 + 审计数据（Skill Registry usage + performance 指标） |

**关键原则：**

- 产品经理在 clarification 中的重复修正、研发人员在 review 中的驳回模式、测试人员在 evaluation 中的补充——这些来自所有系统用户的信号记录在审计数据中，共同构成进化数据源。SKILL 沉淀不能从单个项目/用户的数据中提取（避免过拟合），需要跨项目信号聚合
- 沉淀产物以 Pending Action 形式进入产品经理等系统用户的 Workbench，走统一的确认→版本化流程
- 沉淀的 SKILL 是平台级知识资产，不属于任何单个项目

---

### issueflow 系统开发人员：以 LOOP 进化 issueflow 本身

issueflow 开发人员与系统用户使用**同一套交互模型**：打开一个绑定 Evolution Observation Loop 的 Workbench，走同样的 Dashboard → Turn → Pending Action → Agent Session 路径。区别只在两点：

1. **Loop 读取的数据源不同**：不是 GitLab issue，而是观测数据（OTel traces / metrics / events）+ 审计数据（agent_run_events / Pending Action 记录 / steering 日志）
2. **产出的影响对象不同**：不是项目的工作项，而是 issueflow 平台自身的 skill、loop 定义和 governance policy

交互路径对照：

```
系统用户：
  Workbench → Issue/MR 详情 → Light Agent 评估 → Pending Action → approve → GitLab 写回

issueflow 开发人员：
  Workbench → Evolution Loop Turn 详情 → Light Agent 评估 → Pending Action → approve → Skill Registry / Policy 更新
```

**关注点：**

- 系统是否真的在推进工作项？loop 的成功率、完成时长、产出质量如何
- 用户是信任系统还是在频繁覆盖系统？steering 频率、reject 率、takeover 频率
- 成本是否可持续？token 消耗趋势、单次 completed issue 的平均成本
- 系统能力是否在退化？comprehension rot、verification debt 是否在累积
- Skill Registry 中 skill 的 adoption rate、驳回率、退化信号

**Evolution Loop 读取的输入信号：**

```
┌── 运行质量信号
│     ├── loop_run 成功率（completed / failed 比例）
│     ├── turn 平均耗时（按 loop 类型和 skill 组合分组）
│     ├── evaluator 结论与最终人工判断的偏差率
│     └── Heavy Agent 委派的成功率与返工率
│
├── 用户行为信号
│     ├── steering 频率和修正内容（人纠正了什么模式）
│     ├── Pending Action reject 率及驳回原因分布
│     ├── takeover 频率及触发场景
│     ├── 重复性补充说明（人对 issue 做了哪些系统本应自动完成的手动补充）
│     └── Memory rebuild / clear 频率（系统记忆失效的信号）
│
├── 预算与成本信号
│     ├── 单次 turn 的 token 消耗分布（按 phase：discovery / execution / evaluate）
│     ├── 单次 completed issue 的总成本
│     ├── 预算超支的触发频率和原因分类
│     └── 不同 model / provider 组合的 cost-efficiency 对比
│
├── 治理信号
│     ├── verification debt 累积速率（多少输出未经独立 evaluator 验证）
│     ├── comprehension rot 信号（用户长期只点通过、不读摘要、不做抽样 review）
│     ├── skill 版本 adoption 率和回滚率
│     └── Gateway Policy 触发的 block / throttle 频率
│
└── 外部 Agent 行为信号
      ├── 各 Heavy Agent（OpenCode / Codex / Copilot CLI）的常见误区分类
      ├── agent 之间的执行质量差异
      └── agent 升级前后的行为变化对比
```

**Evolution Loop 的 Pending Action 产出物（在开发人员 Dashboard 中显式呈现）：**

| 产物 | 内容 | 影响对象 |
|------|------|---------|
| `skill_evolution_proposal` | issueflow 平台 skill 的版本升级建议：新增/修改约束、调整判断阈值、优化 prompt 预设 | Skill Registry 中已发布的 skill |
| `loop_improvement_report` | loop 定义改进建议：SOUL / PRINCIPLE / DESIGN 层的建议变更 | Loop 家族定义模板 |
| `governance_report` | 平台治理报告：验证债务趋势、认知退化风险、预算偏差、建议收紧策略 | Gateway Policy、Governance 策略 |
| `agent_adapter_recommendation` | 外部 Agent 适配建议：哪个 Heavy Agent 在哪种场景下表现更好 | A2A Adapter Layer 配置 |

**一次 Evolution Turn 的完整流程：**

```
Evolution Observation Loop 按周期或事件触发 Turn
  → Evolution Loop 读取观测数据 + 审计数据
    → Load Memory（上次进化决策和效果）
      → Light Agent 分析信号 → 识别改进模式
        → 生成 Pending Action（skill_evolution_proposal / loop_improvement_report 等）
          → 进入开发人员的 Dashboard Pending Action 队列
            → 开发人员审查：approve / reject / revise
              → approve → 执行（Skill Registry 发布新版本 / Policy 更新）
                → 写回 Evolution Memory（记录决策和预期效果）
                  → 灰度应用 → 下次 Turn 对比效果 → 决定推广或回滚
```

**依赖的底层能力（对开发人员透明，由 Evolution Loop 自动读取）：**

- **可观测平台**（OTel 兼容）：统一 trace / metric / event 数据
- **记忆系统**（mem0）：记录进化决策和效果，避免重复试错
- **Skill Registry**：版本管理、兼容性判断、灰度发布、回滚
- **审计表**（Postgres append-only event log）：用户行为信号

**原则：**

> 平台自我进化不是自动修改代码，而是基于观测数据生成可信的改进建议。所有进化产物以 Pending Action 形式进入开发人员的 Workbench，走统一确认链路。系统提供信号和建议，人做决策。

---

### 统一交互模型与进化机制

交互模型只有一套——所有角色共用 Workbench shell + Dashboard + Pending Action + Agent Session。差异在 Loop 的配置层：Workbench 绑定了什么类型的 Loop、Loop 读什么数据、产出什么。

进化机制也只有一个——Evolution Loop 读取观测数据+审计数据 → Light Agent 分析 → 产出 Pending Action → 人工确认 → 版本化写入 Skill Registry。但同一个进化机制有两个观察方向，产出不同层次的 SKILL：

| 观察方向 | 观察什么 | 产出 SKILL 类型 | SKILL 的消费者 | 写入目标 |
|------|------|------|------|------|
| **协作进化** | 系统用户如何推进项目（clarification 修正模式、review 驳回模式、evaluation 补充模式） | 项目协作 SKILL（clarification / review / architecture / validation） | 产品经理、研发、测试、架构设计师 | Skill Registry |
| **系统进化** | issueflow 平台自身如何运行（loop 成功率、用户信任度、预算效率、治理信号、agent 表现） | 平台运行 SKILL（orchestration / evaluation / budget / governance） | issueflow 系统开发人员 | Skill Registry + Policy 配置 |

两者共享同一套输入源（观测数据 + 审计数据）、同一条进化流程、同一个 Skill Registry。区别仅在于：**进化后的 SKILL 是给人用的还是给系统用的。**

结论：不需要为进化机制做两套实现。Evolution Loop 在配置层面区分 observation scope 和 output skill type 即可。


## 还需要继续讨论的重要点

1. evaluator 的结论性消息以什么标准结构提交给上层 LOOP
2. 人工介入、evaluator 结论、系统策略限制三者在运行时的优先级和覆盖关系
3. SKILL / LOOP 定义变更的人工确认流程, 是统一审批模型还是分别处理
