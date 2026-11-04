import { describe, expect, it } from "vitest";
import type { CleanupLocation, CleanupResult } from "@/types/system";
import { cleanableLocations, recommendedLocations, summariseCleanup } from "./cleaner";

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
  it("adds up what was freed and keeps each reason once", () => {
    const denied = "the administrator prompt was declined";
    expect(
      summariseCleanup([
        result({ freedBytes: 100, skippedFiles: 2 }),
        result({ freedBytes: 50 }),
        result({ error: denied }),
        result({ error: denied }),
      ]),
    ).toEqual({ freed: 150, skipped: 2, failures: [denied] });
  });
});
