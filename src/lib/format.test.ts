import { describe, expect, it } from "vitest";
import {
  formatAgo,
  formatBytes,
  formatFrequency,
  formatPercent,
  formatRate,
  formatUptime,
  percentOf,
} from "./format";

describe("formatBytes", () => {
  it("uses binary units", () => {
    expect(formatBytes(0)).toBe("0 B");
    expect(formatBytes(1536)).toBe("1.5 KB");
    expect(formatBytes(34_135_121_920)).toBe("31.8 GB");
    expect(formatBytes(254_275_481_600)).toBe("237 GB");
  });

  it("handles missing and invalid values", () => {
    expect(formatBytes(null)).toBe("—");
    expect(formatBytes(-1)).toBe("—");
    expect(formatBytes(Number.NaN)).toBe("—");
  });
});

describe("other formatters", () => {
  it("formats rates and percents", () => {
    expect(formatRate(2048)).toBe("2.0 KB/s");
    expect(formatRate(null)).toBe("—");
    expect(formatPercent(42.6)).toBe("43%");
    expect(formatPercent(null)).toBe("—");
  });

  it("formats uptime compactly", () => {
    expect(formatUptime(59)).toBe("0m");
    expect(formatUptime(3 * 3600 + 5 * 60)).toBe("3h 5m");
    expect(formatUptime(2 * 86_400 + 7200)).toBe("2d 2h");
  });

  it("formats frequency", () => {
    expect(formatFrequency(3500)).toBe("3.50 GHz");
    expect(formatFrequency(800)).toBe("800 MHz");
    expect(formatFrequency(0)).toBe("—");
  });

  it("says how long ago, in the largest whole unit", () => {
    const now = 1_000_000_000;
    expect(formatAgo(now - 30_000, now)).toBe("just now");
    expect(formatAgo(now + 5_000, now)).toBe("just now");
    expect(formatAgo(now - 60_000, now)).toBe("1 minute ago");
    expect(formatAgo(now - 59 * 60_000, now)).toBe("59 minutes ago");
    expect(formatAgo(now - 3 * 3_600_000, now)).toBe("3 hours ago");
    expect(formatAgo(now - 49 * 3_600_000, now)).toBe("2 days ago");
  });

  it("clamps percentOf", () => {
    expect(percentOf(5, 0)).toBe(0);
    expect(percentOf(150, 100)).toBe(100);
    expect(percentOf(25, 100)).toBe(25);
  });
});
