/** Seconds of idle pings before the user is asked to load the connection. */
export const BASELINE_SECONDS = 10;
export const LOAD_SECONDS = 30;
export const TEST_SECONDS = BASELINE_SECONDS + LOAD_SECONDS;

/**
 * Traffic above the idle level that counts as load. About 2 Mbit/s: a connection slower than
 * that can't be filled by this much, and below it nothing queues.
 */
export const LOAD_MIN_BYTES_PER_SEC = 250_000;
/** Seconds at that level before the load phase counts as having reached the connection. */
export const LOAD_MIN_SECONDS = 5;

export type Phase = "baseline" | "load" | "done";

/** Round-trip time in ms; `null` when no reply came; `undefined` when the target wasn't asked. */
export type Reading = number | null | undefined;

export interface Tick {
  /** Seconds since the test started. */
  second: number;
  gateway: Reading;
  internet: Reading;
  /** Download plus upload across all adapters; `null` when no sample was available. */
  bytesPerSec: number | null;
}

export interface TargetResult {
  idleMs: number | null;
  loadedMs: number | null;
  /** Loaded minus idle, not clamped: a negative number is a real measurement. */
  addedMs: number | null;
  idleLossPercent: number | null;
  loadedLossPercent: number | null;
}

export interface BufferbloatResult {
  /** Whether traffic actually rose during the load phase. Without it the numbers mean nothing. */
  loadReached: boolean;
  /** Seconds in the load phase above the threshold. */
  loadedSeconds: number;
  baselineBytesPerSec: number;
  peakBytesPerSec: number | null;
  /** `null` when that target was never pinged (no gateway). */
  gateway: TargetResult | null;
  internet: TargetResult | null;
}

export function phaseAt(second: number): Phase {
  if (second < BASELINE_SECONDS) return "baseline";
  return second < TEST_SECONDS ? "load" : "done";
}

const mean = (values: number[]) =>
  values.length === 0 ? null : values.reduce((a, b) => a + b, 0) / values.length;

function phaseStats(ticks: Tick[], key: "gateway" | "internet") {
  const asked = ticks.map((t) => t[key]).filter((r): r is number | null => r !== undefined);
  const replies = asked.filter((r): r is number => r !== null);
  return {
    averageMs: mean(replies),
    lossPercent: asked.length === 0 ? null : ((asked.length - replies.length) / asked.length) * 100,
  };
}

function targetResult(
  baseline: Tick[],
  load: Tick[],
  key: "gateway" | "internet",
): TargetResult | null {
  if ([...baseline, ...load].every((t) => t[key] === undefined)) return null;
  const idle = phaseStats(baseline, key);
  const loaded = phaseStats(load, key);
  return {
    idleMs: idle.averageMs,
    loadedMs: loaded.averageMs,
    addedMs:
      idle.averageMs !== null && loaded.averageMs !== null
        ? loaded.averageMs - idle.averageMs
        : null,
    idleLossPercent: idle.lossPercent,
    loadedLossPercent: loaded.lossPercent,
  };
}

/** Averages, added latency, loss and whether the load phase really loaded the connection. */
export function analyse(ticks: Tick[]): BufferbloatResult {
  const baseline = ticks.filter((t) => phaseAt(t.second) === "baseline");
  const load = ticks.filter((t) => phaseAt(t.second) === "load");
  const rates = (list: Tick[]) => list.flatMap((t) => t.bytesPerSec ?? []);
  const baselineBytesPerSec = mean(rates(baseline)) ?? 0;
  const loadRates = rates(load);
  const loadedSeconds = loadRates.filter(
    (b) => b - baselineBytesPerSec >= LOAD_MIN_BYTES_PER_SEC,
  ).length;
  return {
    loadReached: loadedSeconds >= LOAD_MIN_SECONDS,
    loadedSeconds,
    baselineBytesPerSec,
    peakBytesPerSec: loadRates.length === 0 ? null : Math.max(...loadRates),
    gateway: targetResult(baseline, load, "gateway"),
    internet: targetResult(baseline, load, "internet"),
  };
}

export function formatMs(value: number | null | undefined): string {
  if (value == null || !Number.isFinite(value)) return "—";
  return `${value.toFixed(Math.abs(value) >= 100 ? 0 : 1)} ms`;
}

/** Signed, so a drop is visible as one. */
export function formatAdded(value: number | null | undefined): string {
  if (value == null || !Number.isFinite(value)) return "—";
  return `${value > 0 ? "+" : ""}${formatMs(value)}`;
}

export function formatLoss(value: number | null | undefined): string {
  if (value == null || !Number.isFinite(value)) return "—";
  return `${value.toFixed(value === 0 || value >= 10 ? 0 : 1)}%`;
}
