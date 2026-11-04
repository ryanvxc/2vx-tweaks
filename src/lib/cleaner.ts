import type { CleanupLocation, CleanupResult } from "@/types/system";

/**
 * What "Recommended" leaves unticked: the Recycle Bin holds files someone may still want, and
 * the rest cost something afterwards (slower first launches, longer first game loads, new
 * package downloads, an emptied Recent list).
 */
const NOT_RECOMMENDED = new Set([
  "amd-shader-cache",
  "directx-shader-cache",
  "nvidia-shader-cache",
  "package-caches",
  "prefetch",
  "recent-files",
  "recycle-bin",
]);

/** Locations worth listing: one this PC doesn't have has nothing to clear. */
export const cleanableLocations = (locations: CleanupLocation[]): CleanupLocation[] =>
  locations.filter((l) => l.status !== "missing").sort((a, b) => a.label.localeCompare(b.label));

export const recommendedLocations = (locations: CleanupLocation[]): string[] =>
  locations.filter((l) => !NOT_RECOMMENDED.has(l.id)).map((l) => l.id);

export interface CleanupSummary {
  freed: number;
  /** Files something still had open. */
  skipped: number;
  /** Why locations weren't cleared at all, one reason each. */
  failures: string[];
}

export function summariseCleanup(results: CleanupResult[]): CleanupSummary {
  return {
    freed: results.reduce((n, r) => n + r.freedBytes, 0),
    skipped: results.reduce((n, r) => n + r.skippedFiles, 0),
    failures: [...new Set(results.flatMap((r) => (r.error ? [r.error] : [])))],
  };
}
