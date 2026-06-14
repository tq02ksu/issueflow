# issueflow

`issueflow` 是一个面向组织级研发协作的 `issue / requirement orchestration` 平台。它以 `Anthropic SKILLS` 为一等公民，用双层 `skill repo` 模型沉淀组织能力：`<platform-skill-repo>` 管系统级 skills 与规则，`<project-skill-repo>` 管单个软件系统的需求、文档、demo 与交付协作；平台核心价值是把需求进入、分诊、补信息、方案确认和开发启动做成受控、可审计、可复用的闭环。

项目并未将单一代码托管平台或 CI 平台固化为永久约束。当前主要支持路径是 `GitLab + OpenCode`，其中 `GitLab CI` 是当前主执行平面。

## 项目背景

AI coding agent 已经可以显著提高研发效率，但在团队和企业场景中仍然存在明显门槛。在一些场景下，高水平个人工作流可能比平台化流程更高效。

但当组织希望把这类能力用于跨角色协作、流程治理和持续交付时，关注点就不再只是个人效率本身，还会扩展到权限边界、经验沉淀、结果审计、成本归属与流程稳定性。

`issueflow` 的目标不是否定超级个体或个人最佳实践，而是为组织场景补充一层可控、可复用、可审计的研发自动化基础设施，让优秀实践更容易沉淀并在团队内复用。

- 使用门槛高，效果高度依赖个人提示词、操作习惯和熟练度。
- 个人使用经验难以沉淀为组织可复用的资产，能力容易停留在个人层面。
- 企业统一付费后，也很难保证工具只被用于工作相关内容，缺少边界与治理能力。

## 战略定位

`issueflow` 是一个以 `Anthropic SKILLS` 为一等公民的 `issue / requirement orchestration` 平台。它不是把 agent 当成单次代码生成器，而是把 `SKILL`、`Issue`、`Git` 和多仓库交付流程组织成一套可持续演进的系统，并把需求治理放在比代码生成更靠前的位置。

### 核心理念

- **需求先于代码**：平台先解决需求进入、分诊、补信息、方案确认和开发启动，再进入实现与交付。
- **SKILL 是组织资产**：`SKILL` 承载方法、规范、模板、约束和执行上下文，应该被审查、版本化和复用。
- **Git 是知识系统**：Git 不只是代码仓库，也是 `SKILL`、需求文档、demo、规则和历史的管理系统。
- **双层 Skill Repo**：`<platform-skill-repo>` 管系统级能力，`<project-skill-repo>` 管单个软件系统的需求与交付上下文。
- **Gateway 是控制面**：平台用状态机、权限边界和确认点控制 agent 何时能推进到开发和写操作。
- **Code Is Cheap**：代码、demo、文档和工作流都应快速迭代，持续逼近更好的产品想法和交付方式。

### 项目结构模型

平台推荐采用“一个平台级 `skill repo` + 多个项目级 `skill repo` + 多个执行仓库”的结构：

```text
┌──────────────────────────────────────────────────────────────────┐
│  <platform-skill-repo>                                           │
│  ├─ skills/                 # 平台级通用 Anthropic SKILLS       │
│  ├─ policies/               # 系统级规则、权限、编排协议        │
│  ├─ templates/              # 通用模板与脚手架                  │
│  └─ docs/                   # 平台级设计与使用文档              │
└──────────────────────────────────────────────────────────────────┘
                               │ 继承 / 复用
                               ▼
┌──────────────────────────────────────────────────────────────────┐
│  <project-skill-repo>                                            │
│  ├─ issues/                 # 单个软件系统的 issue 与上下文      │
│  ├─ skills/                 # 该项目沉淀和覆盖的 Anthropic       │
│  │                          # SKILLS                             │
│  ├─ docs/                   # 架构、SOP、仓库说明                │
│  ├─ repos/                  # 其它代码仓库在哪里、职责是什么     │
│  ├─ demos/ui/               # 持续演进的 UI demo                │
│  └─ README.md               # 项目导航与协作约定                 │
└──────────────────────────────────────────────────────────────────┘
                               │
                               │ 编排 / 同步
                               ▼
┌──────────────────────────────┐  ┌──────────────────────────────┐
│  app-repo-a                  │  │  app-repo-b                  │
│  ├─ src/                     │  │  ├─ src/                     │
│  ├─ .gitlab-ci.yml           │  │  ├─ .gitlab-ci.yml           │
│  └─ README.md                │  │  └─ README.md                │
└──────────────────────────────┘  └──────────────────────────────┘
```

### 工作流程

1. 平台先从 `<platform-skill-repo>` 提供系统级 skills、规则和模板。
2. 用户在项目级 `<project-skill-repo>` 中创建或推进单个软件系统的 Issue。
3. 平台基于系统级能力、项目上下文和项目沉淀的 `SKILL` 选择或组合执行路径。
4. Agent 加载对应 `SKILL`，在目标代码仓库或文档仓库中执行任务。
5. 代码变更、验证结果、MR/PR 状态和设计产物回写到 `<project-skill-repo>`。
6. 平台与项目通过持续更新的文档、UI demo 和 `SKILL`，把一次性交付转化为长期资产。

### 当前落地与演进

| 维度 | 说明 |
| --- | --- |
| **平台核心** | 以 `SKILL` 为中心的 Agent 编排、Issue 推进与多仓库协同 |
| **技能载体** | 优先使用 Git 管理平台级与项目级 `SKILL`、规则和历史 |
| **平台主仓** | `<platform-skill-repo>` 承载系统级 skills、规则与模板 |
| **项目主仓** | `<project-skill-repo>` 作为单个软件系统的协调层和知识主仓 |
| **当前落地** | 当前主要支持路径是 `GitLab + OpenCode`，由 `GitLab CI` 承担主要执行任务 |
| **演进方向** | 在不绑定单一平台的前提下，逐步抽象出更通用的 `SKILL` 编排协议与执行模型 |

## 核心特性

1. **需求编排闭环**：覆盖 issue 接入、分诊、补信息、方案确认、开发启动、MR/PR 跟踪和结果回写。
2. **双层 Skill Repo**：`<platform-skill-repo>` 沉淀系统级 skills、规则和模板；`<project-skill-repo>` 沉淀单个软件系统的需求、文档、demo、仓库地图和项目级 skills。
3. **受控开发入口**：通过状态机、权限边界和人工确认点控制何时可以进入开发、创建 MR/PR 或触发高风险动作。
4. **多仓库项目上下文**：项目级仓库维护相关代码仓库的位置、职责和状态，让跨仓库需求有统一入口。
5. **可演进产品资产**：UI demo、需求文档、架构说明和成熟做法持续回写到项目级仓库，减少一次性产物。
6. **当前执行路径**：当前主要支持 `GitLab + OpenCode + GitLab CI`，用于先跑通需求到交付的受控闭环。

## 产品目标

`issueflow` 的目标是让用户选择或定制由 `SKILL + workflow + guardrails` 组成的 `Agent Definition`，在 Gateway 控制下完成 issue 分诊、需求整理、方案设计、开发启动、MR 提交与结果回写，并把用户实践持续沉淀为可审查、可版本化、可复用的 `SKILL`。

### Agent Definition

`Agent Definition` 不是一个无限自治的机器人，而是一组受控能力定义：

- **SKILL 集合**：agent 可以加载哪些平台级或项目级 `SKILL`。
- **目标**：agent 被允许完成的业务目标，例如 issue triage、需求整理、方案设计、开发实现、MR 准备。
- **工作流**：agent 必须遵循的阶段、输入输出和确认点。
- **权限边界**：agent 可以读取或请求操作哪些外部系统能力。
- **状态机与 guardrails**：Gateway 用于判断当前阶段是否允许继续推进或执行写操作。

用户可以直接选择系统提供的 `Agent Definition`，例如：

- `issue-triage-agent`：整理 issue、分类、识别缺失信息、建议 label。
- `requirement-design-agent`：把 issue 转成需求说明、验收标准、UI demo 任务。
- `dev-agent`：在开发准入后执行实现、准备分支和 MR。
- `review-agent`：检查 MR、总结风险、生成 review 建议。

### 用户自定义 SKILL

用户可以在 `<project-skill-repo>` 中定义项目级 `SKILL`，定制 agent 的做事方式：

- 如何理解需求和业务语言。
- 如何拆分任务和判断影响仓库。
- 如何更新需求文档、UI demo 和验收标准。
- 如何写代码、跑验证、准备 MR。
- 如何生成总结、风险说明和后续建议。

平台级 `<platform-skill-repo>` 提供默认能力、规则和 guardrails；项目级 `<project-skill-repo>` 负责覆盖、补充和沉淀单个软件系统的具体做法。

### SKILL 演进

系统可以根据用户行为生成 `SKILL` 升级建议，但不应静默修改核心 `SKILL`。

推荐机制是：

1. 平台观察用户如何补充需求、修改方案、拒绝建议、调整 MR。
2. 系统生成 `skill improvement proposal`。
3. proposal 以 MR 形式进入 `<project-skill-repo>` 或 `<platform-skill-repo>`。
4. 人审核后合并，`SKILL` 才正式升级。

这样可以让 `SKILL` 持续进化，同时保留 Git 的 review、diff、历史和回滚能力。

## 架构总览

```mermaid
C4Context
    title issueflow System Context

    Person(user, "平台用户", "产品、设计、研发、测试等角色")
    System(platform, "issueflow Platform", "SKILL 编排、Issue 推进、权限控制、状态跟踪")
    SystemDb(platformSkillRepo, "Platform Skill Repo", "平台级 SKILL、规则、模板")
    SystemDb(projectSkillRepo, "Project Skill Repo", "单个软件系统的 Issue、SKILL、文档、仓库地图、UI demo")
    SystemDb(codeRepos, "Code Repositories", "一个或多个业务代码仓库")
    System(runtime, "Agent Runtime", "加载 SKILL 并在目标仓库执行任务")

    Rel(user, projectSkillRepo, "创建 Issue、维护文档、查看 demo")
    Rel(user, platform, "确认任务、查看状态")
    Rel(platform, platformSkillRepo, "读取平台级 SKILL、规则与模板")
    Rel(platform, projectSkillRepo, "读取/更新 Issue、项目 SKILL 与项目文档")
    Rel(platform, runtime, "下发编排任务")
    Rel(runtime, platformSkillRepo, "加载平台级 SKILL 和规则")
    Rel(runtime, projectSkillRepo, "加载项目 SKILL 和上下文")
    Rel(runtime, codeRepos, "在目标仓库执行开发/验证/文档任务")
    Rel(platform, codeRepos, "同步 MR/PR、校验与交付状态")
```

- `平台用户`：包括产品、设计、研发、测试等角色。
- `Platform Skill Repo`：平台级知识主仓，统一承载系统级 `SKILL`、规则、模板和编排约束。
- `Project Skill Repo`：单个软件系统的知识主仓，统一承载 issue、项目 `SKILL`、规则文档、仓库地图和持续演进的 UI demo。
- `Code Repositories`：被编排的业务代码仓库，可以是单仓也可以是多仓。
- `issueflow Platform`：负责技能匹配、任务编排、状态跟踪、权限控制和结果聚合。
- `Agent Runtime`：同时加载平台级与项目级 `SKILL` 和上下文，在目标仓库中执行具体任务。

## 当前定位

- 平台核心定位：**以 `Anthropic SKILLS` 为一等公民的 `issue / requirement orchestration` 平台**
- Git 定位：**`SKILL` 的存储、版本与历史管理系统**
- Skill Repo 模型：**`<platform-skill-repo>` 管系统级能力，`<project-skill-repo>` 管单个软件系统**
- 当前主要支持路径：`GitLab + OpenCode`
- `GitLab CI` 是当前主要执行平面
- `Robot Gateway` 使用 Rust 实现，负责受控工作流入口与状态管理
- Gateway 已预留 GitLab OAuth2 登录与回调入口，回调路径为 `/auth/gitlab/callback`
- Gateway 页面保持轻量服务端渲染
- 持久化在生产环境使用 `PostgreSQL`，默认集成测试流程使用嵌入式 `SQLite`

## OAuth2 回调地址

当前 GitLab OAuth2 入口由 Gateway 承载：

- 登录入口：`GET /auth/gitlab/login`
- 回调入口：`GET /auth/gitlab/callback`
- GitLab 应用中配置的 Redirect URI：`<gateway-base-url>/auth/gitlab/callback`

本地开发示例：

```text
http://localhost:3000/auth/gitlab/callback
```

启用 GitLab OAuth2 时需要配置：

- `GITLAB_OAUTH_CLIENT_ID`
- `GITLAB_OAUTH_CLIENT_SECRET`
- `GITLAB_OAUTH_REDIRECT_URI`
- `OAUTH_STATE_SIGNING_SECRET`

可选配置：

- `GITLAB_OAUTH_AUTHORIZE_URL`
- `GITLAB_OAUTH_TOKEN_URL`
- `GITLAB_OAUTH_SCOPES`

## 典型使用场景

### 场景 1：多仓库功能迭代

```text
1. 产品在 <project-skill-repo> 提交 Issue："重做组织设置页"
2. 平台先从 <platform-skill-repo> 读取通用 skills 与模板，再叠加该项目沉淀的 skills：product-discovery、ui-demo、frontend-implementation、backend-api
3. Agent 先更新 docs 与 demos/ui，帮助对齐方案
4. 平台再将任务分发到 frontend-repo 与 backend-repo
5. 各仓库独立提交 MR，状态回传到 <project-skill-repo>
6. 项目级仓库持续沉淀更新后的 skill、文档和 demo
```

### 场景 2：项目治理与知识沉淀

```text
1. 团队在 <project-skill-repo> 维护仓库地图，说明所有相关代码库的位置和职责
2. 设计者持续迭代 demos/ui，验证复杂交互与信息结构
3. 研发把成熟做法沉淀为 skills，供后续 agent 复用
4. 新需求进入后，平台优先复用已有 skills，而不是每次从零构造流程
```

## 仓库结构

### 本仓库 (issueflow)

`issueflow` 是平台核心代码仓库，包含：

- `src/`：Rust Gateway 应用代码
- `tests/`：Rust 集成测试
- `internal/pages/templates/`：轻量 Gateway HTML 模板
- `scripts/robot/integrations/gitlab-ci/`：GitLab CI 集成模板、任务包装脚本与使用文档

### 平台级 Skill Repo

平台推荐维护一个平台级 `skill repo` 作为系统级能力主仓，典型结构：

```text
<platform-skill-repo>/
├─ skills/                  # 平台级通用 Anthropic SKILLS
├─ policies/                # 权限、状态机、编排规则
├─ templates/               # 通用模板与脚手架
└─ docs/                    # 平台设计与使用文档
```

### 项目级 Skill Repo

单个软件系统推荐使用一个项目级 `skill repo` 作为主协调仓，典型结构：

```text
<project-skill-repo>/
├─ issues/                  # 单个软件系统的 issue 与补充上下文
├─ skills/                  # 项目级 Anthropic SKILLS
│  ├─ product-discovery/
│  │  └─ SKILL.md
│  ├─ ui-demo/
│  │  └─ SKILL.md
│  └─ implementation/
│     └─ SKILL.md
├─ docs/                    # 架构、SOP、规范、仓库地图
├─ repos/                   # 其它代码仓库的位置与说明
├─ demos/
│  └─ ui/                   # 持续维护的 UI demo
└─ README.md                # 项目入口说明
```

### 业务代码仓库

具体的业务代码仓库保持独立管理、独立发布：

```text
<app-repo>/
├─ src/
├─ .gitlab-ci.yml
└─ README.md
```

## 相关文档

- 设计说明：`docs/DESIGN.md`
- GitLab CI 集成：`scripts/robot/integrations/gitlab-ci/README.md`

## Frontend Development

仓库现在包含一个位于 `web/` 的 Vue Workbench skeleton。

安装前端依赖：

```bash
cd web
npm install
```

启动前端开发服务器：

```bash
cd web
npm run dev
```

Vite dev server 会把 `/auth/*`、`/api/*` 和 `/internal/*` 代理到运行在 `http://127.0.0.1:3000` 的 Rust Gateway。

构建供 Gateway 集成使用的前端产物：

```bash
cd web
npm run build
```

运行前端检查：

```bash
cd web
npm run lint
npm run test
```
