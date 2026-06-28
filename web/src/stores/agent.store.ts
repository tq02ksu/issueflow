import { defineStore } from "pinia";
import { ref } from "vue";

export interface AgentSession {
  id: string;
  user_id: number;
  workbench_id: number;
  title: string;
  latest_state: string | null;
  last_message_at: string;
  created_at: string;
  updated_at: string;
}

export interface AgentMessage {
  id: number | string;
  role: "user" | "assistant" | "tool" | "custom";
  message_kind: string;
  content: string;
  created_at?: string;
}

export interface ActiveRun {
  run_id: string;
  status: string;
}

interface ToolCallContent {
  toolCallId: string;
  name: string;
  args: string;
  result?: unknown;
}

export const useAgentStore = defineStore("agent", () => {
  const sessions = ref<AgentSession[]>([]);
  const activeSessionId = ref<string | null>(null);
  const messages = ref<AgentMessage[]>([]);
  const activeRun = ref<ActiveRun | null>(null);
  const streaming = ref(false);

  function setSessions(list: AgentSession[]) {
    sessions.value = list;
  }

  function setActiveSession(id: string) {
    activeSessionId.value = id;
  }

  function setHistory(msgs: AgentMessage[]) {
    messages.value = msgs;
  }

  function addUserMessage(text: string) {
    messages.value.push({
      id: Date.now().toString(),
      role: "user",
      message_kind: "text",
      content: text,
      created_at: new Date().toISOString(),
    });
  }

  function beginAssistantMessage(msgId: string) {
    messages.value.push({
      id: msgId,
      role: "assistant",
      message_kind: "text",
      content: "",
    });
  }

  function appendAssistantDelta(msgId: string, delta: string) {
    const msg = messages.value.find((m) => m.id === msgId);
    if (msg && msg.role === "assistant") {
      msg.content += delta;
    }
  }

  function upsertToolCall(
    toolCallId: string,
    name: string,
    args: string,
    result?: unknown,
  ) {
    const existing = messages.value.find((m) => m.id === toolCallId);
    if (existing) {
      const current = parseToolCallContent(existing.content);
      existing.content = JSON.stringify({
        toolCallId,
        name: name || current.name,
        args: args ? current.args + args : current.args,
        result: result ?? current.result,
      });
    } else {
      messages.value.push({
        id: toolCallId,
        role: "tool",
        message_kind: "tool_call",
        content: JSON.stringify({ toolCallId, name, args, result }),
      });
    }
  }

  function appendCustomEvent(value: unknown) {
    messages.value.push({
      id: Date.now().toString(),
      role: "custom",
      message_kind: "custom",
      content: JSON.stringify(value),
    });
  }

  function parseToolCallContent(content: string): ToolCallContent {
    try {
      return JSON.parse(content) as ToolCallContent;
    } catch {
      return {
        toolCallId: "",
        name: "",
        args: "",
      };
    }
  }

  function setActiveRun(run: ActiveRun | null) {
    activeRun.value = run;
  }

  function setStreaming(v: boolean) {
    streaming.value = v;
  }

  function clearMessages() {
    messages.value = [];
  }

  return {
    sessions,
    activeSessionId,
    messages,
    activeRun,
    streaming,
    setSessions,
    setActiveSession,
    setHistory,
    addUserMessage,
    beginAssistantMessage,
    appendAssistantDelta,
    upsertToolCall,
    appendCustomEvent,
    setActiveRun,
    setStreaming,
    clearMessages,
  };
});
