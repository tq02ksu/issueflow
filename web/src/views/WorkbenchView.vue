<template>
  <app-shell active-key="overview">
    <n-card :bordered="false" class="panel">
      <template #header>
        <span>Agent Workbench</span>
      </template>

      <div v-if="store.phase.value === 'draft' || store.phase.value === 'confirming'">
        <issue-draft-card
          v-if="store.draft.value"
          data-test="issue-draft"
          :project-id="store.draft.value.projectId"
          :title="store.draft.value.title"
          :description="store.draft.value.description"
        />
        <n-button
          v-if="store.phase.value === 'draft'"
          data-test="confirm-issue-btn"
          type="primary"
          @click="store.confirmDraft()"
        >
          Confirm
        </n-button>
        <p v-if="store.phase.value === 'confirming'">Confirming...</p>
      </div>

      <div v-if="store.phase.value === 'created' && store.created.value" data-test="issue-created-result">
        <p>Created</p>
        <p>Issue #{{ store.created.value.iid }}: {{ store.created.value.title }}</p>
        <a :href="store.created.value.webUrl" target="_blank" rel="noopener">
          View in GitLab
        </a>
      </div>

      <div v-if="store.phase.value === 'idle'">
        <h2>Agent Workbench skeleton</h2>
        <p>
          This initial workspace proves the frontend stack, route structure, and Gateway
          integration without inventing domain modules early.
        </p>
      </div>
    </n-card>
  </app-shell>
</template>

<script setup lang="ts">
import { NButton, NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import IssueDraftCard from "@/components/issue/IssueDraftCard.vue";
import { useSessionStore } from "@/stores/session";

const store = useSessionStore();
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
