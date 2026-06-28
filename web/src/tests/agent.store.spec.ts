import { createPinia, setActivePinia } from "pinia";
import { useAgentStore } from "@/stores/agent.store";
import { beforeEach } from "vitest";

beforeEach(() => {
  setActivePinia(createPinia());
});

describe("agent store — messages", () => {
  it("addUserMessage pushes a user message", () => {
    const store = useAgentStore();
    store.addUserMessage("hello");
    expect(store.messages.length).toBe(1);
    expect(store.messages[0].role).toBe("user");
    expect(store.messages[0].content).toBe("hello");
  });

  it("beginAssistantMessage creates an empty assistant message", () => {
    const store = useAgentStore();
    store.beginAssistantMessage("msg-1");
    expect(store.messages.length).toBe(1);
    expect(store.messages[0].role).toBe("assistant");
    expect(store.messages[0].content).toBe("");
  });

  it("appendAssistantDelta appends to the correct message", () => {
    const store = useAgentStore();
    store.beginAssistantMessage("msg-1");
    store.appendAssistantDelta("msg-1", "Hello ");
    store.appendAssistantDelta("msg-1", "World");
    expect(store.messages[0].content).toBe("Hello World");
  });

  it("upsertToolCall creates a tool call message", () => {
    const store = useAgentStore();
    store.upsertToolCall("tc-1", "search", "");
    expect(store.messages.length).toBe(1);
    expect(store.messages[0].message_kind).toBe("tool_call");
  });

  it("upsertToolCall updates existing tool call", () => {
    const store = useAgentStore();
    store.upsertToolCall("tc-1", "search", '{"query":"');
    store.upsertToolCall("tc-1", "", 'test"}', { result: "ok" });
    const data = JSON.parse(store.messages[0].content);
    expect(data.name).toBe("search");
    expect(data.args).toBe('{"query":"test"}');
    expect(data.result).toEqual({ result: "ok" });
  });

  it("appendCustomEvent pushes a custom message", () => {
    const store = useAgentStore();
    store.appendCustomEvent({ kind: "a2ui_render", payload: {} });
    expect(store.messages.length).toBe(1);
    expect(store.messages[0].role).toBe("custom");
    expect(store.messages[0].message_kind).toBe("custom");
  });

  it("clearMessages empties the message list", () => {
    const store = useAgentStore();
    store.addUserMessage("x");
    store.clearMessages();
    expect(store.messages.length).toBe(0);
  });
});

describe("agent store — sessions", () => {
  it("setSessions updates the session list", () => {
    const store = useAgentStore();
    store.setSessions([
      {
        id: "s1",
        user_id: 1,
        workbench_id: 1,
        title: "foo",
        latest_state: null,
        last_message_at: "2025",
        created_at: "2025",
        updated_at: "2025",
      },
    ]);
    expect(store.sessions.length).toBe(1);
    expect(store.sessions[0].title).toBe("foo");
  });

  it("setActiveSession sets the active session id", () => {
    const store = useAgentStore();
    store.setActiveSession("abc");
    expect(store.activeSessionId).toBe("abc");
  });
});

describe("agent store — run state", () => {
  it("setActiveRun and setStreaming track run progress", () => {
    const store = useAgentStore();
    store.setActiveRun({ run_id: "r1", status: "running" });
    store.setStreaming(true);
    expect(store.activeRun?.run_id).toBe("r1");
    expect(store.streaming).toBe(true);
  });
});
