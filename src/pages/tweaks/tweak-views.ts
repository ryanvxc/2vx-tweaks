import type { ReactNode } from "react";
import {
  ADVANCED_TABS,
  CLEANER_TABS,
  HARDWARE_TABS,
  NETWORK_TABS,
  TWEAK_TABS,
  type NavTab,
} from "@/app/navigation";
import type { TweakStatus } from "@/types/engine";

/** What a tab's own content can reach: the catalogue and the page's review. */
export interface TweakViewContext {
  statuses: TweakStatus[];
  select: (ids: string[]) => void;
  disabled: boolean;
}

/** A page built on the tweak grid: its title, tabs and whatever a tab adds around the tweaks. */
export interface TweakView {
  route: string;
  title: string;
  subtitle: string;
  tabs: NavTab[];
  defaultTab: string;
  /** Section headings on every tab, or only on tabs with more than one section. */
  headings?: "always" | "several";
  /** Between the tabs and the search row. */
  banner?: ReactNode;
  /** The right end of the search row. */
  end?: (tab: string) => ReactNode;
  /** Between the search row and the tweaks. */
  intro?: (tab: string) => ReactNode;
  /** A tab's own content, in place of the search row and tweaks. */
  body?: (tab: string, context: TweakViewContext) => ReactNode;
}

export const TWEAKS_VIEW: TweakView = {
  route: "/tweaks",
  title: "Tweaks",
  subtitle: "Windows, privacy, gaming, apps, tasks and power.",
  tabs: TWEAK_TABS,
  defaultTab: "core",
  headings: "always",
};

export const HARDWARE_VIEW: TweakView = {
  route: "/hardware",
  title: "Hardware",
  subtitle: "Settings for the parts inside this PC.",
  tabs: HARDWARE_TABS,
  defaultTab: "gpu",
};

export const NETWORK_VIEW: TweakView = {
  route: "/network",
  title: "Network",
  subtitle: "Network settings, game priority and your adapters.",
  tabs: NETWORK_TABS,
  defaultTab: "tweaks",
};

export const ADVANCED_VIEW: TweakView = {
  headings: "always",
  route: "/advanced",
  title: "Advanced",
  subtitle: "Security status, interrupts, devices and the system timer.",
  tabs: ADVANCED_TABS,
  defaultTab: "security",
};

export const CLEANER_VIEW: TweakView = {
  route: "/cleaner",
  title: "PC Cleaner",
  subtitle: "Free up space and choose what stays on this PC.",
  tabs: CLEANER_TABS,
  defaultTab: "system",
};
