import { defineStore } from "pinia";
import { reactive, ref } from "vue";

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
  created_at: string;
}

export interface GitLabProject {
  id: number;
  name: string;
  path_with_namespace: string;
  namespace: { id: number; name: string; kind: string };
}

type IssueFlowPhase = "idle" | "draft" | "confirming" | "created";

export const useSessionStore = defineStore("session", () => {
  const oidcResult = reactive({ value: "idle" as OidcResult, reason: "" });

  const draft = reactive<{ value: IssueDraft | null }>({ value: null });
  const created = reactive<{ value: CreatedIssue | null }>({ value: null });
  const phase = reactive<{ value: IssueFlowPhase }>({ value: "idle" });
  const workbenches = ref<Workbench[]>([]);
  const currentWorkbenchId = reactive<{ value: number | null }>({ value: null });

  function captureOidcResult(result: OidcResult, reason = "") {
    oidcResult.value = result;
    oidcResult.reason = reason;
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
  }

  return {
    oidcResult,
    draft,
    created,
    phase,
    workbenches,
    currentWorkbenchId,
    captureOidcResult,
    setDraft,
    confirmDraft,
    setCreated,
    setWorkbenches,
    setCurrentWorkbench,
  };
});
