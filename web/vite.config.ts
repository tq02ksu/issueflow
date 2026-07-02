import { fileURLToPath, URL } from "node:url";
import { defineConfig, loadEnv } from "vite";
import vue from "@vitejs/plugin-vue";

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), "");
  const isMockMode = (env.VITE_APP_MODE || "default") === "mock";

  return {
    plugins: [
      vue(),
      {
        name: "issueflow-mock-auth-login",
        configureServer(server) {
          if (!isMockMode) {
            return;
          }

          server.middlewares.use((req, res, next) => {
            if (req.url === "/api/auth/login") {
              res.statusCode = 302;
              res.setHeader(
                "Location",
                "/auth/callback/oidc?result=success&token=mock-token",
              );
              res.end();
              return;
            }

            next();
          });
        },
      },
    ],
    resolve: {
      alias: {
        "@": fileURLToPath(new URL("./src", import.meta.url)),
      },
    },
    define: {
      __APP_MODE__: JSON.stringify(env.VITE_APP_MODE || "default"),
    },
    server: {
      host: "127.0.0.1",
      port: 5173,
      middlewareMode: false,
      proxy: {
        "/api": "http://127.0.0.1:8080",
      },
    },
    build: {
      outDir: "dist",
      assetsDir: "assets",
      sourcemap: true,
      rollupOptions: {
        output: {
          entryFileNames: "assets/app.js",
          chunkFileNames: "assets/[name].js",
          assetFileNames: (assetInfo) => {
            const assetNames =
              assetInfo.names ?? (assetInfo.name ? [assetInfo.name] : []);

            if (assetNames.some((name) => name.endsWith(".css"))) {
              return "assets/app.css";
            }

            return "assets/[name][extname]";
          },
        },
      },
    },
  };
});
