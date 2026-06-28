import "naive-ui/es/vitest-setup.mjs";
import { createRequire } from "node:module";

if (typeof window !== "undefined") {
  const require = createRequire(import.meta.url);
  const resizeObserverUmd =
    require("@juggle/resize-observer/lib/exports/resize-observer.umd.js") as {
      addEventListener?: typeof window.addEventListener;
      removeEventListener?: typeof window.removeEventListener;
    };

  globalThis.ResizeObserver = window.ResizeObserver;
  resizeObserverUmd.addEventListener = window.addEventListener.bind(window);
  resizeObserverUmd.removeEventListener =
    window.removeEventListener.bind(window);
}
