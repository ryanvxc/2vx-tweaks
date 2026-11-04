import {
  Activity,
  AppWindow,
  ArchiveRestore,
  Brush,
  Cpu,
  EyeOff,
  Gamepad2,
  Gauge,
  Gpu,
  House,
  MemoryStick,
  Network,
  Package,
  PackageX,
  Rocket,
  Server,
  Settings,
  Trash2,
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
export type CleanerTab = "system" | "services" | "uninstall" | "autoruns";

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

export const CLEANER_TABS: NavTab<CleanerTab>[] = [
  { value: "system", label: "System Cleaner", icon: Brush },
  { value: "services", label: "Services", icon: Server },
  { value: "uninstall", label: "Uninstall", icon: PackageX },
  { value: "autoruns", label: "Autoruns", icon: Rocket },
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
        tabsInSidebar: true,
      },
      {
        to: "/hardware",
        label: "Hardware",
        icon: Cpu,
        description: "What's inside this PC",
      },
      {
        to: "/cleaner",
        label: "PC Cleaner",
        icon: Trash2,
        description: "Clean files, services, apps and startup",
        tabs: CLEANER_TABS,
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
