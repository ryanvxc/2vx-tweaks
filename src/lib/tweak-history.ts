import type { SnapshotRecord } from "@/types/engine";

/** What the journal says about one tweak beyond its detected state. */
export interface TweakHistory {
  lastUsedMs: number;
  /** The last change didn't go through. The text is the engine's, when it gave one. */
  failed: string | null;
  /** Its last change finished after this boot, so a restart is still to come. */
  sinceBoot: boolean;
}

const FAILED = new Set<SnapshotRecord["status"]>(["failed", "rolledBack", "interrupted"]);

/**
 * Reads what happened to each tweak from the snapshots, newest first. The engine keeps no per-tweak
 * flag for these, and doesn't need one: the newest snapshot that names a tweak has the answer.
 */
export function tweakHistory(
  snapshots: SnapshotRecord[],
  uptimeSecs: number | null,
  now = Date.now(),
): Map<string, TweakHistory> {
  const bootMs = uptimeSecs === null ? null : now - uptimeSecs * 1000;
  const history = new Map<string, TweakHistory>();
  for (const snapshot of snapshots) {
    for (const id of snapshot.tweakIds) {
      if (history.has(id)) continue;
      const failed = FAILED.has(snapshot.status);
      history.set(id, {
        lastUsedMs: snapshot.createdAtMs,
        failed: failed ? (snapshot.error ?? "The change was rolled back.") : null,
        sinceBoot:
          snapshot.kind === "apply" &&
          snapshot.status === "completed" &&
          bootMs !== null &&
          (snapshot.finishedAtMs ?? snapshot.createdAtMs) > bootMs,
      });
    }
  }
  return history;
}
