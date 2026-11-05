import { toast } from "sonner";
import { create } from "zustand";
import { restartNote } from "@/lib/engine-text";
import type { RestartRequirement } from "@/types/engine";

export const useRestartDialog = create<{ open: boolean }>(() => ({ open: false }));

/** Says what a change still needs, and offers the restart. It never restarts by itself. */
export function announceRestart(restart: RestartRequirement) {
  const note = restartNote(restart);
  if (!note) return;
  if (restart === "signOut") {
    toast.info(note);
    return;
  }
  toast.info(note, {
    description: "Test the PC before you change more.",
    action: { label: "Restart now", onClick: () => useRestartDialog.setState({ open: true }) },
  });
}
