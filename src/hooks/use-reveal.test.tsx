import { act, render, screen } from "@testing-library/react";
import { afterEach, describe, expect, it, vi } from "vitest";
import { useReveal } from "./use-reveal";

function List({ total, list }: { total: number; list: string }) {
  const { limit, end } = useReveal(2, total, list);
  return (
    <main>
      <output>{Math.min(limit, total)}</output>
      {limit < total && <div ref={end} />}
    </main>
  );
}

let nearEnd: () => void;

function watchTheEnd() {
  vi.stubGlobal(
    "IntersectionObserver",
    class {
      constructor(callback: (entries: { isIntersecting: boolean }[]) => void) {
        nearEnd = () => callback([{ isIntersecting: true }]);
      }
      observe() {}
      disconnect() {}
    },
  );
}

afterEach(() => {
  vi.unstubAllGlobals();
});

describe("useReveal", () => {
  it("draws another batch each time the end comes near, then stops watching", () => {
    watchTheEnd();
    const { container } = render(<List total={5} list="a" />);
    expect(screen.getByRole("status")).toHaveTextContent("2");

    act(() => nearEnd());
    expect(screen.getByRole("status")).toHaveTextContent("4");

    act(() => nearEnd());
    expect(screen.getByRole("status")).toHaveTextContent("5");
    expect(container.querySelector("output + div")).toBeNull();
  });

  it("starts again for a different list", () => {
    watchTheEnd();
    const { rerender } = render(<List total={5} list="a" />);
    act(() => nearEnd());
    expect(screen.getByRole("status")).toHaveTextContent("4");

    rerender(<List total={5} list="b" />);
    expect(screen.getByRole("status")).toHaveTextContent("2");
  });

  it("draws everything when it can't watch", () => {
    vi.stubGlobal("IntersectionObserver", undefined);
    render(<List total={5} list="a" />);
    expect(screen.getByRole("status")).toHaveTextContent("5");
  });
});
