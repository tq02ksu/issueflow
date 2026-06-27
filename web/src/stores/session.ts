import { defineStore } from "pinia";
import { reactive, ref } from "vue";
import { apiFetch } from "@/utils/api";

type OidcResult = "idle" | "success" | "error";

export interface IssueDraft {
  projectId: number;
  title: string;
  description: string;
}

export interface CreatedIssue {
  id: number;
  iid: number;
  projectId: number;
  title: string;
  webUrl: string;
}

export interface Workbench {
  id: number;
  project_id: number;
  project_name: string;
  project_path: string;
  name: string;
  created_at: string;
}

export interface GitLabProject {
  id: number;
  name: string;
  path_with_namespace: string;
  namespace: { id: number; name: string; kind: string };
}

export interface UserInfo {
  user_id: number;
  sub: string;
}

export interface Capabilities {
  features: string[];
}

type IssueFlowPhase = "idle" | "draft" | "confirming" | "created";

const TOKEN_KEY = "issueflow_token";

function loadToken(): string | null {
  return localStorage.getItem(TOKEN_KEY);
}

function saveToken(token: string) {
  localStorage.setItem(TOKEN_KEY, token);
}

function clearToken() {
  localStorage.removeItem(TOKEN_KEY);
}

function authHeaders(): Record<string, string> {
  const token = loadToken();
  if (!token) return {};
  return { Authorization: `Bearer ${token}` };
}

async function authFetch(url: string, init?: RequestInit): Promise<Response> {
  return apiFetch(url, {
    ...init,
    headers: {
      ...authHeaders(),
      ...init?.headers,
    },
  });
}

export const useSessionStore = defineStore("session", () => {
  const oidcResult = reactive({ value: "idle" as OidcResult, reason: "" });
  const token = ref<string | null>(loadToken());
  const user = ref<UserInfo | null>(null);

  const draft = reactive<{ value: IssueDraft | null }>({ value: null });
  const created = reactive<{ value: CreatedIssue | null }>({ value: null });
  const phase = reactive<{ value: IssueFlowPhase }>({ value: "idle" });
  const workbenches = ref<Workbench[]>([]);
  const currentWorkbenchId = reactive<{ value: number | null }>({ value: null });
  const capabilities = ref<Capabilities>({ features: [] });

  function captureOidcResult(result: OidcResult, reason = "", jwt?: string) {
    oidcResult.value = result;
    oidcResult.reason = reason;
    if (jwt) {
      saveToken(jwt);
      token.value = jwt;
    }
  }

  async function checkAuth(): Promise<boolean> {
    if (!token.value) return false;
    try {
      const resp = await authFetch("/api/auth/me");
      if (resp.ok) {
        user.value = await resp.json();
        return true;
      }
    } catch {
      // network error
    }
    clearToken();
    token.value = null;
    return false;
  }

  function setDraft(d: IssueDraft) {
    draft.value = d;
    phase.value = "draft";
  }

  function confirmDraft() {
    phase.value = "confirming";
  }

  function setCreated(issue: CreatedIssue) {
    created.value = issue;
    phase.value = "created";
  }

  function setWorkbenches(list: Workbench[]) {
    workbenches.value = list;
  }

  function setCurrentWorkbench(id: number | null) {
    currentWorkbenchId.value = id;
    if (id !== null) {
      fetchCapabilities(id);
    } else {
      capabilities.value = { features: [] };
    }
  }

  async function fetchCapabilities(workbenchId: number) {
    try {
      const resp = await authFetch(`/api/workbenches/${workbenchId}/capabilities`);
      if (resp.ok) {
        capabilities.value = await resp.json();
      }
    } catch { /* ignore */ }
  }

  return {
    oidcResult,
    token,
    user,
    draft,
    created,
    phase,
    workbenches,
    currentWorkbenchId,
    capabilities,
    captureOidcResult,
    checkAuth,
    setDraft,
    confirmDraft,
    setCreated,
    setWorkbenches,
    setCurrentWorkbench,
    fetchCapabilities,
    authFetch,
  };
});
