export class ResizeObserver {
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

export class ResizeObserverEntry {}

export class ResizeObserverSize {}
