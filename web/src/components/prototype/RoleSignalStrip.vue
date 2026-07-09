<template>
  <div v-if="role" class="signal-strip">
    <div class="signal-strip__cards">
      <article
        v-for="card in role.signalCards"
        :key="card.id"
        class="signal-card"
        :class="`signal-card--${card.tone}`"
      >
        <div class="signal-card__label">{{ card.label }}</div>
        <div class="signal-card__value">{{ card.value }}</div>
        <div class="signal-card__hint">{{ card.hint }}</div>
      </article>
    </div>
    <nav class="signal-strip__entries">
      <RouterLink
        v-for="entry in role.quickEntries"
        :key="entry.id"
        :to="entry.to"
        class="signal-entry"
      >
        {{ t(entry.labelKey) }}
      </RouterLink>
    </nav>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from "vue-router";
import { useI18n } from "vue-i18n";
import type { PrototypeRoleView } from "@/mock/prototype.types";

defineProps<{ role: PrototypeRoleView | null }>();

const { t } = useI18n();
</script>

<style scoped>
.signal-strip {
  display: grid;
  gap: 16px;
}
.signal-strip__cards {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 16px;
}
.signal-card {
  padding: 16px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}
.signal-card--attention {
  border-color: rgba(251, 191, 36, 0.5);
  background: rgba(251, 191, 36, 0.08);
}
.signal-card--positive {
  border-color: rgba(15, 118, 110, 0.35);
  background: rgba(15, 118, 110, 0.08);
}
.signal-card__label {
  color: var(--if-color-muted);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.04em;
}
.signal-card__value {
  margin: 8px 0 4px;
  font-size: 22px;
  font-weight: 800;
}
.signal-card__hint {
  color: var(--if-color-muted);
  font-size: 12px;
  line-height: 1.5;
}
.signal-strip__entries {
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}
.signal-entry {
  display: inline-flex;
  padding: 8px 14px;
  border-radius: 999px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  background: rgba(255, 255, 255, 0.7);
  color: var(--if-color-text);
  font-size: 13px;
  font-weight: 600;
  text-decoration: none;
}
.signal-entry:hover {
  border-color: var(--if-color-accent);
  color: var(--if-color-accent-strong);
}
</style>
