import { describe, expect, it } from "vitest";
import type { MetricsSample } from "@/types/system";
import { samplesIn, usagePoints } from "./usage-points";

const sample = (seconds: number, cpu: number, gpu: number | null = 50): MetricsSample =>
  ({
    timestampMs: seconds * 1000,
    cpuPercent: cpu,
    memoryUsedBytes: 8,
    memoryTotalBytes: 32,
    networkRxBytesPerSec: cpu * 100,
    networkTxBytesPerSec: 10,
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
      { ago: -2, cpu: 20, gpu: 50, ram: 25, rx: 2000, tx: 10 },
      { ago: 0, cpu: 70, gpu: 50, ram: 25, rx: 7000, tx: 10 },
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

  it("keeps five minutes in the long range, in fifteen-second averages", () => {
    const samples = Array.from({ length: 301 }, (_, i) => sample(i, 10));
    expect(usagePoints(samples, "1m").length).toBeLessThanOrEqual(13);
    const long = usagePoints(samples, "5m");
    expect(long.length).toBeGreaterThan(18);
    expect(long.length).toBeLessThanOrEqual(21);
    expect(long[0]?.ago).toBeLessThan(-280);
  });

  it("averages download and upload on their own", () => {
    const [point] = usagePoints([sample(100, 10), sample(101, 30)]);
    expect(point).toMatchObject({ rx: 2000, tx: 10 });
  });
});

describe("samplesIn", () => {
  it("returns only what the range covers", () => {
    const samples = [sample(0, 1), sample(200, 2), sample(290, 3), sample(300, 4)];
    expect(samplesIn(samples, "1m").map((s) => s.cpuPercent)).toEqual([3, 4]);
    expect(samplesIn(samples, "5m")).toHaveLength(4);
    expect(samplesIn([], "1m")).toEqual([]);
  });
});
