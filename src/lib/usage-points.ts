import type { MetricsSample } from "@/types/system";
import { percentOf } from "./format";

export type Range = "1m" | "5m";

// The sampler keeps five minutes, so the long range needs no extra history. Longer buckets
// keep the five-minute line as calm as the one-minute one.
export const RANGES: Record<Range, { seconds: number; bucketMs: number }> = {
  "1m": { seconds: 60, bucketMs: 5_000 },
  "5m": { seconds: 300, bucketMs: 15_000 },
};

export interface UsagePoint {
  /** Seconds before the newest sample, as a negative number. */
  ago: number;
  cpu: number;
  gpu: number | null;
  ram: number;
  /** Bytes per second, summed over all adapters. */
  rx: number;
  tx: number;
}

const mean = (values: number[]) => values.reduce((sum, v) => sum + v, 0) / values.length;
const round = (v: number) => Math.round(v * 10) / 10;

/** The samples inside the range, newest last. */
export function samplesIn(samples: MetricsSample[], range: Range): MetricsSample[] {
  const now = samples.at(-1)?.timestampMs ?? 0;
  return samples.filter((s) => now - s.timestampMs <= RANGES[range].seconds * 1000);
}

/**
 * The range as bucket averages, which reads calmer than every one-second spike.
 * Buckets follow the clock, so a finished point keeps its value and only slides left.
 */
export function usagePoints(samples: MetricsSample[], range: Range = "1m"): UsagePoint[] {
  const { seconds, bucketMs } = RANGES[range];
  const now = samples.at(-1)?.timestampMs ?? 0;
  const buckets = new Map<number, MetricsSample[]>();
  for (const sample of samples) {
    if (now - sample.timestampMs > seconds * 1000) continue;
    const key = Math.floor(sample.timestampMs / bucketMs);
    const bucket = buckets.get(key);
    if (bucket) bucket.push(sample);
    else buckets.set(key, [sample]);
  }

  return [...buckets.values()].map((group) => {
    const gpu = group.flatMap((s) => s.gpus[0]?.usagePercent ?? []);
    return {
      ago: Math.round((group.at(-1)!.timestampMs - now) / 1000),
      cpu: round(mean(group.map((s) => s.cpuPercent))),
      gpu: gpu.length > 0 ? round(mean(gpu)) : null,
      ram: round(mean(group.map((s) => percentOf(s.memoryUsedBytes, s.memoryTotalBytes)))),
      rx: Math.round(mean(group.map((s) => s.networkRxBytesPerSec))),
      tx: Math.round(mean(group.map((s) => s.networkTxBytesPerSec))),
    };
  });
}
