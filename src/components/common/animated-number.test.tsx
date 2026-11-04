import { render, screen, waitFor } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { useUiStore } from "@/stores/ui-store";
import { AnimatedNumber } from "./animated-number";

describe("AnimatedNumber", () => {
  it("counts up to the value", async () => {
    useUiStore.setState({ reduceMotion: false });
    render(
      <span data-testid="n">
        <AnimatedNumber value={42} duration={30} />
      </span>,
    );
    expect(screen.getByTestId("n")).toHaveTextContent("0");
    await waitFor(() => expect(screen.getByTestId("n")).toHaveTextContent("42"));
  });

  it("carries on from the last value when it changes", async () => {
    useUiStore.setState({ reduceMotion: false });
    const { rerender } = render(
      <span data-testid="n">
        <AnimatedNumber value={10} duration={20} />
      </span>,
    );
    await waitFor(() => expect(screen.getByTestId("n")).toHaveTextContent("10"));
    rerender(
      <span data-testid="n">
        <AnimatedNumber value={20} duration={20} />
      </span>,
    );
    expect(screen.getByTestId("n")).toHaveTextContent("10");
    await waitFor(() => expect(screen.getByTestId("n")).toHaveTextContent("20"));
  });

  it("shows the value straight away with Reduce motion on", () => {
    useUiStore.setState({ reduceMotion: true });
    render(
      <span data-testid="n">
        <AnimatedNumber value={1024} format={(v) => `${v} MB`} />
      </span>,
    );
    expect(screen.getByTestId("n")).toHaveTextContent("1024 MB");
  });
});
