import { defineStore } from "pinia";

type OAuthResult = "idle" | "success" | "error";

export const useSessionStore = defineStore("session", {
  state: () => ({
    oauthResult: "idle" as OAuthResult,
    oauthReason: "",
  }),
  actions: {
    captureOAuthResult(result: OAuthResult, reason = "") {
      this.oauthResult = result;
      this.oauthReason = reason;
    },
  },
});
