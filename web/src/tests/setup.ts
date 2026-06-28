import * as resizeObserverModule from "@juggle/resize-observer";

const globalTarget = globalThis as typeof globalThis & {
  addEventListener?: typeof window.addEventListener;
  removeEventListener?: typeof window.removeEventListener;
};

const resizeObserverTarget = resizeObserverModule as typeof resizeObserverModule & {
  addEventListener?: typeof window.addEventListener;
  removeEventListener?: typeof window.removeEventListener;
};

if (typeof window !== "undefined") {
  const addEventListener = window.addEventListener.bind(window);
  const removeEventListener = window.removeEventListener.bind(window);

  globalTarget.addEventListener = addEventListener;
  globalTarget.removeEventListener = removeEventListener;
  resizeObserverTarget.addEventListener = addEventListener;
  resizeObserverTarget.removeEventListener = removeEventListener;
}
