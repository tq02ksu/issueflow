# issueflow

Chat 驱动的 issue 管理 agent。

## 工作方式

1. 在 chat 中描述需求。
2. agent 理解需求、补全缺失上下文、整理为结构化 issue 草稿。
3. 你确认后，agent 将其写入 GitLab。
4. agent 继续推进 issue——分诊、校验、启动开发——由 `skills` 指导。

## Skills 驱动 agent

`skills` 定义了 agent *如何* 处理 issue。不同项目有不同的约定——`skills` 把这些约定沉淀到版本化 Git 仓库中，让 agent 按此执行。

- **项目 skill repo** 承载项目级 `skills`、issue、文档、仓库地图和 UI demo。
- **平台 skill repo** 提供系统级默认规则，当项目未自定义时兜底。

这意味着 agent 的行为透明、可审计，并通过正常的 Git 工作流演进——review、diff、merge、rollback。

## 快速开始

```bash
# 1. 构建前端
cd web && npm install && npm run build && cd ..

# 2. 启动网关
GIT_WEBHOOK_SECRET=local-dev-secret cargo run

# 3. 验证
curl http://127.0.0.1:8080/status/ping
# → ok
```

网关默认监听 `127.0.0.1:8080`。OIDC 默认关闭，显式配置后才启用。

## 配置

完整配置说明见 [docs/CONFIG.md](docs/CONFIG.md)。

## 本地开发

完整本地开发环境说明（含 OIDC、GitLab webhook 联调、chat 驱动 issue 创建）见 [docs/local-development.md](docs/local-development.md)。

## Docker

```bash
docker build -t issueflow .
docker run -p 8080:8080 -e GIT_WEBHOOK_SECRET=local-dev-secret issueflow
```

## Skills 作用方式

`issueflow` 不假设所有 issue 都应该用同一种方式处理。不同业务类型、不同团队约定、不同仓库习惯，可以通过 Git 仓库中的 `skills` 告诉 agent 应该如何理解和推进这些对象。

issue 所在仓库是处理方式的首选来源。当项目仓库中存在合适的 `skills` 时，它们覆盖平台默认值；当不存在时，平台默认值兜底。

## 架构

系统架构、设计目标、安全模型和基于阶段的权限控制见 [docs/DESIGN.md](docs/DESIGN.md)。
