<template>
  <n-card :bordered="false" class="settings-card">
    <template #header>Skills</template>
    <div class="settings-card__body">
      <div
        v-for="skill in skills"
        :key="skill.id"
        class="skill-panel__skill"
      >
        <div class="skill-panel__skill-header">
          <div>
            <strong>{{ skill.name }}</strong>
            <p>{{ skill.id }}</p>
          </div>
          <n-button tertiary @click="$emit('mockUpload')">Upload version</n-button>
        </div>

        <div class="skill-panel__versions">
          <article
            v-for="version in skill.versions"
            :key="version.id"
            class="skill-panel__version"
          >
            <div>
              <strong>{{ version.id }}</strong>
              <p>{{ version.uiProfile.tone }} / {{ version.uiProfile.density }}</p>
            </div>
            <div class="skill-panel__actions">
              <n-switch
                :value="version.enabled"
                @update:value="$emit('toggleVersion', version.id, $event)"
              />
              <n-button
                size="small"
                :type="activeVersionId === version.id ? 'primary' : 'default'"
                @click="$emit('setActiveVersion', version.id)"
              >
                {{ activeVersionId === version.id ? "Active" : "Make active" }}
              </n-button>
            </div>
          </article>
        </div>
      </div>
    </div>
  </n-card>
</template>

<script setup lang="ts">
import { NButton, NCard, NSwitch } from "naive-ui";
import type { PrototypeSkill } from "@/mock/prototype.types";

defineProps<{
  skills: PrototypeSkill[];
  activeVersionId: string | undefined;
}>();

defineEmits<{
  setActiveVersion: [versionId: string];
  toggleVersion: [versionId: string, enabled: boolean];
  mockUpload: [];
}>();
</script>

<style scoped>
.settings-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.9);
}

.settings-card__body {
  display: grid;
  gap: 16px;
}

.skill-panel__skill {
  display: grid;
  gap: 12px;
}

.skill-panel__skill-header,
.skill-panel__version,
.skill-panel__actions {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}

.skill-panel__skill-header p,
.skill-panel__version p {
  margin: 4px 0 0;
  color: var(--if-color-muted);
}

.skill-panel__versions {
  display: grid;
  gap: 10px;
}

.skill-panel__version {
  padding: 14px;
  border-radius: var(--if-radius-sm);
  border: 1px solid rgba(216, 204, 184, 0.9);
  background: rgba(255, 255, 255, 0.7);
}
</style>
