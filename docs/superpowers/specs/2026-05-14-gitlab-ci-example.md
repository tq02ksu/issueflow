# GitLab CI 完整配置示例

## 目的

本文档给出一个更完整的 `.gitlab-ci.yml` 单文件示例，同时覆盖：

1. 机器人流程
2. 正常 MR 编译与测试流程
3. 主干分支打包与部署流程
4. Tag / release 流程

这个示例的核心结论是：

GitLab CI 不需要拆成两个独立配置文件，也不需要假设“只有一条固定 pipeline”。
可以在同一个 `.gitlab-ci.yml` 中，通过 `workflow.rules` 和各 job 的 `rules`，让不同触发来源进入不同流程。

主设计文档：

`docs/superpowers/specs/2026-05-14-gitlab-dev-robot-design.md`

API 参考文档：

`docs/superpowers/specs/2026-05-14-gitlab-api-reference.md`

## 适用场景

这个示例覆盖以下场景：

1. `trigger` 来源的机器人任务
2. `merge_request_event` 来源的 MR 构建与测试
3. `push` 到主分支后的打包与部署
4. `tag` 触发的 release 流程

## 流程映射

### 机器人流程

1. issue 新建或 `/triage` -> `issue-triage`
2. `/validate` -> `issue-validate`
3. `/start-dev` 创建空 MR -> `mr-plan-draft`
4. 确认链接点击成功 -> `mr-implement`
5. 自动验证或 `/verify` -> `mr-verify`

### 正常交付流程

1. MR 分支有新提交 -> `lint`、`test`、`build`
2. 主分支收到合并结果 -> `package`、`deploy_staging`
3. 打 tag -> `release_build`、`release_publish`

## 关键设计原则

1. 机器人 pipeline 和正常交付 pipeline 共用一个 `.gitlab-ci.yml`。
2. 机器人任务只在 `CI_PIPELINE_SOURCE == "trigger"` 时运行。
3. 正常 MR 验证只在 `CI_PIPELINE_SOURCE == "merge_request_event"` 时运行。
4. 主分支打包部署只在 `push` 到默认分支时运行。
5. release 只在 tag pipeline 中运行。
6. `mr-implement` 推送代码后，会自然触发正常 MR pipeline，因此不需要机器人重复发明一套 build/test。

## 推荐变量

Robot Gateway 触发 pipeline 时，建议至少传入：

1. `ROBOT_JOB_TYPE`
2. `PROJECT_ID`
3. `RESOURCE_TYPE`
4. `RESOURCE_ID`
5. `ISSUE_ID`
6. `MR_ID`
7. `COMMENT_ID`
8. `TRIGGER_USER`
9. `CORRELATION_ID`

## 单文件 `.gitlab-ci.yml` 完整示例

```yaml
stages:
  - robot
  - quality
  - build
  - package
  - deploy
  - release

default:
  image: node:22-bullseye
  interruptible: true
  before_script:
    - npm ci

workflow:
  rules:
    - if: '$CI_PIPELINE_SOURCE == "trigger"'
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
    - if: '$CI_COMMIT_TAG'
    - if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH'
    - when: never

variables:
  GIT_STRATEGY: fetch
  ROBOT_JOB_TYPE: ""
  PROJECT_ID: ""
  RESOURCE_TYPE: ""
  RESOURCE_ID: ""
  ISSUE_ID: ""
  MR_ID: ""
  COMMENT_ID: ""
  TRIGGER_USER: ""
  CORRELATION_ID: ""

.robot_job_template:
  stage: robot
  script:
    - test -n "$ROBOT_JOB_TYPE"
    - test -n "$PROJECT_ID"
    - mkdir -p robot-output
    - ./scripts/robot/run-job.sh
  artifacts:
    when: always
    expire_in: 7 days
    paths:
      - robot-output/
  rules:
    - when: never

.mr_quality_template:
  stage: quality
  rules:
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'

.default_branch_template:
  rules:
    - if: '$CI_PIPELINE_SOURCE == "push" && $CI_COMMIT_BRANCH == $CI_DEFAULT_BRANCH'

.tag_template:
  rules:
    - if: '$CI_COMMIT_TAG'

issue-triage:
  extends: .robot_job_template
  rules:
    - if: '$CI_PIPELINE_SOURCE == "trigger" && $ROBOT_JOB_TYPE == "issue-triage"'

issue-validate:
  extends: .robot_job_template
  rules:
    - if: '$CI_PIPELINE_SOURCE == "trigger" && $ROBOT_JOB_TYPE == "issue-validate"'

mr-plan-draft:
  extends: .robot_job_template
  rules:
    - if: '$CI_PIPELINE_SOURCE == "trigger" && $ROBOT_JOB_TYPE == "mr-plan-draft"'

mr-implement:
  extends: .robot_job_template
  rules:
    - if: '$CI_PIPELINE_SOURCE == "trigger" && $ROBOT_JOB_TYPE == "mr-implement"'

mr-verify:
  extends: .robot_job_template
  rules:
    - if: '$CI_PIPELINE_SOURCE == "trigger" && $ROBOT_JOB_TYPE == "mr-verify"'

lint:
  extends: .mr_quality_template
  script:
    - npm run lint

test:
  extends: .mr_quality_template
  script:
    - npm test -- --ci
  artifacts:
    when: always
    reports:
      junit: reports/junit.xml

build:
  stage: build
  rules:
    - if: '$CI_PIPELINE_SOURCE == "merge_request_event"'
  script:
    - npm run build
  artifacts:
    expire_in: 7 days
    paths:
      - dist/

package:
  stage: package
  extends: .default_branch_template
  script:
    - npm run build
    - tar -czf app.tar.gz dist/
  artifacts:
    expire_in: 30 days
    paths:
      - app.tar.gz

deploy_staging:
  stage: deploy
  extends: .default_branch_template
  needs:
    - package
  environment:
    name: staging
  script:
    - ./scripts/deploy/deploy-staging.sh app.tar.gz

release_build:
  stage: release
  extends: .tag_template
  script:
    - npm ci
    - npm run build
    - tar -czf release-${CI_COMMIT_TAG}.tar.gz dist/
  artifacts:
    expire_in: 90 days
    paths:
      - release-${CI_COMMIT_TAG}.tar.gz

release_publish:
  stage: release
  extends: .tag_template
  needs:
    - release_build
  script:
    - ./scripts/release/publish.sh release-${CI_COMMIT_TAG}.tar.gz
```

## 这个示例如何工作

### 1. 机器人 pipeline

当 Gateway 调用：

`POST /projects/:id/trigger/pipeline`

并传入：

`ROBOT_JOB_TYPE=mr-plan-draft`

则：

1. `workflow.rules` 允许这次 trigger pipeline 创建。
2. 只有匹配 `ROBOT_JOB_TYPE` 的机器人 job 会运行。
3. 正常的 `lint`、`test`、`build`、`deploy`、`release` 不会运行。

### 2. MR 正常验证 pipeline

当 `mr-implement` job 修改代码并 push 到 MR 分支后，GitLab 会自动产生 `merge_request_event` pipeline。

这时会运行：

1. `lint`
2. `test`
3. `build`

这条 pipeline 才是团队日常看的 MR 构建验证结果。

### 3. 主分支交付 pipeline

当 MR 合并到默认分支后，GitLab 会产生一次 `push` pipeline。

这时会运行：

1. `package`
2. `deploy_staging`

这样正常的打包和部署不受机器人逻辑干扰。

### 4. Release pipeline

当创建 tag 时，会运行：

1. `release_build`
2. `release_publish`

这使 release 版本流程和机器人流程完全分离。

## 为什么这样设计

### 不把所有事都放进机器人 job

原因：

1. 机器人 job 负责“理解需求、推动状态机、生成方案、辅助实现”。
2. 正常交付 job 负责“编译、测试、打包、部署、发版”。

这样更符合工程职责分离，也更接近 GitLab 的原生工作方式。

### 不再单独需要 `/plan` 和 `/implement`

原因：

1. `/start-dev` 后自动触发 `mr-plan-draft`
2. 点击确认链接成功后自动触发 `mr-implement`

所以这两个阶段已经被状态机和 pipeline 触发机制接管。

## `run-job.sh` 统一分发示例

```bash
#!/usr/bin/env bash
set -euo pipefail

case "$ROBOT_JOB_TYPE" in
  issue-triage)
    ./scripts/robot/issue-triage.sh
    ;;
  issue-validate)
    ./scripts/robot/issue-validate.sh
    ;;
  mr-plan-draft)
    ./scripts/robot/mr-plan-draft.sh
    ;;
  mr-implement)
    ./scripts/robot/mr-implement.sh
    ;;
  mr-verify)
    ./scripts/robot/mr-verify.sh
    ;;
  *)
    echo "Unknown ROBOT_JOB_TYPE: $ROBOT_JOB_TYPE" >&2
    exit 1
    ;;
esac
```

## Gateway 触发机器人 pipeline 示例

```bash
curl --request POST \
  --form ref=main \
  --form token="$TRIGGER_TOKEN" \
  --form "variables[ROBOT_JOB_TYPE]=mr-plan-draft" \
  --form "variables[PROJECT_ID]=group/project" \
  --form "variables[RESOURCE_TYPE]=mr" \
  --form "variables[RESOURCE_ID]=123" \
  --form "variables[ISSUE_ID]=45" \
  --form "variables[MR_ID]=123" \
  --form "variables[COMMENT_ID]=678" \
  --form "variables[TRIGGER_USER]=alice" \
  --form "variables[CORRELATION_ID]=req-20260514-0001" \
  "https://gitlab.example.com/api/v4/projects/group%2Fproject/trigger/pipeline"
```

## 推荐的脚本职责划分

### 机器人脚本

1. `scripts/robot/issue-triage.sh`
2. `scripts/robot/issue-validate.sh`
3. `scripts/robot/mr-plan-draft.sh`
4. `scripts/robot/mr-implement.sh`
5. `scripts/robot/mr-verify.sh`

### 交付脚本

1. `scripts/deploy/deploy-staging.sh`
2. `scripts/release/publish.sh`

## 实现建议

1. 机器人逻辑和交付逻辑可以共用一个仓库，但不要共用同一组脚本。
2. `mr-verify` 更适合作为“验证结果汇总器”，而不是再次重复 lint、test、build。
3. 如果项目后续变复杂，可以再把这份单文件配置拆成 `include` 方式，但第一版没必要。
4. 如果 `mr-implement` 会自动 commit 和 push，需要为 CI 账号配置最小权限的 token。
5. 如果要避免机器人 push 触发无限循环，应在机器人逻辑中约束只触发正常 MR pipeline，不再次触发机器人 trigger pipeline。

## 与主设计文档的关联点

这个示例直接对应主设计文档中的：

1. `Issue -> /start-dev -> 空 MR -> 方案确认 -> 开发 -> 验证`
2. `GitLab CI 承载机器人主要逻辑`
3. `正常编译、打包、部署、release 保持原生流水线`

因此，推荐把它作为第一版的参考实现模板，而不是把所有行为都硬编码到单一 job 中。
