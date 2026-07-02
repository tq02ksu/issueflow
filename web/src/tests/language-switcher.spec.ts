import { mount } from "@vue/test-utils";
import { afterEach, describe, expect, it } from "vitest";
import { i18n } from "@/i18n";
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";

describe("LanguageSwitcher", () => {
  afterEach(() => {
    localStorage.clear();
    i18n.global.locale.value = "en";
  });

  it("switches the active locale and persists it", async () => {
    localStorage.clear();
    i18n.global.locale.value = "en";

    const wrapper = mount(LanguageSwitcher, {
      global: { plugins: [i18n] },
    });

    await wrapper.get('[data-locale="zh-CN"]').trigger("click");

    expect(i18n.global.locale.value).toBe("zh-CN");
    expect(localStorage.getItem("issueflow.locale")).toBe("zh-CN");
  });
});
