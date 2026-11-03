import { describe, expect, it } from "vitest";
import {
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

  it("clamps percentOf", () => {
    expect(percentOf(5, 0)).toBe(0);
    expect(percentOf(150, 100)).toBe(100);
    expect(percentOf(25, 100)).toBe(25);
  });
});
