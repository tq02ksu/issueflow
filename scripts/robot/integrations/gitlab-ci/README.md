# GitLab CI Integration

This directory contains the current GitLab CI integration assets for robot execution.

## Files

- `gitlab-ci.robot.yml`: reusable GitLab CI pipeline template for robot jobs, MR validation, default-branch packaging and deployment, and tag-based release flow.
- `run-job.sh`: robot job dispatcher used by trigger-based GitLab pipelines.

## Design

The template keeps robot execution and regular delivery in the same GitLab CI model, but routes them through different pipeline sources:

- `trigger`: robot jobs such as `issue-triage`, `mr-plan-draft`, and `mr-implement`
- `merge_request_event`: MR compile and test jobs
- `push` to the default branch: package and deployment jobs
- `tag`: release build and publish jobs

All jobs run in Docker-based GitLab CI job images. The template separates images by responsibility:

- `ROBOT_RUNTIME_IMAGE`: OpenCode and robot runtime image
- `BUILD_RUNTIME_IMAGE`: compile, test, and package runtime image
- `DEPLOY_RUNTIME_IMAGE`: deploy and release publish runtime image

## Usage

Add a root `.gitlab-ci.yml` that includes the template:

```yaml
include:
  - local: scripts/robot/integrations/gitlab-ci/gitlab-ci.robot.yml
```

Then set project or group CI variables for the commands that match your repository.

Example command set:

```text
MR_COMPILE_COMMAND=cargo build --locked
MR_TEST_COMMAND=cargo test --locked
PACKAGE_COMMAND=mkdir -p dist && cargo build --release --locked && cp target/release/issueflow dist/
PACKAGE_ARTIFACT_PATH=dist
DEPLOY_COMMAND=./scripts/deploy/deploy-staging.sh
RELEASE_BUILD_COMMAND=mkdir -p release && cargo build --release --locked && cp target/release/issueflow release/
RELEASE_ARTIFACT_PATH=release
RELEASE_PUBLISH_COMMAND=./scripts/release/publish.sh
```

Robot jobs can be implemented in either of two ways:

- set a CI variable such as `ROBOT_MR_IMPLEMENT_COMMAND`
- add a script at `scripts/robot/integrations/gitlab-ci/jobs/<robot-job-type>.sh`

If neither exists, `run-job.sh` fails fast and tells you which command variable or script path is missing.

## Pipeline Parameters

Gateway-triggered robot pipelines should provide these variables:

- `ROBOT_JOB_TYPE`: one of `issue-triage`, `issue-validate`, `mr-plan-draft`, `mr-implement`, `mr-verify`, `release-prepare`, `release-publish`
- `PROJECT_ID`: GitLab project identifier
- `RESOURCE_TYPE`: source resource type such as `issue`, `merge_request`, or `release`
- `RESOURCE_ID`: source resource identifier
- `ISSUE_ID`: related issue IID when present
- `MR_ID`: related merge request IID when present
- `COMMENT_ID`: triggering note/comment identifier when present
- `TRIGGER_USER`: user who triggered the action
- `CORRELATION_ID`: workflow correlation identifier used across Gateway, CI, and OpenCode

Repository-level delivery jobs use these command variables:

- `MR_COMPILE_COMMAND`: command used in the MR compile job
- `MR_TEST_COMMAND`: command used in the MR test job
- `PACKAGE_COMMAND`: command used in the default-branch package job
- `PACKAGE_ARTIFACT_PATH`: file or directory copied into `package-output/`
- `DEPLOY_COMMAND`: command used in the staging deployment job
- `RELEASE_BUILD_COMMAND`: command used in the tag-based release build job
- `RELEASE_ARTIFACT_PATH`: file or directory copied into `release-output/`
- `RELEASE_PUBLISH_COMMAND`: command used in the release publish job

Robot dispatch also supports these per-job command variables:

- `ROBOT_ISSUE_TRIAGE_COMMAND`
- `ROBOT_ISSUE_VALIDATE_COMMAND`
- `ROBOT_MR_PLAN_DRAFT_COMMAND`
- `ROBOT_MR_IMPLEMENT_COMMAND`
- `ROBOT_MR_VERIFY_COMMAND`
- `ROBOT_RELEASE_PREPARE_COMMAND`
- `ROBOT_RELEASE_PUBLISH_COMMAND`

## Notes

- `deploy-staging` and `release-publish-job` are marked `manual` to avoid accidental delivery or release.
- The template is a reusable integration asset, not an automatically enabled repository root pipeline.
- If you need Docker image build and push inside jobs, extend the template with `services: [docker:dind]` and replace the command variables with your container build steps.
