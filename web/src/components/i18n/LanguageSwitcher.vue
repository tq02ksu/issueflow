<template>
  <div
    class="language-switcher"
    role="group"
    :aria-label="t('common.locale.label')"
  >
    <button
      v-for="option in options"
      :key="option.value"
      class="language-switcher__button"
      :class="{ 'language-switcher__button--active': locale === option.value }"
      type="button"
      :data-locale="option.value"
      @click="onSelect(option.value)"
    >
      {{ option.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useI18n } from "vue-i18n";
import { setLocale, type AppLocale } from "@/i18n";

const { t, locale } = useI18n();

const options = computed(() => [
  { value: "en" as const, label: t("common.locale.english") },
  { value: "zh-CN" as const, label: t("common.locale.chinese") },
]);

function onSelect(value: AppLocale) {
  setLocale(value);
}
</script>

<style scoped>
.language-switcher {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  padding: 4px;
  border: 1px solid rgba(216, 204, 184, 0.85);
  border-radius: 999px;
  background: rgba(255, 255, 255, 0.82);
  backdrop-filter: blur(12px);
}

.language-switcher__button {
  border: 0;
  border-radius: 999px;
  padding: 7px 11px;
  background: transparent;
  color: var(--if-color-muted);
  font: inherit;
  font-size: 12px;
  font-weight: 700;
  cursor: pointer;
  transition:
    background-color 140ms ease,
    color 140ms ease;
}

.language-switcher__button--active {
  background: rgba(15, 118, 110, 0.14);
  color: var(--if-color-accent-strong);
}
</style>
