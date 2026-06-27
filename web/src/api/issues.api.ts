import { apiFetch } from "@/utils/api";
import { authHeaders } from "./helpers";

export interface GitlabIssue {
  id: number;
  iid: number;
  project_id: number;
  title: string;
  description: string | null;
  state: string;
  web_url: string;
  created_at: string | null;
  updated_at: string | null;
}

export async function listProjectIssues(projectId: number): Promise<GitlabIssue[]> {
  const resp = await apiFetch(`/api/projects/${projectId}/issues`, {
    headers: authHeaders(),
  });
  if (!resp.ok) return [];
  return resp.json();
}
