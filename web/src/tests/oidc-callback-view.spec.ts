import { mount } from "@vue/test-utils";
import { createPinia } from "pinia";
import { createMemoryHistory, createRouter } from "vue-router";
import App from "@/App.vue";
import { i18n } from "@/i18n";
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
        plugins: [createPinia(), router, i18n],
      },
    }),
    router,
  };
}

describe("OIDC callback view", () => {
  it("shows a success state from the gateway redirect", async () => {
    const { wrapper } = await renderAt("/auth/callback/oidc?result=success");

    expect(wrapper.text()).toContain("Sign-in completed");
    expect(wrapper.text()).toContain("Opening the workbench");
  });
});
