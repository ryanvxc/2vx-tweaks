import { describe, expect, it } from "vitest";
import type { CleanupLocation, CleanupResult } from "@/types/system";
import { cleanableLocations, didClean, recommendedLocations, summariseCleanup } from "./cleaner";

const location = (
  id: string,
  label: string,
  status: CleanupLocation["status"] = "measured",
): CleanupLocation => ({ id, label, description: "", path: "", bytes: 0, files: 0, status });

const result = (patch: Partial<CleanupResult>): CleanupResult => ({
  id: "x",
  freedBytes: 0,
  removedFiles: 0,
  skippedFiles: 0,
  status: "cleared",
  error: null,
  ...patch,
});

describe("cleanableLocations", () => {
  it("drops what this PC doesn't have and sorts by name", () => {
    const rows = cleanableLocations([
      location("b", "Clear B"),
      location("gone", "Clear Gone", "missing"),
      location("a", "Clear A", "denied"),
    ]);
    expect(rows.map((l) => l.id)).toEqual(["a", "b"]);
  });
});

describe("recommendedLocations", () => {
  it("never ticks the Recycle Bin or anything with a cost afterwards", () => {
    const ids = recommendedLocations([
      location("user-temp", "Temp"),
      location("recycle-bin", "Bin"),
      location("prefetch", "Prefetch"),
      location("nvidia-shader-cache", "Shaders"),
    ]);
    expect(ids).toEqual(["user-temp"]);
  });
});

describe("summariseCleanup", () => {
  it("adds up what was freed and keeps each failure with its location", () => {
    const summary = summariseCleanup([
      result({ id: "a", freedBytes: 100, skippedFiles: 2 }),
      result({ id: "b", freedBytes: 50 }),
      result({ id: "c", status: "failed", error: "access denied" }),
      result({ id: "d", status: "failed", error: null }),
    ]);
    expect(summary).toEqual({
      freed: 150,
      skipped: 2,
      cleared: 2,
      declined: [],
      failures: [
        { id: "c", reason: "access denied" },
        { id: "d", reason: "Couldn't be cleared" },
      ],
    });
    expect(didClean(summary)).toBe(true);
  });

  it("doesn't count a declined prompt as a success", () => {
    const summary = summariseCleanup([
      result({ id: "a", status: "elevationDeclined" }),
      result({ id: "b", status: "elevationDeclined" }),
    ]);
    expect(summary.cleared).toBe(0);
    expect(summary.declined).toEqual(["a", "b"]);
    expect(summary.failures).toEqual([]);
    expect(didClean(summary)).toBe(false);
  });

  it("reports both sides of a partial run", () => {
    const summary = summariseCleanup([
      result({ id: "a", freedBytes: 10 }),
      result({ id: "b", status: "elevationDeclined" }),
    ]);
    expect(summary).toMatchObject({ freed: 10, cleared: 1, declined: ["b"] });
    expect(didClean(summary)).toBe(true);
  });
});
