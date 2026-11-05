import { isBlocked, type Tone } from "@/lib/engine-text";
import type { TweakMeta, TweakStatus } from "@/types/engine";

/**
 * A named set of tweaks. Like a gaming profile it adds no behaviour of its own: picking one selects
 * those tweaks for the normal review, and what's in effect is read from detected state.
 */
export interface TweakPreset {
  id: string;
  name: string;
  summary: string;
  includes: (meta: TweakMeta) => boolean;
}

// A preset changes many settings in one go, so it holds only what's unlikely to surprise anyone:
// low risk, nothing it can break, and confirmed to exist in Windows or the driver. Visuals are
// taste, not performance, so no preset or profile built on this ever includes one.
const safe = (meta: TweakMeta) =>
  meta.category !== "visuals" &&
  meta.risk === "low" &&
  meta.warning === null &&
  meta.evidence !== "community";

/**
 * Each list is chosen by hand, like the service presets, so a new tweak can't walk into one. `safe`
 * stays as the backstop: raise a tweak's risk or give it a warning and it drops out of its list.
 */
export const PRESET_TWEAKS = {
  general: [
    "memory.svchost-split",
    "performance.kernel-in-ram",
    "storage.long-paths",
    "storage.registry-backup",
    "storage.trim",
    "windows.autoplay-nonvolume",
    "windows.notif-startup",
    "windows.remote-assistance",
    "windows.separate-process",
    "windows.startup-delay",
  ],
  gaming: [
    "gaming.audio-ducking",
    "gaming.filter-keys",
    "gaming.game-bar-controller",
    "gaming.game-mode-auto",
    "gaming.system-responsiveness",
    "gaming.toggle-keys",
    "windows.narrator-key",
    "windows.sticky-keys-shortcut-off",
  ],
  privacy: [
    "privacy.ad-id-policy",
    "privacy.advertising-id-off",
    "privacy.ait",
    "privacy.app-launch-tracking-off",
    "privacy.autologger-diagtrack",
    "privacy.cdm-tips",
    "privacy.ceip",
    "privacy.click-to-do",
    "privacy.clipboard-cloud",
    "privacy.cloud-search",
    "privacy.copilot-policy",
    "privacy.error-report-policy",
    "privacy.feedback-requests-off",
    "privacy.inking-typing-collection-off",
    "privacy.inventory",
    "privacy.language-list-opt-out",
    "privacy.lock-camera",
    "privacy.mobility",
    "privacy.mrt-report",
    "privacy.online-tips",
    "privacy.onesettings",
    "privacy.push-to-install",
    "privacy.recall",
    "privacy.search-history",
    "privacy.search-location",
    "privacy.spotlight-welcome",
    "privacy.tailored-policy",
    "privacy.tipc",
    "privacy.web-search",
    "windows.sync-ads",
  ],
  hardware: ["gpu.nv-cpl-optin", "gpu.nv-tray", "tasks.amd-uep"],
  apps: [
    "apps.brave-background",
    "apps.chrome-background",
    "apps.chrome-metrics",
    "apps.chrome-promos",
    "apps.chrome-sb-reporting",
    "apps.chrome-url-data",
    "apps.edge-experiments",
    "apps.edge-nav-errors",
    "apps.edge-personalization",
    "apps.edge-promotional",
    "apps.edge-rewards",
    "apps.edge-telemetry",
    "apps.firefox-default-agent",
    "apps.firefox-studies",
    "apps.firefox-telemetry",
    "apps.office-telemetry",
  ],
} as const;

const listed = (preset: keyof typeof PRESET_TWEAKS) => {
  const ids = new Set<string>(PRESET_TWEAKS[preset]);
  return (meta: TweakMeta) => ids.has(meta.id) && safe(meta);
};

export const TWEAK_PRESETS: TweakPreset[] = [
  {
    id: "general",
    name: "General",
    summary: "Windows, memory and storage",
    includes: listed("general"),
  },
  {
    id: "gaming",
    name: "Gaming",
    summary: "Game Mode, scheduling and stray hotkeys",
    includes: listed("gaming"),
  },
  {
    id: "privacy",
    name: "Privacy",
    summary: "Telemetry, tracking, suggestions and Copilot",
    includes: listed("privacy"),
  },
  {
    id: "hardware",
    name: "This hardware",
    summary: "Vendor data collection and tray apps",
    includes: listed("hardware"),
  },
  {
    id: "apps",
    name: "Apps",
    summary: "Browser and app telemetry and promotions",
    includes: listed("apps"),
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
