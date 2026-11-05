import { useState } from "react";
import { HardDriveDownload } from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { CardGrid, OptimisationCard } from "@/components/common/optimisation-card";
import { Button } from "@/components/ui/button";
import { formatTimestamp } from "@/lib/engine-text";
import { backend, toBackendError } from "@/services/backend";
import { optimizeDrives, useTaskStore } from "@/stores/task-store";
import { useUiStore } from "@/stores/ui-store";

/** Windows' own optimisation, on every fixed drive. It runs weekly anyway; this runs it now. */
export function OptimizeDrives() {
  const optimizing = useTaskStore((s) => s.optimizing);
  const lastRun = useUiStore((s) => s.drivesLastRun);
  const [confirming, setConfirming] = useState(false);

  return (
    <>
      <CardGrid>
        <OptimisationCard
          icon={HardDriveDownload}
          title="Optimize Drives"
          description={
            lastRun === null
              ? "Windows' own TRIM for SSDs and defragmenting for hard drives."
              : `TRIM for SSDs, defragmenting for hard drives. Last ran ${formatTimestamp(lastRun)}.`
          }
          footer={
            <div className="flex gap-2 [&>*]:flex-1">
              <Button disabled={optimizing} onClick={() => setConfirming(true)}>
                {optimizing ? "Running…" : "Optimize"}
              </Button>
              <Button
                variant="outline"
                onClick={() => {
                  backend
                    .openTool("optimizeDrives")
                    .catch((err: unknown) => toast.error(toBackendError(err).message));
                }}
              >
                Windows' tool
              </Button>
            </div>
          }
        />
      </CardGrid>
      <ConfirmDialog
        open={confirming}
        onOpenChange={setConfirming}
        title="Optimize every fixed drive?"
        description="Windows chooses the work for each drive. USB drives are left out."
        consequences={[
          "Windows asks for permission once",
          "A hard drive can take hours, and an SSD a few minutes",
        ]}
        risk="low"
        confirmLabel="Optimize"
        onConfirm={() => {
          setConfirming(false);
          void optimizeDrives();
        }}
      />
    </>
  );
}
