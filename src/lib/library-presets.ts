import { isBlocked } from "@/lib/engine-text";
import type { TweakCategory, TweakMeta, TweakStatus } from "@/types/engine";

/**
 * A rule over the Library. Like a gaming profile it adds no behaviour of its own: picking one
 * selects those tweaks for the normal review, and what's in effect is read from detected state.
 */
export interface LibraryPreset {
  id: string;
  name: string;
  summary: string;
  includes: (meta: TweakMeta) => boolean;
}

// A preset changes many settings in one go, so it holds only what's unlikely to surprise anyone:
// low risk, nothing it can break, and confirmed to exist in Windows or the driver.
const safe = (meta: TweakMeta) =>
  meta.section !== null &&
  meta.risk === "low" &&
  meta.warning === null &&
  meta.evidence !== "community";

const inCategories =
  (...categories: TweakCategory[]) =>
  (meta: TweakMeta) =>
    safe(meta) && categories.includes(meta.category);

export const LIBRARY_PRESETS: LibraryPreset[] = [
  {
    id: "general",
    name: "General",
    summary: "Windows, memory and storage",
    includes: inCategories("windows", "memory", "storage"),
  },
  {
    id: "gaming",
    name: "Gaming",
    summary: "Game settings and scheduling",
    includes: inCategories("gaming"),
  },
  {
    id: "privacy",
    name: "Privacy",
    summary: "Telemetry, tracking and suggestions",
    includes: inCategories("privacy"),
  },
  {
    id: "hardware",
    name: "This hardware",
    summary: "Settings for this PC's CPU and GPU",
    includes: (meta) => safe(meta) && meta.hardware !== "any",
  },
  {
    id: "apps",
    name: "Apps",
    summary: "Browsers and installed apps",
    includes: inCategories("apps"),
  },
];

export interface PresetProgress {
  /** The preset's tweaks this PC can use. */
  tweaks: TweakStatus[];
  /** Those not in effect yet: what picking the preset selects. */
  open: TweakStatus[];
  /** Those 2VX recorded, so it can restore them. */
  undoable: TweakStatus[];
}

export function presetProgress(preset: LibraryPreset, statuses: TweakStatus[]): PresetProgress {
  const tweaks = statuses.filter((t) => preset.includes(t.meta) && !isBlocked(t.compatibility));
  return {
    tweaks,
    open: tweaks.filter((t) => t.state !== "applied"),
    undoable: tweaks.filter((t) => t.undoable),
  };
}
