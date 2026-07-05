<template>
  <app-shell active-key="loops-create" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div class="prototype-page__eyebrow">{{ t("prototype.loops.createEyebrow") }}</div>
        <h1>{{ t("prototype.loops.createTitle") }}</h1>
      </div>
      <div class="create-form">
        <n-card :bordered="false" class="create-card">
          <template #header>{{ t("prototype.loops.quickCreate") }}</template>
          <div class="create-card__body">
            <n-select
              v-model:value="loopType"
              :options="typeOptions"
              :placeholder="t('prototype.loops.selectLoopType')"
            />
            <n-input
              v-model:value="loopName"
              :placeholder="t('prototype.loops.namePlaceholder')"
            />
            <n-select
              v-model:value="boundObject"
              :options="boundOptions"
              :placeholder="t('prototype.loops.selectBoundObject')"
            />
            <n-select
              v-model:value="schedule"
              :options="scheduleOptions"
              :placeholder="t('prototype.loops.selectSchedule')"
            />
            <n-button type="primary" @click="onSubmit">
              {{ t("prototype.loops.create") }}
            </n-button>
            <p class="create-card__note">{{ t("prototype.loops.mockNote") }}</p>
          </div>
        </n-card>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { NButton, NCard, NInput, NSelect } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();

const loopType = ref<string | null>(null);
const loopName = ref("");
const boundObject = ref<string | null>(null);
const schedule = ref<string | null>(null);

const typeOptions = [
  { label: "Issue", value: "issue" },
  { label: "MR", value: "mr" },
  { label: "Milestone", value: "milestone" },
];

const boundOptions = computed(() =>
  store.visibleIssues.map((i) => ({
    label: `Issue #${i.iid} — ${i.title}`,
    value: `issue-${i.id}`,
  })),
);

const scheduleOptions = [
  { label: "Every 1 hour", value: "hourly" },
  { label: "Every 4 hours", value: "4h" },
  { label: "Daily", value: "daily" },
  { label: "Weekly", value: "weekly" },
  { label: "On event", value: "event" },
];

function onSubmit() {
  // mock: just show feedback
  window.alert(t("prototype.loops.created"));
}
</script>

<style scoped>
.prototype-page {
  display: grid;
  gap: 24px;
  max-width: 640px;
}

.prototype-page__eyebrow {
  margin-bottom: 10px;
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.prototype-page__header h1 {
  margin: 0;
}

.create-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}

.create-card__body {
  display: grid;
  gap: 14px;
}

.create-card__note {
  color: var(--if-color-muted);
  font-size: 13px;
}
</style>
