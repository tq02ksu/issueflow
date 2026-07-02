import "naive-ui/es/vitest-setup.mjs";
import { afterAll, afterEach, beforeAll, vi } from "vitest";
import { server } from "@/mocks/server";

const resizeObserverMock = vi.hoisted(() => {
  class ResizeObserverMock {
    constructor(callback: ResizeObserverCallback) {
      void callback;
    }

    observe(target: Element) {
      void target;
    }

    unobserve(target: Element) {
      void target;
    }

    disconnect() {}
  }

  return { ResizeObserverMock };
});

vi.mock("@juggle/resize-observer", () => ({
  ResizeObserver: resizeObserverMock.ResizeObserverMock,
}));

vi.mock("@juggle/resize-observer/lib/exports/resize-observer.umd.js", () => ({
  ResizeObserver: resizeObserverMock.ResizeObserverMock,
}));

beforeAll(() => {
  server.listen({ onUnhandledRequest: "bypass" });
});

afterEach(() => {
  server.resetHandlers();
});

afterAll(() => {
  server.close();
});

if (typeof window !== "undefined") {
  window.ResizeObserver =
    resizeObserverMock.ResizeObserverMock as typeof window.ResizeObserver;
  globalThis.addEventListener = window.addEventListener.bind(window);
  globalThis.removeEventListener = window.removeEventListener.bind(window);
  const globalObject = globalThis as typeof globalThis & {
    global?: typeof globalThis & {
      addEventListener?: typeof window.addEventListener;
      removeEventListener?: typeof window.removeEventListener;
    };
  };
  globalObject.global = globalThis;
  globalObject.global.addEventListener = window.addEventListener.bind(window);
  globalObject.global.removeEventListener =
    window.removeEventListener.bind(window);
  window.addEventListener("error", (event) => {
    if (String(event.error).includes("global.removeEventListener")) {
      event.preventDefault();
    }
  });
}

globalThis.ResizeObserver =
  resizeObserverMock.ResizeObserverMock as typeof globalThis.ResizeObserver;
