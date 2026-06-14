import { mount } from "@vue/test-utils";
import { createPinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "@/App.vue";
import { routes } from "@/router";

async function renderAt(path: string) {
  const router = createRouter({
    history: createMemoryHistory(),
    routes,
  });

  await router.push(path);
  await router.isReady();

  return mount(App, {
    global: {
      plugins: [createPinia(), router],
    },
  });
}

describe("App routing", () => {
  it("renders the landing page at root", async () => {
    const wrapper = await renderAt("/");

    expect(wrapper.text()).toContain("Issueflow Gateway");
    expect(wrapper.text()).toContain("Continue with GitLab");
  });
});
