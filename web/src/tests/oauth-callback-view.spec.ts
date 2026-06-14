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

  return {
    wrapper: mount(App, {
      global: {
        plugins: [createPinia(), router],
      },
    }),
    router,
  };
}

describe("OAuth callback view", () => {
  it("shows a success state from the gateway redirect", async () => {
    const { wrapper } = await renderAt("/auth/callback/gitlab?result=success");

    expect(wrapper.text()).toContain("GitLab connected");
    expect(wrapper.text()).toContain("Opening the workbench");
  });
});
