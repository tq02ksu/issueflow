<script setup lang="ts">
import { A2UISurface } from "a2ui-vue";

defineProps<{
  surfaceId: string;
}>();
</script>

<template>
  <div class="a2ui-surface-host">
    <A2UISurface :surface-id="surfaceId" />
  </div>
</template>

<style scoped>
.a2ui-surface-host {
  padding: 16px;
  max-width: 100%;
  overflow-x: auto;
}
</style>
