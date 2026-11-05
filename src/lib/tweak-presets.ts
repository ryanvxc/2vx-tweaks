import { isBlocked, type Tone } from "@/lib/engine-text";
import type { TweakCategory, TweakMeta, TweakStatus } from "@/types/engine";

/**
 * A rule over the long lists. Like a gaming profile it adds no behaviour of its own: picking one
 * selects those tweaks for the normal review, and what's in effect is read from detected state.
 */
export interface TweakPreset {
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

export const TWEAK_PRESETS: TweakPreset[] = [
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

/**
 * Hardware's two profile cards. EXM's are NVIDIA Profile Inspector settings; these are the same kind
 * of rule as a preset, so each only ever selects tweaks that already exist, for the normal review.
 */
/** One word for a named list's state, wherever it shows. */
export function presetStatus(inEffect: number, total: number): { tone: Tone; label: string } {
  if (total === 0) return { tone: "neutral", label: "Nothing for this PC" };
  if (inEffect === total) return { tone: "success", label: "All on" };
  return { tone: "neutral", label: inEffect > 0 ? "Partly on" : "Off" };
}

export const GPU_PROFILES: TweakPreset[] = [
  {
    id: "gpu-nvidia",
    name: "2VX NVIDIA Profile",
    summary: "NVIDIA's tray, capture camera, feedback and display power saving",
    includes: (meta) => safe(meta) && meta.category === "gpu" && meta.hardware === "nvidiaGpu",
  },
  {
    id: "gpu-windows",
    name: "2VX Graphics Profile",
    summary: "Longer GPU timeouts and fewer desktop composition extras",
    includes: (meta) => safe(meta) && meta.category === "gpu" && meta.hardware === "any",
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

export function presetProgress(preset: TweakPreset, statuses: TweakStatus[]): PresetProgress {
  const tweaks = statuses.filter((t) => preset.includes(t.meta) && !isBlocked(t.compatibility));
  return {
    tweaks,
    // An unread setting can't be backed up, so a preset leaves it for now.
    open: tweaks.filter((t) => t.state !== "applied" && t.state !== "unknown"),
    undoable: tweaks.filter((t) => t.undoable),
  };
}
