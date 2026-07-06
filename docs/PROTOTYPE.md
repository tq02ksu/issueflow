# issueflow Prototype 设计

## 1. 文档目标

本文不是技术实现说明，也不是 MVP 范围确认，而是把 **整个系统设计** 收敛成一个适合原型阶段的产品结构：

- 有哪些主要页面
- 左侧菜单如何组织
- 每个页面核心看什么、做什么
- 功能模块之间如何对应
- 哪些属于 MVP，哪些属于后续扩展

目标是让 `issueflow` 从"架构概念"变成一个可以直接讨论、画图、拆任务的 **原型系统**，同时保持：

- 默认路径简单
- 术语尽量自解释
- 复杂能力按需展开

## 2. 原型设计原则

原型的重点不是"帮助用户理解概念"，而是把这些功能模块 **更形象地组织进系统里**，让团队能直观看到：

1. **哪些是主工作面**
   高频、持续关注、需要反复进入的模块，应成为一级主菜单和主视图骨架，例如 `Dashboard`、`Turns`、`Approvals`、`Memory`、`Agents`。

2. **哪些是上下文面**
   不独立承载主任务、但在多个页面都会被频繁参考的模块，应作为侧栏、摘要卡片、标签页或嵌入区块出现，例如当前 memory、预算、风险、pending actions。

3. **哪些是治理与配置面**
   低频但结构复杂、配置性强的模块，不应挤占主工作流，而应进入独立管理区，例如 `Gateway`、`Governance`、`Settings`、`Skills`。

4. **哪些是运行时面，哪些是资源面**
   `Turns`、`Agents`、`Approvals` 更偏运行时；`Skills`、`Memory`、`Projects`、`Environment Profiles` 更偏资源面。原型必须把这两类对象在结构上分开。

5. **模块组织要反映系统边界，而不是技术实现细节**
   页面分组应该优先体现业务边界和操作对象，避免把数据库表、内部服务或后端分层直接投射成菜单。

6. **高频功能前置，低频功能后置**
   高频查看和处理动作应尽量在 1-2 次点击内到达；低频配置、历史审计、深层治理功能可以放在二级页或标签页中。

7. **原型要允许 MVP 先落一部分，但结构上能容纳完整系统**
   即使某些能力先用 mock 数据、占位按钮或只读视图表示，它们也要出现在正确的系统位置上。

8. **默认界面必须自解释**
   优先使用用户能直接理解的对象名和动作名，例如"待确认""执行记录""系统记忆"，而不是默认暴露过多内部术语。内部概念如 `Loop`、`Skill`、`Gateway` 可以保留，但应通过副标题、说明文字或高级视图承载。

因此原型设计要避免三种偏差：

- 只按"概念解释"组织，导致页面像文档目录而不是系统
- 只按"当前 MVP"组织，导致后续模块无处安放
- 只按"技术实现"组织，导致用户工作流被后端结构牵着走

### 2.1 功能交互设计考虑点

结合控制平面产品和长会话工作台的使用特点，原型阶段应重点考虑以下交互因素：

- **高频关注点**：用户最常看的通常是 loop 状态、turn 结果、运行中的子 Agent 及状态、最新事件流、待审批项、memory 摘要，因此这些信息应高密度、短路径、默认可见
- **低频但关键操作**：如 loop 配置、provider 设置、secret 管理、skill 绑定，虽然访问频率低，但出错成本高，交互应更保守、更结构化
- **持续监控 vs 单次操作**：Dashboard、Turns、Approvals 更偏持续监控；Settings、系统管理更偏单次配置，两类页面不应使用同样的信息密度和操作节奏
- **对象优先**：系统主对象（loop、turn、agent、approval、memory）和可插拔的事实模块（issue、MR、milestone）是系统的核心实体，交互入口围绕对象展开，机制层后置
- **状态优先**：对控制平面来说，`running`、`waiting approval`、`blocked`、`failed` 比普通内容更重要，必须在列表和详情中优先显示
- **渐进展开**：默认先展示当前最相关的信息；推理细节、完整事件流、历史版本、治理原因等放到展开区、标签页或二级页

### 2.2 各功能模块的交互方式

建议按模块性质采用不同交互方式，而不是统一套一种页面模板：

- **Dashboard**：以总结卡片 + 状态面板 + 快速操作为主，是高密度总览面
- **Turns**：以时间线 + 多 Agent 面板 + 输出区为主，是最强的运行时观察面
- **Approvals**：以队列列表 + 审批详情卡片为主，强调快速判断与显式确认
- **Memory**：以摘要视图 + diff/history + 对象关联视图为主，强调"当前理解"和"变化"
- **Agents**：以状态面板 + 实例列表 + 心跳/能力信息为主，偏观察和诊断
- **Skills**：以目录/注册表 + 详情页 + 绑定关系页为主，偏资产管理
- **Gateway**：以仪表概览 + 用量日志 + 策略配置为主，偏治理与观测
- **Governance**：以告警列表 + 风险报告 + 建议卡片为主，偏监督与收敛
- **Settings**：以结构化表单 + 分组配置页为主，偏低频管理

交互上可以再归并成四类：

1. **工作流型**：Dashboard、Turns、Approvals
2. **认知型**：Memory、Governance
3. **资源型**：Skills、Projects、Environment Profiles
4. **配置型**：Gateway、Settings、Integrations

### 2.3 简单易用与自解释原则

为保证系统默认可用、可懂，原型阶段建议遵循：

- **先对象, 后机制**：先让用户看到 loop、turn、待确认、记忆，再暴露 skill、gateway、governance 等机制层
- **先结果, 后过程**：列表页和详情页默认先展示当前结论、当前状态、下一步动作，再展开执行细节
- **先常用, 后高级**：高频页面保留在主导航；低频治理与诊断能力进入"系统"分组或二级入口
- **实现细节不直接上屏**：例如 `mem0`、`Temporal`、`Wasmtime` 是实现决策，不应默认成为用户心智模型中的一级对象
- **异常与介入要显式**：当 turn 被 evaluator 拦下、需要人工介入、或被 steering 改写时，界面必须直接显示原因和当前控制权状态

## 3. 原型系统定位

原型系统应该呈现为一个 **Loop Workbench / Control Plane**，而不是普通 AI Chat。

从用户视角，它更像：

- 一个管理长期运行 loop 的工作台
- 一个查看 agent、memory、approval、budget、risk 的控制面板
- 一个把 GitLab 工作对象、AI 运行、人工确认连接起来的系统

## 4. 核心概念

### 4.1 一个 Workbench = 一个 Loop

系统内一个 workbench 绑定一个 loop。loop 的所有配置（调度策略、状态机、skill 引用、预算策略、通知策略）就是 workbench 的配置，统一在 Settings 中管理。多 workbench 切换 = 多 loop 切换。

### 4.2 Turn = Loop 的一次执行周期

Turn 是 loop 的一次完整执行周期，对应论文中的 turn 概念。一次 turn 包含：

- **多对象处理**：可以同时处理多个 issue、MR、milestone
- **多 Agent 协作**：Loop Agent 作为 orchestrator 调起多个 executor/evaluator，跟进执行问题，整理结果
- **三阶段流程**：fetch → execute → evaluate → conclude

起于 trigger（schedule / event / manual），终于 completed / failed / waiting_approval。

### 4.3 Loop Agent = Orchestrator

Loop Agent 的职责不是直接完成所有重型执行，而是负责：

- 根据预算决定执行范围和强度
- 规划任务、调起 Agent（executor / evaluator / external agent）
- 跟进 Agent 执行过程中遇到的问题
- 整理执行结果、做评估、做优化
- 实时更新任务状态和总结信息

在执行时，Loop Agent 会组合使用：

- 不同的 SKILL
- 不同的 Agent Runtime（编程任务用 OpenCode/Codex，通用任务用 Hermes/OpenClaw）
- 不同类型的子 Agent

在整个过程中，Loop Agent 需要持续监督并管理子 Agent 的：

- 工作状态
- 生命周期
- 当前阶段
- 当前问题

并支持运行时交互：**打断**、**steering**、**stop**、其他必要的人工介入。

因此 Loop Agent 更接近 orchestrator / manager，而不是单一执行器。

### 4.4 LOOP 的四层结构

来自 PRINCIPLE 的定义，一个 LOOP 由三层不变 + 一层变化组成：

| 层 | 稳定性 | 回答 | 内容 |
|-----|--------|------|------|
| SOUL | 不变 | 为什么做 | 任务使命、长期目标、价值优先级 |
| PRINCIPLE | 不变 | 什么是对的做法 | 行为宪法：不编造、不确定要说明、写操作默认待确认 |
| EXECUTION | 变化 | 现在怎么做 | SHORT-TERM GOALS + DESIGN + KNOWLEDGE + RULE |
| SKILL | 可升级 | 用什么能力 | LOOP 引用的能力对象，支持版本管理，升级需人工确认 |

### 4.5 LOOP 自我进化

系统预置 LOOP 进化原则，采用系统级 LOOP 指导 LOOP 进化。LOOP 在执行过程中自我评估，发现问题后自我修正，形成闭环。

**依赖**：
- **记忆系统**：采用 mem0，记录 LOOP 执行过程中的问题和改进建议
- **可观测平台**：otel 兼容，提供统一的元数据定义，方便 LOOP 评估系统反思

### 4.6 Agent Runtime 插件化

不同类型 LOOP 的执行环境需要不同的 Agent Runtime（编程任务用 OpenCode/Codex，通用任务用 Hermes/OpenClaw），系统采用插件方式集成，不自己开发 Runtime。

## 5. 信息架构总览

建议原型采用 **左侧主菜单 + 顶部上下文栏 + 中央工作区 + 右侧详情/操作面板** 的结构。

### 5.1 全局布局

#### 左侧主菜单

用于切换一级能力域。

#### 顶部上下文栏

用于显示当前：

- workspace / project
- 当前用户
- 当前 loop 上下文
- 全局搜索入口
- 通知/待审批入口

#### 中央工作区

用于承载页面主内容：

- 列表 / 详情 / 时间线 / 可视化状态 / 表单 / 执行日志

#### 右侧侧栏

用于承载和当前对象强相关的辅助内容：

- 当前状态摘要 / 预算消耗 / 最新 memory / 快捷操作 / 相关 pending actions

## 6. 一级菜单设计

菜单分为 **默认主菜单** 和 **高级系统菜单** 两层。

### 默认主菜单

| 菜单 | 作用 | MVP 关系 |
| --- | --- | --- |
| Dashboard | 全局总览、活跃 loop 状态卡、待处理项、最近 turns、快速操作入口 | MVP 核心 |
| Turns | 查看每次 turn 的时间线、多 Agent 执行、成本、输出、draft actions | MVP 核心 |
| Agents | 运行中的子 Agent 实例、状态、心跳、绑定的 turn | MVP 核心 |
| Approvals | 所有待确认写操作与审批历史 | MVP 核心 |
| Memory | Loop / Engineering / Governance 三层记忆 | MVP 先做简化 |
| Fact Modules | 可插拔的外部工作对象：issues、MRs、milestones，查看状态和推进情况 | MVP 部分需要 |
| Settings | 用户 profile、loop 配置、persona、skills、集成、secret、通知 | MVP 部分需要 |

### 高级系统菜单

| 菜单 | 作用 | 默认展示策略 |
| --- | --- | --- |
| Skills | skill 列表、版本、绑定关系 | 默认收起到"系统"分组 |
| AI Gateway | 模型路由、预算、provider、调用日志 | 默认收起到"系统"分组 |
| Governance | 验证债务、风险告警、认知退化、治理建议 | 默认收起到"系统"分组 |

## 7. 页面设计

### 7.1 Dashboard

#### 页面目标

让用户一眼知道系统现在在发生什么。

#### 页面模块

1. **活跃 Loop 状态卡**
   - 当前 loop 状态（healthy / blocked / waiting_approval）
   - 绑定 project / 对象
   - 下次 turn 时间
   - 预算使用概览

2. **待处理事项区**
   - 待审批 action 数量
   - 预算告警
   - 验证失败
   - 外部 agent 异常

3. **最近 Turns**
   - 最近 N 次 turn 的摘要：状态、耗时、处理了多少对象、多少 agent 参与、成本

4. **重点风险提示**
   - 卡住太久的对象
   - 重复失败的 turn
   - 长期无人处理的 pending action

5. **快速操作入口**
   - 手动触发 turn
   - 查看待审批项
   - 进入 Settings 调整 loop 配置

### 7.2 Turns

#### Turn 列表页

展示：

- turn id
- 状态（created / executing / waiting_approval / completed / failed）
- 触发来源（manual / schedule / event）
- 处理的 objects 数量（3 issues, 1 MR）
- 参与的 agents 数量
- 时间 / 耗时
- 总 cost

#### Turn 详情页

这是原型里最重要的页面之一。

**布局**：
- 左侧：Event Timeline
- 中间：Target Objects（每个对象的处理结果）+ Draft Actions（待审批的写操作草稿）
- 右侧：Agent 参与面板 + 预算消耗 + Memory 影响

**页面区块**：

1. Turn Header（状态、触发方式、loop 名称、起止时间）

2. Timeline / Event Stream
   - created → fetching_objects → memory_loaded
   - → executor_invoked (per agent, per target)
   - → evaluator_confirmed (per target)
   - → conclusion_generated
   - → approval_requested → completed / failed

3. Target Objects（本轮处理了哪些对象）
   - 每个对象的 actions 和 result
   - Draft actions 草稿（评论内容、风险等级）

4. Agent 参与面板（本轮哪些 agent 参与了）
   - 每个 agent 的角色（executor / evaluator）、模型、tokens、cost、状态

5. Conclusion（结论摘要 + 建议）

6. Operator Actions（操作者动作）
   - 停止一个 agent 的工具调用
   - 发送 steering 消息
   - 停止本次 turn
   - 停止整个 loop

### 7.3 Approvals

`Approvals` 是 `issueflow` 区别于普通 agent 的核心页面。

#### 审批列表页

展示：

- action 类型
- 风险等级
- 目标对象
- 来源 turn
- 创建时间
- 当前状态

筛选：pending / approved / rejected / execution_failed / 按风险等级

#### 审批详情页

应包含：

1. **Action Summary** — 将要写入 / 执行什么
2. **Why this action exists** — 来自哪次 turn、哪个 evaluator 的结论
3. **Draft content** — 评论/更新草稿的完整内容
4. **Memory relation** — 与当前 memory 的关系
5. **Approve / Reject / Comment** — 确认操作

原型阶段必须让用户感受到：

> 系统不是偷偷执行，而是在关键写操作前停下来等人。

### 7.4 Memory

`Memory` 页面用来建立 `issueflow` 的独特认知。

#### 为什么单独做页面

因为 memory 不是 transcript，必须让用户看到"系统记住了什么"。

#### Memory 总览页

展示分层：

- **Loop Memory**：当前 loop 对各个对象的理解和结论
- **Engineering Memory**：跨 loop 的工程级记忆（项目、标准、风险模式）
- **Governance Memory**：治理引擎的输出（验证债务、合规风险）

每层展示：摘要、已知风险、已知阻塞、建议下一步、最近更新时间、来源 turns

原型阶段可以先把 session memory 淡化，突出后 3 层。

### 7.5 Fact Modules（可插拔事实模块）

#### 页面目标

管理和查看 loop 绑定的外部工作对象（issue、MR、milestone）。这些可插拔的事实模块以**受控状态机**的方式管理其流转，每一步状态变更都需经过 turn 的评估和 approval 的确认。

每个事实模块的详情中应当可见：
- 当前 state machine 状态和可执行的下一步流转
- 最近的 turn 对它的处理结果
- 相关的 memory 摘要

### 7.6 Settings

Settings 按 LOOP 四层结构组织：

- **Profile**：当前用户、显示身份、默认偏好
- **Loop Configuration**：
  - SOUL：任务使命、长期目标、价值优先级
  - PRINCIPLE：行为宪法（准确性优先/速度优先、是否允许主动澄清、写操作阈值）
  - EXECUTION：
    - SHORT-TERM GOALS：当前阶段目标、子目标拆解
    - DESIGN：执行流程策略（plan → act → observe、多步/单步、重规划策略）
    - KNOWLEDGE：输入配置、memory 上下文策略
    - RULE：输出格式约束（JSON/Markdown、长度限制）
  - SKILL：skill 引用、版本选择、启用/禁用
- **Projects**：GitLab 项目绑定
- **Integrations**：OAuth / PAT、外部 agent provider、memory service
- **Environment Profiles**：环境配置
- **Secrets & Access**：secret 管理、审计
- **Notifications**：日报/周报、待审批提醒、风险告警

### 7.6 高级系统页面（占位）

#### Agents
- Loop Core 服务状态、心跳、活跃 loop 数、队列长度
- Worker Agent 实例：当前任务、绑定 turn、状态
- External Agents：provider、capability、状态、成本

#### Skills
- skill 列表：name / scope / version / type / status
- skill 版本管理：启用/禁用、版本切换
- skill 绑定关系：哪些 loop 使用了哪些 skill

#### AI Gateway
- 模型路由层级（cheap-fast / balanced / high-reasoning）
- 预算（当前 turn / loop 日预算 / 月预算）
- Provider 配置与 fallback 规则
- 调用日志（每次模型调用的 tokens、cost、延迟）

#### Governance
- 验证债务：哪些对象长期未经独立验证
- 风险告警：高风险 action、权限异常、重复失败 turn
- 认知退化：用户长期只点通过、长期不看摘要
- 改进建议：skill 演进提案、loop 改进报告

## 8. 原型的推荐菜单树

```text
Dashboard
Turns
  ├── All Turns
  └── Turn Detail
Agents
  ├── Loop Core
  ├── Worker Agents
  └── External Agents
Approvals
  ├── Pending
  ├── History
  └── Approval Detail
Memory
  ├── Overview
  ├── Loop Memory
  ├── Engineering Memory
  └── Governance Memory
Fact Modules
  ├── Issues
  ├── MRs
  └── Milestones
Settings
  ├── Profile
  ├── Loop Configuration
  │   ├── SOUL
  │   ├── PRINCIPLE
  │   ├── EXECUTION
  │   └── SKILL
  ├── Projects
  ├── Integrations
  ├── Environment Profiles
  ├── Secrets & Access
  └── Notifications
System
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

## 9. 原型阶段建议优先展示的页面

如果原型时间有限，最值得优先做的是这些页面：

1. **Dashboard**
2. **Turn Detail**（含多 agent 时间线 + draft actions + cost）
3. **Agents**（子 Agent 状态面板 + 事件流）
4. **Approvals**（pending 队列 + approve/reject）
5. **Fact Modules**（Issue / MR / Milestone 的 workflow 状态视图）
6. **Memory Overview**
7. **Settings**（loop 配置部分）
8. **AI Gateway**（预算 + 调用日志）

## 10. 核心交互分析

以下分析从 `MVP.md`、`DESIGN.md`、`PRINCIPLE.md`、工作项状态机、工程记忆中心、AG-UI/A2UI、Loop Engineering 模型和 UI 原型设计等文档中提取系统核心交互，并按交互链路的性质归类。

### 10.1 交互全景图

```
                         ┌─────────────────────────┐
                         │   Milestone Pressure     │
                         │   (跨对象压力传导)        │
                         └───────────┬─────────────┘
                                     │ 影响优先级/紧急度
                                     ▼
┌──────────┐   trigger   ┌──────────┐   execute   ┌───────────┐
│ Schedule  │───────────▶│   Turn   │────────────▶│ Light Agent│
│  Webhook  │            │          │             │ (orchestr) │
│  Manual   │            └────┬─────┘             └─────┬─────┘
└──────────┘                 │                         │
                             │ evaluate                │ escalate
                             ▼                         ▼
                      ┌──────────┐             ┌──────────────┐
                      │  Memory  │◀── write ──│ Heavy Agent  │
                      │ (持久化)  │            │ (OpenCode/   │
                      └────┬─────┘            │  Codex etc.) │
                           │                  └──────────────┘
                           │ produce
                           ▼
                    ┌──────────────┐
                    │Pending Action │
                    │ (待确认动作)   │
                    └──────┬───────┘
                           │ confirm / reject
                           ▼
                    ┌──────────────┐
                    │  GitLab 写回  │
                    │ (comment/     │
                    │  issue/MR)    │
                    └──────────────┘

        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
          Human Intervention Layer
        │  break · steer · stop · takeover · approve  │
        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘

        ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
          Skill Overlay (UI Profile + Policy)
        │  不改结构 · 影响 emphasis / density / tone  │
        └ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
```

系统有五条核心交互链和三条横向贯穿的机制层，共同构成 `issueflow` 的产品骨架。

### 10.2 主循环：Loop → Turn → Memory → Approval → Next Turn

这是 `issueflow` 最核心的产品叙事，定义在 `MVP.md` 的 GitLab Issue Review Loop 场景和 `PRINCIPLE.md` 的对象原则中。

**一次 Turn 的完整生命周期：**

```
Trigger (schedule / webhook / manual)
  → Fetch Objects (issue / MR / milestone 上下文)
    → Load Memory (上次 loop memory + engineering memory)
      → Execute (Light Agent 分析 + 可选 Heavy Agent 执行)
        → Evaluate (evaluator 确认结论质量)
          → Write Memory (更新 loop_memory + engineering_memory)
            → Produce Pending Actions (草稿评论 / 状态更新 / 委派请求)
              → Wait for Approval (人工确认)
                → Execute Write-back (GitLab comment / issue update)
                  → Conclude Turn (completed / failed / waiting_approval)
```

**关键交互节点：**

- **Trigger → Turn Start**：Schedule 按固定周期或事件触发，也可手动触发。每次 Turn 起于 trigger，终于 completed / failed / waiting_approval。（来源：`MVP.md` §2、§8）
- **Turn → Memory Read**：系统读取上次 loop memory（当前理解 + 上次结论 + pending 状态），确保不是每次从头开始。（来源：`MVP.md` §10）
- **Execute → Memory Write**：执行结果折叠进 loop memory；工程结论写入 engineering memory；治理信号进入 governance memory。（来源：`DESIGN.md` §18.3）
- **Turn → Pending Action**：任何外部写操作默认不直接执行，先形成待确认动作。MVP 默认停在人工确认前。（来源：`PRINCIPLE.md` 写操作原则）
- **Approval → GitLab Write-back**：确认后以当前用户 session token 执行，不超出用户实际权限。（来源：`Engineering Memory Hub Design` §Product Boundary）

### 10.3 工作项推进交互：State Machine + Pending Action

这是系统的执行语言，定义在 `Work Item State Machine Design` 和 `Engineering Memory Hub Design`。

**Issue 状态流转链：**

```
new → clarifying → planned → ready_for_execution → in_execution → done
                            │                      │
                            └────── blocked ←──────┘
```

**MR 状态流转链：**

```
draft → in_review → changes_requested → in_review (循环)
                  → ready_to_merge → merged
         │                            │
         └──────── blocked ←──────────┘
```

**核心交互序列（以 Issue 为例）：**

1. 用户在工作台选中一个 Issue
2. Light Agent 读取当前 Issue 上下文（标题、描述、评论、标签、状态）
3. Light Agent 加载相关 Skills，评估当前状态和提议的下一状态
4. 结果持久化为 scoped memory（project 级 state + workbench 级 context + personal 级 note）
5. 系统生成 Pending Action（如 `update_gitlab_issue` / `publish_gitlab_comment`）
6. 用户审查并确认
7. 若需 Heavy Execution：系统记录或触发委派到选定的 Heavy Agent（OpenCode / Codex）

**交互设计要点：**

- 当前状态、提议的下一状态、为什么、缺什么、是否需要 Heavy Agent 升级 —— 这五条信息必须在工作项详情中优先可见
- 状态变更和写回必须经过 Pending Action 确认，不允许静默变更
- State enum 是平台硬编码的，但状态转移的判断语义由 Light Agent + Skills 控制

**数据来源**：`Work Item State Machine Design` §Work Item States、§Primary Workflow；`Engineering Memory Hub Design` §Core Model、§Relationship Model

### 10.4 Agent Session 实时交互：AG-UI + A2UI

定义在 `Agent Session AG-UI + A2UI Design`。这是用户在 Workbench 中与 Agent 实时对话的通道。

**Session 生命周期交互：**

```
用户打开 /workbench/agent
  → 加载 Session 列表 → 选择一个 Session
    → 发送消息
      → 创建/继续 Durable Run
        → Worker 认领 Run → 加载上下文 → 发射 AG-UI 事件流
          → 事件先写持久化存储 → 再 fanout 给订阅者
            → 前端 SSE 消费事件 → 渲染到 UI
```

**AG-UI 事件 → UI 映射：**

| 事件 | UI 行为 |
|------|---------|
| `RunStarted` | 进入 running 状态 |
| `TextMessage*` | 流式渲染助理文本 |
| `ToolCall*` | 展示工具执行卡片（GitLab / Wiki / 仓库读取） |
| `CustomEvent(kind="a2ui_render")` | 渲染或更新 A2UI 交互表面 |
| `CustomEvent(kind="a2ui_submit")` | 用户交互数据返回，Run 继续 |
| `RunFinished` | 停止 loading，持久化结束 |
| `RunError` | 展示可恢复错误 UI |

**关键协议边界：**

- `ToolCall*` 事件仅用于真实逻辑工具（GitLab、Wiki、仓库读写）
- `A2UI` payload 只通过 `CustomEvent` 传输，必须带 `kind` 字段
- `kind: "a2ui_render"` 由 Agent 产出 UI 描述；`kind: "a2ui_submit"` 由用户交互数据返回
- A2UI 绝不放入 `ToolCallArgs` 或 `ToolCallResult`

**断线恢复交互：**

- 浏览器刷新 → 重新加载 Session → 回放已有事件 → 重订阅活跃 Run
- 服务重启 → Worker 扫描未完成 Run → 回收过期 lease → 从持久化状态恢复

**数据来源**：`Agent Session AG-UI + A2UI Design` §Data Model、§Event-to-UI Mapping、§Refresh and Restart Recovery

### 10.5 Light Agent / Heavy Agent 协作交互

定义在 `Work Item State Machine Design` §Agent Model。这是 `issueflow` 区别于通用 Coding Agent 的关键边界。

**分工原则：**

```
issueflow (控制平面)
  │
  ├── Light Agent（系统内主执行体）
  │    负责：读取上下文、加载 Skills、评估状态、提议转移、
  │          写 Memory、准备 Pending Action、协调角色视角、
  │          决定是否需要 Heavy Agent
  │
  └── Heavy Agent（外部执行引擎）
       负责：重型仓库理解、代码生成/修改、重型测试执行
       实例：OpenCode、Codex、Copilot CLI
```

**协作交互序列：**

1. Light Agent 评估 Issue 当前状态
2. Light Agent 判定：当前工作项是否需要重型执行（写代码 / 跑测试）
3. 若需要 → Light Agent 记录委派意图到 Pending Action，选定合适的 Heavy Agent 实现
4. 用户确认委派 → Heavy Agent 接管执行
5. Heavy Agent 完成后返回结果
6. Light Agent 读取结果，评估是否满足状态转移条件
7. Light Agent 更新 Memory 并推进工作项状态

**关键边界：**

- `issueflow` 拥有工作项推进权
- Heavy Agent 拥有重型执行权
- 交接是显式的，不是隐式的
- Light Agent 持续监督管理 Heavy Agent 的工作状态、生命周期、当前阶段

**数据来源**：`Work Item State Machine Design` §Agent Model、§Boundary Between Light and Heavy Agents；`PRINCIPLE.md` Agent Runtime

### 10.6 跨对象压力传导：Milestone → Issue → MR

定义在 `Loop Engineering Model Design` §Core Loop Families。这是 `issueflow` 区别于 Jira 的核心差异化能力。

**压力传导模型：**

```
Milestone (压力对象)
  │
  ├── 影响 Issue 优先级（哪些 issue 最阻塞 milestone 交付）
  ├── 影响 MR 紧急度（哪些 MR 必须加速 review）
  ├── 驱动 Escalation 时机（什么时候必须人工介入）
  └── 驱动 Next-Best-Action 排序
```

**核心交互序列：**

1. Milestone Pressure Loop 定期评估 milestone 整体健康度
2. 识别被阻塞的工作项和主导的停滞状态
3. 向下传导：调整关联 Issue 的推荐优先级和 MR 的紧急度标记
4. 向上报告：风险与滑移信号进入 Dashboard 和 Governance 视图
5. 用户可在 Dashboard 中看到 milestone 压力地图

**验证重点：**

- 被阻塞项可见
- 主导停滞状态可见
- 风险和滑移信号显式

**数据来源**：`Loop Engineering Model Design` §Milestone Pressure Loop；`DESIGN.md` §19

### 10.7 人工介入交互：多层控制

定义在 `PRINCIPLE.md` 人工介入原则。系统必须原生支持多种介入强度，不能只支持最终审批。

**介入强度分级：**

| Level | 名称 | 行为 | 影响范围 |
|-------|------|------|---------|
| 1 | **Steer** | 发送 steering 消息到运行中的 Agent | 调整方向但不中断 |
| 2 | **Stop Tool Call** | 停止当前正在执行的单个工具调用 | 当前工具 |
| 3 | **Stop Run** | 停止本次 Turn 的全部执行 | 当前 Turn |
| 4 | **Stop Loop** | 停止整个 Loop，不再接受任何 trigger | 整个 Loop |
| 5 | **Takeover** | 人工完全接管当前工作流 | Agent 退为观察模式 |

**交互设计要点：**

- 每级介入在 Dashboard 和 Turn 详情页中都有独立的触发入口
- 介入操作必须即时生效，不等待当前步骤完成
- 介入历史记录在 Turn Event 时间线中，作为可审计事件

**数据来源**：`PRINCIPLE.md` 人工介入原则；`DESIGN.md` §4

### 10.8 Skill 驱动的 UI 适应交互

定义在 `Workbench UI Prototype Design` §Skill-Driven UI Emphasis。Skill 不改页面骨架，但影响信息呈现方式。

**UI Profile 控制维度：**

- `tone`：文案风格
- `density`：信息密度
- `overview_emphasis`：概览页重点
- `issue_field_priority` / `mr_field_priority`：字段排序优先级
- `default_expanded_sections`：默认展开区
- `recommended_action_order`：推荐动作排序

**交互流：**

```
用户选择/切换 Skill Version
  → 前端解析 Skill UI Profile
    → 当前路由渲染固定的 Page Skeleton
      → 页面内容在 Skeleton 内应用 Skill 的 emphasis rules
        → emphasis labels / field order / 展开状态 / 推荐排序 变化
        → 但路由结构、导航模型、核心分栏布局均不变
```

**设计原则：**

- Skill 可影响 emphasis、density、tone、defaults、recommendation ordering
- Skill 不可替换或重排整页结构
- 用户始终知道自己在哪个页面，不会因切换 Skill 而迷失方向

**数据来源**：`Workbench UI Prototype Design` §Skill-Driven UI Emphasis

### 10.9 Memory 读写交互：多 Scope 多 Kind

定义在 `Engineering Memory Hub Design` 和 `Work Item State Machine Design` §Memory Model。Memory 不是聊天记录，是系统的当前理解。

**Scope × Kind 矩阵：**

| | `issue_state` | `issue_context` | `issue_note` | `policy_note` |
|------|--------------|----------------|-------------|-------------|
| `project` | ✓ 共享的项目级状态理解 | — | — | — |
| `workbench` | — | ✓ 工作台级结构化上下文 | — | — |
| `personal` | — | — | ✓ 个人备注/关注点 | — |
| `system` | — | — | — | ✓ 平台默认策略 |

**Turn 中的 Memory 交互：**

1. **Read**：Turn 开始时读取 `project + issue_state`（上一次的共享状态理解）
2. **Evaluate**：Light Agent 评估后生成新的 `issue_state`
3. **Write**：结果持久化，revision 递增
4. **Surface**：用户在前端可看到当前理解和上次 Turn 造成的变化

**Memory Control 交互（低频）：**

- Clear current workbench memory（需二次确认）
- Rebuild current workbench memory（需二次确认）

**数据来源**：`Engineering Memory Hub Design` §Engineering Memory、§Relationship Model；`Work Item State Machine Design` §Memory Model

### 10.10 背景准备 → 显式确认交互

定义在 `Engineering Memory Hub Design` §Product Boundary 和 `DESIGN.md` 安全与权限边界。这是 MVP 的核心安全模型。

**无认证用户时：**

```
Webhook / Schedule / Tool Call / Agent Session
  → 触发准备（只读分析、需求结构化、草稿生成）
    → 更新 engineering_memory
      → 创建 pending_action
        → 持久化等待
          → [用户登录]
            → 用户在 Dashboard / Workbench 看到 pending action
              → 用户显式确认
                → 以当前用户 session token 执行 GitLab 写回
```

**关键约束：**

- 不使用服务侧 GitLab API Token
- 所有 GitLab 写操作绑定当前用户权限
- 高风险操作须经 Gateway Policy 与状态机双重校验
- 完整 Issue 描述替换前必须向用户展示最终渲染效果
- 未来可选：用户可配置个人 PAT 启用后台自动化（需加密存储、可撤销、scope 绑定、操作 allowlist 约束）

**数据来源**：`Engineering Memory Hub Design` §Product Boundary、§Relationship Model；`DESIGN.md` §21；`MVP.md` §2

### 10.11 原型阶段建议优先实现的交互

以下交互即使先用假数据，也建议在原型中体现：

1. **从 Dashboard 进入 Turn 详情**
2. **从 Turn 详情查看当前 memory**
3. **从 Turn 详情进入审批动作**
4. **审批通过后回到 action history**
5. **从 Memory 查看某次 turn 对认知造成的变化**
6. **从 Dashboard / Settings 手动触发一次 turn**
7. **Agent Session 中发送消息 → 流式返回 → A2UI 表单交互 → 提交**
8. **Issue 详情页显示：当前状态 + 提议下一状态 + 缺什么 + 是否需要 Heavy Agent**
9. **Milestone 视图显示被阻塞项和压力传导信号**
10. **运行中 Turn 的 Steer / Stop 介入操作**
11. **Skill Version 切换后 UI emphasis 变化（但不改变页面骨架）**
12. **Memory Clear / Rebuild 的二次确认交互**

这些交互串起来后，用户就能真正理解三条核心链路：

> loop → turn → memory → approval → next turn
> issue state → light agent evaluation → pending action → user confirmation → GitLab write-back
> agent session → AG-UI stream → A2UI render → user interaction → A2UI submit

这是 `issueflow` 最核心的产品叙事。

## 11. 原型与 MVP 的关系

原型不等于 MVP。

### MVP
强调：先做通一个闭环、可运行、可演示、可验证产品方向。

### Prototype
强调：把完整系统轮廓表达清楚、让未来模块有安放位置、帮助团队统一理解产品结构。

因此原型里可以有：未来页面、占位功能、mock 数据、未实现的按钮和入口。但这些内容必须 **结构正确**。

## 12. 原型阶段的视觉重点

UI/UX 上，建议突出以下元素：

- **状态感**：running、waiting approval、blocked、failed 要非常清楚
- **时间感**：上次运行、下次运行、等待时长、历史变化
- **对象感**：每个 turn 在处理哪些 issue / MR / milestone
- **介入点**：用户什么时候该看、该批、该改、该接管
- **记忆感**：系统不是聊天记录，而是在形成当前理解

应尽量避免：

- 过多空白大卡片
- 过强"通用 AI 助手"视觉暗示
- 把审批、memory、turn 都塞进一个聊天线程里

## 13. Turn 数据模型（设计参考）

```
Turn
├── 元信息
│   ├── id, workbenchId (loop 绑定), status, trigger
│   ├── startTime, endTime, durationSecs
│
├── 处理对象 (多个)
│   └── TurnTarget[]
│       ├── objectType: "issue" | "mr" | "milestone"
│       ├── objectId
│       ├── actions (做了什么)
│       └── result (处理结果)
│
├── 参与方 (Loop Agent orchestration)
│   └── TurnAgentRun[]
│       ├── agentId, agentName
│       ├── role: "executor" | "evaluator"
│       ├── model: "cheap-fast" | "balanced" | "high-reasoning"
│       ├── status, tokensUsed, cost, retries
│       └── responsibleFor (处理了哪些 target)
│
├── Timeline (有序事件流)
│   └── TurnEvent[]  (timestamp + kind + message + agentId + targetId)
│       created → fetching_objects → memory_loaded
│       → executor_invoked (per agent, per target)
│       → evaluator_confirmed → conclusion_generated
│       → approval_requested → completed/failed
│
├── 产出物
│   ├── conclusion (自然语言结论)
│   ├── draftActions (草稿写操作: 评论内容 + 风险等级)
│   └── recommendations (建议的下一步)
│
├── Memory 影响
│   ├── memoryRead (本次 turn 读取了哪些记忆)
│   └── memoryWritten (本次 turn 写入了什么)
│
└── 预算
    └── totalTokens, totalCost
```

## 14. 一句话结论

`issueflow` 的 prototype 不应该从"聊天页"开始，而应该从 **默认简单、进阶展开的控制台 / 工作台** 开始。

最合理的产品结构是：

> **用 Dashboard、Turns、Approvals、Memory 作为第一层骨架，把 Settings 留在主导航，把 Agents、Skills、AI Gateway、Governance 收进 System 分组。**
