import { prototypeSkills } from "./prototype.data";
import type {
  IssueWorkflowState,
  MrWorkflowState,
  SkillUiProfile,
  WorkflowSummaryItem,
} from "./prototype.types";

const defaultProfile: SkillUiProfile = {
  tone: "direct",
  density: "balanced",
  overviewEmphasis: ["blocked", "planned"],
  issueFieldPriority: ["state", "nextActionSummary", "blockerSummary"],
  mrFieldPriority: ["state", "reviewSummary", "nextActionSummary"],
  milestoneFieldPriority: ["goal", "riskSummary", "nextActionSummary"],
  defaultExpandedSections: ["state"],
  recommendedActionOrder: ["clarify_scope", "unblock"],
};

export function getSkillUiProfile(
  versionId: string | undefined,
): SkillUiProfile {
  if (!versionId) {
    return defaultProfile;
  }

  for (const skill of prototypeSkills) {
    const version = skill.versions.find((item) => item.id === versionId);
    if (version) {
      return version.uiProfile;
    }
  }

  return defaultProfile;
}

export function summarizeStates<TState extends string>(
  states: readonly TState[],
): WorkflowSummaryItem<TState>[] {
  const counts = new Map<TState, number>();

  for (const state of states) {
    counts.set(state, (counts.get(state) ?? 0) + 1);
  }

  return Array.from(counts.entries()).map(([state, count]) => ({
    state,
    count,
  }));
}

export function sortIssueStatesByProfile(
  summary: WorkflowSummaryItem<IssueWorkflowState>[],
  profile: SkillUiProfile,
) {
  return [...summary].sort((left, right) => {
    const leftIndex = profile.overviewEmphasis.indexOf(left.state);
    const rightIndex = profile.overviewEmphasis.indexOf(right.state);

    const normalizedLeft = leftIndex === -1 ? Number.MAX_SAFE_INTEGER : leftIndex;
    const normalizedRight =
      rightIndex === -1 ? Number.MAX_SAFE_INTEGER : rightIndex;

    return normalizedLeft - normalizedRight;
  });
}

export function sortMrStatesByProfile(
  summary: WorkflowSummaryItem<MrWorkflowState>[],
  profile: SkillUiProfile,
) {
  return [...summary].sort((left, right) => {
    const leftIndex = profile.overviewEmphasis.indexOf(left.state);
    const rightIndex = profile.overviewEmphasis.indexOf(right.state);

    const normalizedLeft = leftIndex === -1 ? Number.MAX_SAFE_INTEGER : leftIndex;
    const normalizedRight =
      rightIndex === -1 ? Number.MAX_SAFE_INTEGER : rightIndex;

    return normalizedLeft - normalizedRight;
  });
}
