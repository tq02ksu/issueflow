<template>
  <app-shell active-key="overview">
    <n-card
      :bordered="false"
      class="panel"
    >
      <template #header>
        <span>Overview</span>
      </template>
      <h2>Agent Workbench skeleton</h2>
      <p>
        This initial workspace proves the frontend stack, route structure, and Gateway
        integration without inventing domain modules early.
      </p>
    </n-card>
  </app-shell>
</template>

<script setup lang="ts">
import { NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
</script>

<style scoped>
.panel {
  max-width: 720px;
  border-radius: var(--if-radius-md);
  box-shadow: var(--if-shadow-panel);
}

h2 {
  margin: 0 0 12px;
}

p {
  margin: 0;
  color: var(--if-color-muted);
  line-height: 1.6;
}
</style>
