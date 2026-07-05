<template>
  <main
    v-if="isMockMode"
    class="landing landing--mock"
    :class="{ 'landing--zh': locale === 'zh-CN' }"
  >
    <div class="landing__topbar">
      <div class="landing__brand">
        <span class="landing__brand-mark">IF</span>
        <strong>issueflow</strong>
      </div>
      <LanguageSwitcher />
    </div>

    <section class="landing__hero">
      <div class="landing__hero-left">
        <div class="landing__eyebrow">{{ t("landing.eyebrow") }}</div>
        <h1>
          <span class="landing__title-line">{{ t("landing.titlePrimary") }}</span>
          <span class="landing__title-line landing__title-line--em">{{ t("landing.titleSecondary") }}</span>
        </h1>
        <p class="landing__lead">{{ t("landing.lead") }}</p>
        <p class="landing__impact">{{ t("landing.impact") }}</p>
        <div class="landing__actions">
          <n-button tag="a" href="/workbench" type="primary" size="large" round>
            {{ t("common.actions.openPrototype") }}
          </n-button>
        </div>
      </div>

      <div class="landing__hero-diagram">
        <div class="loop-system">

          <div class="loop-soul">
            <div class="loop-soul__label">SOUL</div>
            <div class="loop-soul__text">{{ t("landing.diagram.soul") }}</div>
          </div>

          <div class="loop-principle">
            <div class="loop-principle__label">PRINCIPLE</div>
            <div class="loop-principle__rules">
              <span>{{ t("landing.diagram.ruleNoFabricate") }}</span>
              <span>{{ t("landing.diagram.ruleApproveWrites") }}</span>
            </div>
          </div>

          <div class="loop-engine-ring">
            <svg class="loop-engine-ring__svg" viewBox="0 0 280 280">
              <defs>
                <linearGradient id="ring-grad" x1="0%" y1="0%" x2="100%" y2="100%">
                  <stop offset="0%" stop-color="rgba(94,234,212,0.3)" />
                  <stop offset="50%" stop-color="rgba(94,234,212,0.12)" />
                  <stop offset="100%" stop-color="rgba(94,234,212,0.3)" />
                </linearGradient>
              </defs>
              <circle cx="140" cy="140" r="120" fill="none" stroke="url(#ring-grad)" stroke-width="32" opacity="0.6" />
              <circle cx="140" cy="140" r="136" fill="none" stroke="rgba(94,234,212,0.15)" stroke-width="1" stroke-dasharray="6 4" />
            </svg>

            <div class="loop-hub">{{ t("landing.diagram.loopHub") }}</div>

            <div class="loop-node loop-node--discover" :style="nodeStyle(0)">
              <span class="loop-node__icon">🔍</span>
              <span class="loop-node__label">{{ t("landing.diagram.discover") }}</span>
            </div>
            <div class="loop-node loop-node--handoff" :style="nodeStyle(1)">
              <span class="loop-node__icon">🤝</span>
              <span class="loop-node__label">{{ t("landing.diagram.handoff") }}</span>
            </div>
            <div class="loop-node loop-node--verify" :style="nodeStyle(2)">
              <span class="loop-node__icon">✅</span>
              <span class="loop-node__label">{{ t("landing.diagram.verify") }}</span>
            </div>
            <div class="loop-node loop-node--persist" :style="nodeStyle(3)">
              <span class="loop-node__icon">💾</span>
              <span class="loop-node__label">{{ t("landing.diagram.persist") }}</span>
            </div>
            <div class="loop-node loop-node--schedule" :style="nodeStyle(4)">
              <span class="loop-node__icon">⏱</span>
              <span class="loop-node__label">{{ t("landing.diagram.schedule") }}</span>
            </div>
          </div>

          <div class="loop-infra">
            <div class="loop-infra__item" v-for="item in infraItems" :key="item.key">
              <span class="loop-infra__icon">{{ item.icon }}</span>
              <span class="loop-infra__label">{{ item.label }}</span>
            </div>
          </div>

          <div class="loop-decision">
            <span class="loop-decision__label">{{ t("landing.diagram.decisionLoop") }}</span>
            <div class="loop-decision__items">
              <span>{{ t("landing.diagram.evaluator") }}</span>
              <span class="loop-decision__sep">→</span>
              <span>{{ t("landing.diagram.humanApproval") }}</span>
              <span class="loop-decision__sep">→</span>
              <span>{{ t("landing.diagram.steerEvolve") }}</span>
            </div>
          </div>

        </div>
      </div>
    </section>

    <p class="landing__tagline">{{ t("landing.tagline") }}</p>
  </main>

  <main v-else class="landing">
    <n-card class="landing__card landing__card--login" :bordered="false">
      <div class="landing__eyebrow">{{ t("landing.loginEyebrow") }}</div>
      <h1>{{ t("landing.loginTitle") }}</h1>
      <p>{{ t("landing.loginBody") }}</p>
      <n-button tag="a" href="/api/auth/login" type="primary" size="large">
        {{ t("common.actions.continueToSignIn") }}
      </n-button>
    </n-card>
  </main>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { isMockMode } from "@/app-mode";
import LanguageSwitcher from "@/components/i18n/LanguageSwitcher.vue";
import { NButton, NCard } from "naive-ui";
import { useI18n } from "vue-i18n";

const { t, locale } = useI18n();

const infraItems = computed(() => [
  { key: "mem0", icon: "🧠", label: t("landing.diagram.mem0") },
  { key: "otel", icon: "📊", label: t("landing.diagram.otel") },
  { key: "gw", icon: "🔌", label: t("landing.diagram.gateway") },
]);

function nodeStyle(index: number) {
  const angle = index * 72 - 90;
  const rad = (angle * Math.PI) / 180;
  const radius = 108;
  const cx = 140;
  const cy = 140;
  const x = cx + radius * Math.cos(rad);
  const y = cy + radius * Math.sin(rad);
  return {
    left: `${x - 48}px`,
    top: `${y - 32}px`,
  };
}
</script>

<style scoped>
.landing {
  min-height: 100vh;
  display: grid;
  place-items: center;
  padding: 24px;
}

.landing__card {
  border-radius: var(--if-radius-lg);
  box-shadow: var(--if-shadow-panel);
}

.landing__card--login {
  width: min(100%, 640px);
}

.landing__eyebrow {
  margin-bottom: 12px;
  color: var(--if-color-accent);
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

h1 {
  margin: 0 0 12px;
  font-size: clamp(2.25rem, 5vw, 3.5rem);
  line-height: 1.05;
}

p {
  margin: 0 0 24px;
  color: var(--if-color-muted);
  line-height: 1.6;
}

.landing--mock {
  display: grid;
  gap: 32px;
  max-width: 1200px;
  margin: 0 auto;
  padding: 28px 24px 56px;
  place-items: stretch;
}

.landing__topbar {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.landing__brand {
  display: flex;
  align-items: center;
  gap: 10px;
  font-size: 18px;
}

.landing__brand-mark {
  width: 34px;
  height: 34px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  border-radius: 10px;
  background: var(--if-color-accent);
  color: #fff;
  font-weight: 800;
  font-size: 13px;
}

/* ─── Hero ─── */

.landing__hero {
  display: grid;
  grid-template-columns: minmax(0, 1fr) minmax(420px, 540px);
  align-items: center;
  gap: 40px;
  padding: 48px 48px 48px 56px;
  border-radius: var(--if-radius-xl);
  background:
    linear-gradient(135deg, #0b192f 0%, #0f423e 45%, #0b192f 100%);
  color: #f8fafc;
  overflow: hidden;
}

.landing__hero-left {
  max-width: 520px;
}

.landing__hero .landing__eyebrow {
  color: rgba(94, 234, 212, 0.8);
  margin-bottom: 16px;
}

.landing__hero h1 {
  margin: 0 0 18px;
  font-size: clamp(2rem, 3.8vw, 2.8rem);
  line-height: 1.12;
  letter-spacing: -0.02em;
}

.landing__title-line {
  display: block;
}

.landing__title-line--em {
  color: rgba(94, 234, 212, 1);
}

.landing--zh .landing__hero h1 {
  font-size: clamp(1.7rem, 3.2vw, 2.3rem);
}

.landing__lead {
  max-width: 480px;
  color: rgba(248, 250, 252, 0.78);
  line-height: 1.7;
  font-size: 14px;
  margin-bottom: 12px;
}

.landing__impact {
  color: rgba(94, 234, 212, 0.7);
  font-size: 13px;
  margin-bottom: 28px;
  font-weight: 600;
}

.landing__actions {
  display: flex;
  gap: 12px;
}

/* ─── System Diagram ─── */

.landing__hero-diagram {
  display: flex;
  align-items: center;
  justify-content: center;
  min-height: 440px;
}

.loop-system {
  position: relative;
  width: 300px;
  display: grid;
  gap: 10px;
  justify-items: center;
}

.loop-soul {
  text-align: center;
  padding: 8px 16px;
  border-radius: 10px;
  background: rgba(94, 234, 212, 0.12);
  border: 1px solid rgba(94, 234, 212, 0.25);
}

.loop-soul__label {
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.14em;
  color: rgba(94, 234, 212, 0.8);
  margin-bottom: 4px;
}

.loop-soul__text {
  font-size: 12px;
  color: rgba(248, 250, 252, 0.75);
}

.loop-principle {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 14px;
  border-radius: 8px;
  background: rgba(94, 234, 212, 0.08);
  border: 1px solid rgba(94, 234, 212, 0.15);
}

.loop-principle__label {
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.14em;
  color: rgba(94, 234, 212, 0.65);
}

.loop-principle__rules {
  display: flex;
  gap: 8px;
  font-size: 11px;
  color: rgba(248, 250, 252, 0.6);
}

/* ─── Engine Ring ─── */

.loop-engine-ring {
  position: relative;
  width: 280px;
  height: 280px;
}

.loop-engine-ring__svg {
  position: absolute;
  inset: 0;
  width: 100%;
  height: 100%;
  animation: ring-rotate 40s linear infinite;
}

@keyframes ring-rotate {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.loop-hub {
  position: absolute;
  inset: 0;
  margin: auto;
  width: 72px;
  height: 72px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  background: rgba(94, 234, 212, 0.18);
  border: 2px solid rgba(94, 234, 212, 0.4);
  font-size: 11px;
  font-weight: 800;
  letter-spacing: 0.06em;
  color: rgba(94, 234, 212, 0.9);
  text-align: center;
  line-height: 1.3;
  z-index: 2;
}

.loop-node {
  position: absolute;
  width: 96px;
  display: grid;
  gap: 3px;
  justify-items: center;
  text-align: center;
  z-index: 2;
}

.loop-node__icon {
  font-size: 18px;
  filter: drop-shadow(0 0 8px rgba(94, 234, 212, 0.3));
}

.loop-node__label {
  font-size: 10px;
  font-weight: 700;
  color: rgba(248, 250, 252, 0.7);
}

/* ─── Infra ─── */

.loop-infra {
  display: flex;
  gap: 12px;
  padding: 8px 16px;
  border-radius: 10px;
  background: rgba(255, 255, 255, 0.04);
  border: 1px solid rgba(255, 255, 255, 0.07);
  width: 100%;
  justify-content: center;
}

.loop-infra__item {
  display: flex;
  align-items: center;
  gap: 6px;
  font-size: 11px;
  color: rgba(248, 250, 252, 0.55);
}

.loop-infra__icon {
  font-size: 14px;
}

/* ─── Decision Loop ─── */

.loop-decision {
  padding: 8px 16px;
  border-radius: 10px;
  background: rgba(251, 191, 36, 0.08);
  border: 1px solid rgba(251, 191, 36, 0.15);
  width: 100%;
  text-align: center;
}

.loop-decision__label {
  display: block;
  font-size: 10px;
  font-weight: 800;
  letter-spacing: 0.12em;
  color: rgba(251, 191, 36, 0.7);
  margin-bottom: 6px;
}

.loop-decision__items {
  display: flex;
  gap: 6px;
  align-items: center;
  justify-content: center;
  flex-wrap: wrap;
  font-size: 11px;
  color: rgba(248, 250, 252, 0.6);
}

.loop-decision__sep {
  color: rgba(248, 250, 252, 0.25);
  font-weight: 300;
}

/* ─── Story ─── */

.landing__tagline {
  text-align: center;
  padding: 12px 0 32px;
  color: var(--if-color-muted);
  font-size: 18px;
  font-weight: 500;
  letter-spacing: 0.02em;
}

/* ─── Responsive ─── */

@media (max-width: 960px) {
  .landing__hero {
    grid-template-columns: 1fr;
    padding: 44px 28px;
    gap: 32px;
  }

  .landing__hero-diagram {
    min-height: 380px;
    order: -1;
  }

  .loop-system {
    width: 260px;
  }

  .loop-engine-ring {
    width: 240px;
    height: 240px;
  }

  .story-grid {
    grid-template-columns: 1fr;
  }
}

@media (max-width: 640px) {
  .landing--mock {
    padding: 20px 16px 40px;
    gap: 24px;
  }

  .landing__hero {
    padding: 32px 20px;
  }
}
</style>
