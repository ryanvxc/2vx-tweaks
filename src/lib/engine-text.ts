import type {
  ChangeStatus,
  Compatibility,
  OperationOutcome,
  RestartRequirement,
  SnapshotKind,
  SnapshotStatus,
  TweakState,
} from "@/types/engine";
import type { BackendError } from "@/types/system";

// User-facing wording for engine states and outcomes, kept in one place so every screen
// describes the same result the same way.

export type Tone = "success" | "warning" | "destructive" | "info" | "neutral";

export interface Message {
  tone: Tone;
  title: string;
  description: string;
}

export const TWEAK_STATE: Record<TweakState, { label: string; tone: Tone }> = {
  applied: { label: "Applied", tone: "success" },
  notApplied: { label: "Not applied", tone: "neutral" },
  partial: { label: "Partly applied", tone: "warning" },
  unknown: { label: "Unknown", tone: "warning" },
};

export const COMPATIBILITY: Record<Compatibility, string | null> = {
  compatible: null,
  needsElevation: "Asks for administrator permission",
  unsupportedBuild: "Not supported on this Windows build",
  unsupportedEdition: "Needs Windows Pro or higher",
  wrongDevice: "Not meant for this kind of PC",
  notInstalled: "Not installed on this PC",
};

/** Mirrors `Compatibility::blocks` in the engine. */
export function isBlocked(compatibility: Compatibility) {
  return compatibility !== "compatible" && compatibility !== "needsElevation";
}

export const SNAPSHOT_STATUS: Record<SnapshotStatus, { label: string; tone: Tone }> = {
  inProgress: { label: "In progress", tone: "info" },
  completed: { label: "Completed", tone: "success" },
  rolledBack: { label: "Rolled back", tone: "warning" },
  failed: { label: "Failed", tone: "destructive" },
  interrupted: { label: "Interrupted", tone: "warning" },
  reverted: { label: "Reverted", tone: "neutral" },
};

export const CHANGE_STATUS: Record<ChangeStatus, { label: string; tone: Tone }> = {
  intent: { label: "Not confirmed", tone: "warning" },
  verified: { label: "Verified", tone: "success" },
  rolledBack: { label: "Rolled back", tone: "neutral" },
  reverted: { label: "Reverted", tone: "neutral" },
  failed: { label: "Failed", tone: "destructive" },
  conflict: { label: "Left as is", tone: "warning" },
  unchanged: { label: "Already restored", tone: "neutral" },
};

export function restartNote(restart: RestartRequirement): string | null {
  switch (restart) {
    case "signOut":
      return "Sign out and back in to finish.";
    case "reboot":
      return "Restart Windows to finish.";
    case "none":
      return null;
  }
}

/** For a tweak that is not applied yet: when its change takes effect. */
export const RESTART_HINT: Record<RestartRequirement, string | null> = {
  none: null,
  signOut: "Takes effect after you sign out and back in",
  reboot: "Takes effect after a restart",
};

/** Snapshots that still have changes on the system and can be undone. */
export function canRevert(kind: SnapshotKind, status: SnapshotStatus): boolean {
  return kind === "apply" && ["completed", "interrupted", "failed"].includes(status);
}

export function describeOutcome(outcome: OperationOutcome, kind: SnapshotKind): Message {
  const n = outcome.changes.length;
  const values = `${n} value${n === 1 ? "" : "s"}`;
  const restart = restartNote(outcome.restart);
  const withRestart = (s: string) => (restart ? `${s} ${restart}` : s);

  if (outcome.snapshotId == null) {
    return {
      tone: "neutral",
      title: "Nothing to change",
      description: "Everything was already in the requested state.",
    };
  }
  if (kind === "revert") {
    return outcome.status === "completed"
      ? {
          tone: "success",
          title: "Reverted",
          description: withRestart("The previous values are back and were read back to confirm."),
        }
      : {
          tone: "warning",
          title: "Partly reverted",
          description:
            "Some values were changed by something else since 2VX set them, or could not be restored, and were left as is. The list shows which.",
        };
  }
  switch (outcome.status) {
    case "completed":
      return {
        tone: "success",
        title: "Applied and verified",
        description: withRestart(
          `${values} changed and read back. You can undo this from History at any time.`,
        ),
      };
    case "rolledBack":
      return {
        tone: "warning",
        title: "Not applied: rolled back",
        description:
          `A change failed, so every value was put back as it was. ${outcome.error ?? ""}`.trim(),
      };
    default:
      return {
        tone: "destructive",
        title: "Failed, and the rollback was incomplete",
        description: `${outcome.error ?? "A change failed."} Some values could not be restored automatically; revert this batch from History.`,
      };
  }
}

export function describeError(error: BackendError): Message {
  switch (error.code) {
    case "conflict":
      return {
        tone: "warning",
        title: "Something changed since you reviewed this",
        description: `Nothing was applied. ${error.message}`,
      };
    case "elevation_required":
      return {
        tone: "warning",
        title: "Administrator permission was not granted",
        description: `Nothing was changed. ${error.message}`,
      };
    case "storage":
      return {
        tone: "destructive",
        title: "Change journal unavailable",
        description: `2VX can't record a backup, so it changed nothing. ${error.message}`,
      };
    case "preview":
      return {
        tone: "info",
        title: "Not available in browser preview",
        description: "Open the desktop app to review and apply real changes.",
      };
    case "policy":
      return {
        tone: "destructive",
        title: "Refused by the safety policy",
        description: error.message,
      };
    default:
      return { tone: "destructive", title: "Something went wrong", description: error.message };
  }
}

const dateTime = new Intl.DateTimeFormat(undefined, { dateStyle: "medium", timeStyle: "short" });

export function formatTimestamp(ms: number): string {
  return dateTime.format(new Date(ms));
}
