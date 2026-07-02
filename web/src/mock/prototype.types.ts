export type IssueWorkflowState =
  | "new"
  | "clarifying"
  | "planned"
  | "ready_for_execution"
  | "in_execution"
  | "blocked"
  | "done";

export type MrWorkflowState =
  | "draft"
  | "in_review"
  | "changes_requested"
  | "ready_to_merge"
  | "merged"
  | "blocked";

export interface PrototypeRole {
  name: string;
  personaSummary: string;
  waysOfWorking: string[];
  goals: string[];
}

export interface PrototypeIssue {
  id: string;
  workbenchId: string;
  iid: number;
  title: string;
  state: IssueWorkflowState;
  blockerSummary: string;
  nextActionSummary: string;
  description: string;
  acceptanceCriteria: string[];
  verificationPlan: string[];
  risks: string[];
}

export interface PrototypeMr {
  id: string;
  workbenchId: string;
  iid: number;
  title: string;
  state: MrWorkflowState;
  reviewSummary: string;
  nextActionSummary: string;
  linkedIssueId: string;
  readinessChecks: string[];
  verificationNotes: string[];
  risks: string[];
}

export interface PrototypeMilestone {
  id: string;
  workbenchId: string;
  title: string;
  goal: string;
  dueDate: string;
  issueIds: string[];
  mrIds: string[];
  riskSummary: string;
  nextActionSummary: string;
}

export interface SkillUiProfile {
  tone: "direct" | "coach" | "operator";
  density: "compact" | "balanced" | "relaxed";
  overviewEmphasis: string[];
  issueFieldPriority: string[];
  mrFieldPriority: string[];
  milestoneFieldPriority: string[];
  defaultExpandedSections: string[];
  recommendedActionOrder: string[];
}

export interface PrototypeSkillVersion {
  id: string;
  version: string;
  enabled: boolean;
  uiProfile: SkillUiProfile;
}

export interface PrototypeSkill {
  id: string;
  name: string;
  versions: PrototypeSkillVersion[];
}

export interface PrototypeActivityItem {
  id: string;
  workbenchId: string;
  kind: "issue" | "mr" | "milestone" | "settings";
  title: string;
  summary: string;
  timestamp: string;
}

export interface PrototypeWorkbench {
  id: string;
  projectId: number;
  projectPath: string;
  name: string;
  role: PrototypeRole;
  activeSkillVersionId: string;
}

export interface PrototypeUserSoul {
  name: string;
  personality: string;
  waysOfWorking: string[];
  defaultGoal: string;
}

export interface PrototypeRecommendedAction {
  id: string;
  title: string;
  summary: string;
  owner: string;
  intent: string;
}

export interface PrototypeMemoryScope {
  scope: "system" | "workbench" | "personal";
  summary: string;
  status: "healthy" | "attention";
}

export interface WorkflowSummaryItem<TState extends string> {
  state: TState;
  count: number;
}
