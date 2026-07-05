# issueflow 技术选型调研清单

## 1. 文档目的

本文基于 `docs/DESIGN.md`，把其中已经明确为核心模块、但尚未落到具体实现方案的部分，整理成一组 **可独立决策的技术选型项**。

目标不是一次性定稿所有方案，而是：

- 先识别哪些决策彼此独立，可以分开推进
- 为每一项给出几种 **比较靠谱** 的候选方案
- 给出更符合当前项目阶段的初步建议
- 明确后续调研应该重点验证什么

## 2. 当前约束与判断前提

结合当前仓库现状，本文默认以下前提成立：

- 后端主栈是 **Rust + tokio + axum + sqlx**
- 前端主栈是 **Vue 3 + Naive UI + Vite**
- 已经有 **AG-UI / A2UI** 相关 crate 和前端依赖
- 项目目标是构建 **控制平面 + loop runtime**，不是再做一个通用 IDE
- 外部 agent、浏览器执行、环境操作都必须受 **预算、权限、审批、可审计性** 约束

因此，本文会优先推荐：

- 能贴合现有 Rust/Axum/SQLx 技术栈的方案
- 能先做出 MVP，再逐步演进的方案
- 能把“治理、权限、审计”当一等公民的方案

## 3. 建议优先决策顺序

如果要排决策先后，建议顺序如下：

1. 调度引擎
2. 状态机实现
3. Loop Agent runtime 形态
4. 记忆与持久化模型
5. AI Gateway 抽象
6. Session / Instance 隔离模型
7. 验证 pipeline
8. 外部 agent 接入层
9. Secret / 环境授权模型
10. 前端运行时传输
11. Skill registry
12. 观测与治理数据面

原因是前六项会直接锁定后续数据模型、部署方式和代码边界。

## 4. 技术选型逐项调研

## 4.1 调度引擎

**对应设计文档**：`Schedule Definition`、`Loop 核心运行模型`、`Phase 1`

调度在 `issueflow` 里不是普通 cron，而是要同时支持：

- 固定周期
- 时间窗
- 外部事件触发
- 条件继续
- retry / backoff
- 可审计和可回放

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. 基于 Postgres 的应用内调度器 | 用数据库保存 schedule、next_run_at、lease、retry 状态；由 tokio worker 拉取执行 | 贴合现有栈；易审计；易和 loop state 对齐；部署简单 | 需要自己处理 lease、抢占、misfire、幂等 |
| B. 引入独立工作流/调度引擎 | 例如 Temporal / Quartz 类思路，外部系统负责调度与恢复 | 调度语义强；容错和恢复能力成熟 | 系统复杂度显著上升；模型和权限逻辑容易分散 |
| C. Redis/消息队列驱动调度 | 用队列延迟任务、事件 fanout、worker 消费 | 事件吞吐高；适合高并发异步任务 | 需要额外基础设施；审计、状态机、回放整合更复杂 |

### 初步建议

**优先选 A：基于 Postgres 的应用内调度器**，并保留未来抽象成独立 runtime 的空间。

理由：

- 当前项目已经使用 `sqlx`，数据库天然适合承载 loop、run、budget、feedback 等核心对象
- `issueflow` 的关键难点不是“超大规模任务调度”，而是 **调度和治理状态的一致性**
- schedule 与 state machine、memory、approval 强耦合，先放在同一事务边界更稳

### 调研方向

1. 如何实现 **多实例下的 lease / 抢占 / failover**
2. 如何定义 **misfire** 语义：错过时间点后是补跑、跳过还是合并
3. 如何统一时间触发、事件触发、条件继续三类入口
4. 如何实现 `retry_cap`、指数退避、抖动和 dead-letter
5. 如何保证同一 loop 在任意时刻只有一个有效 run

## 4.2 状态机实现

**对应设计文档**：`State Machine`、`Loop 定义方式`

这里至少有两层状态机：

- system state machine
- user state machine

它们既要可治理，也要允许后续配置化。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. Rust 枚举 + 显式转移表 | 状态是强类型枚举，允许的边和 guard 在 Rust 中定义 | 类型安全强；实现清晰；便于单测 | 初期不够灵活；变更需要发版 |
| B. 表驱动状态机 | 状态、事件、guard、action 持久化到数据库或配置文件 | 可配置；适合未来给不同 loop 定制 | 实现复杂；调试成本更高；类型约束弱 |
| C. 外部状态机 DSL/库 | 引入专门状态机框架或 DSL | 表达力强；可视化潜力高 | 容易过度设计；要适配现有领域模型 |

### 初步建议

**Phase 1 用 A，Phase 2 往 A+B 混合演进。**

更具体地说：

- 核心系统状态先用 Rust 枚举和显式转移表实现
- 状态机定义对象可以持久化，但真正允许的转移仍由后端校验
- 等模型稳定后，再把可配置的 guard / policy 外提

这样可以避免一开始把“状态机平台化”做得太重。

### 调研方向

1. 状态转移是否需要区分 `event`、`command`、`side effect`
2. guard 的执行边界放在哪里：状态机层、policy 层还是 runtime 层
3. 用户级状态是否允许租户/项目自定义
4. 是否需要把状态机版本化，并与 loop definition 绑定
5. 是否需要支持人工修复状态和审计追踪

## 4.3 Loop Agent runtime 形态

**对应设计文档**：`轻量级 Loop Agent`、`当前推荐实现分层`

Loop Agent 是系统核心，但它到底和主服务是什么关系，需要尽快定。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. 嵌入主服务进程 | scheduler、runtime、API 都在同一个后端服务里 | 实现最快；本地开发最简单；事务边界自然 | 长任务会影响 API 服务；扩展性一般 |
| B. 同仓独立 worker 进程 | control plane 提供 API，worker 进程跑 loop runtime | 结构更清晰；便于横向扩展；资源隔离更好 | 部署和运维复杂度上升 |
| C. 完全托管到外部工作流系统 | runtime 逻辑放到外部 orchestration 平台 | 可利用成熟调度/恢复能力 | 领域逻辑被拆散；治理边界难收拢 |

### 初步建议

**短期 A，结构上为 B 预留边界。**

具体建议：

- 当前先在单后端中实现 runtime 核心
- 代码层面提前拆出 `scheduler`、`run executor`、`memory coordinator`、`verification coordinator`
- 数据模型和服务接口按“未来可拆成 worker”设计
- Loop 控制 agent 本体不要放进 Wasm runtime；它更适合做长期驻留的 Rust 服务端 orchestrator
- 如果后续需要受限、可替换、可版本化的 skill 执行层，可在宿主服务中引入 **Wasmtime/WASI** 作为 `skill host`

这里建议明确区分两层：

- **Loop Core**：服务端 Rust runtime，负责调度、状态机、memory、approval、外部 agent 协调
- **Skill Host**：服务端 Wasmtime/WASI 宿主，负责装载和执行受限 skill

这类分层比“直接用 Wasmtime 跑 Loop Agent 本体”更合理，因为控制 agent 的主要复杂度在长生命周期编排、流式通信、持久化与权限治理，而不是受限计算执行。

### 调研方向

1. API 请求线程与长运行 loop 之间如何隔离
2. 是否需要单独的内部任务队列接口
3. run 执行失败后的恢复，是原地 resume 还是重新拉起新 instance
4. 单机和多机部署时的所有权模型怎么定义
5. runtime 是否需要专用资源配额

## 4.4 外部 agent 接入层

**对应设计文档**：`A2A Adapter Layer`、`Phase 3`

外部 agent 是可替换执行器，因此接入层必须抽象得足够稳定。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. 统一 provider trait + adapter 插件 | 统一抽象任务提交、流式输出、取消、成本、能力协商 | 最贴合当前架构；可逐个接 provider | 需要自己定义事实标准 |
| B. 直接对每个 provider 单独集成 | OpenCode、Codex、Hermes 各自写一套调用逻辑 | 起步快；适合验证单个 provider | 很快变得难维护；平台边界被侵蚀 |
| C. 强依赖某个通用协议或网关 | 把 provider 接入主要交给外部协议层 | 理论上可复用生态 | 一旦协议能力不够，平台会被反向限制 |

### 初步建议

**优先选 A。**

推荐内部统一抽象至少包含：

- task submission
- status / heartbeat
- stream chunks
- structured artifacts
- cancel
- cost report
- capability negotiation

补充一个边界约束：

- 外部重型 agent 的通信、心跳、取消、成本归集应由宿主 Rust adapter 负责，而不是放进 Wasm skill 中执行

### 调研方向

1. 各类外部 agent 是否都支持流式状态回传
2. 成本、token、wall time 能否统一建模
3. 外部 agent 的“可中断”和“可恢复”语义是否一致
4. 输入上下文裁剪规则如何统一
5. 是否要区分同步工具调用型 adapter 和长任务型 adapter

## 4.5 前端运行时传输

**对应设计文档**：`AG-UI`、`A2UI`、`实时可视化、Break 与 Steering`

这里要解决的是用户如何实时看到 loop 运行，并及时下发 break / steer / takeover。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. SSE + REST 命令通道 | AG-UI 用 SSE 下行推流，控制命令走 HTTP API，A2UI 事件嵌在流中 | 和 axum 很匹配；前端简单；适合服务器推送 | 双通道一致性需要设计 |
| B. WebSocket 全双工 | 所有消息统一走一个长连接 | 实时性强；命令/状态统一 | 连接管理、鉴权、恢复更复杂 |
| C. GraphQL subscription 或 gRPC-web | 以更强 schema 管理实时流 | 规范性好 | 当前栈不自然；接入成本偏高 |

### 初步建议

**优先选 A。**

原因：

- 现有栈已经适合做流式 HTTP
- 对 `issueflow` 来说，状态流通常远多于用户实时指令
- `break`、`pause`、`steer` 等上行命令并不需要高频双工

### 调研方向

1. SSE 断线重连后的 replay 语义如何定义
2. A2UI surface 的版本与幂等更新如何建模
3. 命令执行结果是否需要回写到同一事件流
4. 多标签页或多人观察同一 run 时怎么同步
5. 前端是否需要本地 event store 做回放

## 4.6 记忆与持久化模型

**对应设计文档**：`Memory`、`记忆层设计`

这项决策会直接影响几乎所有模块的数据结构。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. 单一 Postgres 为主，事件 + 投影视图 | event log 追加写入，latest memory / pending actions 等做投影表 | 一致性强；事务友好；最适合审计 | 复杂查询和全文检索能力有限 |
| B. Postgres + 搜索/向量存储双层 | 结构化状态在 Postgres，检索索引在 OpenSearch / pgvector / 向量库 | 兼顾治理和检索 | 系统复杂度提升，需要同步策略 |
| C. 文档数据库为主 | 把 memory 主要放进文档型存储 | 模式灵活 | 对事务、审计、强约束不友好 |

### 初步建议

**优先选 A，预留向 B 演进。**

推荐落法：

- `agent_run_events` 做 append-only event log
- `loop_memory` / `engineering_memory` / `pending_actions` 做结构化投影
- 对“最新理解”与“待确认动作”做明确 schema，而不是只存长文本

是否引入向量检索，建议等真实检索场景稳定后再决定。

### 调研方向

1. 哪些对象必须是强 schema，哪些可以 JSONB 承载
2. replay 时的事实来源是 event log 还是 snapshot
3. memory 写入是否要求事务性地和 run 状态更新绑定
4. `engineering_memory` 是否要按 issue/MR/project 多级索引
5. 搜索需求是否真的需要向量检索，还是全文 + 标签就够

## 4.7 AI Gateway

**对应设计文档**：`AI 网关与预算控制`

AI Gateway 不是普通模型客户端，而是平台级治理中枢。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. 应用内 gateway 模块 | provider adapter、budget、policy、secret access 都在后端内部 | 边界清晰；实现快；利于一体化治理 | 后端模块会比较重 |
| B. 独立 AI Gateway 服务 | 单独服务负责所有模型路由和策略执行 | 可供多个系统复用；扩展性更强 | 运维复杂；本项目现阶段偏重 |
| C. 依赖第三方 AI proxy | 把路由和观测外包给第三方网关 | 起步快 | 权限、审计、细粒度策略受制于外部能力 |

### 初步建议

**优先选 A。**

建议 AI Gateway 内部至少包含：

- provider registry
- model tier mapping
- budget ledger hooks
- policy evaluation hooks
- secret access broker
- request / response audit envelope

### 调研方向

1. 模型路由规则写死在代码还是做配置化
2. 不同 provider 的 token / cost 归一化怎么做
3. fallback 是否允许跨 provider、跨模型族
4. evaluator 是否强制与 generator 分离
5. 网关在什么粒度记录审计：prompt、摘要、token、artifact、tool use

## 4.8 验证 / Evaluator Pipeline

**对应设计文档**：`Verification`、`验证、债务与治理`

这里决定的是平台如何防止“只会生成，不会验证”。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. 策略驱动的多阶段验证流水线 | 先跑确定性检查，再跑 evaluator，再决定是否需要人工确认 | 最符合设计目标；可治理；可分层成本 | 编排稍复杂 |
| B. 只做同模型 self-check | 生成后让同一模型自评 | 实现最简单 | 可靠性弱；容易形成自证闭环 |
| C. 全依赖外部 CI / 浏览器结果 | 模型只负责生成，验证交给外部测试系统 | 对代码改动很有效 | 对规划、风险判断、文本结论类问题覆盖不足 |

### 初步建议

**优先选 A，并把 C 作为其中一类 verifier。**

建议默认顺序：

1. policy / permission checks
2. deterministic checks
3. independent evaluator
4. human approval if needed

### 调研方向

1. evaluator 是否必须和 generator 使用不同模型或不同温度配置
2. 验证失败如何回馈到 loop：重试、降级、转人工还是冻结
3. 如何记录验证债务，并让它真正影响后续调度
4. 不同 skill 的默认验证要求如何声明
5. 浏览器验证、测试验证、文本评审如何统一结果模型

## 4.9 Session / Instance 隔离模型

**对应设计文档**：`Session / Instance 生命周期管理`

这项直接关系到安全、资源控制和后续环境执行能力。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. 进程级隔离 + 临时目录/worktree | 每个 instance 对应独立进程和临时工作目录 | MVP 快；本地调试方便 | 安全隔离一般；浏览器和密钥边界较弱 |
| B. 容器级隔离 | 每个 instance 在容器内运行，挂载受控 workspace 和 secret | 安全性与资源隔离更均衡 | 需要容器编排和镜像管理 |
| C. 微 VM / 强隔离沙箱 | Firecracker 这类强隔离环境 | 安全性最好 | 复杂且重，不适合早期 |

### 初步建议

**MVP 从 A 起步，涉及浏览器与环境操作时尽快进入 B。**

原因是：

- 文档明确提到 browser context、temp workspace、secret detachment
- 一旦要做“AI 搭环境 + 浏览器验证”，进程级隔离很快不够稳

### 调研方向

1. instance 的最小资源单位是什么：进程、容器还是浏览器上下文
2. worktree 生命周期和 run 生命周期是否一一对应
3. 浏览器上下文是否需要和代码执行环境绑定
4. orphan cleanup、secret detachment 如何保证最终一致
5. 是否需要针对高风险操作提升隔离级别

## 4.10 Secret / 环境授权模型

**对应设计文档**：`环境密钥管理与环境技能`、`安全与权限边界`

这里不是简单“把密钥注入环境变量”，而是要做最小能力暴露。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. Gateway 签发短期 capability | agent 只拿到有 scope/time/operation 限制的短期能力 | 最符合设计目标；审计清晰 | 设计和实现都较复杂 |
| B. 从 Vault/Secret Manager 临时注入环境变量 | 运行时注入短期凭证或静态密钥 | 实现相对直接 | agent 仍然可能读取原始凭证 |
| C. Proxy 代调用模型 | agent 不拿密钥，只调用受控代理，由代理实际访问外部系统 | 最小暴露最好 | 对每类外部系统都要做代理接口 |

### 初步建议

**推荐 A+C 组合。**

具体理解：

- 对通用模型调用、浏览器权限、环境工具能力，用 A
- 对 GitLab 写操作、高风险外部写入，优先用 C，把真正的外部调用封装在平台代理内

### 调研方向

1. capability 的最小粒度如何定义：资源、操作、时间、额度
2. agent 能否接触到原始 token，还是必须完全代理化
3. capability 如何吊销、过期、续租
4. 日志里保留哪些审计字段而不泄露敏感信息
5. GitLab OAuth session、PAT、环境 secret 三者如何统一抽象

## 4.11 Skill Registry

**对应设计文档**：`Skill 注册中心与版本管理`、`Skill 进化机制`

skill 既是能力插件，也是组织记忆资产，所以注册中心的形态很关键。

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. Git 中保存 skill 源文件，数据库做索引与发布态 | skill 内容、元数据、diff 在 Git；运行态信息在 DB | 审计天然好；适合版本 diff；贴合工程实践 | 需要处理 Git 与 DB 同步 |
| B. 全 DB 原生 registry | skill 内容、版本、依赖都存在数据库 | 查询方便；统一管理 | 对文本 diff、代码评审、回滚体验较弱 |
| C. OCI/package registry 化 | 把 skill 做成可发布制品 | 发布流程规范化强 | 对当前阶段过重 |

### 初步建议

**优先选 A。**

推荐方式：

- skill 源内容放在 Git 管理
- registry 数据库存放版本索引、兼容性、状态、灰度信息
- 运行时加载的是“已发布版本”，不是任意工作区文件

### 调研方向

1. skill manifest 最小 schema 怎么定
2. skill 的输入输出契约用 JSON Schema、Rust struct 还是自定义定义法
3. version graph 是否允许分支和灰度通道
4. 不同 scope 的 skill 冲突时如何解析优先级
5. skill evolution proposal 如何进入审查与发布流程

## 4.12 观测与治理数据面

**对应设计文档**：`统一反馈与人工确认系统`、`记忆层设计`、`验证、债务与治理`

这里要解决的是两个不同层次的问题：

- 运维观测：系统有没有出错、慢不慢、耗不耗资源
- 治理观测：loop 有没有失控、预算有没有偏、用户有没有只点通过

### 靠谱选项

| 选项 | 方案说明 | 优点 | 风险/代价 |
| --- | --- | --- | --- |
| A. Postgres 审计表 + OpenTelemetry | 产品级事件进数据库，系统级指标/trace 走 OTel | 结构清晰；兼顾审计和运维 | 需要两套观测面 |
| B. 全部自定义 event log | 所有内容只写统一事件表 | 实现统一 | 运维查询和指标分析会比较吃力 |
| C. 事件进分析型存储 | 例如 ClickHouse/BigQuery 一类分析仓 | 查询和报表强 | 早期成本高，治理规则反而不容易先落地 |

### 初步建议

**优先选 A。**

推荐拆法：

- Postgres：feedback、approval、budget ledger、verification debt、governance reports
- OTel：request traces、runtime spans、provider latency、error rate、queue lag

### 调研方向

1. 哪些事件需要 append-only 审计，不允许更新覆盖
2. 哪些治理指标要实时计算，哪些可以离线汇总
3. 如何建立 run、loop、user、project 四个维度的关联查询
4. 成本观测是否要精确到 provider / model / skill / verifier
5. 认知退化、验证债务这类指标如何从事件中推导

## 5. 一组比较稳妥的默认组合

如果目标是先做出一版可运行、可治理、且不容易返工的实现，当前最稳妥的默认组合是：

| 主题 | 建议默认方案 |
| --- | --- |
| 调度引擎 | Postgres-backed 应用内调度器 |
| 状态机 | Rust 强类型状态 + 显式转移表 |
| Loop runtime | 先嵌入主服务，边界按未来 worker 化设计 |
| 外部 agent 接入 | 内部统一 provider trait + adapter |
| 前端传输 | SSE 下行 + REST 上行 |
| 记忆层 | Postgres event log + projection |
| AI Gateway | 应用内 gateway 模块 |
| 验证 pipeline | policy + deterministic + independent evaluator + human approval |
| Instance 隔离 | MVP 进程级，环境执行升级到容器级 |
| Secret 模型 | capability + proxy 组合 |
| Skill registry | Git 为源、DB 为索引和发布态 |
| 观测治理 | Postgres 审计 + OpenTelemetry |

## 6. 最值得先做的调研 Spike

为了尽快降低架构不确定性，建议先做以下 6 个 spike：

1. **调度与 lease spike**：验证 Postgres 下多实例抢占、补跑、retry 是否可控
2. **状态机骨架 spike**：验证 system/user state machine 的 Rust 实现边界
3. **run event store spike**：验证 event log + projection 的数据模型是否顺手
4. **SSE runtime spike**：验证 AG-UI 流、A2UI surface、break/steer 命令闭环
5. **provider adapter spike**：先接一个外部 agent，跑通 submit/stream/cancel/cost
6. **instance isolation spike**：验证代码执行 + 浏览器验证的最小隔离单元

## 7. 总结

从当前设计看，`issueflow` 最关键的技术选型不是“选哪个最强模型”，而是：

- 怎么把 **调度、状态机、记忆、验证、预算、权限** 放进同一个可治理系统
- 怎么让外部 agent 保持可替换，而平台核心不被 provider 反向绑定
- 怎么先做出 **足够稳的 MVP**，再把高复杂度能力逐步外扩

因此，整体上应优先选择 **贴合现有 Rust + SQLx + Axum 栈、强调审计和边界、支持逐步演进** 的方案，而不是过早引入重量级分布式基础设施。
