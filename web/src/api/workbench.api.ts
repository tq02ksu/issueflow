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

export interface Capabilities {
  features: string[];
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

export async function create(input: CreateWorkbenchInput): Promise<Workbench | null> {
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

export async function update(id: number, input: UpdateWorkbenchInput): Promise<Workbench | null> {
  const resp = await apiFetch(`/api/workbenches/${id}`, {
    method: "PUT",
    headers: { ...authHeaders(), "Content-Type": "application/json" },
    body: JSON.stringify(input),
  });
  if (!resp.ok) return null;
  return resp.json();
}

export async function getCapabilities(id: number): Promise<Capabilities | null> {
  const resp = await apiFetch(`/api/workbenches/${id}/capabilities`, {
    headers: authHeaders(),
  });
  if (!resp.ok) return null;
  return resp.json();
}
