import type { SnapshotRecord, TweakStatus } from "@/types/engine";
import { isBlocked } from "./engine-text";

export interface QuickstartFacts {
  /** A batch of tweaks is in effect. */
  applied: boolean;
  /** Recommended tweaks still to apply. `null` until they're known. */
  recommendedLeft: number | null;
  /** The user added a game, chose a list or turned Game Mode on. */
  gameModeSet: boolean;
  /** Steps with nothing to read from the PC, ticked once they've been opened. */
  opened: string[];
}

export interface QuickstartStep {
  id: string;
  title: string;
  text: string;
  to: string;
  done: (facts: QuickstartFacts) => boolean;
}

const visited = (id: string) => (facts: QuickstartFacts) => facts.opened.includes(id);

export const QUICKSTART_STEPS: QuickstartStep[] = [
  {
    id: "tweak",
    title: "Apply a tweak",
    text: "Change one setting, with a full undo.",
    to: "/tweaks",
    done: (f) => f.applied,
  },
  {
    id: "recommended",
    title: "Apply the recommended tweaks",
    text: "Low-risk changes for this PC.",
    to: "/tweaks",
    done: (f) => f.recommendedLeft === 0,
  },
  {
    id: "game-mode",
    title: "Set up Game Mode",
    text: "Add a game or choose what it turns on.",
    to: "/gaming",
    done: (f) => f.gameModeSet,
  },
  {
    id: "startup",
    title: "Check your startup apps",
    text: "See what starts with Windows.",
    to: "/startup",
    done: visited("startup"),
  },
  {
    id: "cleaner",
    title: "Scan for junk files",
    text: "See what temporary files are using.",
    to: "/cleaner",
    done: visited("cleaner"),
  },
  {
    id: "backups",
    title: "Look through your backups",
    text: "Undo any change from here.",
    to: "/restore",
    done: visited("backups"),
  },
  {
    id: "monitoring",
    title: "Watch your PC live",
    text: "CPU, GPU, memory and network.",
    to: "/monitoring",
    done: visited("monitoring"),
  },
];

/** Recommended tweaks that can still be applied here. */
export const openRecommended = (statuses: TweakStatus[]) =>
  statuses.filter(
    (t) => t.meta.recommended && t.state !== "applied" && !isBlocked(t.compatibility),
  );

/** Batches whose changes are still in place, so each one is a backup that can be restored. */
export const savedBackups = (snapshots: SnapshotRecord[]) =>
  snapshots.filter((s) => s.kind === "apply" && s.status === "completed");
