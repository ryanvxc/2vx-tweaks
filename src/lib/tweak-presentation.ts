import {
  AppWindow,
  Archive,
  AudioLines,
  BatteryCharging,
  Ban,
  BellOff,
  BellRing,
  Binary,
  Bluetooth,
  BluetoothOff,
  Bot,
  Bug,
  Building2,
  Cable,
  CalendarClock,
  Captions,
  ChartNoAxesColumn,
  CirclePlay,
  Clapperboard,
  CloudDownload,
  CloudOff,
  Cpu,
  CreditCard,
  Dices,
  Eye,
  EyeOff,
  FileClock,
  FileStack,
  FileText,
  FileType,
  Fingerprint,
  FlaskConical,
  FolderOpen,
  FolderSync,
  Gamepad2,
  Gauge,
  Glasses,
  Globe,
  Gpu,
  HardDrive,
  HardDriveDownload,
  HeartPulse,
  History,
  HousePlug,
  Keyboard,
  KeyboardOff,
  KeyRound,
  Languages,
  Layers,
  Layers2,
  LayoutPanelLeft,
  Map as MapIcon,
  MapPin,
  MapPinOff,
  Megaphone,
  MemoryStick,
  MessageSquareOff,
  MessagesSquare,
  MicOff,
  MonitorCog,
  MonitorDot,
  MonitorSmartphone,
  MonitorUp,
  Mouse,
  MousePointerClick,
  Network,
  Package,
  PackageSearch,
  PanelRight,
  PenLine,
  Plug,
  Power,
  Printer,
  Puzzle,
  Radar,
  Radio,
  RefreshCcw,
  Rocket,
  RotateCw,
  Route,
  Scale,
  ScanSearch,
  Search,
  Server,
  ServerCog,
  Share2,
  ShieldHalf,
  ShoppingBag,
  Smartphone,
  Sparkles,
  Stethoscope,
  Store,
  SunDim,
  TimerReset,
  Trophy,
  Usb,
  UserRound,
  Users,
  VideoOff,
  Wifi,
  EthernetPort,
  Wind,
  Wrench,
  Zap,
  type LucideIcon,
} from "lucide-react";
import type { CleanerTab, HardwareTab, TweakTab } from "@/app/navigation";
import type { TweakCategory, TweakMeta } from "@/types/engine";

// How tweaks look and where they sit in the UI. Kept apart from the engine metadata: icons
// and grouping are presentation, the Rust catalogue stays about behaviour. Engine categories
// say what a tweak touches; tabs are what someone goes looking for.

/** Every tab that lists tweaks, across the pages that have them. */
export type PlacedTab =
  | Exclude<TweakTab, "presets">
  | HardwareTab
  | Extract<CleanerTab, "services">
  | "tweaks"
  | "priority"
  | "adapters"
  | "security"
  | "devices";

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

/**
 * Each section of the longer lists has one icon, used on its heading and its rows. Not one per
 * tweak: 1,200 icons would stop meaning anything.
 */
export const SECTION_ICON: Record<string, LucideIcon> = {
  // Windows
  Explorer: FolderOpen,
  "Taskbar and Start": LayoutPanelLeft,
  System: MonitorCog,
  "Visual effects": Sparkles,
  "Startup and shutdown": Power,
  // Privacy
  "App permissions": KeyRound,
  Telemetry: ChartNoAxesColumn,
  "Advertising and suggestions": Megaphone,
  "Sync and cross-device": RefreshCcw,
  Search: Search,
  "Speech and typing": Captions,
  "System reports": FileText,
  "Copilot and AI": Bot,
  // Gaming
  "Game settings": Gamepad2,
  Input: MousePointerClick,
  "Per-game priority": Trophy,
  "Game Bar and capture": Clapperboard,
  Audio: AudioLines,
  // Apps
  Edge: Globe,
  Chrome: Globe,
  Firefox: Globe,
  "Brave, Opera and Vivaldi": Globe,
  Office: FileStack,
  "Other apps": Package,
  // Tasks
  Maintenance: Wrench,
  "Third-party": Plug,
  "Input sync": Keyboard,
  "Family and account": Users,
  Network: Network,
  Diagnostics: Stethoscope,
  "Flighting and experiments": FlaskConical,
  "Maps and location": MapIcon,
  // Power
  "Sleep and buttons": Power,
  "Display and devices": MonitorDot,
  Disk: HardDrive,
  "USB and PCIe": Usb,
  // Hardware
  NVIDIA: Gpu,
  AMD: Gpu,
  Intel: Gpu,
  "Windows graphics": MonitorUp,
  "Scheduler and timers": TimerReset,
  "Power plan": Zap,
  "Paging and cache": MemoryStick,
  Prefetch: Rocket,
  Drives: HardDriveDownload,
  "File system": FileType,
  // Network
  TCP: Share2,
  SMB: FolderSync,
  "Winsock and resolution": Binary,
  "DNS cache": ScanSearch,
  "Discovery and Wi-Fi": Wifi,
  "Ethernet adapter": EthernetPort,
  "Wi-Fi adapter": Wifi,
  QoS: Scale,
  // Services
  "Store and apps": Store,
  "Sync and account": UserRound,
  "Network discovery": Radar,
  "Backup and storage": Archive,
  "Search and shell": Search,
  "Telemetry and diagnostics": HeartPulse,
  "Smart card and sign-in": CreditCard,
  "Remote access": MonitorSmartphone,
  Enterprise: Building2,
  "Hyper-V": ServerCog,
  "Phone and cellular": Smartphone,
  Bluetooth: Bluetooth,
  VPN: ShieldHalf,
  Xbox: Dices,
  "Mixed reality": Glasses,
  "Printing and scanning": Printer,
  "Location and sensors": MapPin,
  "Peer networking": HousePlug,
};

/** A hand-picked icon, then its section's, then its category's. */
export function tweakIcon(
  meta: Pick<TweakMeta, "id" | "category"> & Partial<Pick<TweakMeta, "section">>,
): LucideIcon {
  return (
    TWEAK_ICON[meta.id] ??
    (meta.section ? SECTION_ICON[meta.section] : undefined) ??
    CATEGORY_ICON[meta.category] ??
    Globe
  );
}

export function sectionIcon(title: string, category: TweakCategory): LucideIcon {
  return SECTION_ICON[title] ?? CATEGORY_ICON[category] ?? Globe;
}

/** Sections per tab, in display order. */
export const TWEAK_SECTIONS: Record<PlacedTab, { title: string; ids: string[] }[]> = {
  // One section: split by topic, each of these would be a lone card in an empty row.
  core: [
    {
      title: "Core",
      ids: [
        "performance.foreground-boost",
        "performance.last-access-off",
        "windows.storage-sense-off",
        "windows.window-animations-off",
      ],
    },
  ],
  gaming: [{ title: "Essentials", ids: ["gaming.mmcss-games-priority", "gaming.game-dvr-off"] }],
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
      title: "Edge",
      ids: [
        "performance.edge-background-mode-off",
        "performance.edge-startup-boost-off",
        "privacy.edge-shopping-off",
        "windows.edge-recommendations-off",
        "windows.edge-sidebar-off",
      ],
    },
  ],
  tasks: [],
  power: [],
  services: [
    {
      title: "Printing and scanning",
      ids: ["services.print-spooler-off", "services.print-notify-off"],
    },
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
      title: "Keyboard and mouse",
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
  adapters: [],
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
      title: "Settings",
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
  gaming: "/tweaks",
  apps: "/tweaks",
  tasks: "/tweaks",
  power: "/tweaks",
  services: "/cleaner",
  gpu: "/hardware",
  cpu: "/hardware",
  ram: "/hardware",
  peripherals: "/hardware",
  storage: "/hardware",
  tweaks: "/network",
  priority: "/network",
  adapters: "/network",
  security: "/advanced",
  devices: "/advanced",
};

export const MORE_SECTION = "More";
export const UNAVAILABLE_SECTION = "Unavailable";

const PLACEMENT = new Map(
  (Object.keys(TWEAK_SECTIONS) as PlacedTab[]).flatMap((tab) =>
    TWEAK_SECTIONS[tab].flatMap((s) => s.ids.map((id) => [id, { tab, section: s.title }] as const)),
  ),
);

/** Where a category's long list sits: every tweak lives with its topic, not on a page of its own. */
const CATEGORY_TAB: Record<TweakCategory, PlacedTab> = {
  gaming: "gaming",
  performance: "core",
  power: "power",
  network: "tweaks",
  windows: "core",
  privacy: "privacy",
  services: "services",
  tasks: "tasks",
  cpu: "cpu",
  gpu: "gpu",
  memory: "ram",
  storage: "storage",
  apps: "apps",
};

/** Driver settings have their own tab, beside the adapters they apply to. */
const ADAPTER_SECTIONS = new Set(["Ethernet adapter", "Wi-Fi adapter"]);

/** What placing a tweak needs to know. Fixtures for the older pages leave `section` out. */
type Placeable = Pick<TweakMeta, "id" | "category"> &
  Partial<Pick<TweakMeta, "section" | "name" | "evidence" | "recommended">>;

/** Where a tweak lives: its one primary location. One not placed yet still shows, under its topic. */
export function tweakPlacement(meta: Placeable) {
  const placed = PLACEMENT.get(meta.id);
  const tab =
    placed?.tab ??
    (meta.section && ADAPTER_SECTIONS.has(meta.section) ? "adapters" : CATEGORY_TAB[meta.category]);
  return {
    tab: tab as string,
    section: placed?.section ?? meta.section ?? MORE_SECTION,
    route: TAB_ROUTE[tab],
    /** Part of a long list, rather than a hand-placed card. */
    listed: !placed && !!meta.section,
  };
}

export interface TweakGroup<T> {
  /** `null` for a flat list, such as search results across every tab. */
  title: string | null;
  tweaks: T[];
  /** A long or secondary list: rows in a section that opens, rather than cards. */
  compact: boolean;
}

/** Cards sit three to a row, and a longer name is cut off on one. */
const CARDS_PER_ROW = 3;
const CARD_NAME_MAX = 33;

/** On a card the section's heading names the product, so the name doesn't have to. */
const CARD_TITLE: Record<string, string> = {
  "performance.edge-background-mode-off": "Stop running in the background",
  "performance.edge-startup-boost-off": "Turn off Startup boost",
  "privacy.edge-shopping-off": "Turn off Shopping",
  "privacy.inking-typing-collection-off": "Stop learning from your typing",
  "privacy.language-list-opt-out": "Keep your language list private",
  "privacy.online-speech-off": "Stop online speech recognition",
  "services.compat-assistant-off": "Disable Compatibility Assistant",
  "windows.edge-recommendations-off": "Stop feature suggestions",
  "windows.edge-sidebar-off": "Hide the sidebar",
};

export function cardTitle(meta: Pick<TweakMeta, "id" | "name">) {
  return CARD_TITLE[meta.id] ?? meta.name;
}

/** How well a tweak from the lists suits a card in `title`'s section: higher is better. */
function cardFit(meta: Placeable, title: string) {
  const family = tweakPlacement(meta).section.split(" ")[0] === title.split(" ")[0];
  return (
    Number(family) * 8 +
    Number(meta.evidence !== "community") * 4 +
    Number((meta.name?.length ?? 0) <= CARD_NAME_MAX) * 2 +
    Number(meta.recommended === true)
  );
}

/** Keep their capital after "More". */
const PROPER_SECTION = /^(?:AMD|Bluetooth|CPU|Edge|GPU|Intel|NVIDIA|RAM|USB|Windows|Xbox)\b/;

function moreTitle(title: string) {
  const rest = PROPER_SECTION.test(title) ? title : title.charAt(0).toLowerCase() + title.slice(1);
  return `More ${rest}`;
}

/** The long lists' sections come from the tweaks themselves, in name order. */
function bySection<T extends { meta: Placeable }>(
  tweaks: T[],
  written: Set<string>,
): TweakGroup<T>[] {
  const titles = [...new Set(tweaks.map((t) => tweakPlacement(t.meta).section))].sort((a, b) =>
    a.localeCompare(b),
  );
  return titles.map((title) => ({
    // A card section can share a name with a list on the same topic.
    title: written.has(title) ? moreTitle(title) : title,
    tweaks: tweaks.filter((t) => tweakPlacement(t.meta).section === title),
    compact: true,
  }));
}

/**
 * Splits tweaks into the sections of `tab`, or one flat list when `tab` is null: the hand-placed
 * sections first, then the long lists. They keep the order they arrive in, unless `catalogue` asks
 * for each hand-placed section's written order. `fillRows` tops each hand-placed section up to a
 * full row of cards from the lists, because what's hand-placed can be Unavailable on this PC.
 * Whatever can't run on this PC goes in a section of its own, always last.
 */
export function groupTweaks<T extends { meta: Placeable }>(
  tweaks: T[],
  {
    tab,
    catalogue,
    fillRows = false,
    unavailable,
  }: {
    tab: PlacedTab | null;
    catalogue: boolean;
    fillRows?: boolean;
    unavailable: (tweak: T) => boolean;
  },
): TweakGroup<T>[] {
  const usable = tweaks.filter((t) => !unavailable(t));
  const blocked = tweaks.filter(unavailable);

  const placed = (tab: PlacedTab): TweakGroup<T>[] => {
    const sections = [...TWEAK_SECTIONS[tab], { title: MORE_SECTION, ids: [] as string[] }];
    const hand = usable.filter((t) => !tweakPlacement(t.meta).listed);
    let lists = usable.filter((t) => tweakPlacement(t.meta).listed);
    const cards = sections.map((section) => {
      const inSection = hand.filter((t) => tweakPlacement(t.meta).section === section.title);
      const tweaks = catalogue
        ? inSection.toSorted(
            (a, b) => section.ids.indexOf(a.meta.id) - section.ids.indexOf(b.meta.id),
          )
        : inSection;
      const short = (CARDS_PER_ROW - (tweaks.length % CARDS_PER_ROW)) % CARDS_PER_ROW;
      if (!fillRows || !catalogue || tweaks.length === 0 || short === 0) {
        return { title: section.title, compact: false, tweaks };
      }
      const extra = lists
        .toSorted((a, b) => cardFit(b.meta, section.title) - cardFit(a.meta, section.title))
        .slice(0, short);
      lists = lists.filter((t) => !extra.includes(t));
      return { title: section.title, compact: false, tweaks: [...tweaks, ...extra] };
    });
    return [...cards, ...bySection(lists, new Set(sections.map((s) => s.title)))];
  };

  const groups = tab ? placed(tab) : [{ title: null, tweaks: usable, compact: true }];
  return [...groups, { title: UNAVAILABLE_SECTION, tweaks: blocked, compact: true }].filter(
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

/**
 * How well a tweak matches a lowercase search, best first, or `null` if it doesn't. `targets` are
 * what it changes, so someone who knows a value or service name can search for that.
 */
export function searchRank(
  meta: Pick<TweakMeta, "name" | "description" | "category"> & Partial<Pick<TweakMeta, "section">>,
  query: string,
  targets: string[] = [],
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
  if (targets.some((t) => t.toLowerCase().includes(query))) return 5;
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
