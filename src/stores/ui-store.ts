import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface UiState {
  sidebarCollapsed: boolean;
  /** Check the update feed once per launch. Downloading and installing always ask first. */
  autoCheckUpdates: boolean;
  /** Show the "update available" notice on Home. */
  showUpdateNotice: boolean;
  /** Version whose Home notice the user dismissed; a newer version shows again. */
  dismissedUpdateVersion: string | null;
  /** Turn off non-essential animation (on top of the OS setting). */
  reduceMotion: boolean;
  /** Show the Quickstart list on Home. */
  showQuickstart: boolean;
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
      toggleSidebar: () => set((s) => ({ sidebarCollapsed: !s.sidebarCollapsed })),
      set: (patch) => set(patch),
    }),
    {
      name: "2vx-ui",
      storage: createJSONStorage(() => localStorage),
    },
  ),
);
