import "naive-ui/es/vitest-setup.mjs";
import { createRequire } from "node:module";
import { ResizeObserver } from "./stubs/resize-observer";

if (typeof window !== "undefined") {
  const require = createRequire(import.meta.url);
  const resizeObserverUmd =
    require("@juggle/resize-observer/lib/exports/resize-observer.umd.js") as {
      addEventListener?: typeof window.addEventListener;
      removeEventListener?: typeof window.removeEventListener;
    };
  resizeObserverUmd.addEventListener = window.addEventListener.bind(window);
  resizeObserverUmd.removeEventListener =
    window.removeEventListener.bind(window);
  window.ResizeObserver = ResizeObserver as typeof window.ResizeObserver;
  globalThis.addEventListener = window.addEventListener.bind(window);
  globalThis.removeEventListener = window.removeEventListener.bind(window);
}

globalThis.ResizeObserver = ResizeObserver as typeof globalThis.ResizeObserver;
