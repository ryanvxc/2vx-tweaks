import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { backend, toBackendError } from "@/services/backend";
import { useRestartDialog } from "@/stores/restart-store";

/** Mounted once, in the app shell. `announceRestart` opens it. */
export function RestartDialog() {
  const open = useRestartDialog((s) => s.open);
  const close = () => useRestartDialog.setState({ open: false });
  return (
    <ConfirmDialog
      open={open}
      onOpenChange={(next) => !next && close()}
      title="Restart now?"
      description="Windows restarts straight away and asks open apps to close."
      consequences={["Save anything you're working on first"]}
      risk="moderate"
      confirmLabel="Restart"
      onConfirm={() => {
        close();
        backend.restartPc().catch((err: unknown) => toast.error(toBackendError(err).message));
      }}
    />
  );
}
