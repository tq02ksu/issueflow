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

  return mount(App, {
    global: {
      plugins: [createPinia(), router, i18n],
    },
  });
}

describe("App routing", () => {
  it("renders the landing page at root", async () => {
    const wrapper = await renderAt("/");

    expect(wrapper.text()).toContain("Issueflow Gateway");
    expect(wrapper.text()).toContain("Continue to sign in");
  });

  it("registers a standalone pending actions route", () => {
    expect(
      routes.some(
        (route) =>
          route.name === "workbench-pending-actions" &&
          route.path === "/workbench/pending-actions",
      ),
    ).toBe(true);
  });
});
