# issueflow

与 `Codex`、`OpenCode` 等 AI Coding 工具配合，用于推进项目执行的 GitLab 工作流项目。

## 它是什么

`issueflow` 是一个面向 AI Coding 工作流的 artifact advancement system。
它关注的是如何通过显式状态、结构化 memory、`skills` 和 agent 协同，把项目里的 artifact 持续往前推进。

它围绕 GitLab 中的 `issue`、`milestone`、`label`、`merge request` 等对象工作，目标是把一个模糊请求逐步推进成可计划、可执行、可交付的项目对象。

## 它解决什么问题

很多 AI Coding 工作流的问题不在“不会写代码”，而在“artifact 推进失控”：issue 质量不稳、标准不清、上下文割裂、执行交接过早，最后导致项目推进不顺。

`issueflow` 要解决的是这层推进问题。它为 AI Coding 工作流提供一个围绕 artifact 状态、项目记忆、角色上下文和执行协同的控制层。

## 核心机制

稳定内核是 artifact state machine。
artifact 的推进不依赖隐式聊天进度，而依赖显式状态迁移。

轻 Agent 加 `skills` 负责大部分推进工作：理解上下文、组织项目记忆、协调角色、决定下一步动作、准备受控执行。

当确实需要重执行时，再交给外部重 Agent。
`issueflow` 不是 `OpenCode` 或 `Codex`；它更像是决定 artifact 应该如何推进、何时交接给重执行系统的那一层。

## 为什么重要

这样，AI Coding 工作流不再只是一次性的 prompt，而会变成一个更可控、可复核、可复用的项目推进系统。

## 为什么欢迎你加入

如果你也在关注这些问题，这个项目很适合一起做：

- AI Coding 协作
- GitLab 工程工作流
- issue 质量和执行标准
- engineering memory / context persistence
- 没有专职 QA 团队时的测试与验证补位
- 超越“代码生成”的项目推进自动化

更多人一起做，这个项目才会更好。
如果你也被“AI 会写代码，但项目还是推进不动”这个问题困住，欢迎一起推进它。

## Skills 与执行方式

`skills` 是 `issueflow` 想重点利用的一种机制。
目标是把项目方法、标准和协作习惯显式化、版本化，而不是藏在一次性的 prompt 里。

## 快速开始

```bash
# 1. 构建前端
cd web && npm install && npm run build && cd ..

# 2. 启动网关
GIT_WEBHOOK_SECRET=local-dev-secret cargo run

# 3. 验证
curl http://127.0.0.1:8080/api/status/ping
# → ok
```

网关默认监听 `127.0.0.1:8080`。OIDC 默认关闭，显式配置后才启用。

## 配置

完整配置说明见 [docs/CONFIG.md](docs/CONFIG.md)。

## 本地开发

完整本地开发环境说明（含 OIDC、GitLab webhook 联调、AI 辅助 GitLab 工作流执行）见 [docs/local-development.md](docs/local-development.md)。

## Docker

```bash
docker build -t issueflow .
docker run -p 8080:8080 -e GIT_WEBHOOK_SECRET=local-dev-secret issueflow
```

## 架构

系统架构、设计目标、安全模型和基于阶段的权限控制见 [docs/DESIGN.md](docs/DESIGN.md)。
