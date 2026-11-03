import { render, screen } from "@testing-library/react";
import userEvent from "@testing-library/user-event";
import { describe, expect, it } from "vitest";
import { TooltipProvider } from "@/components/ui/tooltip";
import type { MetricsSample, SystemSnapshot } from "@/types/system";
import { UtilisationChart } from "./utilisation-chart";

const GIB = 1024 ** 3;

const sample = (timestampMs: number, gpuModel: string | null): MetricsSample =>
  ({
    timestampMs,
    cpuPercent: 10,
    perCorePercent: [],
    memoryUsedBytes: 8 * GIB,
    memoryTotalBytes: 32 * GIB,
    gpus: [{ model: gpuModel, usagePercent: 40, memoryTotalBytes: null }],
  }) as unknown as MetricsSample;

const SNAPSHOT = {
  cpu: { brand: "Intel(R) Core(TM) i5-14600KF  " },
  gpus: [{ vendor: "NVIDIA", model: "Snapshot GPU", dedicatedMemoryBytes: null }],
} as unknown as SystemSnapshot;

const pick = (user: ReturnType<typeof userEvent.setup>, name: string) =>
  user.click(screen.getByRole("radio", { name }));

// By slot: a plain "GPU" title would also match the GPU tab.
const title = (text: string) =>
  expect(document.querySelector('[data-slot="card-title"]')).toHaveTextContent(text);

describe("UtilisationChart title", () => {
  it("names the device for a single-series view", async () => {
    const user = userEvent.setup();
    render(
      <UtilisationChart
        samples={[sample(0, "NVIDIA GeForce RTX 4070"), sample(1000, "NVIDIA GeForce RTX 4070")]}
        snapshot={SNAPSHOT}
      />,
      { wrapper: TooltipProvider },
    );
    title("Utilisation");
    await pick(user, "CPU");
    title("Intel(R) Core(TM) i5-14600KF");
    await pick(user, "GPU");
    title("NVIDIA GeForce RTX 4070");
    await pick(user, "Memory");
    title("32.0 GB memory");
    await pick(user, "All");
    title("Utilisation");
  });

  it("falls back to the snapshot, then a plain label", async () => {
    const user = userEvent.setup();
    const { rerender } = render(
      <UtilisationChart samples={[sample(0, null), sample(1000, null)]} snapshot={SNAPSHOT} />,
      { wrapper: TooltipProvider },
    );
    await pick(user, "GPU");
    title("Snapshot GPU");

    rerender(<UtilisationChart samples={[]} snapshot={undefined} />);
    title("GPU");
    await pick(user, "CPU");
    title("CPU");
  });
});
