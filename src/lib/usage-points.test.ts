import { describe, expect, it } from "vitest";
import type { MetricsSample } from "@/types/system";
import { usagePoints } from "./usage-points";

const sample = (seconds: number, cpu: number, gpu: number | null = 50): MetricsSample =>
  ({
    timestampMs: seconds * 1000,
    cpuPercent: cpu,
    memoryUsedBytes: 8,
    memoryTotalBytes: 32,
    gpus: [{ usagePercent: gpu }],
  }) as unknown as MetricsSample;

describe("usagePoints", () => {
  it("averages each five seconds and ends at the newest sample", () => {
    const points = usagePoints([
      sample(100, 10),
      sample(101, 20),
      sample(104, 30),
      sample(105, 60),
      sample(106, 80),
    ]);
    expect(points).toEqual([
      { ago: -2, cpu: 20, gpu: 50, ram: 25 },
      { ago: 0, cpu: 70, gpu: 50, ram: 25 },
    ]);
  });

  it("keeps a finished point when a new sample arrives", () => {
    const first = [sample(100, 10), sample(104, 30), sample(105, 60)];
    const [before] = usagePoints(first);
    const [after] = usagePoints([...first, sample(106, 90)]);
    expect(after?.cpu).toBe(before?.cpu);
    expect(after?.ago).toBe((before?.ago ?? 0) - 1);
  });

  it("leaves out anything older than a minute", () => {
    expect(usagePoints([sample(0, 99), sample(100, 10)])).toHaveLength(1);
  });

  it("averages only the readings a GPU reported, and has none without them", () => {
    expect(usagePoints([sample(100, 1, 40), sample(101, 1, null)])[0]?.gpu).toBe(40);
    expect(usagePoints([sample(100, 1, null)])[0]?.gpu).toBeNull();
  });

  it("has nothing to plot without samples", () => {
    expect(usagePoints([])).toEqual([]);
  });
});
