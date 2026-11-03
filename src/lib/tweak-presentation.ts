import {
  Ban,
  Eye,
  EyeOff,
  Fingerprint,
  Gamepad2,
  Gauge,
  Globe,
  Languages,
  Megaphone,
  MonitorCog,
  Network,
  PanelRight,
  Power,
  Rocket,
  Server,
  ShoppingBag,
  type LucideIcon,
} from "lucide-react";
import type { TweakCategory, TweakMeta } from "@/types/engine";

// How tweaks look in the UI. Kept apart from the engine metadata: icons and tab grouping
// are presentation, the Rust catalogue stays about behaviour.

export const CATEGORY_LABEL: Record<TweakCategory, string> = {
  gaming: "Gaming",
  performance: "Performance",
  power: "Power",
  network: "Network",
  privacy: "Privacy",
  services: "Services",
  windows: "Windows & apps",
};

const CATEGORY_ICON: Record<TweakCategory, LucideIcon> = {
  gaming: Gamepad2,
  performance: Gauge,
  power: Power,
  network: Network,
  privacy: EyeOff,
  services: Server,
  windows: MonitorCog,
};

const TWEAK_ICON: Record<string, LucideIcon> = {
  "performance.edge-startup-boost-off": Rocket,
  "performance.edge-background-mode-off": Ban,
  "privacy.app-launch-tracking-off": Eye,
  "privacy.language-list-opt-out": Languages,
  "privacy.edge-shopping-off": ShoppingBag,
  "privacy.advertising-id-off": Fingerprint,
  "windows.edge-sidebar-off": PanelRight,
  "windows.edge-recommendations-off": Megaphone,
};

export function tweakIcon(meta: Pick<TweakMeta, "id" | "category">): LucideIcon {
  return TWEAK_ICON[meta.id] ?? CATEGORY_ICON[meta.category] ?? Globe;
}

/** "New" for tweaks first shipped in the running app's `major.minor`. */
export function isNewTweak(meta: Pick<TweakMeta, "addedIn">, appVersion: string | undefined) {
  if (!appVersion) return false;
  const [major, minor] = appVersion.split(".");
  return meta.addedIn === `${major}.${minor}`;
}
