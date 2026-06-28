import type { A2uiMessage } from "a2ui-vue";
import type { AgentMessage } from "@/stores/agent.store";

interface A2UIRenderEvent {
  kind: "a2ui_render";
  payload: A2uiMessage | A2uiMessage[];
}

interface AgentCustomEnvelope {
  value?: unknown;
}

function isRecord(value: unknown): value is Record<string, unknown> {
  return !!value && typeof value === "object";
}

export function parseA2UIRenderEvent(value: unknown): A2UIRenderEvent | null {
  const candidate = unwrapAgentCustomEnvelope(value);
  if (!isRecord(candidate) || candidate.kind !== "a2ui_render") return null;

  const { payload } = candidate;
  if (Array.isArray(payload) || isRecord(payload)) {
    return payload
      ? ({ kind: "a2ui_render", payload } as unknown as A2UIRenderEvent)
      : null;
  }

  return null;
}

function unwrapAgentCustomEnvelope(value: unknown): unknown {
  if (!isRecord(value)) return value;

  const envelope = value as AgentCustomEnvelope;
  return envelope.value ?? value;
}

export function extractA2UIMessages(messages: AgentMessage[]): A2uiMessage[] {
  return messages.flatMap((message) => {
    if (message.role !== "custom" || message.message_kind !== "custom") {
      return [];
    }

    try {
      const parsed = JSON.parse(message.content);
      const event = parseA2UIRenderEvent(parsed);
      if (!event) return [];
      return Array.isArray(event.payload) ? event.payload : [event.payload];
    } catch {
      return [];
    }
  });
}

export function useA2UIBridge() {
  function handleCustom(value: unknown) {
    return parseA2UIRenderEvent(value);
  }

  function buildSubmit(surfaceId: string, payload: Record<string, unknown>) {
    return {
      role: "user" as const,
      content: {
        kind: "a2ui_submit",
        surface_id: surfaceId,
        payload,
      },
    };
  }

  return { handleCustom, buildSubmit };
}
