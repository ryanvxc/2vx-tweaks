import type { ReactElement } from "react";
import { render as rtlRender, screen } from "@testing-library/react";
import { describe, expect, it } from "vitest";
import { TooltipProvider } from "@/components/ui/tooltip";
import type { CpuDetails, GpuDriver } from "@/types/system";
import { CpuFacts, GpuFacts, PeripheralFacts } from "./hardware-facts";

const render = (ui: ReactElement) => rtlRender(<TooltipProvider>{ui}</TooltipProvider>);

const cpu: CpuDetails = {
  performanceCores: 8,
  efficientCores: 0,
  logicalProcessors: 16,
  l2CacheBytes: null,
  l3CacheBytes: 32 * 1024 ** 2,
};

describe("hardware facts", () => {
  it("leaves out null values and one-kind-of-core CPUs' efficient cores", () => {
    render(<CpuFacts cpu={cpu} />);
    expect(screen.getByText("Performance cores")).toBeInTheDocument();
    expect(screen.getByText("L3 cache")).toBeInTheDocument();
    expect(screen.queryByText("L2 cache")).not.toBeInTheDocument();
    expect(screen.queryByText("Efficient cores")).not.toBeInTheDocument();
    expect(screen.queryByText(/unknown/i)).not.toBeInTheDocument();
  });

  it("warns about Windows' basic display driver", () => {
    const basic: GpuDriver = {
      name: "Microsoft Basic Display Adapter",
      version: "10.0.1",
      date: "2006-06-21",
      provider: "Microsoft",
      isBasicDriver: true,
    };
    render(<GpuFacts drivers={[basic]} />);
    expect(screen.getByText("Microsoft Basic Display Driver")).toBeInTheDocument();
  });

  it("shows no warning or missing fields for a normal driver", () => {
    const driver: GpuDriver = {
      name: "NVIDIA GeForce RTX",
      version: "560.1",
      date: null,
      provider: null,
      isBasicDriver: false,
    };
    render(<GpuFacts drivers={[driver]} />);
    expect(screen.queryByText("Microsoft Basic Display Driver")).not.toBeInTheDocument();
    expect(screen.getByText("560.1")).toBeInTheDocument();
    expect(screen.queryByText("Driver date")).not.toBeInTheDocument();
    expect(screen.queryByText("Provider")).not.toBeInTheDocument();
    expect(screen.getByText("NVIDIA drivers")).toBeInTheDocument();
  });

  it("folds peripherals after eight", () => {
    const devices = Array.from({ length: 10 }, (_, i) => ({
      name: `Mouse ${i}`,
      class: "mouse" as const,
      problem: i === 0,
    }));
    render(<PeripheralFacts devices={devices} />);
    expect(screen.getByText("Show 2 more")).toBeInTheDocument();
    expect(screen.queryByText("Mouse 9")).not.toBeInTheDocument();
    expect(screen.getByText("Windows reports a problem")).toBeInTheDocument();
  });
});
