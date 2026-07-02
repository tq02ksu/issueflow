<template>
  <n-card class="action-card" :bordered="false">
    <template #header>
      <div class="action-card__header">
        <span>{{ title }}</span>
        <span class="action-card__tone">{{ tone }}</span>
      </div>
    </template>
    <div class="action-card__list">
      <article
        v-for="action in actions"
        :key="action.id"
        class="action-card__item"
      >
        <strong>{{ action.title }}</strong>
        <p>{{ action.summary }}</p>
        <span>{{ action.owner }}</span>
      </article>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { NCard } from "naive-ui";
import type { PrototypeRecommendedAction } from "@/mock/prototype.types";

withDefaults(
  defineProps<{
    title?: string;
    tone?: string;
    actions: PrototypeRecommendedAction[];
  }>(),
  {
    title: "Recommended next actions",
    tone: "direct",
  },
);
</script>

<style scoped>
.action-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.88);
}

.action-card__header {
  display: flex;
  justify-content: space-between;
  gap: 12px;
  align-items: center;
  font-weight: 700;
}

.action-card__tone {
  color: var(--if-color-muted);
  font-size: 12px;
  text-transform: uppercase;
}

.action-card__list {
  display: grid;
  gap: 12px;
}

.action-card__item {
  padding: 14px;
  border: 1px solid rgba(216, 204, 184, 0.9);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}

.action-card__item p {
  margin: 8px 0 10px;
  color: var(--if-color-muted);
  line-height: 1.5;
}

.action-card__item span {
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
}
</style>
