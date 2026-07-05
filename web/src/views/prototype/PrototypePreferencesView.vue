<template>
  <app-shell active-key="settings-preferences" prototype-mode>
    <div class="page">
      <div class="page__header">
        <h1>{{ t("shell.preferences") }}</h1>
      </div>

      <div class="section-grid">
        <n-card :bordered="false" class="section-card">
          <div class="card-body">
            <div class="field">
              <label>{{ t("prototype.settings.roleName") }}</label>
              <n-input v-model:value="name" />
            </div>
            <div class="field">
              <label>{{ t("prototype.preferences.personality") }}</label>
              <n-input
                v-model:value="personality"
                type="textarea"
                :autosize="{ minRows: 3, maxRows: 5 }"
                :placeholder="t('prototype.preferences.personalityPlaceholder')"
              />
            </div>
            <div class="field">
              <label>{{ t("prototype.preferences.waysOfWorking") }}</label>
              <n-input
                v-model:value="ways"
                type="textarea"
                :autosize="{ minRows: 3, maxRows: 6 }"
                :placeholder="t('prototype.preferences.waysOfWorkingPlaceholder')"
              />
            </div>
            <div class="field">
              <label>{{ t("prototype.settings.defaultGoal") }}</label>
              <n-input
                v-model:value="goal"
                type="textarea"
                :autosize="{ minRows: 2, maxRows: 4 }"
                :placeholder="t('prototype.preferences.goalPlaceholder')"
              />
            </div>
            <n-button type="primary" size="small" @click="save">
              {{ t("common.actions.save") }}
            </n-button>
          </div>
        </n-card>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { NButton, NCard, NInput } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();

const name = ref("");
const personality = ref("");
const ways = ref("");
const goal = ref("");

watch(
  () => store.currentUserSoul,
  (soul) => {
    name.value = soul.name;
    personality.value = soul.personality;
    ways.value = soul.waysOfWorking.join("\n");
    goal.value = soul.defaultGoal;
  },
  { immediate: true },
);

function save() {
  store.updateUserSoul({
    personality: personality.value.trim(),
    waysOfWorking: ways.value
      .split("\n")
      .map((s) => s.trim())
      .filter(Boolean),
    defaultGoal: goal.value.trim(),
  });
}
</script>

<style scoped>
.page {
  max-width: 600px;
}

.page__header h1 {
  margin: 0 0 20px;
  font-size: 22px;
}

.section-grid {
  display: grid;
  gap: 16px;
}

.section-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.85);
}

.card-body {
  display: grid;
  gap: 14px;
}

.field {
  display: grid;
  gap: 6px;
}

.field label {
  font-size: 13px;
  font-weight: 600;
}
</style>
