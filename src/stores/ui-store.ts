import { create } from "zustand";
import { createJSONStorage, persist } from "zustand/middleware";

interface UiState {
  sidebarCollapsed: boolean;
  toggleSidebar: () => void;
}

// UI preferences only. Anything that must survive reinstalls or be backed up belongs
// in the backend (the change journal is SQLite), not here.
export const useUiStore = create<UiState>()(
  persist(
    (set) => ({
      sidebarCollapsed: false,
      toggleSidebar: () => set((s) => ({ sidebarCollapsed: !s.sidebarCollapsed })),
    }),
    {
      name: "2vx-ui",
      storage: createJSONStorage(() => localStorage),
    },
  ),
);
