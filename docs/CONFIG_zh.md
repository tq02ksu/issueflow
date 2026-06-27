# Gateway 配置

`issueflow` Gateway 采用单实例、单 OIDC issuer 的登录模型。

## 配置来源

Gateway 按以下顺序加载配置，后者覆盖前者：

1. 内置默认值
2. `config/issueflow.toml`
3. 项目根目录 `.env`
4. 进程环境变量

内置默认值：

| 字段 | 默认值 |
|---|---|
| `server.listen_addr` | `127.0.0.1:8080` |
| `oidc.enabled` | `false` |
| `oidc.scopes` | `["openid", "profile", "email"]` |

## TOML 配置

推荐把本地开发配置写在 `config/issueflow.toml`：

```toml
[server]
listen_addr = "127.0.0.1:8080"

[git]
webhook_secret = "replace-me"

[oidc]
enabled = true
issuer = "https://gitlab.com"
client_id = "replace-me"
client_secret = "replace-me"
redirect_uri = "http://127.0.0.1:8080/auth/callback"
state_signing_secret = "replace-me"
scopes = ["openid", "profile", "email"]
```

当 `oidc.enabled = false` 时，其他 `oidc.*` 字段可以省略。

## 环境变量

TOML 字段与环境变量的映射：

| TOML | 环境变量 | 是否必填 |
|---|---|---|
| `server.listen_addr` | `LISTEN_ADDR` | 否 |
| `git.webhook_secret` | `GIT_WEBHOOK_SECRET` | 是 |
| `git.base_url` | `GIT_BASE_URL` | 创建 issue 时需要 |
| `git.token` | `GIT_TOKEN` | 创建 issue 时需要 |
| `oidc.enabled` | `OIDC_ENABLED` | 否 |
| `oidc.issuer` | `OIDC_ISSUER` | 启用 OIDC 时 |
| `oidc.client_id` | `OIDC_CLIENT_ID` | 启用 OIDC 时 |
| `oidc.client_secret` | `OIDC_CLIENT_SECRET` | 启用 OIDC 时 |
| `oidc.redirect_uri` | `OIDC_REDIRECT_URI` | 启用 OIDC 时 |
| `oidc.scopes` | `OIDC_SCOPES` | 否 |
| `oidc.state_signing_secret` | `OIDC_STATE_SIGNING_SECRET` | 启用 OIDC 时 |

`OIDC_SCOPES` 使用空格分隔：

```text
OIDC_SCOPES=openid profile email
```

## OIDC 约定

| 路由 | 用途 |
|---|---|
| `GET /auth/login` | Gateway 协议入口 |
| `GET /auth/callback` | OIDC 回调入口 |
| `GET /auth/callback/oidc` | 前端结果页，不应配置到身份提供方 |

OIDC provider 上配置的 Redirect URI 应为 `<gateway-base-url>/auth/callback`。

## Discovery

Gateway 通过 OIDC issuer discovery 自动拉取元数据：

```text
<issuer>/.well-known/openid-configuration
```

`OIDC_ISSUER` / `oidc.issuer` 必须指向一个支持标准 OIDC discovery 的 issuer。当前主路径是 GitLab，但配置模型本身不绑定 GitLab 专有字段。
