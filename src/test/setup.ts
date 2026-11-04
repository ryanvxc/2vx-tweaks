import "@testing-library/jest-dom/vitest";
import { cleanup } from "@testing-library/react";
import { afterEach, beforeEach } from "vitest";
import { useUiStore } from "@/stores/ui-store";

afterEach(() => cleanup());

// Counting numbers would otherwise show in-between values to assertions. The release script
// tests run in Node, with no storage for the store to persist to.
if (typeof localStorage !== "undefined") {
  beforeEach(() => useUiStore.setState({ reduceMotion: true }));
}

// jsdom lacks ResizeObserver, which Recharts' ResponsiveContainer and Radix rely on.
class ResizeObserverStub {
  observe() {}
  unobserve() {}
  disconnect() {}
}
globalThis.ResizeObserver ??= ResizeObserverStub as unknown as typeof ResizeObserver;
