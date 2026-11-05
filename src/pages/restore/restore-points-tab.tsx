import { useState } from "react";
import { FileClock, History, RotateCcw, ShieldCheck } from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { StatusLabel } from "@/components/common/notice";
import { CardGrid, OptimisationCard } from "@/components/common/optimisation-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useMakeRestorePoint } from "@/hooks/use-restore-point";
import { refreshRestorePoints, useRestorePoints } from "@/hooks/use-system";
import { formatTimestamp, type Tone } from "@/lib/engine-text";
import { backend, toBackendError } from "@/services/backend";
import type { ProtectionStatus, WindowsTool } from "@/types/system";

const PROTECTION: Record<ProtectionStatus, { tone: Tone; label: string }> = {
  on: { tone: "success", label: "On" },
  off: { tone: "warning", label: "Off" },
  blocked: { tone: "warning", label: "Blocked by a policy" },
  unknown: { tone: "neutral", label: "Unknown" },
};

const openTool = (tool: WindowsTool) => {
  backend.openTool(tool).catch((err: unknown) => toast.error(toBackendError(err).message));
};

/**
 * 2VX lists points and makes them. Going back to one stays in Windows' own wizard, which shows what
 * will change and handles the restart.
 */
export function RestorePointsTab() {
  const { data, error, loading, refetch } = useRestorePoints();
  const { making, make } = useMakeRestorePoint();
  const [confirming, setConfirming] = useState(false);
  const [enabling, setEnabling] = useState(false);
  const protection = data?.protection ?? "unknown";

  const enable = async () => {
    setEnabling(true);
    try {
      await backend.enableSystemRestore();
      toast.success("System Protection is on");
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      refreshRestorePoints();
      setEnabling(false);
    }
  };

  return (
    <div className="flex flex-1 flex-col gap-3">
      <CardGrid>
        <OptimisationCard
          icon={FileClock}
          title="Make a restore point"
          description="Windows saves its system files and settings, so you can go back to today."
          footer={
            <Button
              className="w-full"
              disabled={making || !data || protection === "off" || protection === "blocked"}
              onClick={() => setConfirming(true)}
            >
              {making ? "Making…" : "Create restore point"}
            </Button>
          }
        />
        <OptimisationCard
          icon={ShieldCheck}
          title="System Protection"
          badge={
            <StatusLabel tone={PROTECTION[protection].tone}>
              {PROTECTION[protection].label}
            </StatusLabel>
          }
          description="Restore points only work while this is on for the system drive."
          footer={
            protection === "off" ? (
              <Button className="w-full" disabled={enabling} onClick={() => void enable()}>
                {enabling ? "Turning on…" : "Turn on"}
              </Button>
            ) : (
              <Button
                variant="outline"
                className="w-full"
                onClick={() => openTool("systemProtection")}
              >
                Open System Protection
              </Button>
            )
          }
        />
        <OptimisationCard
          icon={RotateCcw}
          title="Go back to a restore point"
          description="Windows' own wizard shows what will change, then restarts the PC."
          footer={
            <Button variant="outline" className="w-full" onClick={() => openTool("systemRestore")}>
              Open System Restore
            </Button>
          }
        />
      </CardGrid>

      <section data-stagger className="flex flex-1 flex-col overflow-hidden panel">
        <div className="flex h-9 shrink-0 items-center justify-between gap-3 border-b px-4 text-[12px] text-muted-foreground">
          <span>Restore point</span>
          <span className="flex items-center gap-3">
            <span>Created</span>
            <DataSourceBadge source="live" />
          </span>
        </div>
        {error ? (
          <div className="p-4">
            <ErrorState message={error.message} onRetry={refetch} />
          </div>
        ) : loading && !data ? (
          <div className="p-4">
            <LoadingRows rows={3} />
          </div>
        ) : data?.points === null ? (
          <EmptyState
            icon={History}
            title="Windows wouldn't list them"
            description="Open System Restore to see them there."
            className="flex-1"
          />
        ) : !data || data.points.length === 0 ? (
          <EmptyState icon={History} title="No restore points yet" className="flex-1" />
        ) : (
          <ul className="divide-y">
            {data.points.map((point) => (
              <li
                key={point.sequence}
                className="flex h-10 items-center justify-between gap-4 px-4 text-[13px]"
              >
                <span data-selectable className="truncate font-medium" title={point.description}>
                  {point.description || "Restore point"}
                </span>
                <span className="shrink-0 text-[12.5px] text-muted-foreground tabular">
                  {point.createdMs === null ? "—" : formatTimestamp(point.createdMs)}
                </span>
              </li>
            ))}
          </ul>
        )}
      </section>

      <ConfirmDialog
        open={confirming}
        onOpenChange={setConfirming}
        title="Make a restore point?"
        description="Windows does the work, and it can take a minute."
        consequences={[
          "Windows asks for permission once",
          "It skips a new point when it made one recently",
        ]}
        risk="low"
        confirmLabel="Create"
        onConfirm={() => {
          setConfirming(false);
          void make();
        }}
      />
    </div>
  );
}
