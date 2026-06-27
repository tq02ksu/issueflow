import { defineStore } from "pinia";
import { reactive } from "vue";

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

export const useSessionStore = defineStore("session", () => {
  const oidcResult = reactive({ value: "idle" as OidcResult, reason: "" });

  const draft = reactive<{ value: IssueDraft | null }>({ value: null });
  const created = reactive<{ value: CreatedIssue | null }>({ value: null });
  const phase = reactive<{ value: IssueFlowPhase }>({ value: "idle" });

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

  return {
    oidcResult,
    draft,
    created,
    phase,
    captureOidcResult,
    setDraft,
    confirmDraft,
    setCreated,
  };
});
