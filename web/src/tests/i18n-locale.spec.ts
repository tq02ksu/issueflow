import { beforeEach, describe, expect, it, vi } from "vitest";

describe("frontend locale resolution", () => {
  beforeEach(() => {
    localStorage.clear();
    vi.restoreAllMocks();
  });

  it("prefers a persisted locale over browser language", async () => {
    localStorage.setItem("issueflow.locale", "zh-CN");
    vi.stubGlobal("navigator", { language: "en-US" });

    const { resolveInitialLocale } = await import("@/i18n/locale");
    expect(resolveInitialLocale()).toBe("zh-CN");
  });

  it("uses browser Chinese on first run", async () => {
    vi.stubGlobal("navigator", { language: "zh-CN" });

    const { resolveInitialLocale } = await import("@/i18n/locale");
    expect(resolveInitialLocale()).toBe("zh-CN");
  });

  it("falls back to English when browser language is unsupported", async () => {
    vi.stubGlobal("navigator", { language: "fr-FR" });

    const { resolveInitialLocale } = await import("@/i18n/locale");
    expect(resolveInitialLocale()).toBe("en");
  });
});
