import { toast } from "sonner";
import { create } from "zustand";
import { backend, toBackendError } from "@/services/backend";
import { useUiStore } from "@/stores/ui-store";
import type { DriveOutcome, RepairOutcome, RepairTool } from "@/types/system";

interface TaskState {
  repairing: RepairTool | null;
  repairs: Partial<Record<RepairTool, RepairOutcome>>;
  optimizing: boolean;
  drives: DriveOutcome[] | null;
}

/**
 * Jobs that outlast a page. Each can run for a long time, so leaving the page mustn't lose its
 * answer. Nothing here is saved: the next launch starts empty.
 */
export const useTaskStore = create<TaskState>(() => ({
  repairing: null,
  repairs: {},
  optimizing: false,
  drives: null,
}));

export async function runRepair(tool: RepairTool, name: string) {
  useTaskStore.setState({ repairing: tool });
  try {
    const outcome = await backend.runRepair(tool);
    useTaskStore.setState((s) => ({ repairs: { ...s.repairs, [tool]: outcome } }));
    if (outcome.succeeded) toast.success(`${name} finished`);
    else toast.warning(`${name} reported a problem`);
  } catch (err) {
    toast.error(toBackendError(err).message);
  } finally {
    useTaskStore.setState({ repairing: null });
  }
}

export async function optimizeDrives() {
  useTaskStore.setState({ optimizing: true });
  try {
    const drives = await backend.optimizeDrives();
    useTaskStore.setState({ drives });
    useUiStore.getState().set({ drivesLastRun: Date.now() });
    const failed = drives.filter((d) => !d.optimized);
    if (failed.length === 0) {
      toast.success(`Optimized ${drives.map((d) => d.drive).join(", ")}`);
    } else {
      toast.warning(`${failed.length} of ${drives.length} drives weren't optimized`, {
        description: failed.map((d) => `${d.drive} ${d.problem ?? ""}`.trim()).join(". "),
      });
    }
  } catch (err) {
    toast.error(toBackendError(err).message);
  } finally {
    useTaskStore.setState({ optimizing: false });
  }
}
