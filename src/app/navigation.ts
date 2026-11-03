import {
  Activity,
  Gamepad2,
  History,
  LayoutDashboard,
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
      {
        to: "/",
        label: "Dashboard",
        icon: LayoutDashboard,
        description: "Your PC at a glance",
      },
      {
        to: "/monitoring",
        label: "Monitoring",
        icon: Activity,
        description: "Live and historical utilisation",
      },
      {
        to: "/history",
        label: "History",
        icon: History,
        description: "Every change 2VX has made, with undo",
      },
    ],
  },
  {
    label: "Optimise",
    items: [
      {
        to: "/gaming",
        label: "Gaming",
        icon: Gamepad2,
        description: "Presets, game profiles and latency settings",
      },
      {
        to: "/system",
        label: "System Tweaks",
        icon: SlidersHorizontal,
        description: "Windows, power, network and privacy configuration",
      },
      {
        to: "/startup",
        label: "Startup",
        icon: Rocket,
        description: "Control what runs when Windows starts",
      },
      {
        to: "/cleaner",
        label: "PC Cleaner",
        icon: Sparkles,
        description: "Reclaim storage from safe locations",
      },
    ],
  },
  {
    label: "Utilities",
    items: [
      {
        to: "/tools",
        label: "Tools",
        icon: Wrench,
        description: "Diagnostics, DNS and Windows utilities",
      },
    ],
  },
];

export const SETTINGS_ITEM: NavItem = {
  to: "/settings",
  label: "Settings",
  icon: Settings,
  description: "Preferences and engine status",
};

export const ALL_NAV_ITEMS: NavItem[] = [...NAV_GROUPS.flatMap((g) => g.items), SETTINGS_ITEM];
