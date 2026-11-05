import {
  Activity,
  AppWindow,
  ArchiveRestore,
  Bolt,
  Brush,
  CalendarClock,
  Cpu,
  EthernetPort,
  Gamepad2,
  Gauge,
  Gpu,
  HardDrive,
  HatGlasses,
  House,
  Keyboard,
  Layers,
  Library,
  ListOrdered,
  MemoryStick,
  MessageSquareCode,
  Monitor,
  Network,
  Package,
  PackageX,
  Rocket,
  Router,
  Server,
  Settings,
  Shield,
  SlidersHorizontal,
  Timer,
  Trash2,
  Waves,
  Zap,
  type LucideIcon,
} from "lucide-react";

export interface NavTab<T extends string = string> {
  value: T;
  label: string;
  icon: LucideIcon;
}

export interface NavItem {
  to: string;
  label: string;
  icon: LucideIcon;
  /** Extra words for the command palette. Never shown. */
  description: string;
  /** Sections of the page, chosen with `?tab=`. */
  tabs?: NavTab[];
  /** Drawn in the brand colour, to stand out from the list. */
  highlight?: boolean;
}

export interface NavGroup {
  label: string;
  items: NavItem[];
}

export type MonitoringTab = "cpu" | "memory" | "gpu" | "network";
export type TweakTab = "core" | "privacy" | "apps" | "services";
export type CleanerTab = "system" | "services" | "uninstall" | "autoruns";
export type HardwareTab = "gpu" | "cpu" | "ram" | "peripherals" | "storage";
export type NetworkTab = "tweaks" | "priority" | "adapters" | "bufferbloat";
export type AdvancedTab = "security" | "msi" | "devices" | "timer";
export type LibraryTab =
  | "presets"
  | "windows"
  | "privacy"
  | "services"
  | "tasks"
  | "gaming"
  | "cpu"
  | "gpu"
  | "memory"
  | "network"
  | "apps"
  | "storage"
  | "power";

export const MONITORING_TABS: NavTab<MonitoringTab>[] = [
  { value: "cpu", label: "CPU", icon: Cpu },
  { value: "memory", label: "Memory", icon: MemoryStick },
  { value: "gpu", label: "GPU", icon: Gpu },
  { value: "network", label: "Network", icon: Network },
];

export const TWEAK_TABS: NavTab<TweakTab>[] = [
  { value: "core", label: "Core", icon: Bolt },
  { value: "privacy", label: "Privacy", icon: HatGlasses },
  { value: "apps", label: "Apps", icon: AppWindow },
  { value: "services", label: "Services", icon: Server },
];

export const CLEANER_TABS: NavTab<CleanerTab>[] = [
  { value: "system", label: "System Cleaner", icon: Brush },
  { value: "services", label: "Services", icon: Server },
  { value: "uninstall", label: "Uninstall", icon: PackageX },
  { value: "autoruns", label: "Autoruns", icon: Rocket },
];

export const HARDWARE_TABS: NavTab<HardwareTab>[] = [
  { value: "gpu", label: "GPU", icon: Gpu },
  { value: "cpu", label: "CPU", icon: Cpu },
  { value: "ram", label: "RAM", icon: MemoryStick },
  { value: "peripherals", label: "Peripherals", icon: Keyboard },
  { value: "storage", label: "Storage", icon: HardDrive },
];

export const NETWORK_TABS: NavTab<NetworkTab>[] = [
  { value: "tweaks", label: "Tweaks", icon: SlidersHorizontal },
  { value: "priority", label: "Network Priority", icon: ListOrdered },
  { value: "adapters", label: "Adapter Tuner", icon: EthernetPort },
  { value: "bufferbloat", label: "Bufferbloat", icon: Waves },
];

export const ADVANCED_TABS: NavTab<AdvancedTab>[] = [
  { value: "security", label: "Security", icon: Shield },
  { value: "msi", label: "MSI Mode", icon: MessageSquareCode },
  { value: "devices", label: "Devices", icon: Monitor },
  { value: "timer", label: "Timer Resolution", icon: Timer },
];

/** Past `presets` each value is a tweak category, which is how a tweak finds its tab. */
export const LIBRARY_TABS: NavTab<LibraryTab>[] = [
  { value: "presets", label: "Presets", icon: Layers },
  { value: "windows", label: "Windows", icon: Monitor },
  { value: "privacy", label: "Privacy", icon: HatGlasses },
  { value: "services", label: "Services", icon: Server },
  { value: "tasks", label: "Tasks", icon: CalendarClock },
  { value: "gaming", label: "Gaming", icon: Gamepad2 },
  { value: "cpu", label: "CPU", icon: Cpu },
  { value: "gpu", label: "GPU", icon: Gpu },
  { value: "memory", label: "Memory", icon: MemoryStick },
  { value: "network", label: "Network", icon: Network },
  { value: "apps", label: "Apps", icon: AppWindow },
  { value: "storage", label: "Storage", icon: HardDrive },
  { value: "power", label: "Power", icon: Zap },
];

export const NAV_GROUPS: NavGroup[] = [
  {
    label: "General",
    items: [
      { to: "/", label: "Home", icon: House, description: "Your PC at a glance" },
      {
        to: "/monitoring",
        label: "Monitoring",
        icon: Activity,
        description: "Live usage over the last five minutes",
        tabs: MONITORING_TABS,
      },
      {
        to: "/restore",
        label: "Restore Centre",
        icon: ArchiveRestore,
        description: "Every change 2VX made, with undo",
      },
      {
        to: "/tools",
        label: "Resources",
        icon: Package,
        description: "System report and Windows tools",
      },
    ],
  },
  {
    label: "Tools",
    items: [
      {
        to: "/gaming",
        label: "2VX Game Mode",
        icon: Gamepad2,
        description: "Turn it on while a game is running",
        highlight: true,
      },
    ],
  },
  {
    label: "Optimisations",
    items: [
      {
        to: "/tweaks",
        label: "Tweaks",
        icon: Gauge,
        description: "Documented settings, each with an undo",
        tabs: TWEAK_TABS,
      },
      {
        to: "/library",
        label: "Library",
        icon: Library,
        description: "Every optimisation by category, with presets",
        tabs: LIBRARY_TABS,
      },
      {
        to: "/hardware",
        label: "Hardware",
        icon: Cpu,
        description: "Settings for the parts inside this PC",
        tabs: HARDWARE_TABS,
      },
      {
        to: "/cleaner",
        label: "PC Cleaner",
        icon: Trash2,
        description: "Clean files, services, apps and startup",
        tabs: CLEANER_TABS,
      },
      {
        to: "/network",
        label: "Network",
        icon: Router,
        description: "Network settings, game priority and adapters",
        tabs: NETWORK_TABS,
      },
      {
        to: "/advanced",
        label: "Advanced",
        icon: Rocket,
        description: "Security status, interrupts, devices and the system timer",
        tabs: ADVANCED_TABS,
      },
    ],
  },
];

export const SETTINGS_ITEM: NavItem = {
  to: "/settings",
  label: "Settings",
  icon: Settings,
  description: "Preferences, updates and engine status",
};

export const ALL_NAV_ITEMS: NavItem[] = [...NAV_GROUPS.flatMap((g) => g.items), SETTINGS_ITEM];
