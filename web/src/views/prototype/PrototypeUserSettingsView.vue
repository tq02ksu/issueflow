<template>
  <app-shell active-key="overview" prototype-mode>
    <div class="settings-page">
      <div class="settings-page__header">
        <div class="settings-page__eyebrow">User Settings</div>
        <h1>Shape the operator, not the page skeleton</h1>
        <p>
          Tune user soul, workbench role, memory controls, and skill versions
          without moving workflow navigation out of the workbench.
        </p>
      </div>

      <div class="settings-grid">
        <n-card :bordered="false" class="settings-card">
          <template #header>User soul</template>
          <div class="settings-card__body">
            <n-input
              v-model:value="personality"
              type="textarea"
              :autosize="{ minRows: 4, maxRows: 6 }"
              placeholder="How this user works"
            />
            <n-input
              v-model:value="ways"
              type="textarea"
              :autosize="{ minRows: 4, maxRows: 6 }"
              placeholder="One operating principle per line"
            />
            <n-input
              v-model:value="goal"
              type="textarea"
              :autosize="{ minRows: 2, maxRows: 4 }"
              placeholder="Default goal"
            />
            <n-button type="primary" @click="saveSoul">Save user soul</n-button>
          </div>
        </n-card>

        <WorkbenchRolePanel
          :role="store.currentWorkbench?.role ?? null"
          @update-role="store.updateWorkbenchRole"
        />

        <SkillVersionPanel
          :skills="store.availableSkills"
          :active-version-id="store.currentWorkbench?.activeSkillVersionId"
          @set-active-version="store.setActiveSkillVersion"
          @toggle-version="store.toggleSkillVersion"
          @mock-upload="store.mockUploadSkill"
        />

        <MemoryControlPanel
          :scopes="store.currentMemoryScopes"
          :last-action="store.lastMemoryAction"
          @clear-memory="store.clearWorkbenchMemory"
          @rebuild-memory="store.rebuildWorkbenchMemory"
        />
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { NButton, NCard, NInput } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import MemoryControlPanel from "@/components/prototype/MemoryControlPanel.vue";
import SkillVersionPanel from "@/components/prototype/SkillVersionPanel.vue";
import WorkbenchRolePanel from "@/components/prototype/WorkbenchRolePanel.vue";
import { usePrototypeStore } from "@/stores/prototype.store";

const store = usePrototypeStore();

const personality = ref("");
const ways = ref("");
const goal = ref("");

watch(
  () => store.currentUserSoul,
  (soul) => {
    personality.value = soul.personality;
    ways.value = soul.waysOfWorking.join("\n");
    goal.value = soul.defaultGoal;
  },
  { immediate: true },
);

function saveSoul() {
  store.updateUserSoul({
    personality: personality.value.trim(),
    waysOfWorking: ways.value
      .split("\n")
      .map((item) => item.trim())
      .filter(Boolean),
    defaultGoal: goal.value.trim(),
  });
}
</script>

<style scoped>
.settings-page {
  display: grid;
  gap: 24px;
}

.settings-page__eyebrow {
  margin-bottom: 10px;
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.settings-page__header h1 {
  margin: 0 0 8px;
}

.settings-page__header p {
  max-width: 760px;
  margin: 0;
  color: var(--if-color-muted);
  line-height: 1.6;
}

.settings-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 20px;
}

.settings-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.9);
}

.settings-card__body {
  display: grid;
  gap: 12px;
}

@media (max-width: 900px) {
  .settings-grid {
    grid-template-columns: 1fr;
  }
}
</style>
