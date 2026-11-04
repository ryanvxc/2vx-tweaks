import { act, fireEvent, render, screen, waitFor } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { clearQueryCache } from "@/hooks/use-backend-query";
import { FACTS } from "@/lib/splash-facts";
import { backend } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import { SplashScreen } from "./splash-screen";

const shownFact = () => FACTS.find((f) => screen.queryByText(f));

// React holds updates until act ends, so a single long step would hide the timers they start.
async function wait(ms: number) {
  for (let t = 0; t < ms; t += 100) await act(() => vi.advanceTimersByTimeAsync(100));
}

describe("SplashScreen", () => {
  beforeEach(() => {
    clearQueryCache();
    useUiStore.setState({ reduceMotion: false });
    vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
  });

  afterEach(() => {
    vi.useRealTimers();
    vi.restoreAllMocks();
  });

  it("holds long enough to see, then leaves by itself", async () => {
    vi.useFakeTimers();
    const onDone = vi.fn();
    render(<SplashScreen onDone={onDone} />);
    await wait(2400);
    expect(onDone).not.toHaveBeenCalled();
    await wait(1000);
    expect(onDone).toHaveBeenCalledOnce();
  });

  it("reports real progress, not time passing", async () => {
    vi.useFakeTimers();
    vi.spyOn(backend, "getTweakStatuses").mockReturnValue(new Promise(() => {}));
    render(<SplashScreen onDone={vi.fn()} />);
    await wait(3000);
    expect(screen.getByRole("status")).toHaveTextContent("Checking your tweaks");
    expect(screen.getByRole("progressbar")).toHaveAttribute("aria-valuenow", "67");
  });

  it("never traps anyone behind a stuck call", async () => {
    vi.useFakeTimers();
    vi.spyOn(backend, "getSystemSnapshot").mockReturnValue(new Promise(() => {}));
    const onDone = vi.fn();
    render(<SplashScreen onDone={onDone} />);
    await wait(7500);
    expect(onDone).not.toHaveBeenCalled();
    await wait(1200);
    expect(onDone).toHaveBeenCalledOnce();
  });

  it("stays while the cursor is playing, but not for ever", async () => {
    vi.useFakeTimers();
    const onDone = vi.fn();
    const { container } = render(<SplashScreen onDone={onDone} />);
    const canvas = container.querySelector("canvas")!;
    const play = async (ms: number) => {
      for (let t = 0; t < ms; t += 200) {
        fireEvent.pointerMove(canvas, { clientX: 10, clientY: 10 });
        await wait(200);
      }
    };

    await play(3400);
    expect(onDone).not.toHaveBeenCalled();
    await play(4400);
    expect(onDone).toHaveBeenCalledOnce();
  });

  it("skips on Esc", async () => {
    const onDone = vi.fn();
    render(<SplashScreen onDone={onDone} />);
    fireEvent.keyDown(window, { key: "Escape" });
    await waitFor(() => expect(onDone).toHaveBeenCalledOnce());
  });

  it("skips on Esc with a window button focused", async () => {
    const onDone = vi.fn();
    render(<SplashScreen onDone={onDone} />);
    const button = screen.getByRole("button", { name: "Close" });
    button.focus();
    fireEvent.keyDown(button, { key: "Escape" });
    await waitFor(() => expect(onDone).toHaveBeenCalledOnce());
  });

  it("shows another fact on Space and on a click", () => {
    render(<SplashScreen onDone={vi.fn()} />);
    const first = shownFact();
    fireEvent.keyDown(window, { key: " " });
    const second = shownFact();
    expect(second).not.toBe(first);

    fireEvent.click(screen.getByRole("button", { name: second }));
    expect(shownFact()).not.toBe(second);
  });

  it("shows a still screen, briefly, with Reduce motion on", async () => {
    vi.useFakeTimers();
    useUiStore.setState({ reduceMotion: true });
    const onDone = vi.fn();
    const { container } = render(<SplashScreen onDone={onDone} />);
    expect(container.querySelector("canvas")).not.toBeInTheDocument();
    expect(screen.queryByText(/pulse/)).not.toBeInTheDocument();
    await wait(1000);
    expect(onDone).not.toHaveBeenCalled();
    await wait(400);
    expect(onDone).toHaveBeenCalledOnce();
  });
});
