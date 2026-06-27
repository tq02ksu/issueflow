import { apiFetch } from "@/utils/api";
import { authHeaders } from "./helpers";

export interface GitLabProject {
  id: number;
  name: string;
  path_with_namespace: string;
  namespace: { id: number; name: string; kind: string };
}

export async function search(query: string): Promise<GitLabProject[]> {
  const resp = await apiFetch(`/api/projects?search=${encodeURIComponent(query)}`, {
    headers: authHeaders(),
  });
  if (!resp.ok) return [];
  return resp.json();
}
