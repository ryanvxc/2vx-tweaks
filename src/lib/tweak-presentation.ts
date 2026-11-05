import {
  AppWindow,
  AudioLines,
  BatteryCharging,
  Ban,
  BellOff,
  Binary,
  Bot,
  Bug,
  CalendarClock,
  Captions,
  ChartNoAxesColumn,
  CirclePlay,
  Clapperboard,
  CloudDownload,
  Cpu,
  Eye,
  EyeOff,
  FileClock,
  FileStack,
  FileText,
  FileType,
  Fingerprint,
  FolderOpen,
  FolderSync,
  Gamepad2,
  Gauge,
  Globe,
  Gpu,
  HardDrive,
  History,
  Keyboard,
  KeyboardOff,
  KeyRound,
  Languages,
  Layers,
  Layers2,
  LayoutDashboard,
  LayoutGrid,
  Lock,
  MapPin,
  MapPinOff,
  Megaphone,
  MemoryStick,
  MessageSquareOff,
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
  Palette,
  PanelBottom,
  PanelLeft,
  PanelRight,
  PanelsTopLeft,
  PenLine,
  Plug,
  Power,
  Printer,
  Puzzle,
  Radar,
  Radio,
  RefreshCcw,
  Rocket,
  Route,
  Rows3,
  Save,
  ScanSearch,
  Search,
  Server,
  Share2,
  ShoppingBag,
  SquareMenu,
  Stethoscope,
  TimerReset,
  Trophy,
  Usb,
  UserRound,
  VideoOff,
  WandSparkles,
  Wifi,
  EthernetPort,
  Wind,
  Zap,
  type LucideIcon,
} from "lucide-react";
import { VISUAL_ICON } from "./visual-icons";
import type { CleanerTab, HardwareTab, TweakTab, VisualsTab } from "@/app/navigation";
import type { TweakCategory, TweakMeta } from "@/types/engine";

// How tweaks look and where they sit in the UI. Kept apart from the engine metadata: icons
// and grouping are presentation, the Rust catalogue stays about behaviour. Engine categories
// say what a tweak touches; tabs are what someone goes looking for.

/** Every tab that lists tweaks, across the pages that have them. */
export type PlacedTab =
  | Exclude<TweakTab, "presets">
  | HardwareTab
  | VisualsTab
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
  visuals: "Visuals",
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
  visuals: Eye,
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
  "services.compat-assistant-off": Puzzle,
  "services.diagtrack-off": Radio,
  "services.geolocation-off": MapPinOff,
  "services.print-spooler-off": Printer,
  "services.sync-host-off": RefreshCcw,
  "windows.autoplay-off": CirclePlay,
  "windows.device-metadata-off": PackageSearch,
  "windows.edge-recommendations-off": Megaphone,
  "windows.edge-sidebar-off": PanelRight,
  "windows.keyboard-repeat-fast": Keyboard,
  "windows.mouse-acceleration-off": Mouse,
  "windows.sticky-keys-shortcut-off": KeyboardOff,
  "windows.window-animations-off": Wind,
};

/**
 * Each section of the longer lists has one icon, used on its heading and its rows. Not one per
 * tweak: 1,200 icons would stop meaning anything.
 */
export const SECTION_ICON: Record<string, LucideIcon> = {
  // Windows
  Explorer: FolderOpen,
  Notifications: BellOff,
  System: MonitorCog,
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
  // Scheduled tasks and third-party services
  "Third-party": Plug,
  "AMD software": Plug,
  Network: Network,
  // Power
  "Sleep and buttons": Power,
  "Display and devices": MonitorDot,
  Disk: HardDrive,
  "USB and PCIe": Usb,
  // Hardware
  NVIDIA: Gpu,
  "Windows graphics": MonitorUp,
  "Scheduler and timers": TimerReset,
  "Power plan": Zap,
  "Paging and cache": MemoryStick,
  "File system": FileType,
  // Network
  TCP: Share2,
  SMB: FolderSync,
  "Winsock and resolution": Binary,
  "DNS cache": ScanSearch,
  "Ethernet adapter": EthernetPort,
  "Wi-Fi adapter": Wifi,
  // Services
  "Sync and account": UserRound,
  "Search and shell": Search,
  "Remote access": MonitorSmartphone,
  // Visuals ("Startup and shutdown" is Windows' above)
  "Theme and colour": Palette,
  "Effects and animation": WandSparkles,
  "Windows and desktop": PanelsTopLeft,
  Taskbar: PanelBottom,
  "Start menu": LayoutGrid,
  "Search and widgets": LayoutDashboard,
  View: Rows3,
  "Navigation pane": PanelLeft,
  "Context menu": SquareMenu,
  "Open and save": Save,
  "Lock and sign-in": Lock,
};

/** A hand-picked icon, then its section's, then its category's. */
export function tweakIcon(
  meta: Pick<TweakMeta, "id" | "category"> & Partial<Pick<TweakMeta, "section">>,
): LucideIcon {
  return (
    TWEAK_ICON[meta.id] ??
    VISUAL_ICON[meta.id] ??
    (meta.section ? SECTION_ICON[meta.section] : undefined) ??
    CATEGORY_ICON[meta.category] ??
    Globe
  );
}

export function sectionIcon(title: string, category: TweakCategory): LucideIcon {
  return SECTION_ICON[title] ?? CATEGORY_ICON[category] ?? Globe;
}

const visualSections = (...titles: string[]) => titles.map((title) => ({ title, ids: [] }));

/** Sections per tab, in display order. */
export const TWEAK_SECTIONS: Record<PlacedTab, { title: string; ids: string[] }[]> = {
  // Visuals are all library tweaks: the sections are listed with no ids only to fix their order,
  // which is the order people meet them in, not the alphabet.
  appearance: [
    { title: "Theme and colour", ids: [] },
    { title: "Effects and animation", ids: ["windows.window-animations-off"] },
    { title: "Windows and desktop", ids: [] },
  ],
  taskbar: visualSections("Taskbar", "Start menu", "Search and widgets"),
  explorer: visualSections("View", "Navigation pane", "Context menu", "Open and save"),
  signin: visualSections("Lock and sign-in", "Startup and shutdown"),
  core: [{ title: "System", ids: [] }],
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
  power: [],
  services: [
    {
      title: "Background services",
      ids: [
        "services.diagtrack-off",
        "services.geolocation-off",
        "services.sync-host-off",
        "services.compat-assistant-off",
        "services.print-spooler-off",
        "services.wsearch",
        "services.cdp",
        "services.nahimic",
      ],
    },
  ],
  gpu: [{ title: "GPU", ids: ["gaming.mpo-off"] }],
  cpu: [{ title: "CPU", ids: ["gaming.system-responsiveness", "power.power-throttling-off"] }],
  ram: [{ title: "Paging and cache", ids: ["performance.kernel-in-ram"] }],
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
  storage: [
    { title: "Storage", ids: ["performance.short-names-off", "performance.last-access-off"] },
  ],
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
      ids: ["windows.autoplay-off", "windows.device-metadata-off"],
    },
  ],
};

/** The page each tab is on. */
const TAB_ROUTE: Record<PlacedTab, string> = {
  core: "/tweaks",
  privacy: "/tweaks",
  gaming: "/tweaks",
  apps: "/tweaks",
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
  appearance: "/visuals",
  taskbar: "/visuals",
  explorer: "/visuals",
  signin: "/visuals",
};

/**
 * Where Windows keeps a Visuals section's setting. Visuals go both ways, so a card that's already
 * set says where to change it, since 2VX has nothing recorded to put back.
 */
export const VISUALS_SETTINGS_PAGE: Record<string, string> = {
  "Theme and colour": "Settings > Personalisation > Colours",
  "Effects and animation": "Settings > Accessibility > Visual effects",
  "Windows and desktop": "Settings > System > Multitasking",
  Taskbar: "Settings > Personalisation > Taskbar",
  "Start menu": "Settings > Personalisation > Start",
  "Search and widgets": "Settings > Personalisation > Taskbar",
  View: "File Explorer's View menu",
  "Navigation pane": "File Explorer's View menu",
  "Open and save": "File Explorer's Options",
  "Lock and sign-in": "Settings > Personalisation > Lock screen",
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
  // Only retired tweaks are left in this category: the rest moved to Privacy and Hardware.
  tasks: "privacy",
  cpu: "cpu",
  gpu: "gpu",
  memory: "ram",
  storage: "storage",
  apps: "apps",
  // Placed by section instead (see VISUALS_SECTION_TAB); this is only where a bad one lands.
  visuals: "appearance",
};

/** The tab each Visuals section is on. A visuals tweak with any other section is a bug. */
export const VISUALS_SECTION_TAB: Record<string, PlacedTab> = Object.fromEntries(
  (["appearance", "taskbar", "explorer", "signin"] as const).flatMap((tab) =>
    TWEAK_SECTIONS[tab].map((s) => [s.title, tab] as const),
  ),
);

/** Driver settings have their own tab, beside the adapters they apply to. */
const ADAPTER_SECTIONS = new Set(["Ethernet adapter", "Wi-Fi adapter"]);

/** What placing a tweak needs to know. Fixtures for the older pages leave `section` out. */
type Placeable = Pick<TweakMeta, "id" | "category"> &
  Partial<Pick<TweakMeta, "section" | "name" | "evidence" | "recommended">>;

/** Where a tweak lives: its one primary location. One not placed yet still shows, under its topic. */
export function tweakPlacement(meta: Placeable) {
  // A tweak moved to Visuals follows its section, even if it was once hand-placed elsewhere.
  const placed = meta.category === "visuals" ? undefined : PLACEMENT.get(meta.id);
  const tab =
    placed?.tab ??
    (meta.section && ADAPTER_SECTIONS.has(meta.section)
      ? "adapters"
      : meta.category === "visuals" && meta.section
        ? (VISUALS_SECTION_TAB[meta.section] ?? CATEGORY_TAB.visuals)
        : CATEGORY_TAB[meta.category]);
  return {
    tab: tab as string,
    section: placed?.section ?? meta.section ?? MORE_SECTION,
    route: TAB_ROUTE[tab],
    /** Came from the library, rather than being hand-placed. */
    listed: !placed && !!meta.section,
  };
}

export interface TweakGroup<T> {
  /** `null` for a flat list, such as search results across every tab. */
  title: string | null;
  tweaks: T[];
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
  "windows.edge-recommendations-off": "Stop feature suggestions",
  "windows.edge-sidebar-off": "Hide the sidebar",
};

export function cardTitle(meta: Pick<TweakMeta, "id" | "name">) {
  return CARD_TITLE[meta.id] ?? meta.name;
}

/** Too specific to round out a row: a priority setting for one game is nobody's essential. */
const NEVER_FILLS = new Set(["Per-game priority"]);

/** How well a library tweak suits a hand-placed section called `title`: higher is better. */
function cardFit(meta: Placeable, title: string) {
  const family = tweakPlacement(meta).section.split(" ")[0] === title.split(" ")[0];
  return (
    Number(family) * 8 +
    Number(meta.evidence !== "community") * 4 +
    Number((meta.name?.length ?? 0) <= CARD_NAME_MAX) * 2 +
    Number(meta.recommended === true)
  );
}

/** The library's sections come from the tweaks themselves, in name order. */
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
 * Splits tweaks into the sections of `tab`, or one flat list when `tab` is null: the hand-placed
 * sections first, then the library's. They keep the order they arrive in, unless `catalogue` asks
 * for each hand-placed section's written order, and tops it up to a full row of cards from the
 * library, because what's hand-placed can be Unavailable on this PC. A library section named like a
 * hand-placed one joins it. Whatever can't run on this PC goes in a section of its own, always last.
 */
export function groupTweaks<T extends { meta: Placeable }>(
  tweaks: T[],
  {
    tab,
    catalogue,
    unavailable,
  }: {
    tab: PlacedTab | null;
    catalogue: boolean;
    unavailable: (tweak: T) => boolean;
  },
): TweakGroup<T>[] {
  const usable = tweaks.filter((t) => !unavailable(t));
  const blocked = tweaks.filter(unavailable);

  const placed = (tab: PlacedTab): TweakGroup<T>[] => {
    const sections = [...TWEAK_SECTIONS[tab], { title: MORE_SECTION, ids: [] as string[] }];
    const hand = usable.filter((t) => !tweakPlacement(t.meta).listed);
    const titles = new Set(sections.map((s) => s.title));
    const inSection = (group: T[], title: string) =>
      group.filter((t) => tweakPlacement(t.meta).section === title);
    const library = usable.filter((t) => tweakPlacement(t.meta).listed);
    let rest = library.filter((t) => !titles.has(tweakPlacement(t.meta).section));
    const written = sections.map(({ title, ids }) => {
      const own = inSection(hand, title);
      const tweaks = [
        ...(catalogue
          ? own.toSorted((a, b) => ids.indexOf(a.meta.id) - ids.indexOf(b.meta.id))
          : own),
        ...inSection(library, title),
      ];
      const short = (CARDS_PER_ROW - (tweaks.length % CARDS_PER_ROW)) % CARDS_PER_ROW;
      if (!catalogue || own.length === 0 || short === 0) return { title, tweaks };
      const extra = rest
        .filter((t) => !NEVER_FILLS.has(tweakPlacement(t.meta).section))
        .toSorted((a, b) => cardFit(b.meta, title) - cardFit(a.meta, title))
        .slice(0, short);
      rest = rest.filter((t) => !extra.includes(t));
      return { title, tweaks: [...tweaks, ...extra] };
    });
    return [...written, ...bySection(rest)];
  };

  const groups = tab ? placed(tab) : [{ title: null, tweaks: usable }];
  return [...groups, { title: UNAVAILABLE_SECTION, tweaks: blocked }].filter(
    (group) => group.tweaks.length > 0,
  );
}

/** Keeps the first `limit` tweaks across the groups, so a long tab stays quick to draw. */
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
