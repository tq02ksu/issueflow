<template>
  <app-shell active-key="skills" prototype-mode>
    <div class="prototype-page">
      <div class="prototype-page__header">
        <div class="prototype-page__eyebrow">
          {{ t("prototype.skillsPage.eyebrow") }}
        </div>
        <h1>{{ t("prototype.skillsPage.title") }}</h1>
        <p>{{ t("prototype.skillsPage.description") }}</p>
      </div>

      <div class="skills-grid">
        <n-card
          :bordered="false"
          class="skills-card skills-card--hero skills-card--full"
        >
          <div class="skills-hero__eyebrow">
            {{ t("prototype.skillsPage.activeSkill") }}
          </div>
          <div class="skills-hero__grid">
            <article
              v-for="skill in store.activeSkills"
              :key="skill.id"
              class="skills-active"
            >
              <div class="skills-active__top">
                <strong>{{ skill.name }}</strong>
                <span class="skills-hero__status">{{
                  t("prototype.skillsPage.statusActive")
                }}</span>
              </div>
              <p class="skills-active__summary">{{ skill.summary }}</p>
              <div class="skills-active__version" v-if="activeVersionOf(skill)">
                <div class="skills-active__version-head">
                  <span class="skills-hero__label">{{
                    t("prototype.skillsPage.activeVersion")
                  }}</span>
                  <strong>{{ activeVersionOf(skill)?.id }}</strong>
                </div>
                <p>{{ activeVersionOf(skill)?.focus }}</p>
              </div>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="skills-card skills-card--full">
          <template #header>{{ t("prototype.skillsPage.available") }}</template>
          <div class="skills-list">
            <article
              v-for="skill in store.availableSkills"
              :key="skill.id"
              class="skills-list__item"
            >
              <div class="skills-list__head">
                <strong>{{ skill.name }}</strong>
                <span class="skills-list__count">
                  {{ skill.versions.filter((v) => v.enabled).length }}/{{
                    skill.versions.length
                  }}
                  {{ t("prototype.skillsPage.versionsEnabled") }}
                </span>
              </div>
              <p class="skills-list__summary">{{ skill.summary }}</p>
              <div class="skills-list__meta">
                <span class="skills-chip">{{ skill.id }}</span>
                <span
                  v-if="isActiveSkill(skill)"
                  class="skills-chip skills-chip--active"
                  >{{ t("prototype.skillsPage.statusActive") }}</span
                >
              </div>
            </article>
          </div>
        </n-card>

        <n-card :bordered="false" class="skills-card skills-card--full">
          <template #header>{{ t("prototype.skillsPage.bindings") }}</template>
          <div class="bindings-list">
            <article
              class="bindings-item"
              v-for="loop in store.workbenchLoops"
              :key="loop.id"
            >
              <strong>{{ loop.name }}</strong>
              <div class="bindings-item__skills">
                <span
                  v-for="ref in loop.skillRefs"
                  :key="ref"
                  class="bindings-chip"
                  >{{ ref }}</span
                >
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
import type {
  PrototypeSkill,
  PrototypeSkillVersion,
} from "@/mock/prototype.types";
import { useI18n } from "vue-i18n";

const store = usePrototypeStore();
const { t } = useI18n();

function activeVersionOf(skill: PrototypeSkill): PrototypeSkillVersion | null {
  const activeId = store.currentWorkbench?.activeSkillVersionId;
  return (
    skill.versions.find((version) => version.id === activeId) ??
    skill.versions.find((version) => version.enabled) ??
    skill.versions[0] ??
    null
  );
}

function isActiveSkill(skill: PrototypeSkill): boolean {
  return store.activeSkills.some((active) => active.id === skill.id);
}
</script>

<style scoped>
.prototype-page {
  display: grid;
  gap: 24px;
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
  margin: 0 0 8px;
}
.prototype-page__header p {
  max-width: 640px;
  margin: 0;
  color: var(--if-color-muted);
}
.skills-grid {
  display: grid;
  grid-template-columns: repeat(2, minmax(0, 1fr));
  gap: 20px;
}
.skills-card {
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
}
.skills-card--full {
  grid-column: span 2;
}
.skills-card--hero {
  background: linear-gradient(
    140deg,
    rgba(17, 24, 39, 0.96),
    rgba(21, 94, 117, 0.86)
  );
  color: #f8fafc;
}
.skills-hero__eyebrow {
  color: rgba(248, 250, 252, 0.7);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-bottom: 12px;
}
.skills-hero__grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(240px, 1fr));
  gap: 14px;
}
.skills-active {
  padding: 14px;
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.08);
}
.skills-active__top {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
  margin-bottom: 6px;
}
.skills-active__top strong {
  font-size: 16px;
}
.skills-active__summary {
  margin: 0;
  color: rgba(248, 250, 252, 0.82);
  font-size: 13px;
  line-height: 1.5;
}
.skills-hero__status {
  display: inline-flex;
  padding: 5px 10px;
  border-radius: 999px;
  background: rgba(94, 234, 212, 0.18);
  color: rgba(94, 234, 212, 0.95);
  font-size: 11px;
  font-weight: 700;
  white-space: nowrap;
}
.skills-active__version {
  margin-top: 12px;
  padding: 10px 12px;
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.08);
}
.skills-active__version-head {
  display: flex;
  align-items: center;
  gap: 10px;
  margin-bottom: 4px;
}
.skills-hero__label {
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: rgba(248, 250, 252, 0.6);
}
.skills-active__version p {
  margin: 0;
  color: rgba(248, 250, 252, 0.82);
  font-size: 13px;
}
.skills-list {
  display: grid;
  gap: 12px;
}
.skills-list__item {
  padding: 14px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}
.skills-list__head {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 12px;
}
.skills-list__count {
  font-size: 12px;
  color: var(--if-color-muted);
  white-space: nowrap;
}
.skills-list__summary {
  margin: 6px 0 10px;
  color: var(--if-color-muted);
  font-size: 13px;
  line-height: 1.5;
}
.skills-list__meta {
  display: flex;
  gap: 8px;
  flex-wrap: wrap;
}
.skills-chip {
  display: inline-flex;
  padding: 4px 10px;
  border-radius: 999px;
  background: rgba(28, 34, 48, 0.06);
  color: var(--if-color-muted);
  font-size: 11px;
  font-weight: 700;
}
.skills-chip--active {
  background: rgba(15, 118, 110, 0.12);
  color: var(--if-color-accent);
}
.bindings-list {
  display: grid;
  gap: 12px;
}
.bindings-item {
  padding: 14px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}
.bindings-item__skills {
  display: flex;
  gap: 6px;
  flex-wrap: wrap;
  margin-top: 8px;
}
.bindings-chip {
  display: inline-flex;
  padding: 4px 10px;
  border-radius: 999px;
  background: rgba(21, 94, 117, 0.1);
  color: var(--if-color-accent-strong);
  font-size: 11px;
  font-weight: 700;
}
@media (max-width: 900px) {
  .skills-grid {
    grid-template-columns: 1fr;
  }
  .skills-card--full {
    grid-column: span 1;
  }
}
</style>
