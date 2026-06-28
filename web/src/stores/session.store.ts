import { defineStore } from "pinia";
import { reactive, ref } from "vue";
import { me, type UserInfo } from "@/api/auth.api";
import {
  list as listWorkbenches,
  create as createWorkbench,
  getCapabilities,
  type Workbench,
  type Capabilities,
} from "@/api/workbench.api";
import type { GitLabProject } from "@/api/projects.api";

export type { Workbench, Capabilities, GitLabProject, UserInfo };

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

export const useSessionStore = defineStore("session", () => {
  const token = ref<string | null>(loadToken());

  const oidcResult = reactive({ value: "idle" as OidcResult, reason: "" });
  const user = ref<UserInfo | null>(null);

  const draft = reactive<{ value: IssueDraft | null }>({ value: null });
  const created = reactive<{ value: CreatedIssue | null }>({ value: null });
  const phase = reactive<{ value: IssueFlowPhase }>({ value: "idle" });
  const workbenches = ref<Workbench[]>([]);
  const currentWorkbenchId = reactive<{ value: number | null }>({
    value: null,
  });
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
    const info = await me(token.value);
    if (info) {
      user.value = info;
      return true;
    }
    clearToken();
    token.value = null;
    return false;
  }

  async function fetchWorkbenches(): Promise<Workbench[]> {
    const list = await listWorkbenches();
    setWorkbenches(list);
    return list;
  }

  async function addWorkbench(input: {
    project_id: number;
    project_path: string;
    name: string;
  }): Promise<Workbench | null> {
    const wb = await createWorkbench(input);
    if (wb) {
      setWorkbenches([...workbenches.value, wb]);
    }
    return wb;
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
      const caps = await getCapabilities(workbenchId);
      if (caps) {
        capabilities.value = caps;
      }
    } catch {
      // capabilities fetch is best-effort
    }
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
    fetchWorkbenches,
    addWorkbench,
    setDraft,
    confirmDraft,
    setCreated,
    setWorkbenches,
    setCurrentWorkbench,
    fetchCapabilities,
  };
});
