# issueflow Prototype 设计

## 1. 文档目标

本文不是技术实现说明，也不是 MVP 范围确认，而是把 **整个系统设计** 收敛成一个适合原型阶段的产品结构：

- 有哪些主要页面
- 左侧菜单如何组织
- 每个页面核心看什么、做什么
- 功能模块之间如何对应
- 哪些属于 MVP，哪些属于后续扩展

目标是让 `issueflow` 从“架构概念”变成一个可以直接讨论、画图、拆任务的 **原型系统**，同时保持：

- 默认路径简单
- 术语尽量自解释
- 复杂能力按需展开

## 2. 原型设计原则

原型的重点不是“帮助用户理解概念”，而是把这些功能模块 **更形象地组织进系统里**，让团队能直观看到：

1. **哪些是主工作面**  
   高频、持续关注、需要反复进入的模块，应成为一级主菜单和主视图骨架，例如 `Loops`、`Runs`、`Approvals`、`Memory`。

2. **哪些是上下文面**  
   不独立承载主任务、但在多个页面都会被频繁参考的模块，应作为侧栏、摘要卡片、标签页或嵌入区块出现，例如当前 memory、预算、风险、pending actions。

3. **哪些是治理与配置面**  
   低频但结构复杂、配置性强的模块，不应挤占主工作流，而应进入独立管理区，例如 `Gateway`、`Governance`、`Settings`、`Skills`。

4. **哪些是运行时面，哪些是资源面**  
   `Runs`、`Agents`、`Approvals` 更偏运行时；`Skills`、`Memory`、`Projects`、`Environment Profiles` 更偏资源面。原型必须把这两类对象在结构上分开。

5. **模块组织要反映系统边界，而不是技术实现细节**  
   页面分组应该优先体现业务边界和操作对象，避免把数据库表、内部服务或后端分层直接投射成菜单。

6. **高频功能前置，低频功能后置**  
   高频查看和处理动作应尽量在 1-2 次点击内到达；低频配置、历史审计、深层治理功能可以放在二级页或标签页中。

7. **原型要允许 MVP 先落一部分，但结构上能容纳完整系统**  
   即使某些能力先用 mock 数据、占位按钮或只读视图表示，它们也要出现在正确的系统位置上。

8. **默认界面必须自解释**  
   优先使用用户能直接理解的对象名和动作名，例如“待确认”“运行记录”“系统记忆”，而不是默认暴露过多内部术语。内部概念如 `Loop`、`Skill`、`Gateway` 可以保留，但应通过副标题、说明文字或高级视图承载。

因此原型设计要避免三种偏差：

- 只按“概念解释”组织，导致页面像文档目录而不是系统
- 只按“当前 MVP”组织，导致后续模块无处安放
- 只按“技术实现”组织，导致用户工作流被后端结构牵着走

### 2.1 功能交互设计考虑点

结合控制平面产品和长会话工作台的使用特点，原型阶段应重点考虑以下交互因素：

- **高频关注点**：用户最常看的通常是 loop 状态、run 结果、待审批项、memory 摘要，因此这些信息应高密度、短路径、默认可见
- **低频但关键操作**：如 loop 配置、provider 设置、secret 管理、skill 绑定，虽然访问频率低，但出错成本高，交互应更保守、更结构化
- **持续监控 vs 单次操作**：Dashboard、Runs、Approvals 更偏持续监控；Create Loop、Settings 更偏单次配置，两类页面不应使用同样的信息密度和操作节奏
- **对象优先**：用户首先关注“哪个 loop / 哪个 issue / 哪次 run / 哪个 approval”，其次才是底层机制，因此交互入口应尽量围绕对象展开
- **状态优先**：对控制平面来说，`running`、`waiting approval`、`blocked`、`failed` 比普通内容更重要，必须在列表和详情中优先显示
- **渐进展开**：默认先展示当前最相关的信息；推理细节、完整事件流、历史版本、治理原因等放到展开区、标签页或二级页

### 2.2 各功能模块的交互方式

建议按模块性质采用不同交互方式，而不是统一套一种页面模板：

- **Loops**：以列表 + 详情 + 配置表单为主，是主要导航入口
- **Runs**：以时间线 + 状态面板 + 输出区为主，是最强的运行时观察面
- **Approvals**：以队列列表 + 审批详情卡片为主，强调快速判断与显式确认
- **Memory**：以摘要视图 + diff/history + 对象关联视图为主，强调“当前理解”和“变化”
- **Agents**：以状态面板 + 实例列表 + 心跳/能力信息为主，偏观察和诊断
- **Skills**：以目录/注册表 + 详情页 + 绑定关系页为主，偏资产管理
- **Gateway**：以仪表概览 + 用量日志 + 策略配置为主，偏治理与观测
- **Governance**：以告警列表 + 风险报告 + 建议卡片为主，偏监督与收敛
- **Settings**：以结构化表单 + 分组配置页为主，偏低频管理

交互上可以再归并成四类：

1. **工作流型**：Loops、Runs、Approvals
2. **认知型**：Memory、Governance
3. **资源型**：Skills、Projects、Environment Profiles
4. **配置型**：Gateway、Settings、Integrations

### 2.3 简单易用与自解释原则

为保证系统默认可用、可懂，原型阶段建议遵循：

- **先对象, 后机制**：先让用户看到 loop、run、待确认、记忆，再暴露 skill、gateway、governance 等机制层
- **先结果, 后过程**：列表页和详情页默认先展示当前结论、当前状态、下一步动作，再展开执行细节
- **先常用, 后高级**：高频页面保留在主导航；低频治理与诊断能力进入“系统”分组或二级入口
- **实现细节不直接上屏**：例如 `mem0`、`Temporal`、`Wasmtime` 是实现决策，不应默认成为用户心智模型中的一级对象
- **异常与介入要显式**：当 run 被 evaluator 拦下、需要人工介入、或被 steering 改写时，界面必须直接显示原因和当前控制权状态

## 3. 原型系统定位

原型系统应该呈现为一个 **Loop Workbench / Control Plane**，而不是普通 AI Chat。

从用户视角，它更像：

- 一个管理长期运行 loop 的工作台
- 一个查看 agent、memory、approval、budget、risk 的控制面板
- 一个把 GitLab 工作对象、AI 运行、人工确认连接起来的系统

## 4. 信息架构总览

建议原型采用 **左侧主菜单 + 顶部上下文栏 + 中央工作区 + 右侧详情/操作面板** 的结构。

### 4.1 全局布局

#### 左侧主菜单

用于切换一级能力域。

#### 顶部上下文栏

用于显示当前：

- workspace / project
- 当前用户
- 当前 loop / run / issue 上下文
- 全局搜索入口
- 通知/待审批入口

#### 中央工作区

用于承载页面主内容：

- 列表
- 详情
- 时间线
- 可视化状态
- 表单
- 运行日志

#### 右侧侧栏

用于承载和当前对象强相关的辅助内容：

- 当前状态摘要
- 预算消耗
- 最新 memory
- 快捷操作
- 相关 pending actions

## 5. 一级菜单设计

建议把菜单分成 **默认主菜单** 和 **高级系统菜单** 两层。这样既保持简单，也不丢掉完整系统结构。

### 默认主菜单

| 菜单 | 作用 | MVP 关系 |
| --- | --- | --- |
| Dashboard | 全局总览、异常、待处理项、活跃 loop | MVP 可做简化版 |
| Loops | loop 定义、列表、创建、配置、启停 | MVP 核心 |
| Runs | 查看具体运行、事件流、输出、状态 | MVP 核心 |
| 待确认（Approvals） | 所有待确认动作与审批历史 | MVP 核心 |
| 系统记忆（Memory） | 查看 loop memory / engineering memory / 历史结论 | MVP 先做简化 |
| Settings | 用户、项目、环境、权限、secret、集成配置 | MVP 部分需要 |

### 高级系统菜单

| 菜单 | 作用 | 默认展示策略 |
| --- | --- | --- |
| Agents | Loop Core、worker agents、external agents 运行态 | 默认收起到“系统”分组 |
| Skills | skill 列表、适用范围、版本、绑定关系 | 默认收起到“系统”分组 |
| AI Gateway | 模型、预算、provider、调用观测 | 默认收起到“系统”分组 |
| Governance | 风险、验证债务、认知退化、治理建议 | 默认收起到“系统”分组 |

这个结构的核心优点是：**默认简单易用，同时完整能力仍然有明确位置。**

## 5.1 调度模块决策

原型阶段需要明确有一个独立的调度模块。

当前决策：

- **固定调度采用 Temporal**
- Temporal 负责承载固定周期触发、调度状态保持、重试与基础运行编排
- 原型中先把 Temporal 视为 `loop schedule runtime`，主要服务于 `Loop -> Run` 的稳定触发
- 调度本身不需要成为一个独立主菜单，而应体现在 `Dashboard`、`Loops`、`Runs` 中

当前不纳入这轮原型重点的内容：

- 资源调度
- 执行资源隔离
- 多种 runtime 资源编排
- 复杂的 worker placement / sandbox placement

也就是说，原型阶段先解决：

- loop 何时触发
- run 如何被稳定拉起
- 调度状态如何可见

先不解决：

- 不同 agent / tool / sandbox 资源如何分配
- 执行环境如何隔离
- 资源竞争和容量治理

这样做的原因是，**调度可见性** 与原型关系很大，而 **资源调度与隔离** 更偏后续执行基础设施，不是当前原型的重点。

## 6. 页面设计

## 6.1 Dashboard

### 页面目标

让用户一眼知道系统现在在发生什么。

### 页面模块

1. **活跃 Loop 卡片**
   - 正在运行的 loop 数
   - 等待审批的 loop 数
   - 最近失败的 loop 数

2. **待处理事项区**
   - 待审批 action
   - 预算告警
   - 验证失败
   - 外部 agent 异常

3. **最近 Runs**
   - 最近 10 次 run
   - 状态、耗时、结果摘要

4. **重点风险提示**
   - 卡住太久的 loop
   - 重复失败的 loop
   - 长期无人处理的 pending action

5. **快速创建入口**
   - 新建 loop
   - 手动触发 run
   - 查看审批

### 原型重点

Dashboard 不需要做复杂 BI，而要突出：

- 系统在持续运行
- 系统有状态
- 系统有待人处理的节点

## 6.2 Loops

`Loops` 是原型系统最核心的一级菜单。

### 6.2.1 Loop 列表页

### 展示内容

- loop 名称
- loop 类型
- 绑定对象（如 GitLab issue / project）
- 当前状态
- 下次运行时间
- 最近一次运行结果
- 最近更新时间

### 筛选项

- enabled / disabled
- healthy / blocked / waiting approval
- 按项目
- 按 owner
- 按 loop 类型

### 快捷动作

- 手动运行
- enable / disable
- 查看详情
- 克隆 loop

### 6.2.2 Loop 详情页

建议分为 5 个标签页：

1. **Overview**
2. **Definition**
3. **Runs**
4. **Memory**
5. **Actions**

#### Overview

展示：

- loop 基本信息
- 当前状态
- 最新摘要
- 下一次调度
- 当前待审批动作
- 近期风险

#### Definition

展示和编辑：

- loop 目标
- 绑定对象
- schedule policy
- state machine policy
- skill refs
- verification policy
- budget policy
- notification policy

原型阶段可以先做成结构化表单，不必一开始做自然语言 schedule authoring。

#### Runs

显示这个 loop 的 run 历史列表。

#### Memory

显示这个 loop 当前记住了什么，以及历史怎么演进。

#### Actions

显示由该 loop 产生的：

- pending actions
- approved actions
- rejected actions

### 6.2.3 Loop 创建页

原型建议分两种入口：

1. **Quick Create**
2. **Advanced Create**

#### Quick Create

适合 MVP / prototype demo：

- 选择 loop 类型
- 选择 GitLab 项目
- 选择 issue / work item
- 设定运行频率
- 选择提示模板
- 开启 loop

#### Advanced Create

面向完整系统：

- schedule
- state machine
- skills
- memory policy
- approval policy
- budget policy
- execution policy
- verification policy

## 6.3 Runs

`Runs` 用来展示“系统不是聊天，而是在执行运行单元”。

这里建议默认突出：

- executor 做了什么
- evaluator 给出了什么结论
- 当前 run 是否需要人工介入

### 6.3.1 Run 列表页

展示：

- run id
- 所属 loop
- 目标对象
- 状态
- 开始时间 / 结束时间
- 耗时
- 触发来源（manual / schedule / event）
- 结果摘要

### 6.3.2 Run 详情页

这是原型里最重要的页面之一。

建议布局：

- 左侧：事件流 / 时间线
- 中间：阶段状态、输出结果、A2UI 卡片
- 右侧：当前 memory、预算、当前 agent、可执行操作

### 页面区块

1. **Run Header**
   - 状态
   - 所属 loop
   - 绑定对象
   - 当前 phase
   - 触发方式

2. **Timeline / Event Stream**
   - run created
   - issue fetched
   - memory loaded
   - executor invoked
   - evaluator confirmed
   - conclusion message generated
   - approval requested
   - completed / failed

3. **Run Output**
   - 当前摘要
   - 变化总结
   - 风险点
   - 建议下一步
   - evaluator 结论
   - 草稿评论

4. **Current State Panel**
   - run 状态
   - 当前 agent
   - 当前模型
   - token / cost
   - retry 次数

5. **Operator Actions**
   - 停止一个工具调用
   - 发送 steering 消息
   - 停止本次 run
   - 停止整个 loop
   - manual takeover

原型阶段即使不全部实现，也应该把这些控制点在界面中占位。

## 6.4 Approvals

`Approvals` 是 `issueflow` 区别于普通 agent 的核心页面。

### 6.4.1 审批列表页

展示：

- action 类型
- 来源 loop
- 来源 run
- 风险等级
- 目标对象
- 创建时间
- 当前状态

筛选：

- pending
- approved
- rejected
- execution_failed
- 按项目
- 按风险等级

### 6.4.2 审批详情页

应包含：

1. **Action Summary**
2. **Why this action exists**
3. **What will be written/executed**
4. **相关 memory / evaluator conclusion**
5. **approve / reject / comment**

如果是 GitLab comment action，应直接展示：

- 目标 issue
- comment 草稿内容
- 生成依据
- 与上次 memory 的关系

原型阶段必须让用户感受到：

> 系统不是偷偷执行，而是在关键写操作前停下来等人。

## 6.5 Memory

`Memory` 页面用来建立 `issueflow` 的独特认知。

### 为什么单独做页面

因为 memory 不是 transcript，必须让用户看到“系统记住了什么”。

实现上可采用 mem0，但界面默认不强调具体实现，而强调：

- 当前系统记住了什么
- 这些记忆来自哪些 runs
- 这些记忆如何影响下一次 loop

### 6.5.1 Memory 总览页

展示分层：

- session memory
- loop memory
- engineering memory
- governance memory

原型阶段可以先把 session memory 淡化，突出后 3 层。

### 6.5.2 Loop Memory 详情页

展示：

- 当前摘要
- 已知风险
- 已知阻塞
- 建议中的下一步
- 最近更新时间
- 来源 runs

### 6.5.3 Engineering Memory 页

按对象组织：

- issue
- merge request
- project
- milestone

每个对象展示：

- 当前理解
- 历史结论
- 风险标签
- 待确认事项

### 6.5.4 Memory Diff / History

展示一次 run 前后 memory 的变化。

这会非常有助于说明：

- loop 的运行不是一次性输出
- 系统认知是逐步演化的

## 6.6 Agents

`Agents` 页面用于展示系统中的执行主体。

建议分成三个标签：

1. **Loop Core**
2. **Worker Agents**
3. **External Agents**

### Loop Core

展示：

- 当前服务状态
- 调度心跳
- 当前运行中的 loop 数
- 队列长度

### Worker Agents

展示：

- 当前 worker 实例
- 所属 run
- 状态
- 生命周期

### External Agents

展示：

- provider
- capability
- 当前任务
- 状态 / heartbeat
- 成本

原型阶段哪怕先用 mock 数据，也应该把这个页面做出来，因为它能帮助用户理解：

> Loop Agent 不等于所有 agent，本体是 orchestrator。

## 6.7 Skills

`Skills` 页面体现系统未来的能力资产化方向。

### 6.7.1 Skill 列表页

展示：

- skill name
- scope
- version
- type
- owner
- status
- risk level

### 6.7.2 Skill 详情页

展示：

- skill 描述
- 输入输出契约
- 适用 loop
- 所需权限
- 验证要求
- 变更历史

### 6.7.3 Skill 绑定关系页

展示：

- 哪些 loop 使用了哪些 skill
- 哪些 skill 是系统级
- 哪些是项目级

### 6.7.4 Prototype 特别说明

即使 MVP 不真正做 skill registry，原型阶段也应该预留这个菜单和页面结构。

原因是它直接决定系统的长期产品形态。

## 6.8 Gateway

`Gateway` 页面展示模型与执行策略中枢。

建议分为：

1. **Model Routing**
2. **Budget**
3. **Providers**
4. **Usage Logs**

### Model Routing

展示不同阶段使用的模型层级：

- cheap-fast
- balanced
- high-reasoning
- specialized

### Budget

展示：

- 当前 run 预算
- 当前 loop 日预算
- 用户 / 项目预算
- 超限记录

### Providers

展示：

- 当前 provider
- 状态
- 配置
- fallback 规则

### Usage Logs

展示：

- 每次模型调用
- token 使用
- 成本
- 响应时间

## 6.9 Governance

`Governance` 页面体现系统的治理价值。

建议分为：

1. **Verification Debt**
2. **Risk Alerts**
3. **Comprehension Rot**
4. **Improvement Proposals**

### Verification Debt

展示：

- 哪些 loop 长期未经独立验证
- 哪些输出被反复复用但未重验

### Risk Alerts

展示：

- 高风险 action
- 权限边界异常
- 重复失败 loop

### Comprehension Rot

展示：

- 用户长期只点通过
- 长期不看摘要
- 长期不做抽样 review

### Improvement Proposals

展示：

- skill evolution proposal
- loop improvement report
- governance report

## 6.10 Settings

建议 Settings 按以下二级菜单拆分：

1. **Profile**
2. **Personas**
3. **Projects**
4. **Integrations**
5. **Environment Profiles**
6. **Secrets & Access**
7. **Notifications**

### Profile

- 当前用户
- 默认偏好
- 默认预算

### Personas

- 说话风格
- 风险偏好
- 审批阈值
- 默认验证强度

### Projects

- GitLab 项目绑定
- 项目级 loop 配置

### Integrations

- GitLab OAuth / PAT
- 外部 agent provider
- mem0 memory service

### Environment Profiles

- dev / staging / preview
- browser-check
- 工具可用性

### Secrets & Access

- secret 来源
- scope
- 审计记录

### Notifications

- 日报/周报
- 待审批提醒
- 风险告警

## 7. 原型的推荐菜单树

建议默认采用下面这版菜单树：

```text
Dashboard
Loops
  ├── All Loops
  ├── Create Loop
  └── Loop Detail
Runs
  ├── All Runs
  └── Run Detail
待确认（Approvals）
  ├── Pending
  ├── History
  └── Approval Detail
系统记忆（Memory）
  ├── Overview
  ├── Loop Memory
  ├── Engineering Memory
  └── Memory History
Settings
  ├── Profile
  ├── Personas
  ├── Projects
  ├── Integrations
  ├── Environment Profiles
  ├── Secrets & Access
  └── Notifications
System
  ├── Agents
  │   ├── Loop Core
  │   ├── Worker Agents
  │   └── External Agents
  ├── Skills
  │   ├── All Skills
  │   ├── Skill Detail
  │   └── Skill Bindings
  ├── AI Gateway
  │   ├── Model Routing
  │   ├── Budget
  │   ├── Providers
  │   └── Usage Logs
  └── Governance
      ├── Verification Debt
      ├── Risk Alerts
      ├── Comprehension Rot
      └── Improvement Proposals
```

## 8. 功能模块与页面的映射

为了让产品结构和系统架构对齐，建议这样映射：

| 功能模块 | 对应页面 |
| --- | --- |
| Loop Definitions | Loops |
| Scheduler | Dashboard / Loops / Runs |
| State Machines | Loop Detail / Run Detail |
| Loop Core Runtime | Runs / Agents |
| AI Gateway | Gateway |
| Feedback & Approval Hub | Approvals / Dashboard |
| Memory Layer | Memory / Run Detail / Loop Detail |
| External Agent Adapter Layer | Agents / Runs |
| Skill Registry | Skills |
| Governance Engine | Governance |
| Env Key Manager | Settings / Environment Profiles / Secrets |

## 9. 原型阶段建议优先展示的 6 个页面

如果原型时间有限，最值得优先做的是这 6 个页面：

1. **Dashboard**
2. **Loop List**
3. **Loop Detail**
4. **Run Detail**
5. **Approvals**
6. **Memory Overview**

只要这 6 个页面足够清晰，用户基本就能理解系统核心价值。

## 10. 原型与 MVP 的关系

原型不等于 MVP。

### MVP

强调：

- 先做通一个闭环
- 可运行
- 可演示
- 可验证产品方向

### Prototype

强调：

- 把完整系统轮廓表达清楚
- 让未来模块有安放位置
- 帮助团队统一理解产品结构

因此原型里可以有：

- 未来页面
- 占位功能
- mock 数据
- 未实现的按钮和入口

但这些内容必须 **结构正确**，而不是随便摆概念。

## 11. 建议的原型层级

建议把原型分三层做。

### Layer 1：MVP 原型

包含：

- Loop List
- Create Loop
- Run Detail
- Approvals
- 基础 Memory 视图

### Layer 2：完整控制平面原型

补充：

- Dashboard
- Runs
- Settings
- Gateway
- Agents

### Layer 3：完整产品方向原型

补充：

- Skills
- Governance
- 高级 Loop Definition
- 多层 Memory
- 环境与权限治理

## 12. 原型中的关键交互

以下交互即使先用假数据，也建议在原型中体现：

1. **从 Loop 列表进入 Run 详情**
2. **从 Run 详情查看当前 memory**
3. **从 Run 详情进入审批动作**
4. **审批通过后回到 action history**
5. **从 Loop 详情回看历史 runs**
6. **从 Memory 查看某次 run 对认知造成的变化**

这些交互串起来后，用户就能真正理解：

> loop -> run -> memory -> approval -> next run

这是 `issueflow` 最核心的产品叙事。

## 13. 原型阶段的视觉重点

UI/UX 上，建议突出以下元素：

- **状态感**：running、waiting approval、blocked、failed 要非常清楚
- **时间感**：上次运行、下次运行、等待时长、历史变化
- **对象感**：每个 loop 在服务哪个 issue / project / object
- **介入点**：用户什么时候该看、该批、该改、该接管
- **记忆感**：系统不是聊天记录，而是在形成当前理解

应尽量避免：

- 过多空白大卡片
- 过强“通用 AI 助手”视觉暗示
- 把审批、memory、run 都塞进一个聊天线程里

## 14. 一句话结论

`issueflow` 的 prototype 不应该从“聊天页”开始，而应该从 **默认简单、进阶展开的控制台 / 工作台** 开始。

最合理的产品结构是：

> **用 Loops、Runs、待确认、系统记忆 作为第一层骨架，把设置保留在主导航，把 Agents、Skills、AI Gateway、Governance 收进高级系统层。**

这样既能承接当前 MVP，又能完整表达整个系统的最终形态。