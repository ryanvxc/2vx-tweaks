import {
  Activity,
  AppWindow,
  ArchiveRestore,
  Cpu,
  EyeOff,
  Gamepad2,
  Gauge,
  Gpu,
  House,
  Layers,
  MemoryStick,
  Network,
  Package,
  Rocket,
  Server,
  Settings,
  Trash2,
  User,
  Users,
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
  /** The sidebar gives each tab its own row instead of one row for the page. */
  tabsInSidebar?: boolean;
  /** Drawn in the brand colour, to stand out from the list. */
  highlight?: boolean;
}

export interface NavGroup {
  label: string;
  items: NavItem[];
}

export type MonitoringTab = "cpu" | "memory" | "gpu" | "network";
export type TweakTab = "windows" | "privacy" | "services";
export type StartupTab = "all" | "currentUser" | "allUsers";

export const MONITORING_TABS: NavTab<MonitoringTab>[] = [
  { value: "cpu", label: "CPU", icon: Cpu },
  { value: "memory", label: "Memory", icon: MemoryStick },
  { value: "gpu", label: "GPU", icon: Gpu },
  { value: "network", label: "Network", icon: Network },
];

export const TWEAK_TABS: NavTab<TweakTab>[] = [
  { value: "windows", label: "Windows", icon: AppWindow },
  { value: "privacy", label: "Privacy", icon: EyeOff },
  { value: "services", label: "Services", icon: Server },
];

export const STARTUP_TABS: NavTab<StartupTab>[] = [
  { value: "all", label: "All", icon: Layers },
  { value: "currentUser", label: "Your account", icon: User },
  { value: "allUsers", label: "All users", icon: Users },
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
        label: "Game Mode",
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
        tabsInSidebar: true,
      },
      {
        to: "/hardware",
        label: "Hardware",
        icon: Cpu,
        description: "What's inside this PC",
      },
      {
        to: "/startup",
        label: "Startup",
        icon: Rocket,
        description: "What starts with Windows",
        tabs: STARTUP_TABS,
      },
      {
        to: "/cleaner",
        label: "PC Cleaner",
        icon: Trash2,
        description: "What temporary files and caches use",
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
