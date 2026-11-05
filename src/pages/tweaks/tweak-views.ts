import type { ReactNode } from "react";
import {
  ADVANCED_TABS,
  HARDWARE_TABS,
  NETWORK_TABS,
  TWEAK_TABS,
  type NavTab,
} from "@/app/navigation";

/** `check` marks what's in effect; `letter` is a tile with the first letter, for game lists. */
export type RowStyle = "check" | "letter";

/** A page built on the tweak grid: its title, tabs and whatever a tab adds around the tweaks. */
export interface TweakView {
  route: string;
  title: string;
  subtitle: string;
  tabs: NavTab[];
  defaultTab: string;
  /** Section headings on every tab, or only on tabs with more than one section. */
  headings?: "always" | "several";
  /** Tabs drawn as rows rather than cards. */
  rows?: Record<string, RowStyle>;
  /** Between the tabs and the search row. */
  banner?: ReactNode;
  /** The right end of the search row. */
  end?: (tab: string) => ReactNode;
  /** Between the search row and the tweaks. */
  intro?: (tab: string) => ReactNode;
  /** A tab's own content, in place of the search row and tweaks. */
  body?: (tab: string) => ReactNode;
}

export const TWEAKS_VIEW: TweakView = {
  route: "/tweaks",
  title: "Tweaks",
  subtitle: "Documented settings, each with an undo.",
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
  subtitle: "Documented network settings and game priority.",
  tabs: NETWORK_TABS,
  defaultTab: "tweaks",
  rows: { priority: "letter" },
};

export const ADVANCED_VIEW: TweakView = {
  headings: "always",
  route: "/advanced",
  title: "Advanced",
  subtitle: "Security status, interrupts, devices and the system timer.",
  tabs: ADVANCED_TABS,
  defaultTab: "security",
  rows: { security: "check", devices: "check" },
};
