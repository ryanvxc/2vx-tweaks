import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";
import type { SavedGame } from "@/lib/game-mode";

interface UiState {
  sidebarCollapsed: boolean;
  /** Check the update feed once per launch. Downloading and installing always ask first. */
  autoCheckUpdates: boolean;
  /** Show the "update available" notice on Home. */
  showUpdateNotice: boolean;
  /** Version whose Home notice the user dismissed; a newer version shows again. */
  dismissedUpdateVersion: string | null;
  /** Turn off non-essential animation. The one switch for it: the OS setting isn't followed. */
  reduceMotion: boolean;
  /** No longer read. Kept so stored preferences stay compatible. */
  showQuickstart: boolean;
  /** Show the loading screen at launch. */
  showSplash: boolean;
  /** Games the user added. Only ever compared with running programs. */
  games: SavedGame[];
  /** What Game Mode turns on. `null` until the user saves their own list. */
  gameModeTweaks: string[] | null;
  /** The batch Game Mode applied, so turning it off undoes that and nothing else. */
  gameModeSnapshot: number | null;
  /** When PC Cleaner last cleared anything. */
  cleanerLastRun: number | null;
  /** When Optimize Drives last finished. */
  drivesLastRun: number | null;
  /** The welcome dialog was seen or wasn't needed. */
  firstRunSeen: boolean;
  /** Quickstart steps that were opened, for those with nothing to read from the PC. */
  quickstartOpened: string[];
  toggleSidebar: () => void;
  set: (patch: Partial<Omit<UiState, "toggleSidebar" | "set">>) => void;
}

// UI preferences only. Anything that must survive reinstalls or be backed up belongs
// in the backend (the change journal is SQLite), not here.
export const useUiStore = create<UiState>()(
  persist(
    (set) => ({
      sidebarCollapsed: false,
      autoCheckUpdates: true,
      showUpdateNotice: true,
      dismissedUpdateVersion: null,
      reduceMotion: false,
      showQuickstart: true,
      showSplash: true,
      games: [],
      gameModeTweaks: null,
      gameModeSnapshot: null,
      cleanerLastRun: null,
      drivesLastRun: null,
      firstRunSeen: false,
      quickstartOpened: [],
      toggleSidebar: () => set((s) => ({ sidebarCollapsed: !s.sidebarCollapsed })),
      set: (patch) => set(patch),
    }),
    {
      name: "2vx-ui",
      storage: createJSONStorage(() => localStorage),
    },
  ),
);
