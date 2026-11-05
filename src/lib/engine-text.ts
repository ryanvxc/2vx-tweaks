import type {
  ChangeStatus,
  Compatibility,
  Evidence,
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
  wrongHardware: "Made for other hardware",
  notInstalled: "Not installed on this PC",
};

export const EVIDENCE: Record<Evidence, { label: string; note: string }> = {
  documented: { label: "Documented", note: "Microsoft or the vendor documents this setting." },
  observed: {
    label: "Found in Windows",
    note: "Windows or the driver reads this value, but it isn't documented.",
  },
  community: {
    label: "Community",
    note: "From community tweak packs. 2VX couldn't confirm Windows reads it.",
  },
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
      return "Sign out to finish.";
    case "reboot":
      return "Restart Windows to finish.";
    case "none":
      return null;
  }
}

/** For a tweak that is not applied yet: when its change takes effect. */
export const RESTART_HINT: Record<RestartRequirement, string | null> = {
  none: null,
  signOut: "Needs sign-out",
  reboot: "Needs a restart",
};

/** Snapshots that still have changes on the system and can be undone. */
export function canRevert(kind: SnapshotKind, status: SnapshotStatus): boolean {
  return kind === "apply" && ["completed", "interrupted", "failed"].includes(status);
}

export function describeOutcome(outcome: OperationOutcome, kind: SnapshotKind): Message {
  const n = outcome.changes.length;
  const values = `${n} value${n === 1 ? "" : "s"}`;
  const restart = restartNote(outcome.restart);

  if (outcome.snapshotId == null) {
    return {
      tone: "neutral",
      title: "Nothing to change",
      description: "Already set.",
    };
  }
  if (kind === "revert") {
    return outcome.status === "completed"
      ? {
          tone: "success",
          title: "Reverted",
          description: restart ?? "",
        }
      : {
          tone: "warning",
          title: "Partly reverted",
          description: "Some values changed since and were left as is.",
        };
  }
  switch (outcome.status) {
    case "completed":
      return {
        tone: "success",
        title: "Applied",
        description: restart ? `${values} changed. ${restart}` : `${values} changed.`,
      };
    case "rolledBack":
      return {
        tone: "warning",
        title: "Not applied",
        description: `Everything was put back. ${outcome.error ?? ""}`.trim(),
      };
    default:
      return {
        tone: "destructive",
        title: "Failed, and the rollback was incomplete",
        description: `${outcome.error ?? "A change failed."} Undo it in the Restore Centre.`,
      };
  }
}

export function describeError(error: BackendError): Message {
  switch (error.code) {
    case "conflict":
      return {
        tone: "warning",
        title: "Changed since you reviewed this",
        description: `Nothing was applied. ${error.message}`,
      };
    case "elevation_required":
      return {
        tone: "warning",
        title: "Admin permission wasn't granted",
        description: `Nothing was changed. ${error.message}`,
      };
    case "storage":
      return {
        tone: "destructive",
        title: "Change journal unavailable",
        description: `Nothing was changed. ${error.message}`,
      };
    case "preview":
      return {
        tone: "info",
        title: "Not available in browser preview",
        description: "Open the desktop app to apply changes.",
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
