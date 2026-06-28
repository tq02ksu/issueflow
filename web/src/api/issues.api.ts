import { apiFetch } from "@/utils/api";
import { authHeaders } from "./helpers";

export interface MilestoneRef {
  id: number;
  title: string;
}

export interface GitlabIssue {
  id: number;
  iid: number;
  project_id: number;
  title: string;
  description: string | null;
  state: string;
  web_url: string;
  milestone: MilestoneRef | null;
  labels: string[];
  created_at: string | null;
  updated_at: string | null;
}

export interface Milestone {
  id: number;
  iid: number;
  title: string;
  description: string | null;
  state: string;
  due_date: string | null;
  web_url: string;
}

export interface IssueNote {
  id: number;
  body: string;
  author_name: string;
  created_at: string;
}

export async function listProjectIssues(
  projectId: number,
): Promise<GitlabIssue[]> {
  const resp = await apiFetch(`/api/projects/${projectId}/issues`, {
    headers: authHeaders(),
  });
  if (!resp.ok) return [];
  return resp.json();
}

export async function listMilestones(projectId: number): Promise<Milestone[]> {
  const resp = await apiFetch(`/api/projects/${projectId}/milestones`, {
    headers: authHeaders(),
  });
  if (!resp.ok) return [];
  return resp.json();
}

export async function listIssueNotes(
  projectId: number,
  issueIid: number,
): Promise<IssueNote[]> {
  const resp = await apiFetch(
    `/api/projects/${projectId}/issues/${issueIid}/notes`,
    {
      headers: authHeaders(),
    },
  );
  if (!resp.ok) return [];
  return resp.json();
}
