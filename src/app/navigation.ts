import {
  Cpu,
  Gamepad2,
  History,
  House,
  Rocket,
  Settings,
  SlidersHorizontal,
  Sparkles,
  Wrench,
  type LucideIcon,
} from "lucide-react";

export interface NavItem {
  to: string;
  label: string;
  icon: LucideIcon;
  description: string;
}

export interface NavGroup {
  label: string;
  items: NavItem[];
}

export const NAV_GROUPS: NavGroup[] = [
  {
    label: "Overview",
    items: [
      { to: "/", label: "Home", icon: House, description: "Your PC at a glance" },
      {
        to: "/hardware",
        label: "Hardware",
        icon: Cpu,
        description: "Live CPU, GPU, memory, storage and network",
      },
    ],
  },
  {
    label: "Optimisations",
    items: [
      {
        to: "/system",
        label: "System Tweaks",
        icon: SlidersHorizontal,
        description: "Documented Windows, privacy and browser settings, each with undo",
      },
      {
        to: "/gaming",
        label: "Gaming",
        icon: Gamepad2,
        description: "Gaming-related settings and what 2VX can measure",
      },
      {
        to: "/startup",
        label: "Startup",
        icon: Rocket,
        description: "What starts with Windows on this PC",
      },
      {
        to: "/cleaner",
        label: "PC Cleaner",
        icon: Sparkles,
        description: "How much space temporary files and caches use",
      },
    ],
  },
  {
    label: "Tools",
    items: [
      {
        to: "/history",
        label: "History",
        icon: History,
        description: "Every change 2VX has made, with undo",
      },
      {
        to: "/tools",
        label: "Tools",
        icon: Wrench,
        description: "System report and diagnostics",
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
