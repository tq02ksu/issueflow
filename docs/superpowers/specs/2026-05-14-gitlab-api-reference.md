# GitLab Hook 与 API 参考

## 目的

本文档整理本方案真正依赖的 GitLab 官方 Hook 事件和 REST API，作为实现时的接口参考。

主设计文档：

`docs/superpowers/specs/2026-05-14-gitlab-dev-robot-design.md`

## 官方文档来源

1. Project webhooks API
   - `https://docs.gitlab.com/api/project_webhooks/`
2. Webhook events
   - `https://docs.gitlab.com/user/project/integrations/webhook_events/`
3. Issues API
   - `https://docs.gitlab.com/api/issues/`
4. Merge requests API
   - `https://docs.gitlab.com/api/merge_requests/`
5. Notes API
   - `https://docs.gitlab.com/api/notes/`
6. Branches API
   - `https://docs.gitlab.com/api/branches/`
7. Pipeline trigger tokens API
   - `https://docs.gitlab.com/api/pipeline_triggers/`

## 本方案需要消费的 Hook 事件

### 1. `Issue Hook`

用途：

1. 新建 issue 时触发分诊。
2. issue 更新时决定是否重新分诊或重建摘要。

关注字段：

1. `object_kind`
2. `event_type`
3. `project.id`
4. `object_attributes.iid`
5. `object_attributes.action`
6. `object_attributes.title`
7. `object_attributes.description`
8. `object_attributes.state`
9. `labels`

### 2. `Note Hook`

用途：

1. 识别 issue 评论中的 `/triage`、`/validate`、`/start-dev`。
2. 识别 MR 评论中的 `/verify`。
3. 区分评论目标是 issue 还是 merge request。

关注字段：

1. `object_attributes.note`
2. `object_attributes.noteable_type`
3. `object_attributes.action`
4. `project_id`
5. `issue.iid`
6. `merge_request.iid`
7. `user.username`

### 3. `Merge Request Hook`

用途：

1. 识别空 MR 创建成功。
2. 跟踪 MR 状态变化。
3. 感知后续代码推送、更新、关闭等事件。

关注字段：

1. `object_attributes.iid`
2. `object_attributes.action`
3. `object_attributes.state`
4. `object_attributes.source_branch`
5. `object_attributes.target_branch`
6. `object_attributes.draft`
7. `changes`

### 4. `Pipeline Hook`

用途：

1. 可选，用于让 Gateway 感知 pipeline 状态变化。
2. 可替代部分内部回调逻辑。

第一版不是强依赖，但推荐预留。

## 本方案需要使用的 Webhook 管理 API

### 1. 创建项目 webhook

接口：

`POST /projects/:id/hooks`

用途：

1. 为项目注册 Robot Gateway webhook 地址。
2. 开启 `issues_events`、`note_events`、`merge_requests_events`。
3. 可选开启 `pipeline_events`。

建议参数：

1. `url`
2. `issues_events=true`
3. `note_events=true`
4. `merge_requests_events=true`
5. `pipeline_events=true` 或按需关闭
6. `token`
7. `enable_ssl_verification=true`

### 2. 测试项目 webhook

接口：

`POST /projects/:id/hooks/:hook_id/test/:trigger`

用途：

1. 联调 Gateway 时测试单类事件是否能正确送达。

### 3. 查询 webhook 事件投递记录

接口：

`GET /projects/:id/hooks/:hook_id/events`

用途：

1. 排查 webhook 调用失败。
2. 检查 Robot Gateway 的返回码和响应内容。

这个接口对排障很有价值，虽然不是业务主链路必需接口。

## 本方案需要使用的 Issues API

### 1. 获取 issue 详情

接口：

`GET /projects/:id/issues/:issue_iid`

用途：

1. CI 读取 issue 标题、描述、标签、状态。
2. 组装 triage 和 validate 的上下文。

### 2. 更新 issue

接口：

`PUT /projects/:id/issues/:issue_iid`

用途：

1. 更新 issue 描述中的 `Robot Summary` 区块。
2. 按需补充标签。
3. 按需改变状态或结构化字段。

本方案最主要用它来“覆盖式更新 issue 摘要区块”。

## 本方案需要使用的 Notes API

### Issue Notes

1. 创建 issue 评论

接口：

`POST /projects/:id/issues/:issue_iid/notes`

用途：

1. 回写 triage 结果。
2. 回写 validate 结果。
3. 提示缺失信息和建议下一步。

2. 更新 issue 评论

接口：

`PUT /projects/:id/issues/:issue_iid/notes/:note_id`

用途：

1. 若选择“单条机器人评论持续更新”模式，可以更新已有评论。
2. 若选择“每次新发评论”模式，则这个接口可不使用。

### Merge Request Notes

1. 创建 MR 评论

接口：

`POST /projects/:id/merge_requests/:merge_request_iid/notes`

用途：

1. 发布方案草稿。
2. 发布确认区块。
3. 发布实现结果和验证结果。

2. 更新 MR 评论

接口：

`PUT /projects/:id/merge_requests/:merge_request_iid/notes/:note_id`

用途：

1. 更新原有方案评论中的确认区块。
2. 将“待确认”改成“已确认”或“已失效”。

## 本方案需要使用的 Branches API

### 1. 创建分支

接口：

`POST /projects/:id/repository/branches`

用途：

1. 收到 `/start-dev` 后创建开发分支。

关键参数：

1. `branch`
2. `ref`

### 2. 查询分支

接口：

`GET /projects/:id/repository/branches/:branch`

用途：

1. 判断目标分支是否已存在。
2. 创建分支前做幂等检查。

## 本方案需要使用的 Merge Requests API

### 1. 创建 MR

接口：

`POST /projects/:id/merge_requests`

用途：

1. `/start-dev` 后创建空 MR。

关键参数：

1. `source_branch`
2. `target_branch`
3. `title`
4. `description`
5. `remove_source_branch`
6. `reviewer_ids` 或按需为空

### 2. 获取 MR 详情

接口：

`GET /projects/:id/merge_requests/:merge_request_iid`

用途：

1. 获取 MR 当前状态。
2. 获取描述、分支、draft 状态。
3. 在确认回调时做状态校验。

### 3. 更新 MR

接口：

`PUT /projects/:id/merge_requests/:merge_request_iid`

用途：

1. 更新 MR 描述中的 `Robot Summary`。
2. 必要时更新标题、标签或状态。
3. 按需将 MR 关闭或重开。

本方案最主要用它来“覆盖式更新 MR 摘要区块”。

## 本方案需要使用的 Pipeline Trigger API

### 1. 触发 pipeline

接口：

`POST /projects/:id/trigger/pipeline`

用途：

1. 触发 `issue-triage`
2. 触发 `issue-validate`
3. 触发 `mr-plan-draft`
4. 触发 `mr-implement`
5. 触发 `mr-verify`

关键参数：

1. `ref`
2. `token`
3. `variables[ROBOT_JOB_TYPE]`
4. `variables[PROJECT_ID]`
5. `variables[RESOURCE_TYPE]`
6. `variables[RESOURCE_ID]`
7. `variables[ISSUE_ID]`
8. `variables[MR_ID]`
9. `variables[COMMENT_ID]`
10. `variables[TRIGGER_USER]`
11. `variables[CORRELATION_ID]`

## 推荐的最小 API 使用清单

如果只保留 MVP 需要的最小集，建议至少实现这些调用：

1. `POST /projects/:id/hooks`
2. `POST /projects/:id/hooks/:hook_id/test/:trigger`
3. `GET /projects/:id/issues/:issue_iid`
4. `PUT /projects/:id/issues/:issue_iid`
5. `POST /projects/:id/issues/:issue_iid/notes`
6. `POST /projects/:id/repository/branches`
7. `GET /projects/:id/repository/branches/:branch`
8. `POST /projects/:id/merge_requests`
9. `GET /projects/:id/merge_requests/:merge_request_iid`
10. `PUT /projects/:id/merge_requests/:merge_request_iid`
11. `POST /projects/:id/merge_requests/:merge_request_iid/notes`
12. `PUT /projects/:id/merge_requests/:merge_request_iid/notes/:note_id`
13. `POST /projects/:id/trigger/pipeline`

## 实现建议

1. 优先用 `project path` 而不是内部数值 ID，减少跨环境搬迁时的耦合。
2. 命令解析基于 `Note Hook`，不要轮询 Notes API 做命令发现。
3. 摘要区块更新使用 `PUT issue` 和 `PUT merge request`，评论内容更新使用 `PUT note`。
4. 方案确认成功后，优先更新原 MR 评论中的确认区块，而不是再发一条新评论。
5. Pipeline 触发统一走一个入口，通过变量区分 job 类型。
