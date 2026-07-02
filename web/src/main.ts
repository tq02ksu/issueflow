import { createApp } from "vue";
import { createPinia } from "pinia";
import { provideA2UI, DEFAULT_CATALOG, defaultTheme } from "a2ui-vue";
import "a2ui-vue/dist/a2ui-vue.css";
import App from "./App.vue";
import { router } from "./router";
import "./styles/tokens.css";
import "./styles/main.css";

async function enableMocks() {
  if (import.meta.env.VITE_APP_MODE !== "mock") {
    return;
  }

  const { worker } = await import("@/mocks/browser");
  await worker.start({ onUnhandledRequest: "bypass" });
}

await enableMocks();

const app = createApp(App);

app.use(createPinia());
app.use(router);
provideA2UI({ app, catalog: DEFAULT_CATALOG, theme: defaultTheme });
app.mount("#app");
