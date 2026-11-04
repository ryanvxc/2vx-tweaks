import type { MetricsSample } from "@/types/system";
import { percentOf } from "./format";

const WINDOW_MS = 60_000;
const BUCKET_MS = 5_000;

export interface UsagePoint {
  /** Seconds before the newest sample, as a negative number. */
  ago: number;
  cpu: number;
  gpu: number | null;
  ram: number;
}

const mean = (values: number[]) => values.reduce((sum, v) => sum + v, 0) / values.length;
const round = (v: number) => Math.round(v * 10) / 10;

/**
 * The last minute as five-second averages, which reads calmer than every one-second spike.
 * Buckets follow the clock, so a finished point keeps its value and only slides left.
 */
export function usagePoints(samples: MetricsSample[]): UsagePoint[] {
  const now = samples.at(-1)?.timestampMs ?? 0;
  const buckets = new Map<number, MetricsSample[]>();
  for (const sample of samples) {
    if (now - sample.timestampMs > WINDOW_MS) continue;
    const key = Math.floor(sample.timestampMs / BUCKET_MS);
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
    };
  });
}
