import { defineStore } from "pinia";

type OidcResult = "idle" | "success" | "error";

export const useSessionStore = defineStore("session", {
  state: () => ({
    oidcResult: "idle" as OidcResult,
    oidcReason: "",
  }),
  actions: {
    captureOidcResult(result: OidcResult, reason = "") {
      this.oidcResult = result;
      this.oidcReason = reason;
    },
  },
});
