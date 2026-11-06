import { useRef } from "react";
import { fireEvent, render } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { noteTabStep } from "@/lib/motion";
import { useUiStore } from "@/stores/ui-store";
import { useSlidingIndicator, useSpotlight, useStaggerIn } from "./use-motion";

function List({ tab, items }: { tab: string | null; items: string[] }) {
  const root = useStaggerIn<HTMLUListElement>(tab);
  return (
    <ul ref={root}>
      {items.map((i) => (
        <li key={i} data-stagger>
          {i}
        </li>
      ))}
    </ul>
  );
}

function Tabs({ active }: { active: string | null }) {
  const { container, indicator } = useSlidingIndicator<HTMLDivElement>('[data-on="true"]');
  return (
    <div ref={container}>
      {["a", "b"].map((t) => (
        <button key={t} data-on={t === active} />
      ))}
      <span ref={indicator} data-testid="mark" />
    </div>
  );
}

function Lit() {
  const root = useRef<HTMLDivElement>(null);
  useSpotlight(root);
  return (
    <div ref={root} data-testid="page">
      <div data-spot data-testid="near" />
      <div data-spot data-testid="far" />
    </div>
  );
}

describe("useSpotlight", () => {
  const place = (el: HTMLElement, left: number) =>
    vi
      .spyOn(el, "getBoundingClientRect")
      .mockReturnValue({ left, right: left + 100, top: 0, bottom: 100 } as DOMRect);

  beforeEach(() => {
    useUiStore.setState({ reduceMotion: false });
    vi.stubGlobal("requestAnimationFrame", (draw: FrameRequestCallback) => {
      draw(0);
      return 1;
    });
  });
  afterEach(() => vi.unstubAllGlobals());

  it("lights the cards in reach, and clears them when the pointer leaves", () => {
    const { getByTestId } = render(<Lit />);
    const near = getByTestId("near");
    const far = getByTestId("far");
    place(near, 0);
    place(far, 900);

    fireEvent.pointerMove(getByTestId("page"), { clientX: 130, clientY: 40 });
    expect(near.hasAttribute("data-lit")).toBe(true);
    expect(near.style.getPropertyValue("--mx")).toBe("130px");
    expect(far.hasAttribute("data-lit")).toBe(false);

    fireEvent.pointerLeave(getByTestId("page"));
    expect(near.hasAttribute("data-lit")).toBe(false);
  });

  it("stays dark with Reduce motion on", () => {
    useUiStore.setState({ reduceMotion: true });
    const { getByTestId } = render(<Lit />);
    place(getByTestId("near"), 0);
    fireEvent.pointerMove(getByTestId("page"), { clientX: 50, clientY: 40 });
    expect(getByTestId("near").hasAttribute("data-lit")).toBe(false);
  });
});

describe("useStaggerIn", () => {
  const animate = vi.fn();

  beforeEach(() => {
    animate.mockClear();
    Element.prototype.animate = animate;
    useUiStore.setState({ reduceMotion: false });
  });
  afterEach(() => {
    delete (Element.prototype as Partial<Element>).animate;
  });

  it("waits for the data, then plays once for each new key", () => {
    const { rerender } = render(<List tab={null} items={["a", "b"]} />);
    expect(animate).not.toHaveBeenCalled();

    rerender(<List tab="one" items={["a", "b"]} />);
    expect(animate).toHaveBeenCalledTimes(2);

    rerender(<List tab="two" items={["a", "b"]} />);
    expect(animate).toHaveBeenCalledTimes(4);
  });

  it("leaves a filtered list alone", () => {
    const { rerender } = render(<List tab="one" items={["a", "b", "c"]} />);
    animate.mockClear();
    rerender(<List tab="one" items={["a"]} />);
    rerender(<List tab="one" items={["a", "b", "c"]} />);
    expect(animate).not.toHaveBeenCalled();
  });

  it("caps the delay and the number of items", () => {
    // Three columns of 100px cards: the wave is a card's row plus its column.
    const rect = vi.spyOn(Element.prototype, "getBoundingClientRect").mockImplementation(function (
      this: Element,
    ) {
      const i = [...(this.parentElement?.children ?? [])].indexOf(this);
      const left = (i % 3) * 100;
      const top = Math.floor(i / 3) * 100;
      return { left, top, width: 100, height: 100 } as DOMRect;
    });
    const items = Array.from({ length: 40 }, (_, i) => String(i));
    render(<List tab="one" items={items} />);
    rect.mockRestore();
    expect(animate).toHaveBeenCalledTimes(24);
    const delays = animate.mock.calls.map(([, options]) => (options as { delay: number }).delay);
    expect(delays.slice(0, 4)).toEqual([0, 30, 60, 30]);
    expect(Math.max(...delays)).toBe(180);
  });

  it("slides in from the side of the tab you moved to, once", () => {
    const from = () =>
      (animate.mock.lastCall?.[0] as { transform: string }[] | undefined)?.[0]?.transform;
    const { rerender } = render(<List tab="one" items={["a"]} />);
    expect(from()).toContain("translateY");

    noteTabStep(-1);
    rerender(<List tab="two" items={["a"]} />);
    expect(from()).toContain("translateX(-22px)");

    rerender(<List tab="three" items={["a"]} />);
    expect(from()).toContain("translateY");
  });

  it("does nothing with Reduce motion on", () => {
    useUiStore.setState({ reduceMotion: true });
    render(<List tab="one" items={["a"]} />);
    expect(animate).not.toHaveBeenCalled();
  });
});

describe("useSlidingIndicator", () => {
  it("moves onto the active item and hides without one", () => {
    const left = vi.spyOn(HTMLElement.prototype, "offsetLeft", "get").mockReturnValue(40);
    const width = vi.spyOn(HTMLElement.prototype, "offsetWidth", "get").mockReturnValue(90);

    const { getByTestId, rerender } = render(<Tabs active="a" />);
    const mark = getByTestId("mark");
    expect(mark.style.getPropertyValue("--x")).toBe("40px");
    expect(mark.style.getPropertyValue("--w")).toBe("90px");
    expect(mark.style.opacity).toBe("1");

    rerender(<Tabs active={null} />);
    expect(mark.style.opacity).toBe("0");

    left.mockRestore();
    width.mockRestore();
  });

  it("keeps transitions on after placing itself the first time", () => {
    const { getByTestId } = render(<Tabs active="a" />);
    expect(getByTestId("mark").style.transition).toBe("");
  });
});
