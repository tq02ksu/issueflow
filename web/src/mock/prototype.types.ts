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

export type PrototypeRoleKey =
  | "developer"
  | "manager"
  | "product"
  | "evolution";

export interface PrototypeSignalCard {
  id: string;
  label: string;
  value: string;
  hint: string;
  tone: "neutral" | "attention" | "positive";
}

export interface PrototypeQuickEntry {
  id: string;
  labelKey: string;
  to: string;
}

export interface PrototypeRoleView {
  key: PrototypeRoleKey;
  sequence: "A" | "B" | "C" | "D";
  workbenchId: string;
  signalCards: PrototypeSignalCard[];
  quickEntries: PrototypeQuickEntry[];
  overviewEmphasis: string[];
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
  focus: string;
  uiProfile: SkillUiProfile;
}

export interface PrototypeSkill {
  id: string;
  workbenchId: string;
  name: string;
  summary: string;
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

export interface PrototypeLoop {
  id: string;
  workbenchId: string;
  name: string;
  type: "issue" | "mr" | "milestone";
  enabled: boolean;
  status: "healthy" | "blocked" | "waiting_approval" | "disabled";
  boundObject: string;
  boundObjectId: string;
  goal: string;
  schedulePolicy: string;
  stateMachinePolicy: string;
  skillRefs: string[];
  verificationPolicy: string;
  budgetPolicy: string;
  notificationPolicy: string;
  nextRunAt: string | null;
  lastRunAt: string | null;
}

export interface WorkflowSummaryItem<TState extends string> {
  state: TState;
  count: number;
}

export type ApprovalStatus =
  | "pending"
  | "approved"
  | "rejected"
  | "execution_failed";

export type TurnStatus =
  | "created"
  | "fetching"
  | "executing"
  | "evaluating"
  | "waiting_approval"
  | "completed"
  | "failed";

export type ApprovalRiskLevel = "low" | "medium" | "high" | "critical";

export interface PrototypeApproval {
  id: string;
  workbenchId: string;
  actionType:
    | "issue_comment"
    | "mr_comment"
    | "milestone_update"
    | "skill_activation"
    | "state_transition";
  sourceLoop: string;
  sourceTurnId: string;
  riskLevel: ApprovalRiskLevel;
  targetObject: string;
  targetUrl: string;
  draftContent: string;
  generationBasis: string;
  memoryRelation: string;
  status: ApprovalStatus;
  createdAt: string;
}

export interface TurnTarget {
  objectType: "issue" | "mr" | "milestone";
  objectId: string;
  actions: string[];
  result: string;
}

export interface TurnAgentRun {
  agentId: string;
  agentName: string;
  role: "executor" | "evaluator" | "external";
  model: "cheap-fast" | "balanced" | "high-reasoning";
  status: "running" | "done" | "failed";
  tokensUsed: number;
  cost: number;
  retries: number;
  responsibleFor: string[];
}

export interface PrototypeTurn {
  id: string;
  workbenchId: string;
  loopName: string;
  targets: TurnTarget[];
  status: TurnStatus;
  triggerSource: "manual" | "schedule" | "event";
  startTime: string;
  endTime: string | null;
  durationSecs: number;
  summary: string;
  conclusion: string;
  events: PrototypeTurnEvent[];
  agents: TurnAgentRun[];
  draftActions: TurnDraftAction[];
  recommendations: string[];
  memoryRead: string[];
  memoryWritten: string;
  totalTokens: number;
  totalCost: number;
}

export interface PrototypeTurnEvent {
  timestamp: string;
  kind:
    | "created"
    | "fetching_objects"
    | "memory_loaded"
    | "executor_invoked"
    | "evaluator_confirmed"
    | "conclusion_generated"
    | "approval_requested"
    | "completed"
    | "failed";
  message: string;
  agentId?: string;
  targetId?: string;
}

export interface TurnDraftAction {
  targetId: string;
  actionType:
    | "issue_comment"
    | "mr_comment"
    | "milestone_update"
    | "state_transition";
  draftContent: string;
  riskLevel: "low" | "medium" | "high" | "critical";
}

export interface PrototypeMemoryItem {
  id: string;
  scope: "loop" | "engineering" | "governance";
  objectType: string;
  objectId: string;
  summary: string;
  knownRisks: string[];
  knownBlockers: string[];
  suggestedNextSteps: string[];
  lastUpdatedAt: string;
  sourceTurnIds: string[];
}
