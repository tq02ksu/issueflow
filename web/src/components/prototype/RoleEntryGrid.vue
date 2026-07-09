<template>
  <section class="role-grid">
    <div class="role-grid__header">
      <div class="role-grid__eyebrow">
        {{ t("prototype.roles.pickerEyebrow") }}
      </div>
      <h2>{{ t("prototype.roles.pickerTitle") }}</h2>
      <p>{{ t("prototype.roles.pickerDescription") }}</p>
    </div>
    <div class="role-grid__cards">
      <button
        v-for="role in store.roleViews"
        :key="role.key"
        :data-role-key="role.key"
        type="button"
        class="role-card"
        @click="enter(role.key)"
      >
        <div class="role-card__name">
          {{ t(`prototype.roles.${role.key}.name`) }}
        </div>
        <div class="role-card__tagline">
          {{ t(`prototype.roles.${role.key}.tagline`) }}
        </div>
        <ul class="role-card__signals">
          <li v-for="card in role.signalCards.slice(0, 3)" :key="card.id">
            {{ card.label }}
          </li>
        </ul>
      </button>
    </div>
  </section>
</template>

<script setup lang="ts">
import { useRouter } from "vue-router";
import { useI18n } from "vue-i18n";
import { usePrototypeStore } from "@/stores/prototype.store";
import type { PrototypeRoleKey } from "@/mock/prototype.types";

const store = usePrototypeStore();
const router = useRouter();
const { t } = useI18n();

function enter(key: PrototypeRoleKey) {
  store.setActiveRole(key);
  router.push("/workbench");
}
</script>

<style scoped>
.role-grid {
  display: grid;
  gap: 20px;
}
.role-grid__eyebrow {
  color: var(--if-color-accent-strong);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
  margin-bottom: 8px;
}
.role-grid__header h1,
.role-grid__header h2 {
  margin: 0 0 8px;
}
.role-grid__header p {
  max-width: 640px;
  margin: 0;
  color: var(--if-color-muted);
}
.role-grid__cards {
  display: grid;
  grid-template-columns: repeat(4, minmax(0, 1fr));
  gap: 16px;
}
.role-card {
  display: grid;
  gap: 10px;
  align-content: start;
  text-align: left;
  padding: 20px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-lg);
  background: rgba(255, 250, 242, 0.92);
  cursor: pointer;
  transition:
    transform 0.12s ease,
    border-color 0.12s ease,
    box-shadow 0.12s ease;
}
.role-card:hover {
  transform: translateY(-2px);
  border-color: var(--if-color-accent);
  box-shadow: var(--if-shadow-panel);
}
.role-card__name {
  font-size: 16px;
  font-weight: 800;
}
.role-card__tagline {
  color: var(--if-color-accent-strong);
  font-size: 13px;
  font-weight: 600;
}
.role-card__signals {
  margin: 4px 0 0;
  padding-left: 16px;
  color: var(--if-color-muted);
  font-size: 12px;
  line-height: 1.6;
}
@media (max-width: 960px) {
  .role-grid__cards {
    grid-template-columns: repeat(2, minmax(0, 1fr));
  }
}
@media (max-width: 560px) {
  .role-grid__cards {
    grid-template-columns: 1fr;
  }
}
</style>
