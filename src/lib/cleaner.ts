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
  /** Locations that ran to the end. */
  cleared: number;
  /** Locations whose administrator prompt was declined: nothing ran there. */
  declined: string[];
  /** Locations that failed, with the reason each gave. */
  failures: { id: string; reason: string }[];
}

export function summariseCleanup(results: CleanupResult[]): CleanupSummary {
  return {
    freed: results.reduce((n, r) => n + r.freedBytes, 0),
    skipped: results.reduce((n, r) => n + r.skippedFiles, 0),
    cleared: results.filter((r) => r.status === "cleared").length,
    declined: results.filter((r) => r.status === "elevationDeclined").map((r) => r.id),
    failures: results
      .filter((r) => r.status === "failed")
      .map((r) => ({ id: r.id, reason: r.error ?? "Couldn't be cleared" })),
  };
}

/** Only a run that cleared something counts as a run: a declined prompt changed nothing. */
export const didClean = (summary: CleanupSummary): boolean => summary.cleared > 0;
