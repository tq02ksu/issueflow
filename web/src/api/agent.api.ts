import { apiFetch } from "@/utils/api";
import { authHeaders } from "./helpers";

export async function listSessions(workbenchId: number) {
  const resp = await apiFetch(`/api/workbenches/${workbenchId}/agent-sessions`, {
    headers: authHeaders(),
  });
  return resp.ok ? resp.json() : [];
}

export async function createSession(workbenchId: number) {
  const resp = await apiFetch(`/api/workbenches/${workbenchId}/agent-sessions`, {
    method: "POST",
    headers: { ...authHeaders(), "Content-Type": "application/json" },
    body: JSON.stringify({}),
  });
  return resp.ok ? resp.json() : null;
}

export async function getSession(workbenchId: number, id: string) {
  const resp = await apiFetch(
    `/api/workbenches/${workbenchId}/agent-sessions/${id}`,
    { headers: authHeaders() },
  );
  return resp.ok ? resp.json() : null;
}

export async function deleteSession(workbenchId: number, id: string) {
  await apiFetch(`/api/workbenches/${workbenchId}/agent-sessions/${id}`, {
    method: "DELETE",
    headers: authHeaders(),
  });
}

export async function renameSession(
  workbenchId: number,
  id: string,
  title: string,
) {
  await apiFetch(`/api/workbenches/${workbenchId}/agent-sessions/${id}`, {
    method: "PATCH",
    headers: { ...authHeaders(), "Content-Type": "application/json" },
    body: JSON.stringify({ title }),
  });
}

export async function createRun(body: unknown) {
  const resp = await apiFetch("/api/agent/runs", {
    method: "POST",
    headers: { ...authHeaders(), "Content-Type": "application/json" },
    body: JSON.stringify(body),
  });
  return resp.ok ? resp.json() : null;
}

export function subscribeRunEvents(runId: string, afterSeq = 0) {
  const token = localStorage.getItem("issueflow_token");
  return fetch(`/api/agent/runs/${runId}/events?after_seq=${afterSeq}`, {
    headers: {
      Authorization: `Bearer ${token}`,
    },
  });
}
