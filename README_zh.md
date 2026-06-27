# issueflow

## 项目简介

`issueflow` 是一个管理和推进 issue 及相关 GitLab 协作对象的智能体，例如 milestone 和 wiki 页面。它按标准化、业界通用的处理过程推进这些对象，并通过 Git 仓库中的 `skills` 适配不同业务类型的处理方式。

## 核心处理过程

- 分诊：识别 issue 或相关 GitLab 对象的类型、优先级和当前状态。
- 补信息：发现缺失上下文并推动补全。
- 方案确认：整理目标、约束和处理方案。
- 开发启动：在满足条件后推进实现工作。
- 结果回写：把处理结果、状态和关键结论回写到 issue 处理过程中。

## Skills 作用方式

`issueflow` 不假设所有 issue、milestone 或 wiki 任务都应该用同一种方式处理。不同业务类型、不同团队约定、不同仓库习惯，可以通过 Git 仓库中的 `skills` 告诉智能体应该如何理解和推进这些对象。

issue 或相关 GitLab 对象所在仓库是处理方式的首选来源。如果该仓库提供了相关 `skills`，智能体优先按这些 `skills` 执行。

## 默认与覆盖关系

平台提供默认的通用处理方式，作为没有业务定制时的基础行为。

如果 issue 或相关 GitLab 对象所在仓库没有提供对应的 `skills`，`issueflow` 就回退到这个默认方式；如果仓库内存在合适的 `skills`，这些 `skills` 就覆盖默认方式，定义该类工作的具体处理规则。

这种设计让处理骨架保持一致，同时允许不同业务在同一个系统里保留各自的处理方式。

## 本地开发

最小本地启动需要 Rust、Node.js，以及一个 Git webhook secret。

1. 先构建前端静态资源：

```bash
cd web
npm install
npm run build
```

2. 使用本地 webhook secret 启动网关：

```bash
GIT_WEBHOOK_SECRET=local-dev-secret PATH="$HOME/.cargo/bin:$PATH" cargo run
```

默认情况下，服务监听 `127.0.0.1:8080`，并且只有在你显式配置时才会开启 OIDC。

3. 验证服务是否启动成功：

```bash
curl http://127.0.0.1:8080/status/ping
```

预期返回：

```text
ok
```

完整本地开发环境说明，包括 `config/issueflow.toml`、OIDC、本地 GitLab webhook 联调、chat 驱动的 GitLab issue 创建和常用测试命令，见 [`docs/local-development.md`](docs/local-development.md)。

### Docker

```bash
docker build -t issueflow .
docker run -p 8080:8080 -e GIT_WEBHOOK_SECRET=local-dev-secret issueflow
```
