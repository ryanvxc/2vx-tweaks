import {
  AppWindow,
  ArchiveRestore,
  Bolt,
  Brush,
  CircuitBoard,
  Cpu,
  Download,
  EthernetPort,
  Eye,
  FileClock,
  FolderOpen,
  Gamepad2,
  Gauge,
  Gpu,
  HardDrive,
  HatGlasses,
  House,
  Keyboard,
  Layers,
  LifeBuoy,
  ListOrdered,
  LogIn,
  MemoryStick,
  MessageSquareCode,
  Monitor,
  MonitorCog,
  Package,
  PackageX,
  Palette,
  PanelBottom,
  Rocket,
  Router,
  Server,
  Settings,
  Shield,
  SlidersHorizontal,
  Timer,
  Trash2,
  Undo2,
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

export type TweakTab = "core" | "privacy" | "gaming" | "apps" | "presets";
export type CleanerTab = "system" | "services" | "uninstall" | "autoruns";
export type HardwareTab = "gpu" | "cpu" | "ram" | "peripherals" | "storage";
export type NetworkTab = "tweaks" | "priority" | "adapters" | "bufferbloat";
export type VisualsTab = "appearance" | "taskbar" | "explorer" | "signin";
export type AdvancedTab = "security" | "msi" | "devices" | "timer";
export type RestoreTab = "changes" | "points" | "fixes";
export type ResourcesTab = "tools" | "downloads";
export type BiosTab = "general" | "cpu" | "memory" | "graphics" | "power";

export const TWEAK_TABS: NavTab<TweakTab>[] = [
  { value: "core", label: "Core", icon: Bolt },
  { value: "privacy", label: "Privacy", icon: HatGlasses },
  { value: "gaming", label: "Gaming", icon: Gamepad2 },
  { value: "apps", label: "Apps", icon: AppWindow },
  { value: "presets", label: "Presets", icon: Layers },
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

export const VISUALS_TABS: NavTab<VisualsTab>[] = [
  { value: "appearance", label: "Appearance", icon: Palette },
  { value: "taskbar", label: "Taskbar and Start", icon: PanelBottom },
  { value: "explorer", label: "File Explorer", icon: FolderOpen },
  { value: "signin", label: "Sign-in", icon: LogIn },
];

export const ADVANCED_TABS: NavTab<AdvancedTab>[] = [
  { value: "security", label: "Security", icon: Shield },
  { value: "msi", label: "MSI Mode", icon: MessageSquareCode },
  { value: "devices", label: "Devices", icon: Monitor },
  { value: "timer", label: "Timer Resolution", icon: Timer },
];

export const RESTORE_TABS: NavTab<RestoreTab>[] = [
  { value: "changes", label: "Changes", icon: Undo2 },
  { value: "points", label: "Restore Points", icon: FileClock },
  { value: "fixes", label: "Fixes", icon: LifeBuoy },
];

export const RESOURCES_TABS: NavTab<ResourcesTab>[] = [
  { value: "tools", label: "Tools", icon: MonitorCog },
  { value: "downloads", label: "Downloads", icon: Download },
];

export const BIOS_TABS: NavTab<BiosTab>[] = [
  { value: "general", label: "General", icon: CircuitBoard },
  { value: "cpu", label: "CPU", icon: Cpu },
  { value: "memory", label: "Memory", icon: MemoryStick },
  { value: "graphics", label: "Graphics", icon: Gpu },
  { value: "power", label: "Power", icon: Zap },
];

export const NAV_GROUPS: NavGroup[] = [
  {
    label: "General",
    items: [
      { to: "/", label: "Home", icon: House, description: "Your PC at a glance" },
      {
        to: "/restore",
        label: "Restore Centre",
        icon: ArchiveRestore,
        description: "Every change 2VX made, restore points and Windows repairs",
        tabs: RESTORE_TABS,
      },
      {
        to: "/tools",
        label: "Resources",
        icon: Package,
        description: "System report, Windows tools and downloads",
        tabs: RESOURCES_TABS,
      },
      {
        to: "/settings",
        label: "Settings",
        icon: Settings,
        description: "Preferences, updates and engine status",
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
        description: "Windows, privacy, games, apps and power",
        tabs: TWEAK_TABS,
      },
      {
        to: "/visuals",
        label: "Visuals",
        icon: Eye,
        description: "Theme, taskbar, File Explorer and the sign-in screen",
        tabs: VISUALS_TABS,
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
      {
        to: "/bios",
        label: "BIOS",
        icon: CircuitBoard,
        description: "What to check in your BIOS. 2VX never changes it",
        tabs: BIOS_TABS,
      },
    ],
  },
];

export const ALL_NAV_ITEMS: NavItem[] = NAV_GROUPS.flatMap((g) => g.items);
