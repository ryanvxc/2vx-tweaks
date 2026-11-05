import {
  AppWindow,
  BatteryCharging,
  Ban,
  BellOff,
  BellRing,
  BluetoothOff,
  Bug,
  Cable,
  CalendarClock,
  ChartNoAxesColumn,
  CirclePlay,
  CloudDownload,
  CloudOff,
  Cpu,
  Eye,
  EyeOff,
  FileClock,
  FileType,
  Fingerprint,
  FolderSync,
  Gamepad2,
  Gauge,
  Globe,
  Gpu,
  HardDrive,
  HardDriveDownload,
  History,
  Keyboard,
  KeyboardOff,
  KeyRound,
  Languages,
  Layers,
  Layers2,
  Map as MapIcon,
  MapPin,
  MapPinOff,
  Megaphone,
  MemoryStick,
  MessageSquareOff,
  MessagesSquare,
  MicOff,
  MonitorCog,
  MonitorSmartphone,
  Mouse,
  Network,
  PackageSearch,
  PanelRight,
  PenLine,
  Power,
  Printer,
  Puzzle,
  Radar,
  Radio,
  RefreshCcw,
  Rocket,
  RotateCw,
  Route,
  Server,
  ShoppingBag,
  Stethoscope,
  SunDim,
  Usb,
  UserRound,
  VideoOff,
  Wind,
  type LucideIcon,
} from "lucide-react";
import type { HardwareTab, TweakTab } from "@/app/navigation";
import type { TweakCategory, TweakMeta } from "@/types/engine";

// How tweaks look and where they sit in the UI. Kept apart from the engine metadata: icons
// and grouping are presentation, the Rust catalogue stays about behaviour. Engine categories
// say what a tweak touches; tabs are what someone goes looking for.

/** Every tab that lists tweaks, across the pages that have them. */
export type PlacedTab = TweakTab | HardwareTab | "tweaks" | "priority" | "security" | "devices";

export const CATEGORY_LABEL: Record<TweakCategory, string> = {
  gaming: "Gaming",
  performance: "Performance",
  power: "Power",
  network: "Network",
  privacy: "Privacy",
  services: "Services",
  tasks: "Tasks",
  windows: "Windows",
  cpu: "CPU",
  gpu: "GPU",
  memory: "Memory",
  storage: "Storage",
  apps: "Apps",
};

const CATEGORY_ICON: Record<TweakCategory, LucideIcon> = {
  gaming: Gamepad2,
  performance: Gauge,
  power: Power,
  network: Network,
  privacy: EyeOff,
  services: Server,
  tasks: CalendarClock,
  windows: MonitorCog,
  cpu: Cpu,
  gpu: Gpu,
  memory: MemoryStick,
  storage: HardDrive,
  apps: AppWindow,
};

export const TWEAK_ICON: Record<string, LucideIcon> = {
  "gaming.game-dvr-off": VideoOff,
  "gaming.mmcss-games-priority": Gamepad2,
  "gaming.mpo-off": Layers2,
  "gaming.system-responsiveness": Gauge,
  "network.delivery-optimization-http": CloudDownload,
  "network.icmp-redirect-off": Route,
  "network.llmnr-off": Radar,
  "network.prefer-ipv4": Network,
  "network.smb-throttling-off": FolderSync,
  "performance.edge-background-mode-off": Ban,
  "performance.edge-startup-boost-off": Rocket,
  "performance.fault-tolerant-heap-off": Bug,
  "performance.foreground-boost": AppWindow,
  "performance.kernel-in-ram": MemoryStick,
  "performance.last-access-off": FileClock,
  "performance.short-names-off": FileType,
  "power.power-throttling-off": BatteryCharging,
  "power.usb-selective-suspend-off": Usb,
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
  "services.diagtrack-off": Radio,
  "services.geolocation-off": MapPinOff,
  "services.maps-broker-off": MapIcon,
  "services.print-notify-off": BellRing,
  "services.print-spooler-off": Printer,
  "services.sensor-monitoring-off": SunDim,
  "services.sensor-off": RotateCw,
  "services.sync-host-off": RefreshCcw,
  "services.wap-push-off": MessagesSquare,
  "services.xbox-auth-off": KeyRound,
  "services.xbox-save-off": CloudOff,
  "windows.autoplay-off": CirclePlay,
  "windows.device-metadata-off": PackageSearch,
  "windows.edge-recommendations-off": Megaphone,
  "windows.edge-sidebar-off": PanelRight,
  "windows.keyboard-repeat-fast": Keyboard,
  "windows.mouse-acceleration-off": Mouse,
  "windows.rdp-com-redirect-off": Cable,
  "windows.rdp-pnp-redirect-off": MonitorSmartphone,
  "windows.sticky-keys-shortcut-off": KeyboardOff,
  "windows.storage-sense-off": HardDriveDownload,
  "windows.window-animations-off": Wind,
};

export function tweakIcon(meta: Pick<TweakMeta, "id" | "category">): LucideIcon {
  return TWEAK_ICON[meta.id] ?? CATEGORY_ICON[meta.category] ?? Globe;
}

/** Sections per tab, in display order. */
export const TWEAK_SECTIONS: Record<PlacedTab, { title: string; ids: string[] }[]> = {
  // One section: split by topic, each of these would be a lone card in an empty row.
  core: [
    {
      title: "Core",
      ids: [
        "performance.foreground-boost",
        "gaming.mmcss-games-priority",
        "performance.last-access-off",
        "gaming.game-dvr-off",
        "windows.storage-sense-off",
        "windows.window-animations-off",
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
  apps: [
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
        "services.diagtrack-off",
        "services.geolocation-off",
        "services.maps-broker-off",
        "services.wap-push-off",
        "services.compat-assistant-off",
        "services.sync-host-off",
      ],
    },
  ],
  gpu: [{ title: "GPU", ids: ["gaming.mpo-off"] }],
  cpu: [{ title: "CPU", ids: ["gaming.system-responsiveness", "power.power-throttling-off"] }],
  ram: [{ title: "RAM", ids: ["performance.kernel-in-ram"] }],
  peripherals: [
    {
      title: "Keyboard & Mouse",
      ids: [
        "windows.mouse-acceleration-off",
        "windows.keyboard-repeat-fast",
        "windows.sticky-keys-shortcut-off",
      ],
    },
    { title: "USB", ids: ["power.usb-selective-suspend-off"] },
  ],
  storage: [{ title: "Storage", ids: ["performance.short-names-off"] }],
  tweaks: [
    {
      title: "Network",
      ids: [
        "network.delivery-optimization-http",
        "network.prefer-ipv4",
        "network.smb-throttling-off",
      ],
    },
  ],
  priority: [
    {
      title: "Games",
      ids: [
        "network.qos-apex",
        "network.qos-cs2",
        "network.qos-fortnite",
        "network.qos-league",
        "network.qos-overwatch",
        "network.qos-rainbow-six",
        "network.qos-rocket-league",
        "network.qos-valorant",
      ],
    },
  ],
  security: [
    {
      title: "Settings",
      ids: [
        "network.llmnr-off",
        "network.icmp-redirect-off",
        "performance.fault-tolerant-heap-off",
      ],
    },
  ],
  devices: [
    {
      title: "Devices",
      ids: [
        "windows.autoplay-off",
        "windows.device-metadata-off",
        "windows.rdp-com-redirect-off",
        "windows.rdp-pnp-redirect-off",
      ],
    },
  ],
};

/** The page each tab is on. */
const TAB_ROUTE: Record<PlacedTab, string> = {
  core: "/tweaks",
  privacy: "/tweaks",
  apps: "/tweaks",
  services: "/tweaks",
  gpu: "/hardware",
  cpu: "/hardware",
  ram: "/hardware",
  peripherals: "/hardware",
  storage: "/hardware",
  tweaks: "/network",
  priority: "/network",
  security: "/advanced",
  devices: "/advanced",
};

export const LIBRARY_ROUTE = "/library";
export const MORE_SECTION = "More";
export const UNAVAILABLE_SECTION = "Unavailable";

const PLACEMENT = new Map(
  (Object.keys(TWEAK_SECTIONS) as PlacedTab[]).flatMap((tab) =>
    TWEAK_SECTIONS[tab].flatMap((s) => s.ids.map((id) => [id, { tab, section: s.title }] as const)),
  ),
);

const FALLBACK_TAB: Record<TweakCategory, PlacedTab> = {
  gaming: "core",
  performance: "core",
  power: "cpu",
  network: "tweaks",
  windows: "core",
  privacy: "privacy",
  services: "services",
  tasks: "services",
  cpu: "cpu",
  gpu: "gpu",
  memory: "ram",
  storage: "storage",
  apps: "apps",
};

/** What placing a tweak needs to know. Fixtures for the older pages leave `section` out. */
type Placeable = Pick<TweakMeta, "id" | "category"> & Partial<Pick<TweakMeta, "section">>;

/**
 * Where a tweak lives: its one primary location. One with a section is in the Library, under its
 * category. One not placed yet still shows, in its category's tab.
 */
export function tweakPlacement(meta: Placeable) {
  if (meta.section) {
    return { tab: meta.category as string, section: meta.section, route: LIBRARY_ROUTE };
  }
  const placed = PLACEMENT.get(meta.id);
  const tab = placed?.tab ?? FALLBACK_TAB[meta.category];
  return { tab: tab as string, section: placed?.section ?? MORE_SECTION, route: TAB_ROUTE[tab] };
}

export interface TweakGroup<T> {
  /** `null` for a flat list, such as search results across every tab. */
  title: string | null;
  tweaks: T[];
}

/** The Library's sections come from the tweaks themselves, in name order. */
function bySection<T extends { meta: Placeable }>(tweaks: T[]): TweakGroup<T>[] {
  const titles = [...new Set(tweaks.map((t) => tweakPlacement(t.meta).section))].sort((a, b) =>
    a.localeCompare(b),
  );
  return titles.map((title) => ({
    title,
    tweaks: tweaks.filter((t) => tweakPlacement(t.meta).section === title),
  }));
}

/**
 * Splits tweaks into the sections of `tab`, or one flat list when `tab` is null. They keep the
 * order they arrive in, unless `catalogue` asks for each section's written order. With `library`
 * the sections are the tweaks' own. Whatever can't run on this PC goes in a section of its own,
 * always last.
 */
export function groupTweaks<T extends { meta: Placeable }>(
  tweaks: T[],
  {
    tab,
    catalogue,
    library = false,
    unavailable,
  }: {
    tab: PlacedTab | null;
    catalogue: boolean;
    library?: boolean;
    unavailable: (tweak: T) => boolean;
  },
): TweakGroup<T>[] {
  const usable = tweaks.filter((t) => !unavailable(t));
  const blocked = tweaks.filter(unavailable);

  const placed = (tab: PlacedTab): TweakGroup<T>[] =>
    [...TWEAK_SECTIONS[tab], { title: MORE_SECTION, ids: [] as string[] }].map((section) => {
      const inSection = usable.filter((t) => tweakPlacement(t.meta).section === section.title);
      return {
        title: section.title,
        tweaks: catalogue
          ? inSection.toSorted(
              (a, b) => section.ids.indexOf(a.meta.id) - section.ids.indexOf(b.meta.id),
            )
          : inSection,
      };
    });

  const groups = !tab
    ? [{ title: null, tweaks: usable }]
    : library
      ? bySection(usable)
      : placed(tab);

  return [...groups, { title: UNAVAILABLE_SECTION, tweaks: blocked }].filter(
    (group) => group.tweaks.length > 0,
  );
}

/** Keeps the first `limit` tweaks across the groups, so a broad search stays quick to draw. */
export function capGroups<T>(groups: TweakGroup<T>[], limit: number): TweakGroup<T>[] {
  let room = limit;
  return groups.flatMap((group) => {
    if (room <= 0) return [];
    const tweaks = group.tweaks.slice(0, room);
    room -= tweaks.length;
    return [{ ...group, tweaks }];
  });
}

/** How well a tweak matches a lowercase search, best first, or `null` if it doesn't. */
export function searchRank(
  meta: Pick<TweakMeta, "name" | "description" | "category"> & Partial<Pick<TweakMeta, "section">>,
  query: string,
): number | null {
  const name = meta.name.toLowerCase();
  if (name.startsWith(query)) return 0;
  if (name.includes(` ${query}`)) return 1;
  if (name.includes(query)) return 2;
  if (meta.description.toLowerCase().includes(query)) return 3;
  const section = meta.section?.toLowerCase() ?? "";
  if (section.includes(query) || CATEGORY_LABEL[meta.category].toLowerCase().includes(query)) {
    return 4;
  }
  return null;
}

/** A tweak's primary location, with its details open. */
export function tweakHref(meta: Placeable) {
  const { route, tab } = tweakPlacement(meta);
  return `${route}?tab=${tab}&tweak=${encodeURIComponent(meta.id)}`;
}

/** "New" for tweaks first shipped in the running app's `major.minor`. */
export function isNewTweak(meta: Pick<TweakMeta, "addedIn">, appVersion: string | undefined) {
  if (!appVersion) return false;
  const [major, minor] = appVersion.split(".");
  return meta.addedIn === `${major}.${minor}`;
}
