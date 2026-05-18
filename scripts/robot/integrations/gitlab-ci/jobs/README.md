# GitLab CI Robot Job Scripts

Place GitLab CI specific robot job handlers here when a repository needs checked-in scripts instead of CI variable commands.

Supported script names match `ROBOT_JOB_TYPE`, for example:

- `issue-triage.sh`
- `mr-plan-draft.sh`
- `mr-implement.sh`
- `release-prepare.sh`

`../run-job.sh` resolves handlers in this order:

1. matching `ROBOT_<JOB>_COMMAND` CI variable
2. `scripts/robot/integrations/gitlab-ci/jobs/<robot-job-type>.sh`
3. `scripts/robot/core/<robot-job-type>.sh`
