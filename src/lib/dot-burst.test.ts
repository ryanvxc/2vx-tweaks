import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useUiStore } from "@/stores/ui-store";
import { burstAt } from "./dot-burst";

const canvases = () => document.body.querySelectorAll("canvas");

describe("burstAt", () => {
  let frames: FrameRequestCallback[];

  beforeEach(() => {
    frames = [];
    useUiStore.setState({ reduceMotion: false });
    Element.prototype.animate = vi.fn();
    vi.stubGlobal("requestAnimationFrame", (draw: FrameRequestCallback) => frames.push(draw));
    vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue({
      clearRect: vi.fn(),
      fillRect: vi.fn(),
    } as unknown as CanvasRenderingContext2D);
  });
  afterEach(() => {
    delete (Element.prototype as Partial<Element>).animate;
    vi.unstubAllGlobals();
    vi.restoreAllMocks();
  });

  it("leaves nothing on the page or running once the dots have gone", () => {
    burstAt(10, 10, { count: 5 });
    expect(canvases()).toHaveLength(1);
    expect(frames).toHaveLength(1);

    // Longer than any dot lives.
    const start = performance.now();
    for (let i = 1; i < 100 && frames.length > 0; i++) frames.shift()!(start + i * 17);
    expect(frames).toHaveLength(0);
    expect(canvases()).toHaveLength(0);
  });

  it("does nothing with Reduce motion on", () => {
    useUiStore.setState({ reduceMotion: true });
    burstAt(10, 10);
    expect(canvases()).toHaveLength(0);
    expect(frames).toHaveLength(0);
  });
});
