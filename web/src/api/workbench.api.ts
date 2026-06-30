import { apiFetch } from "@/utils/api";
import { authHeaders } from "./helpers";

export interface Workbench {
  id: number;
  user_id?: number;
  project_id: number;
  project_name?: string;
  project_path: string;
  name: string;
  created_at?: string;
  updated_at?: string;
}

export type CapabilityFeature =
  | "overview"
  | "issues"
  | "pending_actions"
  | "releases";

export interface Capabilities {
  features: CapabilityFeature[];
}

export interface EngineeringMemory {
  id: string;
  project_id: number;
  artifact_type: string;
  artifact_id: string;
  status: string;
  revision: number;
  updated_by_user_id?: number | null;
  input_text: string;
  input_context: string;
  spec: string;
  validation_suggestions: string;
  risk_notes: string;
  evaluation_summary: string;
  created_at?: string;
  updated_at?: string;
}

export interface RefreshEngineeringMemoryInput {
  project_id: number;
  artifact_type: string;
  artifact_id: string;
  input_text: string;
  input_context: unknown;
  spec: unknown;
  validation_suggestions: unknown;
  risk_notes: unknown;
  evaluation_summary: unknown;
}

export interface PendingAction {
  id: string;
  workbench_id: number;
  project_id: number;
  artifact_type: string;
  artifact_id: string;
  action_type: string;
  status: string;
  payload: string;
  source_session_id?: string | null;
  source_run_id?: string | null;
  created_by_user_id?: number | null;
  assigned_user_id?: number | null;
  confirmed_by_user_id?: number | null;
  executed_run_id?: string | null;
  created_at?: string;
  updated_at?: string;
}

export interface PendingActionPreview {
  kind: string;
  title: string;
  body: string;
}

export interface PendingActionDetail {
  action: PendingAction;
  preview: PendingActionPreview | null;
}

export interface IssueStateEvaluation {
  current_state?: string;
  currentState?: string;
  proposed_next_state?: string;
  proposedNextState?: string;
  summary: string;
  missing_context?: string[];
  missingContext?: string[];
  blockers?: string[];
  role_notes?: {
    product?: string[];
    engineering?: string[];
    delivery?: string[];
  };
  roleNotes?: {
    product?: string[];
    engineering?: string[];
    delivery?: string[];
  };
}

export interface EngineeringMemoryRecord {
  id: string;
  artifact_type: string;
  artifact_id: string;
  scope_type: string;
  scope_key: string;
  scope_project_id?: number | null;
  scope_workbench_id?: number | null;
  scope_user_id?: number | null;
  memory_kind: string;
  status: string;
  revision: number;
  updated_by_user_id?: number | null;
  input_text: string;
  input_context: string;
  source_snapshot?: string | null;
  spec: string;
  validation_suggestions: string;
  risk_notes: string;
  evaluation_summary: string;
  created_at?: string;
  updated_at?: string;
}

export interface IssueStateDetail {
  projectMemory: EngineeringMemoryRecord | null;
  personalNote: EngineeringMemoryRecord | null;
  pendingAction: PendingAction | null;
}

export interface IssueStateEvaluationResult {
  workbenchContext: EngineeringMemoryRecord;
  projectState: EngineeringMemoryRecord;
  pendingAction: PendingAction;
}

export interface CreatePendingActionInput {
  project_id: number;
  artifact_type: string;
  artifact_id: string;
  action_type: string;
  payload: unknown;
  source_session_id?: string | null;
  source_run_id?: string | null;
  assigned_user_id?: number | null;
}

export async function list(): Promise<Workbench[]> {
  const resp = await apiFetch("/api/workbenches", { headers: authHeaders() });
  if (!resp.ok) return [];
  return resp.json();
}

export interface CreateWorkbenchInput {
  project_id: number;
  project_path: string;
  name: string;
}

export async function create(
  input: CreateWorkbenchInput,
): Promise<Workbench | null> {
  const resp = await apiFetch("/api/workbenches", {
    method: "POST",
    headers: { ...authHeaders(), "Content-Type": "application/json" },
    body: JSON.stringify(input),
  });
  if (!resp.ok) return null;
  return resp.json();
}

export interface UpdateWorkbenchInput {
  project_id: number;
  project_path: string;
  name: string;
}

export async function update(
  id: number,
  input: UpdateWorkbenchInput,
): Promise<Workbench | null> {
  const resp = await apiFetch(`/api/workbenches/${id}`, {
    method: "PUT",
    headers: { ...authHeaders(), "Content-Type": "application/json" },
    body: JSON.stringify(input),
  });
  if (!resp.ok) return null;
  return resp.json();
}

export async function getCapabilities(
  id: number,
): Promise<Capabilities | null> {
  const resp = await apiFetch(`/api/workbenches/${id}/capabilities`, {
    headers: authHeaders(),
  });
  if (!resp.ok) return null;
  return resp.json();
}

export async function refreshEngineeringMemory(
  workbenchId: number,
  input: RefreshEngineeringMemoryInput,
): Promise<EngineeringMemory | null> {
  const resp = await apiFetch(
    `/api/workbenches/${workbenchId}/memory/refresh`,
    {
      method: "POST",
      headers: { ...authHeaders(), "Content-Type": "application/json" },
      body: JSON.stringify(input),
    },
  );
  if (!resp.ok) return null;
  return resp.json();
}

export async function createPendingAction(
  workbenchId: number,
  input: CreatePendingActionInput,
): Promise<PendingAction | null> {
  const resp = await apiFetch(
    `/api/workbenches/${workbenchId}/pending-actions`,
    {
      method: "POST",
      headers: { ...authHeaders(), "Content-Type": "application/json" },
      body: JSON.stringify(input),
    },
  );
  if (!resp.ok) return null;
  return resp.json();
}

export async function listPendingActions(
  workbenchId: number,
): Promise<PendingAction[]> {
  const resp = await apiFetch(
    `/api/workbenches/${workbenchId}/pending-actions`,
    {
      headers: authHeaders(),
    },
  );
  if (!resp.ok) return [];
  return resp.json();
}

export async function getPendingAction(
  id: string,
): Promise<PendingActionDetail | null> {
  const resp = await apiFetch(`/api/pending-actions/${id}`, {
    headers: authHeaders(),
  });
  if (!resp.ok) return null;
  return resp.json();
}

export async function confirmPendingAction(
  id: string,
): Promise<PendingAction | null> {
  const resp = await apiFetch(`/api/pending-actions/${id}/confirm`, {
    method: "POST",
    headers: authHeaders(),
  });
  if (!resp.ok) return null;
  return resp.json();
}

export async function getIssueState(
  workbenchId: number,
  issueIid: number,
): Promise<IssueStateDetail | null> {
  const resp = await apiFetch(
    `/api/workbenches/${workbenchId}/issues/${issueIid}/state`,
    {
      headers: authHeaders(),
    },
  );
  if (!resp.ok) return null;
  return resp.json();
}

export async function evaluateIssueState(
  workbenchId: number,
  issueIid: number,
): Promise<IssueStateEvaluationResult | null> {
  const resp = await apiFetch(
    `/api/workbenches/${workbenchId}/issues/${issueIid}/state/evaluate`,
    {
      method: "POST",
      headers: authHeaders(),
    },
  );
  if (!resp.ok) return null;
  return resp.json();
}
