# issueflow MVP 方案

## 1. 目标

这个 MVP 的目标不是把 `issueflow` 的全部设计一次做完，而是用 **最小功能集合** 验证它的核心价值：

> `issueflow` 不是另一个聊天 agent，而是一个能把 **loop、memory、schedule、human approval** 组织起来的控制平面。

因此，MVP 必须满足三个要求：

1. **能被看见**：用户能直接看到 loop 在跑、跑出了什么、下一步是什么
2. **能证明核心价值**：不是一次性生成，而是“持续运行 + 记忆 + 人工确认”
3. **实现简单且可替换**：先用最轻实现，但边界不能写死，后面能升级成完整架构

## 2. MVP 只做一个场景

MVP 建议只做一个最小闭环场景：

## **GitLab Issue Review Loop**

用户为某个 GitLab issue 创建一个 loop，系统按固定周期或手动触发执行：

1. 读取 issue 标题、描述、评论、标签、状态
2. 读取上一次 loop memory
3. 生成：
   - 当前 issue 摘要
   - 自上次运行后的变化
   - 风险/阻塞点
   - 建议下一步动作
   - 一段可提交到 GitLab 的草稿评论
4. 把结果写入 memory
5. 在 UI 中实时展示 run 过程和最终结果
6. 生成一个 **待确认动作**：是否把草稿评论真正发到 GitLab

默认 **不自动写 GitLab**。用户只需要看到：

- loop 是定义对象，不是一次性 prompt
- loop 会记住上次发生了什么
- loop 会定期继续工作
- loop 会把写操作停在人工确认前

这已经足以证明 `issueflow` 的核心方向是对的。

## 3. MVP 明确不做什么

为了控制复杂度，MVP 不做以下内容：

- 不做自然语言 schedule 解析
- 不做双层完整状态机 DSL
- 不做外部 agent 多 provider 适配
- 不做 environment executor / browser validation
- 不做 skill registry / version graph / evolution
- 不做自动模型切换
- 不做复杂预算体系
- 不做 takeover / break / pause / resume 全套交互
- 不做无用户上下文的 GitLab 写操作

这些能力都保留扩展点，但不进入 MVP 的“必须完成”范围。

## 4. MVP 的最小技术决策

以下技术决策以 **实现最简单、未来不难替换** 为原则。

| 主题 | MVP 决策 | 为什么简单 | 为什么不影响后续替换 |
| --- | --- | --- | --- |
| 持久化 | **SQLite 先行**，通过 `sqlx` 访问 | 本地开发和 demo 最简单 | 后续可迁移到 Postgres，领域表结构不变 |
| 调度 | **应用内 scheduler**，只支持 fixed interval + manual trigger | 不引入外部调度系统 | 后续可替换为 Postgres-backed 或独立 worker |
| 状态机 | **Rust 枚举 + 少量显式状态转移** | 最容易写对、最好测试 | 后续可演进为表驱动/配置化 |
| Loop runtime | **嵌入主服务进程** | 无需拆服务 | 代码按模块分层后可拆 worker |
| 模型接入 | **单 provider、单模型、单 adapter** | 快速打通闭环 | 先抽 trait，后面可挂多 provider |
| GitLab 接入 | **只做读 + 用户确认后的单一写动作** | 风险低，边界清晰 | 后面可加更多 action 和代理能力 |
| UI 实时流 | **SSE 下行 + REST 上行** | 最贴合现有 axum/Vue 栈 | 后续可换 WebSocket，不影响领域层 |
| Memory | **run event log + latest loop memory 两张主表** | 结构最少但可用 | 后续可拆 engineering memory / governance memory |
| Approval | **pending action 列表 + approve/reject API** | 足够证明 human-in-the-loop | 后续可扩展为完整 feedback hub |
| 预算 | **只做 run 级软上限记录，不做复杂治理** | 避免过早设计 | 后续可扩展到日预算、用户预算、项目预算 |
| Secret | **先复用当前用户 session access token** | 不引入 capability 系统 | 后续可升级为 proxy/capability 模型 |
| Instance 隔离 | **主进程内 async task**，不单独做容器/浏览器隔离 | MVP 不跑环境执行 | 后续仅在 environment executor 层新增隔离 |

## 5. 为什么 SQLite 而不是一开始上 Postgres

从长期架构看，Postgres 更适合正式调度和多实例运行；但 **MVP 阶段优先 SQLite** 更合适，因为：

- 当前目标是尽快看到闭环效果，不是验证高并发或 HA
- 本项目已使用 `sqlx`，SQLite 到 Postgres 的迁移成本可控
- MVP 只需要单实例 scheduler，不需要数据库抢占 lease

但要注意两条边界：

1. **不要把 SQLite 特性写死到领域逻辑里**
2. **schema 设计按未来 Postgres 使用方式来做**

也就是说，SQLite 只是 MVP 的落地介质，不是最终架构承诺。

## 6. 最小功能模块集合

MVP 只需要以下 8 个模块。

## 6.1 Loop Definition

最小字段：

- `id`
- `name`
- `gitlab_project_id`
- `gitlab_issue_iid`
- `schedule_interval_minutes`
- `prompt_preset`
- `enabled`

只支持一种 loop 类型：`issue_review`

不做通用 DSL，不做多 skill 组合。

## 6.2 Scheduler

只支持两种触发方式：

- manual trigger
- fixed interval

最小行为：

- 定时扫描启用的 loop
- 为符合条件的 loop 创建 run
- 防止同一个 loop 同时出现多个 running run

## 6.3 Run Executor

执行流程固定：

1. 读取 loop definition
2. 拉取 GitLab issue 数据
3. 读取上次 loop memory
4. 调用模型生成结构化结果
5. 写入 run events
6. 更新 latest loop memory
7. 如有草稿评论，创建 pending action

这就是 MVP 的“Loop Agent”。

## 6.4 AI Gateway Lite

这里只做一个很薄的版本：

- 统一模型调用入口
- 统一记录 token / cost（即使先估算也可以）
- 统一 prompt 模板入口

只支持一个 provider，一个模型 tier。

不要在业务代码里直接散落模型调用。

## 6.5 Loop Memory

MVP 只保留两类持久化信息：

- **run event log**：这次 run 发生了什么
- **latest loop memory**：系统当前对这个 issue 的最新理解

建议 `latest loop memory` 最少结构化存：

- summary
- changes_since_last_run
- risks
- suggested_next_step
- draft_comment
- updated_at

## 6.6 Pending Actions / Approval

只做一种待确认动作：

- `post_issue_comment`

用户可以：

- 查看草稿
- approve
- reject

approve 后才调用 GitLab comment API。

## 6.7 GitLab Adapter

MVP 只做最少接口：

- get issue detail
- list notes/comments
- create comment

不要提前做通用 GitLab API 平台层，只做当前场景真正需要的 3 个能力。

## 6.8 Workbench UI

MVP 只需要四个页面/区域：

1. **Loop 列表**
2. **Loop 创建表单**
3. **Run 详情页**
4. **Pending Action 审批区**

Run 详情页中需要让用户明显看到：

- 当前状态
- 当前 run 日志/事件流
- 最新 memory 摘要
- 草稿评论

这就是“最小可见效果”。

## 7. MVP 的一个最小数据模型

建议 MVP 先只建这些核心表：

- `loops`
- `loop_runs`
- `loop_run_events`
- `loop_memories`
- `pending_actions`

可选但建议预留：

- `model_usages`

其中关系尽量简单：

- 一个 `loop` 有多个 `loop_runs`
- 一个 `loop_run` 有多个 `loop_run_events`
- 一个 `loop` 只有一个最新 `loop_memory`
- 一个 `loop_run` 可以生成零个或多个 `pending_actions`

## 8. MVP 的最小状态集合

### Loop 状态

- `enabled`
- `disabled`

### Run 状态

- `queued`
- `running`
- `waiting_approval`
- `completed`
- `failed`

### Pending Action 状态

- `pending`
- `approved`
- `rejected`
- `executed`
- `execution_failed`

这已经足够支撑闭环。

## 9. 一条最关键的实现原则

MVP 可以简化实现，但 **不能破坏未来架构边界**。

因此必须坚持：

1. **模型调用必须经过统一 gateway**
2. **GitLab 调用必须经过 adapter**
3. **run 执行必须有 event log**
4. **memory 必须独立于 transcript**
5. **写 GitLab 前必须经过 pending action**

只要这五条守住，后面就算把 SQLite 换 Postgres、把内嵌 runtime 换独立 worker、把单 provider 换成多 provider，也不会推翻 MVP。

## 10. 这个 MVP 真正证明了什么

如果这个 MVP 做成，能证明的不是“模型能总结 issue”，而是：

1. `issueflow` 能把一个工作对象变成 **长期运行 loop**
2. 系统能把 **上次运行结果** 作为后续输入，而不是每次重新开始
3. 系统能把 **AI 输出停在待确认动作**，而不是直接外写
4. 用户能在 UI 中看到 **运行、记忆、建议、审批** 的完整闭环

这四点一旦成立，后面再加 schedule DSL、skills、verification、external agents，都是沿着已验证方向扩展，而不是重新赌产品方向。

## 11. 推荐开发顺序

建议按下面顺序实现，保证每一步都能看到效果：

1. `loops` / `loop_runs` / `loop_memories` 数据模型
2. GitLab 读取能力
3. 单次 manual run
4. AI Gateway Lite
5. run event log + Run 详情页
6. pending action + approve/reject
7. fixed interval scheduler
8. Loop 列表与创建页面

这样即使只完成前 5 步，也已经有一个可演示的“半 MVP”。

## 12. 人员安排

## 12.1 最小可行团队

如果目标是尽快做出一个可信 MVP，建议 **3 人核心团队 + 0.5 产品/设计支持**。

| 角色 | 人数 | 主要职责 |
| --- | --- | --- |
| Tech Lead / Backend | 1 | 数据模型、scheduler、run executor、AI gateway lite、整体架构把关 |
| Fullstack / Frontend | 1 | Loop 列表、创建页、run 详情页、pending action 审批 UI、SSE 接入 |
| Backend / Integration | 1 | GitLab adapter、approval 执行链路、认证会话、日志与错误处理 |
| Product/Design（兼职） | 0.5 | 场景收敛、信息结构、审批流和 run 页交互收口 |

这是最推荐的配置。

## 12.2 更极限的最小配置

如果资源非常紧，也可以 **2 个工程师先做第一版**：

| 角色 | 人数 | 主要职责 |
| --- | --- | --- |
| Engineer A | 1 | 后端主线：数据、runtime、scheduler、模型调用 |
| Engineer B | 1 | 前端 + GitLab 接入 + 审批闭环 |

但这种配置的风险是：

- UI 容易做得过于粗糙，影响“可见效果”
- GitLab 接入、前端、后端同时压在 2 人身上，节奏容易卡住

所以 2 人适合做 **技术验证版**，不适合做正式对外展示版。

## 12.3 不需要的人

MVP 阶段不需要专职：

- 专门的 ML engineer
- 专门的 infra/platform engineer
- 专门的 data engineer

因为当前难点不在模型训练，也不在大规模基础设施，而在 **产品闭环 + 架构边界**。

## 13. 建议的 4 周执行节奏

### Week 1

- 定义最小数据模型
- 打通 GitLab 读取
- 做 manual run

### Week 2

- 接入 AI Gateway Lite
- 落 run event log
- 做 Run 详情页

### Week 3

- 做 pending action 审批流
- 打通 GitLab comment 写入
- 补基础错误处理与状态流

### Week 4

- 加 fixed interval scheduler
- 完成 Loop 列表与创建页
- 打磨 demo 路径

## 14. MVP 完成标准

满足以下标准，就算 MVP 完成：

1. 用户能创建一个 `issue_review` loop
2. 用户能手动触发 run，也能按固定周期自动触发
3. run 会读取 issue + 上次 memory，并生成新的摘要和建议
4. 结果会持久化，并在 UI 中回看
5. 系统会产出待确认评论草稿
6. 用户批准后，评论能真正写入 GitLab
7. 默认不发生任何未经确认的 GitLab 写操作

## 15. 一句话结论

最值得做的 MVP 不是“接很多 agent”或“做复杂技能系统”，而是：

> **先做一个能围绕 GitLab issue 持续运行、会记住上次结果、会产出待确认动作、并能在 UI 中被看见的 loop。**

这是实现最简单、最能证明方向、同时又不会锁死后续架构的 MVP。
