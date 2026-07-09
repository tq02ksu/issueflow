<template>
  <div class="loop-chat">
    <p class="loop-chat__hint">{{ t("prototype.agents.loopCoreChatHint") }}</p>
    <div class="loop-chat__messages">
      <article
        v-for="message in messages"
        :key="message.id"
        class="loop-chat__msg"
        :class="`loop-chat__msg--${message.role}`"
      >
        <span class="loop-chat__author">
          {{ message.role === "agent" ? "Loop Agent" : "You" }}
        </span>
        <p class="loop-chat__text">{{ message.text }}</p>
      </article>
    </div>
    <div class="loop-chat__composer">
      <textarea
        v-model="draft"
        class="loop-chat__input"
        data-test="loop-chat-input"
        rows="3"
        :placeholder="t('prototype.agents.chatPlaceholder')"
        @keydown.enter.ctrl.prevent="send"
        @keydown.enter.meta.prevent="send"
      ></textarea>
      <div class="loop-chat__composer-actions">
        <n-button
          type="primary"
          data-test="loop-chat-send"
          :disabled="!draft.trim()"
          @click="send"
        >
          {{ t("prototype.agents.chatSend") }}
        </n-button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { NButton } from "naive-ui";
import { useI18n } from "vue-i18n";
import { usePrototypeStore } from "@/stores/prototype.store";

interface ChatMessage {
  id: number;
  role: "agent" | "user";
  text: string;
}

const store = usePrototypeStore();
const { t } = useI18n();

const workbenchName = store.currentWorkbench?.name ?? "this workbench";
const activeLoops = store.workbenchLoops.filter((loop) => loop.enabled).length;

let nextId = 1;
const messages = ref<ChatMessage[]>([
  {
    id: nextId++,
    role: "agent",
    text: `Loop Agent for ${workbenchName}. I'm orchestrating ${activeLoops} active loop(s). Ask me about turns, blockers, or pending approvals.`,
  },
]);
const draft = ref("");

function agentReply(prompt: string): string {
  const pending = store.pendingApprovals.length;
  const blocked = store.visibleIssues.filter(
    (issue) => issue.state === "blocked",
  ).length;
  return `On ${workbenchName}: ${blocked} blocked issue(s) and ${pending} pending approval(s). Re "${prompt}", I can draft a next step and stop before any write for your confirmation.`;
}

function send() {
  const text = draft.value.trim();
  if (!text) {
    return;
  }
  messages.value.push({ id: nextId++, role: "user", text });
  messages.value.push({ id: nextId++, role: "agent", text: agentReply(text) });
  draft.value = "";
}
</script>

<style scoped>
.loop-chat {
  display: grid;
  gap: 12px;
}
.loop-chat__hint {
  margin: 0;
  color: var(--if-color-muted);
  font-size: 13px;
}
.loop-chat__messages {
  display: grid;
  gap: 10px;
  max-height: 260px;
  overflow-y: auto;
  padding: 4px;
}
.loop-chat__msg {
  padding: 10px 12px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: var(--if-radius-sm);
  background: rgba(255, 255, 255, 0.7);
}
.loop-chat__msg--agent {
  background: rgba(21, 94, 117, 0.08);
  border-color: rgba(21, 94, 117, 0.18);
}
.loop-chat__msg--user {
  background: rgba(255, 255, 255, 0.85);
}
.loop-chat__author {
  display: block;
  font-size: 11px;
  font-weight: 700;
  letter-spacing: 0.04em;
  color: var(--if-color-accent-strong);
  margin-bottom: 4px;
}
.loop-chat__text {
  margin: 0;
  font-size: 13px;
  line-height: 1.5;
}
.loop-chat__composer {
  display: grid;
  gap: 8px;
}
.loop-chat__composer-actions {
  display: flex;
  justify-content: flex-end;
}
.loop-chat__input {
  width: 100%;
  min-height: 84px;
  padding: 12px 14px;
  border: 1px solid rgba(216, 204, 184, 0.8);
  border-radius: 12px;
  background: rgba(255, 255, 255, 0.85);
  color: var(--if-color-text);
  font: inherit;
  font-size: 14px;
  line-height: 1.5;
  resize: vertical;
}
.loop-chat__input:focus {
  outline: none;
  border-color: var(--if-color-accent);
}
</style>
