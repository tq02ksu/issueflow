import { createRun, subscribeRunEvents } from "@/api/agent.api";
import { useAgentStore } from "@/stores/agent.store";

export function useAgentRun() {
  const store = useAgentStore();

  async function run(request: {
    threadId: string;
    workbenchId: number;
    messages: unknown[];
  }) {
    const created = await createRun(request);
    if (!created) return;

    const { runId } = created;
    store.setActiveRun({ run_id: runId, status: "queued" });
    store.setStreaming(true);

    const response = await subscribeRunEvents(runId, 0);
    if (!response.ok || !response.body) {
      store.setStreaming(false);
      return;
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    let buffer = "";
    let currentEvent = "";

    while (true) {
      const { done, value } = await reader.read();
      if (done) break;

      buffer += decoder.decode(value, { stream: true });

      const lines = buffer.split("\n");
      buffer = lines.pop() || "";

      for (const line of lines) {
        const trimmed = line.trim();
        if (trimmed.startsWith("event:")) {
          currentEvent = trimmed.slice(6).trim();
        } else if (trimmed.startsWith("data:")) {
          const data = trimmed.slice(5).trim();
          if (currentEvent && data) {
            handleEvent(currentEvent, data, store);
          }
          currentEvent = "";
        }
      }
    }

    store.setActiveRun(null);
    store.setStreaming(false);
  }

  return { run };
}

function handleEvent(
  eventType: string,
  data: string,
  store: ReturnType<typeof useAgentStore>,
) {
  let parsed: Record<string, unknown> | null = null;
  try {
    parsed = JSON.parse(data);
  } catch {
    // plain text data
  }

  switch (eventType) {
    case "TEXT_MESSAGE_START": {
      if (parsed?.messageId) {
        store.beginAssistantMessage(parsed.messageId as string);
      }
      break;
    }
    case "TEXT_MESSAGE_CONTENT": {
      if (parsed?.messageId && parsed?.delta) {
        store.appendAssistantDelta(
          parsed.messageId as string,
          parsed.delta as string,
        );
      }
      break;
    }
    case "TOOL_CALL_START": {
      if (parsed?.toolCallId && parsed?.toolCallName) {
        store.upsertToolCall(
          parsed.toolCallId as string,
          parsed.toolCallName as string,
          "",
        );
      }
      break;
    }
    case "TOOL_CALL_ARGS": {
      if (parsed?.toolCallId && parsed?.delta) {
        store.upsertToolCall(
          parsed.toolCallId as string,
          "",
          parsed.delta as string,
        );
      }
      break;
    }
    case "TOOL_CALL_RESULT": {
      if (parsed?.toolCallId && parsed?.content) {
        store.upsertToolCall(
          parsed.toolCallId as string,
          "",
          "",
          parsed.content,
        );
      }
      break;
    }
    case "CUSTOM":
    case "custom": {
      store.appendCustomEvent(parsed || data);
      break;
    }
    case "RUN_ERROR": {
      store.addUserMessage(`Error: ${parsed?.message || "unknown error"}`);
      break;
    }
  }
}
