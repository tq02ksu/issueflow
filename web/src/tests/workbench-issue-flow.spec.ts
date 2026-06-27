import { mount } from "@vue/test-utils";
import { createPinia, setActivePinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "@/App.vue";
import { routes } from "@/router";
import { useSessionStore } from "@/stores/session";

async function renderAt(path: string) {
  const pinia = createPinia();
  setActivePinia(pinia);

  const router = createRouter({
    history: createMemoryHistory(),
    routes,
  });

  await router.push(path);
  await router.isReady();

  const wrapper = mount(App, {
    global: {
      plugins: [pinia, router],
    },
  });

  return { wrapper, router, pinia };
}

describe("Workbench issue flow", () => {
  it("renders a structured issue draft state", async () => {
    const { wrapper, pinia } = await renderAt("/workbench");
    const store = useSessionStore(pinia);
    store.setDraft({ projectId: 123, title: "Draft onboarding issue", description: "Created by agent" });

    await wrapper.vm.$nextTick();

    expect(wrapper.find('[data-test="issue-draft"]').exists()).toBe(true);
    expect(wrapper.text()).toContain("project");
    expect(wrapper.text()).toContain("title");
  });

  it("renders a confirmation action for the draft", async () => {
    const { wrapper, pinia } = await renderAt("/workbench");
    const store = useSessionStore(pinia);
    store.setDraft({ projectId: 123, title: "Draft onboarding issue", description: "Created by agent" });

    await wrapper.vm.$nextTick();

    expect(wrapper.find('[data-test="confirm-issue-btn"]').exists()).toBe(true);
    expect(wrapper.text()).toContain("Confirm");
  });

  it("renders a created result state", async () => {
    const { wrapper, pinia } = await renderAt("/workbench");
    const store = useSessionStore(pinia);
    store.setCreated({
      id: 456,
      iid: 12,
      projectId: 123,
      title: "Draft onboarding issue",
      webUrl: "https://gitlab.example.com/group/project/-/issues/12",
    });

    await wrapper.vm.$nextTick();

    expect(wrapper.find('[data-test="issue-created-result"]').exists()).toBe(true);
    expect(wrapper.text()).toContain("Created");
  });
});
