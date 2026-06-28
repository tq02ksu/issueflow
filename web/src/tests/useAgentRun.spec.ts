import { createPinia, setActivePinia } from "pinia";
import { useAgentStore } from "@/stores/agent.store";
import { useAgentRun } from "@/composables/useAgentRun";
import { beforeEach, vi } from "vitest";

const mockCreateRun = vi.fn();
const mockSubscribeRunEvents = vi.fn();

vi.mock("@/api/agent.api", () => ({
  createRun: (...args: unknown[]) => mockCreateRun(...args),
  subscribeRunEvents: (...args: unknown[]) => mockSubscribeRunEvents(...args),
}));

function sseChunks(lines: string[]): Uint8Array[] {
  return lines.map((l) => new TextEncoder().encode(l + "\n"));
}

function mockResponse(body: ReadableStream<Uint8Array> | null, ok = true) {
  return { ok, body };
}

beforeEach(() => {
  setActivePinia(createPinia());
  vi.resetAllMocks();
});

describe("useAgentRun", () => {
  it("creates a run and streams TEXT_MESSAGE events into store", async () => {
    const store = useAgentStore();
    const { run } = useAgentRun();

    mockCreateRun.mockResolvedValueOnce({
      runId: "r1",
      threadId: "t1",
      status: "queued",
    });

    const stream = new ReadableStream<Uint8Array>({
      start(controller) {
        for (const chunk of sseChunks([
          "event:TEXT_MESSAGE_START",
          'data:{"type":"TEXT_MESSAGE_START","messageId":"m1","role":"assistant"}',
          "",
          "event:TEXT_MESSAGE_CONTENT",
          'data:{"type":"TEXT_MESSAGE_CONTENT","messageId":"m1","delta":"Hello "}',
          "",
          "event:TEXT_MESSAGE_CONTENT",
          'data:{"type":"TEXT_MESSAGE_CONTENT","messageId":"m1","delta":"World"}',
          "",
        ])) {
          controller.enqueue(chunk);
        }
        controller.close();
      },
    });

    mockSubscribeRunEvents.mockReturnValueOnce(mockResponse(stream));

    await run({
      threadId: "t1",
      workbenchId: 1,
      messages: [{ role: "user", content: "hi" }],
    });

    const assistant = store.messages.find((m) => m.role === "assistant");
    expect(assistant).toBeDefined();
    expect(assistant!.content).toBe("Hello World");
    expect(store.streaming).toBe(false);
  });

  it("stops and resets streaming on non-ok response", async () => {
    const store = useAgentStore();
    store.setStreaming(true);
    const { run } = useAgentRun();

    mockCreateRun.mockResolvedValueOnce({
      runId: "r2",
      threadId: "t2",
      status: "queued",
    });

    mockSubscribeRunEvents.mockReturnValueOnce(mockResponse(null, false));

    await run({
      threadId: "t2",
      workbenchId: 1,
      messages: [],
    });

    expect(store.streaming).toBe(false);
  });

  it("handles null body on response", async () => {
    const store = useAgentStore();
    store.setStreaming(true);
    const { run } = useAgentRun();

    mockCreateRun.mockResolvedValueOnce({
      runId: "r3",
      threadId: "t3",
      status: "queued",
    });

    mockSubscribeRunEvents.mockReturnValueOnce(mockResponse(null));

    await run({
      threadId: "t3",
      workbenchId: 1,
      messages: [],
    });

    expect(store.streaming).toBe(false);
  });

  it("dispatches TOOL_CALL events", async () => {
    const store = useAgentStore();
    const { run } = useAgentRun();

    mockCreateRun.mockResolvedValueOnce({
      runId: "r4",
      threadId: "t4",
      status: "queued",
    });

    const stream = new ReadableStream<Uint8Array>({
      start(controller) {
        for (const chunk of sseChunks([
          "event:TOOL_CALL_START",
          'data:{"type":"TOOL_CALL_START","toolCallId":"tc1","toolCallName":"search"}',
          "",
          "event:TOOL_CALL_ARGS",
          'data:{"type":"TOOL_CALL_ARGS","toolCallId":"tc1","delta":"{\\"query\\":\\"}',
          "",
          "event:TOOL_CALL_ARGS",
          'data:{"type":"TOOL_CALL_ARGS","toolCallId":"tc1","delta":"test\\"}',
          "",
          "event:TOOL_CALL_RESULT",
          'data:{"type":"TOOL_CALL_RESULT","messageId":"m2","toolCallId":"tc1","content":{"items":[]},"role":"tool"}',
          "",
        ])) {
          controller.enqueue(chunk);
        }
        controller.close();
      },
    });

    mockSubscribeRunEvents.mockReturnValueOnce(mockResponse(stream));

    await run({
      threadId: "t4",
      workbenchId: 1,
      messages: [],
    });

    const toolMsg = store.messages.find((m) => m.message_kind === "tool_call");
    expect(toolMsg).toBeDefined();
    const data = JSON.parse(toolMsg!.content);
    expect(data.result).toEqual({ items: [] });
  });

  it("handles CUSTOM events", async () => {
    const store = useAgentStore();
    const { run } = useAgentRun();

    mockCreateRun.mockResolvedValueOnce({
      runId: "r5",
      threadId: "t5",
      status: "queued",
    });

    const stream = new ReadableStream<Uint8Array>({
      start(controller) {
        for (const chunk of sseChunks([
          "event:custom",
          'data:{"type":"CUSTOM","name":"a2ui","value":{"kind":"a2ui_render","payload":{}}}',
          "",
        ])) {
          controller.enqueue(chunk);
        }
        controller.close();
      },
    });

    mockSubscribeRunEvents.mockReturnValueOnce(mockResponse(stream));

    await run({
      threadId: "t5",
      workbenchId: 1,
      messages: [],
    });

    const customMsg = store.messages.find((m) => m.role === "custom");
    expect(customMsg).toBeDefined();
  });

  it("returns early when createRun returns null", async () => {
    const store = useAgentStore();
    const { run } = useAgentRun();

    mockCreateRun.mockResolvedValueOnce(null);

    await run({
      threadId: "t6",
      workbenchId: 1,
      messages: [],
    });

    expect(store.streaming).toBe(false);
    expect(mockSubscribeRunEvents).not.toHaveBeenCalled();
  });
});
