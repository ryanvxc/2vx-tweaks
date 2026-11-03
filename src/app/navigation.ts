import {
  Activity,
  AppWindow,
  Bell,
  Cpu,
  EyeOff,
  Gamepad2,
  Gpu,
  History,
  House,
  Info,
  Layers,
  MemoryStick,
  Network,
  Palette,
  Rocket,
  Server,
  Settings,
  ShieldCheck,
  SlidersHorizontal,
  Sparkles,
  User,
  Users,
  Wrench,
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
  description: string;
  /** Sections of the page, chosen with `?tab=`. */
  tabs?: NavTab[];
}

export interface NavGroup {
  label: string;
  items: NavItem[];
}

export type MonitoringTab = "cpu" | "memory" | "gpu" | "network";
export type TweakTab = "windows" | "privacy" | "apps" | "services";
export type StartupTab = "all" | "currentUser" | "allUsers";
export type SettingsTab = "general" | "appearance" | "updates" | "security" | "about";

export const MONITORING_TABS: NavTab<MonitoringTab>[] = [
  { value: "cpu", label: "CPU", icon: Cpu },
  { value: "memory", label: "Memory", icon: MemoryStick },
  { value: "gpu", label: "GPU", icon: Gpu },
  { value: "network", label: "Network", icon: Network },
];

export const TWEAK_TABS: NavTab<TweakTab>[] = [
  { value: "windows", label: "Windows", icon: AppWindow },
  { value: "privacy", label: "Privacy", icon: EyeOff },
  { value: "apps", label: "Apps", icon: Layers },
  { value: "services", label: "Services", icon: Server },
];

export const STARTUP_TABS: NavTab<StartupTab>[] = [
  { value: "all", label: "All", icon: Layers },
  { value: "currentUser", label: "Your account", icon: User },
  { value: "allUsers", label: "All users", icon: Users },
];

export const SETTINGS_TABS: NavTab<SettingsTab>[] = [
  { value: "general", label: "General", icon: Settings },
  { value: "appearance", label: "Appearance", icon: Palette },
  { value: "updates", label: "Updates", icon: Bell },
  { value: "security", label: "Security", icon: ShieldCheck },
  { value: "about", label: "About", icon: Info },
];

export const NAV_GROUPS: NavGroup[] = [
  {
    label: "Overview",
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
        to: "/hardware",
        label: "Hardware",
        icon: Cpu,
        description: "What's inside this PC",
      },
    ],
  },
  {
    label: "Optimise",
    items: [
      {
        to: "/tweaks",
        label: "Tweaks",
        icon: SlidersHorizontal,
        description: "Documented settings, each with an undo",
        tabs: TWEAK_TABS,
      },
      {
        to: "/gaming",
        label: "Gaming",
        icon: Gamepad2,
        description: "Pick a profile, then review what it changes",
      },
    ],
  },
  {
    label: "Maintain",
    items: [
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
        icon: Sparkles,
        description: "What temporary files and caches use",
      },
      {
        to: "/restore",
        label: "Restore Centre",
        icon: History,
        description: "Every change 2VX made, with undo",
      },
      {
        to: "/tools",
        label: "Tools",
        icon: Wrench,
        description: "System report and Windows tools",
      },
    ],
  },
];

export const SETTINGS_ITEM: NavItem = {
  to: "/settings",
  label: "Settings",
  icon: Settings,
  description: "Preferences, updates and engine status",
  tabs: SETTINGS_TABS,
};

export const ALL_NAV_ITEMS: NavItem[] = [...NAV_GROUPS.flatMap((g) => g.items), SETTINGS_ITEM];

/** A page's header line, from the one place it's defined. */
export function pageDescription(to: string) {
  return ALL_NAV_ITEMS.find((i) => i.to === to)?.description;
}
