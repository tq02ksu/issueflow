<template>
  <app-shell active-key="skills" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div class="prototype-page__eyebrow">{{ t("prototype.skillsPage.eyebrow") }}</div>
        <h1>{{ t("prototype.skillsPage.title") }}</h1>
        <p>{{ t("prototype.skillsPage.description") }}</p>
      </div>

      <div class="skills-grid">
        <n-card :bordered="false" class="skills-card" v-for="skill in store.availableSkills" :key="skill.id">
          <template #header>
            <div class="skills-card__header">
              <span>{{ skill.name }}</span>
              <span class="skills-card__id">{{ skill.id }}</span>
            </div>
          </template>
          <div class="skills-card__body">
            <div class="skills-card__versions">
              <article class="skills-version" v-for="v in skill.versions" :key="v.id">
                <div>
                  <strong>{{ v.id }}</strong>
                  <p>{{ v.uiProfile.tone }} / {{ v.uiProfile.density }}</p>
                </div>
                <div class="skills-version__meta">
                  <span class="skills-version__status" :class="v.enabled ? 'skills-version__status--enabled' : 'skills-version__status--disabled'">
                    {{ v.enabled ? "enabled" : "disabled" }}
                  </span>
                  <span v-if="v.id === store.currentWorkbench?.activeSkillVersionId" class="skills-version__active">active</span>
                </div>
              </article>
            </div>
          </div>
        </n-card>

        <n-card :bordered="false" class="skills-card skills-card--full">
          <template #header>{{ t("prototype.skillsPage.bindings") }}</template>
          <div class="bindings-list">
            <article class="bindings-item" v-for="loop in store.workbenchLoops" :key="loop.id">
              <strong>{{ loop.name }}</strong>
              <div class="bindings-item__skills">
                <span v-for="ref in loop.skillRefs" :key="ref" class="bindings-chip">{{ ref }}</span>
              </div>
            </article>
          </div>
        </n-card>
      </div>
    </div>
  </app-shell>
</template>

<script setup lang="ts">
import { NCard } from "naive-ui";
import AppShell from "@/components/layout/AppShell.vue";
import { usePrototypeStore } from "@/stores/prototype.store";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();
</script>

<style scoped>
.prototype-page { display: grid; gap: 24px; }
.prototype-page__eyebrow { margin-bottom: 10px; color: var(--if-color-accent-strong); font-size: 12px; font-weight: 700; letter-spacing: 0.12em; text-transform: uppercase; }
.prototype-page__header h1 { margin: 0 0 8px; }
.prototype-page__header p { max-width: 640px; margin: 0; color: var(--if-color-muted); }
.skills-grid { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 20px; }
.skills-card { border-radius: var(--if-radius-lg); background: rgba(255, 250, 242, 0.92); }
.skills-card--full { grid-column: span 2; }
.skills-card__header { display: flex; justify-content: space-between; align-items: center; gap: 12px; }
.skills-card__id { color: var(--if-color-muted); font-size: 12px; }
.skills-card__body { display: grid; gap: 12px; }
.skills-card__versions { display: grid; gap: 10px; }
.skills-version { display: flex; justify-content: space-between; align-items: center; gap: 12px; padding: 12px; border: 1px solid rgba(216, 204, 184, 0.8); border-radius: var(--if-radius-sm); background: rgba(255, 255, 255, 0.7); }
.skills-version p { margin: 4px 0 0; color: var(--if-color-muted); font-size: 12px; }
.skills-version__meta { display: flex; gap: 6px; align-items: center; }
.skills-version__status { display: inline-flex; padding: 3px 8px; border-radius: 999px; font-size: 11px; font-weight: 700; }
.skills-version__status--enabled { background: rgba(15, 118, 110, 0.12); color: var(--if-color-accent); }
.skills-version__status--disabled { background: rgba(28, 34, 48, 0.06); color: var(--if-color-muted); }
.skills-version__active { background: rgba(21, 94, 117, 0.14); color: var(--if-color-accent-strong); padding: 3px 8px; border-radius: 999px; font-size: 11px; font-weight: 700; }
.bindings-list { display: grid; gap: 12px; }
.bindings-item { padding: 14px; border: 1px solid rgba(216, 204, 184, 0.8); border-radius: var(--if-radius-sm); background: rgba(255, 255, 255, 0.7); }
.bindings-item__skills { display: flex; gap: 6px; flex-wrap: wrap; margin-top: 8px; }
.bindings-chip { display: inline-flex; padding: 4px 10px; border-radius: 999px; background: rgba(21, 94, 117, 0.1); color: var(--if-color-accent-strong); font-size: 11px; font-weight: 700; }
@media (max-width: 900px) { .skills-grid { grid-template-columns: 1fr; } .skills-card--full { grid-column: span 1; } }
</style>
