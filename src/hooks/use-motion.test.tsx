import { render } from "@testing-library/react";
import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { useUiStore } from "@/stores/ui-store";
import { useSlidingIndicator, useStaggerIn } from "./use-motion";

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
    const items = Array.from({ length: 40 }, (_, i) => String(i));
    render(<List tab="one" items={items} />);
    expect(animate).toHaveBeenCalledTimes(24);
    const delays = animate.mock.calls.map(([, options]) => (options as { delay: number }).delay);
    expect(Math.max(...delays)).toBe(144);
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
