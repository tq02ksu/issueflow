import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vitest/config";
import vue from "@vitejs/plugin-vue";

export default defineConfig({
  plugins: [vue()],
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./src", import.meta.url)),
      "@juggle/resize-observer/lib/exports/resize-observer.umd.js":
        fileURLToPath(
          new URL("./src/tests/stubs/resize-observer.ts", import.meta.url),
        ),
      "@juggle/resize-observer": fileURLToPath(
        new URL("./src/tests/stubs/resize-observer.ts", import.meta.url),
      ),
    },
  },
  test: {
    environment: "jsdom",
    environmentOptions: {
      url: "http://localhost:5173",
    },
    globals: true,
    setupFiles: ["./src/tests/setup.ts"],
  },
});
