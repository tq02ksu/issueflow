#!/bin/sh
set -eu

if [ -z "${ROBOT_JOB_TYPE:-}" ]; then
  echo "ROBOT_JOB_TYPE is required" >&2
  exit 1
fi

if [ -z "${PROJECT_ID:-}" ]; then
  echo "PROJECT_ID is required" >&2
  exit 1
fi

mkdir -p robot-output

job_key=$(printf '%s' "$ROBOT_JOB_TYPE" | tr '[:lower:]-' '[:upper:]_')
job_command_var="ROBOT_${job_key}_COMMAND"
job_command=$(eval "printf '%s' \"\${$job_command_var:-}\"")

core_script="scripts/robot/core/${ROBOT_JOB_TYPE}.sh"
integration_script="scripts/robot/integrations/gitlab-ci/jobs/${ROBOT_JOB_TYPE}.sh"

{
  printf 'robot_job_type=%s\n' "$ROBOT_JOB_TYPE"
  printf 'project_id=%s\n' "$PROJECT_ID"
  printf 'resource_type=%s\n' "${RESOURCE_TYPE:-}"
  printf 'resource_id=%s\n' "${RESOURCE_ID:-}"
  printf 'issue_id=%s\n' "${ISSUE_ID:-}"
  printf 'mr_id=%s\n' "${MR_ID:-}"
  printf 'comment_id=%s\n' "${COMMENT_ID:-}"
  printf 'trigger_user=%s\n' "${TRIGGER_USER:-}"
  printf 'correlation_id=%s\n' "${CORRELATION_ID:-}"
} > robot-output/context.env

if [ -n "$job_command" ]; then
  echo "Running ${ROBOT_JOB_TYPE} from \$$job_command_var"
  exec sh -c "$job_command"
fi

if [ -f "$integration_script" ]; then
  echo "Running ${ROBOT_JOB_TYPE} from $integration_script"
  exec sh "$integration_script"
fi

if [ -f "$core_script" ]; then
  echo "Running ${ROBOT_JOB_TYPE} from $core_script"
  exec sh "$core_script"
fi

echo "No handler found for ROBOT_JOB_TYPE=$ROBOT_JOB_TYPE" >&2
echo "Set $job_command_var or add one of the following scripts:" >&2
echo "  $integration_script" >&2
echo "  $core_script" >&2
exit 1
