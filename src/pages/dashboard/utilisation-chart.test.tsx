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
    networkRxBytesPerSec: 2048,
    networkTxBytesPerSec: 1024,
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

const stats = () => document.querySelector('[data-slot="card-description"]');

describe("UtilisationChart network and range", () => {
  const two = [sample(0, "GPU"), sample(1000, "GPU")];

  it("plots network on its own scale, with speed, average and peak", async () => {
    const user = userEvent.setup();
    render(<UtilisationChart samples={two} snapshot={SNAPSHOT} />, { wrapper: TooltipProvider });
    await pick(user, "Network");
    title("All adapters");
    expect(stats()).toHaveTextContent(
      "Down 2.0 KB/s · up 1.0 KB/s · average down 2.0 KB/s · peak down 2.0 KB/s",
    );
  });

  it("shows now, average and peak for a single device, and nothing on All", async () => {
    const user = userEvent.setup();
    render(<UtilisationChart samples={two} snapshot={SNAPSHOT} />, { wrapper: TooltipProvider });
    expect(stats()).toBeNull();
    await pick(user, "CPU");
    expect(stats()).toHaveTextContent("Now 10% · average 10% · peak 10%");
    await pick(user, "RAM");
    expect(stats()).toHaveTextContent("Now 8.0 GB (25%) · average 25% · peak 25%");
  });

  it("says so when Windows gives no GPU reading", async () => {
    const user = userEvent.setup();
    const noGpu = (t: number) => ({ ...sample(t, null), gpus: [] });
    render(<UtilisationChart samples={[noGpu(0), noGpu(1000)]} snapshot={SNAPSHOT} />, {
      wrapper: TooltipProvider,
    });
    await pick(user, "GPU");
    expect(stats()).toHaveTextContent("Windows gave no GPU reading");
  });

  it("measures the stats over the chosen range", async () => {
    const user = userEvent.setup();
    const long = [
      { ...sample(0, "GPU"), cpuPercent: 90 },
      sample(200_000, "GPU"),
      sample(201_000, "GPU"),
    ];
    render(<UtilisationChart samples={long} snapshot={SNAPSHOT} />, { wrapper: TooltipProvider });
    await pick(user, "CPU");
    expect(stats()).toHaveTextContent("peak 10%");
    await user.click(screen.getByRole("radio", { name: "5m" }));
    expect(stats()).toHaveTextContent("peak 90%");
    expect(screen.getByRole("radio", { name: "1m" })).not.toBeChecked();
  });
});

describe("UtilisationChart title", () => {
  it("opens on All, with each device and its newest value in the title", () => {
    render(
      <UtilisationChart
        samples={[sample(0, "NVIDIA GeForce RTX 4070"), sample(1000, "NVIDIA GeForce RTX 4070")]}
        snapshot={SNAPSHOT}
      />,
      { wrapper: TooltipProvider },
    );
    expect(screen.getByRole("radio", { name: "All" })).toBeChecked();
    title("CPU10%GPU40%RAM25%");
  });

  it("marks a device with no reading instead of showing zero", () => {
    const noGpu = (timestampMs: number) => ({ ...sample(timestampMs, null), gpus: [] });
    render(<UtilisationChart samples={[noGpu(0), noGpu(1000)]} snapshot={SNAPSHOT} />, {
      wrapper: TooltipProvider,
    });
    title("GPU—");
  });

  it("names the device for a single-series view", async () => {
    const user = userEvent.setup();
    render(
      <UtilisationChart
        samples={[sample(0, "NVIDIA GeForce RTX 4070"), sample(1000, "NVIDIA GeForce RTX 4070")]}
        snapshot={SNAPSHOT}
      />,
      { wrapper: TooltipProvider },
    );
    await pick(user, "CPU");
    title("Intel(R) Core(TM) i5-14600KF");
    await pick(user, "GPU");
    title("NVIDIA GeForce RTX 4070");
    await pick(user, "RAM");
    title("32.0 GB system memory");
    await pick(user, "CPU");
    title("Intel(R) Core(TM) i5-14600KF");
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
    await pick(user, "RAM");
    title("Memory");
  });
});
