<template>
  <app-shell active-key="overview" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div>
          <div class="prototype-page__eyebrow">{{ t("prototype.overview.eyebrow") }}</div>
          <h1>{{ t("prototype.overview.title") }}</h1>
          <p>{{ t("prototype.overview.description") }}</p>
        </div>
      </div>

      <OverviewCards
        :workbench="store.currentWorkbench"
        :issue-summary="store.issueWorkflowSummary"
        :mr-summary="store.mrWorkflowSummary"
        :actions="store.recommendedActions"
        :activity="store.activity"
        :ui-profile="store.activeUiProfile"
      />
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import AppShell from "@/components/layout/AppShell.vue";
import OverviewCards from "@/components/prototype/OverviewCards.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();
</script>

<style scoped>
.prototype-page {
  display: grid;
  gap: 24px;
}

.prototype-page__header h1 {
  margin: 0 0 8px;
  font-size: clamp(2rem, 4vw, 2.8rem);
}

.prototype-page__header p {
  max-width: 760px;
  margin: 0;
  color: var(--if-color-muted);
  line-height: 1.6;
}

.prototype-page__eyebrow {
  margin-bottom: 10px;
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}
</style>
