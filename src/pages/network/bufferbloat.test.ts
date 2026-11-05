import { describe, expect, it } from "vitest";
import {
  analyse,
  BASELINE_SECONDS,
  formatAdded,
  formatLoss,
  formatMs,
  phaseAt,
  TEST_SECONDS,
  type Reading,
  type Tick,
} from "./bufferbloat";

/** A full run: idle readings, then loaded ones, at the given traffic levels. */
function run(opts: {
  idle: Reading;
  loaded: Reading | Reading[];
  idleBytes?: number;
  loadBytes?: number;
  gateway?: boolean;
}): Tick[] {
  const { idle, loaded, idleBytes = 10_000, loadBytes = 5_000_000, gateway = true } = opts;
  return Array.from({ length: TEST_SECONDS }, (_, second) => {
    const baseline = second < BASELINE_SECONDS;
    const reading = baseline
      ? idle
      : Array.isArray(loaded)
        ? loaded[(second - BASELINE_SECONDS) % loaded.length]
        : loaded;
    return {
      second,
      gateway: gateway ? reading : undefined,
      internet: reading,
      bytesPerSec: baseline ? idleBytes : loadBytes,
    };
  });
}

describe("phaseAt", () => {
  it("splits the run into baseline, load and done", () => {
    expect(phaseAt(0)).toBe("baseline");
    expect(phaseAt(BASELINE_SECONDS - 1)).toBe("baseline");
    expect(phaseAt(BASELINE_SECONDS)).toBe("load");
    expect(phaseAt(TEST_SECONDS - 1)).toBe("load");
    expect(phaseAt(TEST_SECONDS)).toBe("done");
  });
});

describe("analyse", () => {
  it("reports the idle and loaded averages and the added latency", () => {
    const r = analyse(run({ idle: 10, loaded: 50 }));
    expect(r.internet).toEqual({
      idleMs: 10,
      loadedMs: 50,
      addedMs: 40,
      idleLossPercent: 0,
      loadedLossPercent: 0,
    });
    expect(r.loadReached).toBe(true);
    expect(r.peakBytesPerSec).toBe(5_000_000);
  });

  it("averages only the replies and counts lost pings as loss", () => {
    const r = analyse(run({ idle: 10, loaded: [20, 40, null] }));
    expect(r.internet?.loadedMs).toBe(30);
    // 1 of every 3 lost
    expect(r.internet?.loadedLossPercent).toBeCloseTo(33.33, 1);
    expect(r.internet?.idleLossPercent).toBe(0);
  });

  it("does not count targets that weren't asked as loss", () => {
    const r = analyse(run({ idle: 10, loaded: 30, gateway: false }));
    expect(r.gateway).toBeNull();
    expect(r.internet?.addedMs).toBe(20);
  });

  it("gives no added latency when a phase had no replies", () => {
    const r = analyse(run({ idle: null, loaded: 30 }));
    expect(r.internet?.idleMs).toBeNull();
    expect(r.internet?.addedMs).toBeNull();
    expect(r.internet?.idleLossPercent).toBe(100);
  });

  it("keeps a negative difference as measured", () => {
    expect(analyse(run({ idle: 12, loaded: 10 })).internet?.addedMs).toBe(-2);
  });

  it("says the load did not reach the connection when traffic stayed low", () => {
    const r = analyse(run({ idle: 10, loaded: 50, loadBytes: 20_000 }));
    expect(r.loadReached).toBe(false);
    expect(r.loadedSeconds).toBe(0);
  });

  it("needs the traffic to last, not a single spike", () => {
    const ticks = run({ idle: 10, loaded: 50, loadBytes: 20_000 });
    ticks[BASELINE_SECONDS + 3]!.bytesPerSec = 9_000_000;
    expect(analyse(ticks).loadReached).toBe(false);
  });

  it("measures load above what the PC was already doing", () => {
    const r = analyse(run({ idle: 10, loaded: 50, idleBytes: 4_000_000, loadBytes: 4_100_000 }));
    expect(r.loadReached).toBe(false);
  });

  it("copes with no samples", () => {
    const r = analyse([]);
    expect(r.loadReached).toBe(false);
    expect(r.peakBytesPerSec).toBeNull();
    expect(r.gateway).toBeNull();
    expect(r.internet).toBeNull();
  });
});

describe("formatting", () => {
  it("formats times, signed differences and loss", () => {
    expect(formatMs(12.34)).toBe("12.3 ms");
    expect(formatMs(150.6)).toBe("151 ms");
    expect(formatMs(null)).toBe("—");
    expect(formatAdded(40)).toBe("+40.0 ms");
    expect(formatAdded(-2)).toBe("-2.0 ms");
    expect(formatLoss(0)).toBe("0%");
    expect(formatLoss(2.5)).toBe("2.5%");
  });
});
