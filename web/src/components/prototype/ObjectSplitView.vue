<template>
  <div class="split-view" :class="`split-view--${density}`">
    <aside class="split-view__list">
      <slot name="list" />
    </aside>
    <section class="split-view__detail">
      <slot name="detail" />
    </section>
  </div>
</template>

<script setup lang="ts">
withDefaults(
  defineProps<{
    density?: "compact" | "balanced" | "relaxed";
  }>(),
  {
    density: "balanced",
  },
);
</script>

<style scoped>
.split-view {
  display: grid;
  grid-template-columns: minmax(280px, 360px) minmax(0, 1fr);
  gap: 20px;
  min-height: calc(100vh - 180px);
}

.split-view__list,
.split-view__detail {
  min-width: 0;
  border-radius: var(--if-radius-lg);
}

.split-view--compact {
  gap: 16px;
}

.split-view--relaxed {
  gap: 24px;
}

@media (max-width: 980px) {
  .split-view {
    grid-template-columns: 1fr;
  }
}
</style>
