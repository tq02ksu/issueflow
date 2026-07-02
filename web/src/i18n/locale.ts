export const SUPPORTED_LOCALES = ["en", "zh-CN"] as const;

export type AppLocale = (typeof SUPPORTED_LOCALES)[number];

export const DEFAULT_LOCALE: AppLocale = "en";
export const LOCALE_STORAGE_KEY = "issueflow.locale";

export function isSupportedLocale(value: string | null): value is AppLocale {
  return value === "en" || value === "zh-CN";
}

export function normalizeBrowserLocale(value: string | undefined): AppLocale {
  if (!value) {
    return DEFAULT_LOCALE;
  }

  if (value.toLowerCase().startsWith("zh")) {
    return "zh-CN";
  }

  return DEFAULT_LOCALE;
}

export function resolveInitialLocale(): AppLocale {
  const stored = localStorage.getItem(LOCALE_STORAGE_KEY);
  if (isSupportedLocale(stored)) {
    return stored;
  }

  return normalizeBrowserLocale(globalThis.navigator?.language);
}

export function persistLocale(locale: AppLocale) {
  localStorage.setItem(LOCALE_STORAGE_KEY, locale);
}
