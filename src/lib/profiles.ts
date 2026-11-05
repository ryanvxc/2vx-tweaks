import { isBlocked } from "@/lib/engine-text";
import type { TweakStatus } from "@/types/engine";

/**
 * A named list of existing tweaks. It adds no system behaviour of its own: picking one opens
 * the normal review of those tweaks, and whether it's active is read from their detected
 * state, never stored. Nothing here may promise a frame-rate gain.
 */
export interface Profile {
  id: string;
  name: string;
  summary: string;
  tweakIds: string[];
}

const RESPONSIVE = ["gaming.mmcss-games-priority", "gaming.system-responsiveness"];
const QUIET = [
  ...RESPONSIVE,
  "performance.edge-background-mode-off",
  "performance.edge-startup-boost-off",
  "privacy.background-apps-off",
];
const EVERYTHING = [...QUIET, "gaming.game-dvr-off"];

/** Everything Game Mode can turn on, in Configure's order. */
export const GAME_MODE_TWEAKS = EVERYTHING;

/**
 * What Game Mode turns on until the user chooses. Only tweaks that need no restart: one that
 * does can't help a game that's already running.
 */
export const DEFAULT_GAME_MODE_TWEAKS = [
  "performance.edge-background-mode-off",
  "performance.edge-startup-boost-off",
  "privacy.background-apps-off",
];

export const GAMING_PROFILES: Profile[] = [
  {
    id: "responsive",
    name: "Responsive",
    summary: "Favour multimedia threads",
    tweakIds: RESPONSIVE,
  },
  {
    id: "quiet",
    name: "Quiet background",
    summary: "Less running behind the game",
    tweakIds: QUIET,
  },
  {
    id: "everything",
    name: "Everything",
    summary: "Game recording off too",
    tweakIds: EVERYTHING,
  },
];

export interface ProfileProgress {
  /** The profile's tweaks in this catalogue, in profile order. */
  tweaks: TweakStatus[];
  /** Those this PC can use. */
  usable: TweakStatus[];
  inEffect: number;
  active: boolean;
}

export function profileProgress(profile: Profile, statuses: TweakStatus[]): ProfileProgress {
  const tweaks = profile.tweakIds.flatMap((id) => statuses.filter((t) => t.meta.id === id));
  const usable = tweaks.filter((t) => !isBlocked(t.compatibility));
  const inEffect = usable.filter((t) => t.state === "applied").length;
  const complete = tweaks.length === profile.tweakIds.length;
  const active = complete && usable.length > 0 && inEffect === usable.length;
  return { tweaks, usable, inEffect, active };
}

/**
 * The active profile with the most tweaks in effect. On a tie the smaller one wins: a larger
 * profile whose extra tweaks are all blocked on this PC hasn't done anything more.
 */
export function activeProfile(statuses: TweakStatus[]): Profile | null {
  let best: Profile | null = null;
  let most = 0;
  for (const profile of GAMING_PROFILES) {
    const { active, inEffect } = profileProgress(profile, statuses);
    if (active && inEffect > most) {
      best = profile;
      most = inEffect;
    }
  }
  return best;
}
