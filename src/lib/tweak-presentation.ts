import {
  AppWindow,
  Ban,
  BellOff,
  BellRing,
  BluetoothOff,
  ChartNoAxesColumn,
  CloudOff,
  Eye,
  EyeOff,
  FileClock,
  Fingerprint,
  Gamepad2,
  Gauge,
  Globe,
  HardDriveDownload,
  History,
  KeyRound,
  Languages,
  Layers,
  Map as MapIcon,
  MapPin,
  Megaphone,
  MessageSquareOff,
  MessagesSquare,
  MicOff,
  MonitorCog,
  Network,
  PanelRight,
  PenLine,
  Power,
  Printer,
  Puzzle,
  RefreshCcw,
  Rocket,
  RotateCw,
  Server,
  ShoppingBag,
  Stethoscope,
  SunDim,
  UserRound,
  VideoOff,
  Wind,
  type LucideIcon,
} from "lucide-react";
import type { TweakTab } from "@/app/navigation";
import type { TweakCategory, TweakMeta } from "@/types/engine";

// How tweaks look and where they sit in the UI. Kept apart from the engine metadata: icons
// and grouping are presentation, the Rust catalogue stays about behaviour. Engine categories
// say what a tweak touches; tabs are what someone goes looking for.

export const CATEGORY_LABEL: Record<TweakCategory, string> = {
  gaming: "Gaming",
  performance: "Performance",
  power: "Power",
  network: "Network",
  privacy: "Privacy",
  services: "Services",
  windows: "Windows",
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

export const TWEAK_ICON: Record<string, LucideIcon> = {
  "gaming.game-dvr-off": VideoOff,
  "gaming.mmcss-games-priority": Gamepad2,
  "performance.edge-background-mode-off": Ban,
  "performance.edge-startup-boost-off": Rocket,
  "performance.foreground-boost": AppWindow,
  "performance.last-access-off": FileClock,
  "privacy.activity-history-off": History,
  "privacy.advertising-id-off": Fingerprint,
  "privacy.app-account-info-off": UserRound,
  "privacy.app-diagnostics-off": Stethoscope,
  "privacy.app-launch-tracking-off": Eye,
  "privacy.app-location-off": MapPin,
  "privacy.app-notification-access-off": BellOff,
  "privacy.background-apps-off": Layers,
  "privacy.diagnostic-data-minimum": ChartNoAxesColumn,
  "privacy.edge-shopping-off": ShoppingBag,
  "privacy.feedback-requests-off": MessageSquareOff,
  "privacy.inking-typing-collection-off": PenLine,
  "privacy.language-list-opt-out": Languages,
  "privacy.online-speech-off": MicOff,
  "services.bluetooth-off": BluetoothOff,
  "services.compat-assistant-off": Puzzle,
  "services.maps-broker-off": MapIcon,
  "services.print-notify-off": BellRing,
  "services.print-spooler-off": Printer,
  "services.sensor-monitoring-off": SunDim,
  "services.sensor-off": RotateCw,
  "services.sync-host-off": RefreshCcw,
  "services.wap-push-off": MessagesSquare,
  "services.xbox-auth-off": KeyRound,
  "services.xbox-save-off": CloudOff,
  "windows.edge-recommendations-off": Megaphone,
  "windows.edge-sidebar-off": PanelRight,
  "windows.storage-sense-off": HardDriveDownload,
  "windows.window-animations-off": Wind,
};

export function tweakIcon(meta: Pick<TweakMeta, "id" | "category">): LucideIcon {
  return TWEAK_ICON[meta.id] ?? CATEGORY_ICON[meta.category] ?? Globe;
}

/** Sections per tab, in display order. Gaming tweaks live on the Gaming page instead. */
export const TWEAK_SECTIONS: Record<TweakTab, { title: string; ids: string[] }[]> = {
  windows: [
    {
      title: "Responsiveness",
      ids: ["performance.foreground-boost", "windows.window-animations-off"],
    },
    { title: "Storage", ids: ["performance.last-access-off", "windows.storage-sense-off"] },
    {
      title: "Microsoft Edge",
      ids: [
        "performance.edge-background-mode-off",
        "performance.edge-startup-boost-off",
        "privacy.edge-shopping-off",
        "windows.edge-recommendations-off",
        "windows.edge-sidebar-off",
      ],
    },
  ],
  privacy: [
    {
      title: "Windows data",
      ids: [
        "privacy.diagnostic-data-minimum",
        "privacy.activity-history-off",
        "privacy.feedback-requests-off",
        "privacy.advertising-id-off",
        "privacy.app-launch-tracking-off",
        "privacy.language-list-opt-out",
        "privacy.online-speech-off",
        "privacy.inking-typing-collection-off",
      ],
    },
    {
      title: "App permissions",
      ids: [
        "privacy.app-location-off",
        "privacy.app-account-info-off",
        "privacy.app-diagnostics-off",
        "privacy.app-notification-access-off",
        "privacy.background-apps-off",
      ],
    },
  ],
  services: [
    { title: "Printing", ids: ["services.print-spooler-off", "services.print-notify-off"] },
    {
      title: "Bluetooth and sensors",
      ids: ["services.bluetooth-off", "services.sensor-off", "services.sensor-monitoring-off"],
    },
    { title: "Xbox", ids: ["services.xbox-auth-off", "services.xbox-save-off"] },
    {
      title: "Other",
      ids: [
        "services.maps-broker-off",
        "services.wap-push-off",
        "services.compat-assistant-off",
        "services.sync-host-off",
      ],
    },
  ],
};

export const MORE_SECTION = "More";

const PLACEMENT = new Map(
  (Object.keys(TWEAK_SECTIONS) as TweakTab[]).flatMap((tab) =>
    TWEAK_SECTIONS[tab].flatMap((s) => s.ids.map((id) => [id, { tab, section: s.title }] as const)),
  ),
);

const FALLBACK_TAB: Record<TweakCategory, TweakTab> = {
  gaming: "windows",
  performance: "windows",
  power: "windows",
  network: "windows",
  windows: "windows",
  privacy: "privacy",
  services: "services",
};

/** Where a tweak lives. One not placed yet still shows, in its category's tab. */
export function tweakPlacement(meta: Pick<TweakMeta, "id" | "category">) {
  return PLACEMENT.get(meta.id) ?? { tab: FALLBACK_TAB[meta.category], section: MORE_SECTION };
}

/** A tweak's primary location, with its details open. */
export function tweakHref(meta: Pick<TweakMeta, "id" | "category">) {
  const id = encodeURIComponent(meta.id);
  return meta.category === "gaming"
    ? `/gaming?tweak=${id}`
    : `/tweaks?tab=${tweakPlacement(meta).tab}&tweak=${id}`;
}

/** "New" for tweaks first shipped in the running app's `major.minor`. */
export function isNewTweak(meta: Pick<TweakMeta, "addedIn">, appVersion: string | undefined) {
  if (!appVersion) return false;
  const [major, minor] = appVersion.split(".");
  return meta.addedIn === `${major}.${minor}`;
}
