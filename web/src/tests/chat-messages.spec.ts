import { defineComponent, h } from "vue";
import { mount } from "@vue/test-utils";
import ChatMessages from "@/components/agent/ChatMessages.vue";

vi.mock("@/components/agent/A2UISurfaceHost.vue", () => ({
  default: defineComponent({
    name: "A2UISurfaceHostStub",
    props: {
      messages: { type: Array, required: true },
    },
    setup(props) {
      return () =>
        h(
          "div",
          { "data-testid": "a2ui-surface-host" },
          `${props.messages.length}`,
        );
    },
  }),
}));

describe("ChatMessages", () => {
  it("renders the A2UI surface host for a2ui_render custom events", () => {
    const wrapper = mount(ChatMessages, {
      props: {
        streaming: false,
        messages: [
          {
            id: "custom-1",
            role: "custom",
            message_kind: "custom",
            content: JSON.stringify({
              type: "CUSTOM",
              name: "a2ui",
              value: {
                kind: "a2ui_render",
                payload: {
                  createSurface: {
                    surfaceId: "surface-1",
                    catalogId: "default",
                  },
                },
              },
            }),
          },
        ],
      },
      global: {
        stubs: {
          NSpace: defineComponent({
            setup(_, { slots }) {
              return () => h("div", slots.default?.());
            },
          }),
          NText: defineComponent({
            setup(_, { slots }) {
              return () => h("span", slots.default?.());
            },
          }),
          NCard: defineComponent({
            setup(_, { slots }) {
              return () => h("div", slots.default?.());
            },
          }),
          ToolCallCard: true,
        },
      },
    });

    expect(wrapper.find('[data-testid="a2ui-surface-host"]').exists()).toBe(
      true,
    );
  });

  it("does not render whitespace-only assistant messages", () => {
    const wrapper = mount(ChatMessages, {
      props: {
        streaming: false,
        messages: [
          {
            id: "assistant-blank",
            role: "assistant",
            message_kind: "text",
            content: "\n",
          },
          {
            id: "assistant-text",
            role: "assistant",
            message_kind: "text",
            content: "hello",
          },
        ],
      },
      global: {
        stubs: {
          NSpace: defineComponent({
            setup(_, { slots }) {
              return () => h("div", slots.default?.());
            },
          }),
          NText: defineComponent({
            setup(_, { slots }) {
              return () => h("span", slots.default?.());
            },
          }),
          NCard: defineComponent({
            setup(_, { slots }) {
              return () => h("div", { class: "card" }, slots.default?.());
            },
          }),
          ToolCallCard: true,
        },
      },
    });

    expect(wrapper.text()).toContain("hello");
    expect(wrapper.text()).not.toContain("...");
  });
});
