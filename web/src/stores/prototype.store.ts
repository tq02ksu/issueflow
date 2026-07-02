import { computed, ref } from "vue";
import { defineStore } from "pinia";
import {
  prototypeActivity,
  prototypeIssues,
  prototypeMemoryScopes,
  prototypeMilestones,
  prototypeMrs,
  prototypeSkills,
  prototypeUserSoul,
  prototypeWorkbenches,
} from "@/mock/prototype.data";
import {
  getSkillUiProfile,
  sortIssueStatesByProfile,
  sortMrStatesByProfile,
  summarizeStates,
} from "@/mock/prototype.ui-profile";
import type {
  PrototypeRecommendedAction,
  PrototypeRole,
} from "@/mock/prototype.types";

export const usePrototypeStore = defineStore("prototype", () => {
  const workbenches = ref(structuredClone(prototypeWorkbenches));
  const skills = ref(structuredClone(prototypeSkills));
  const issues = ref(structuredClone(prototypeIssues));
  const mrs = ref(structuredClone(prototypeMrs));
  const milestones = ref(structuredClone(prototypeMilestones));
  const activityItems = ref(structuredClone(prototypeActivity));
  const userSoul = ref(structuredClone(prototypeUserSoul));
  const memoryScopes = ref(structuredClone(prototypeMemoryScopes));
  const currentWorkbenchId = ref("alpha");
  const selectedIssueId = ref("issue-101");
  const selectedMrId = ref("mr-88");
  const selectedMilestoneId = ref("ms-q3");
  const lastMemoryAction = ref<"idle" | "cleared" | "rebuilt">("idle");

  const currentWorkbench = computed(
    () =>
      workbenches.value.find(
        (workbench) => workbench.id === currentWorkbenchId.value,
      ) ?? null,
  );

  const availableSkills = computed(() => skills.value);
  const prototypeWorkbenchesList = computed(() => workbenches.value);
  const currentUserSoul = computed(() => userSoul.value);
  const currentMemoryScopes = computed(() => memoryScopes.value);

  const activeUiProfile = computed(() =>
    getSkillUiProfile(currentWorkbench.value?.activeSkillVersionId),
  );

  const visibleIssues = computed(() =>
    issues.value.filter((issue) => issue.workbenchId === currentWorkbenchId.value),
  );

  const visibleMrs = computed(() =>
    mrs.value.filter((mr) => mr.workbenchId === currentWorkbenchId.value),
  );

  const visibleMilestones = computed(() =>
    milestones.value.filter(
      (milestone) => milestone.workbenchId === currentWorkbenchId.value,
    ),
  );

  const activity = computed(() =>
    activityItems.value.filter(
      (item) => item.workbenchId === currentWorkbenchId.value,
    ),
  );

  const selectedIssue = computed(
    () =>
      visibleIssues.value.find((issue) => issue.id === selectedIssueId.value) ??
      visibleIssues.value[0] ??
      null,
  );

  const selectedMr = computed(
    () =>
      visibleMrs.value.find((mr) => mr.id === selectedMrId.value) ??
      visibleMrs.value[0] ??
      null,
  );

  const selectedMilestone = computed(
    () =>
      visibleMilestones.value.find(
        (milestone) => milestone.id === selectedMilestoneId.value,
      ) ??
      visibleMilestones.value[0] ??
      null,
  );

  const issueWorkflowSummary = computed(() =>
    sortIssueStatesByProfile(
      summarizeStates(visibleIssues.value.map((issue) => issue.state)),
      activeUiProfile.value,
    ),
  );

  const mrWorkflowSummary = computed(() =>
    sortMrStatesByProfile(
      summarizeStates(visibleMrs.value.map((mr) => mr.state)),
      activeUiProfile.value,
    ),
  );

  const selectedMilestoneIssues = computed(() => {
    if (!selectedMilestone.value) {
      return [];
    }

    return visibleIssues.value.filter((issue) =>
      selectedMilestone.value?.issueIds.includes(issue.id),
    );
  });

  const selectedMilestoneMrs = computed(() => {
    if (!selectedMilestone.value) {
      return [];
    }

    return visibleMrs.value.filter((mr) =>
      selectedMilestone.value?.mrIds.includes(mr.id),
    );
  });

  const selectedMilestoneIssueSummary = computed(() =>
    summarizeStates(selectedMilestoneIssues.value.map((issue) => issue.state)),
  );

  const selectedMilestoneMrSummary = computed(() =>
    summarizeStates(selectedMilestoneMrs.value.map((mr) => mr.state)),
  );

  const recommendedActions = computed<PrototypeRecommendedAction[]>(() => {
    const actions: PrototypeRecommendedAction[] = [];

    if (selectedIssue.value) {
      actions.push({
        id: "action-issue",
        title: selectedIssue.value.nextActionSummary,
        summary: selectedIssue.value.blockerSummary,
        owner: `Issue #${selectedIssue.value.iid}`,
        intent: "start_dev_handoff",
      });
    }

    if (selectedMr.value) {
      actions.push({
        id: "action-mr",
        title: selectedMr.value.nextActionSummary,
        summary: selectedMr.value.reviewSummary,
        owner: `MR !${selectedMr.value.iid}`,
        intent: "resolve_review",
      });
    }

    if (selectedMilestone.value) {
      actions.push({
        id: "action-milestone",
        title: selectedMilestone.value.nextActionSummary,
        summary: selectedMilestone.value.riskSummary,
        owner: selectedMilestone.value.title,
        intent: "unblock",
      });
    }

    return actions.sort((left, right) => {
      const order = activeUiProfile.value.recommendedActionOrder;
      const leftIndex = order.indexOf(left.intent);
      const rightIndex = order.indexOf(right.intent);
      const normalizedLeft = leftIndex === -1 ? Number.MAX_SAFE_INTEGER : leftIndex;
      const normalizedRight =
        rightIndex === -1 ? Number.MAX_SAFE_INTEGER : rightIndex;

      return normalizedLeft - normalizedRight;
    });
  });

  function selectWorkbench(id: string) {
    currentWorkbenchId.value = id;
    selectedIssueId.value =
      issues.value.find((issue) => issue.workbenchId === id)?.id ?? "";
    selectedMrId.value = mrs.value.find((mr) => mr.workbenchId === id)?.id ?? "";
    selectedMilestoneId.value =
      milestones.value.find((milestone) => milestone.workbenchId === id)?.id ?? "";
  }

  function selectIssue(id: string) {
    selectedIssueId.value = id;
  }

  function selectMr(id: string) {
    selectedMrId.value = id;
  }

  function selectMilestone(id: string) {
    selectedMilestoneId.value = id;
  }

  function setActiveSkillVersion(versionId: string) {
    if (!currentWorkbench.value) {
      return;
    }

    currentWorkbench.value.activeSkillVersionId = versionId;
  }

  function toggleSkillVersion(versionId: string, enabled: boolean) {
    for (const skill of skills.value) {
      const version = skill.versions.find((item) => item.id === versionId);
      if (version) {
        version.enabled = enabled;
        return;
      }
    }
  }

  function mockUploadSkill() {
    const skill = skills.value[0];
    if (!skill) {
      return;
    }

    const nextVersion = `2.${skill.versions.length + 1}.0`;
    skill.versions.unshift({
      id: `${skill.id}@${nextVersion}`,
      version: nextVersion,
      enabled: false,
      uiProfile: {
        tone: "operator",
        density: "compact",
        overviewEmphasis: ["blocked", "ready_for_execution", "in_review"],
        issueFieldPriority: ["state", "nextActionSummary", "blockerSummary"],
        mrFieldPriority: ["state", "nextActionSummary", "reviewSummary"],
        milestoneFieldPriority: ["goal", "riskSummary", "nextActionSummary"],
        defaultExpandedSections: ["state", "verification"],
        recommendedActionOrder: [
          "unblock",
          "resolve_review",
          "start_dev_handoff",
        ],
      },
    });
  }

  function updateWorkbenchRole(role: PrototypeRole) {
    if (!currentWorkbench.value) {
      return;
    }

    currentWorkbench.value.role = role;
  }

  function updateUserSoul(input: {
    personality: string;
    waysOfWorking: string[];
    defaultGoal: string;
  }) {
    userSoul.value = {
      ...userSoul.value,
      personality: input.personality,
      waysOfWorking: input.waysOfWorking,
      defaultGoal: input.defaultGoal,
    };
  }

  function clearWorkbenchMemory() {
    lastMemoryAction.value = "cleared";
  }

  function rebuildWorkbenchMemory() {
    lastMemoryAction.value = "rebuilt";
  }

  return {
    currentWorkbenchId,
    selectedIssueId,
    selectedMrId,
    selectedMilestoneId,
    prototypeWorkbenchesList,
    currentWorkbench,
    currentUserSoul,
    currentMemoryScopes,
    availableSkills,
    activeUiProfile,
    visibleIssues,
    visibleMrs,
    visibleMilestones,
    activity,
    selectedIssue,
    selectedMr,
    selectedMilestone,
    selectedMilestoneIssues,
    selectedMilestoneMrs,
    selectedMilestoneIssueSummary,
    selectedMilestoneMrSummary,
    issueWorkflowSummary,
    mrWorkflowSummary,
    recommendedActions,
    lastMemoryAction,
    selectWorkbench,
    selectIssue,
    selectMr,
    selectMilestone,
    setActiveSkillVersion,
    toggleSkillVersion,
    mockUploadSkill,
    updateWorkbenchRole,
    updateUserSoul,
    clearWorkbenchMemory,
    rebuildWorkbenchMemory,
  };
});
