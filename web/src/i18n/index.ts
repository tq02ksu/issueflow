import { createI18n } from "vue-i18n";
import { en } from "./locales/en";
import { zhCN } from "./locales/zh-CN";
import { persistLocale, resolveInitialLocale, type AppLocale } from "./locale";

export type { AppLocale } from "./locale";

export const messages = {
  en,
  "zh-CN": zhCN,
} as const;

export const i18n = createI18n({
  legacy: false,
  locale: resolveInitialLocale(),
  fallbackLocale: "en",
  messages,
});

export function setLocale(locale: AppLocale) {
  i18n.global.locale.value = locale;
  persistLocale(locale);
}
