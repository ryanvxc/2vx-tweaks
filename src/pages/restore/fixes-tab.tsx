import { useState } from "react";
import { FileCheck2, PackageCheck, Wrench, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { StatusLabel } from "@/components/common/notice";
import { CardGrid, OptimisationCard } from "@/components/common/optimisation-card";
import { Button } from "@/components/ui/button";
import { backend, toBackendError } from "@/services/backend";
import { runRepair, useTaskStore } from "@/stores/task-store";
import type { RepairTool } from "@/types/system";

const REPAIRS: {
  tool: RepairTool;
  icon: LucideIcon;
  name: string;
  description: string;
  order: string;
}[] = [
  {
    tool: "windowsImage",
    icon: PackageCheck,
    name: "Repair the Windows image",
    description: "DISM checks the copy Windows repairs itself from, and may use Windows Update.",
    order: "Run this first if Windows files keep failing the check.",
  },
  {
    tool: "systemFiles",
    icon: FileCheck2,
    name: "Check Windows files",
    description: "System File Checker scans protected files and replaces any that are damaged.",
    order: "A good first step when Windows misbehaves.",
  },
];

/** Windows' own repair tools. 2VX runs them as they are and shows what they said. */
export function FixesTab() {
  const repairing = useTaskStore((s) => s.repairing);
  const repairs = useTaskStore((s) => s.repairs);
  const [confirming, setConfirming] = useState<(typeof REPAIRS)[number] | null>(null);
  const finished = REPAIRS.flatMap((r) => {
    const outcome = repairs[r.tool];
    return outcome ? [{ ...r, outcome }] : [];
  });

  return (
    <div className="flex flex-1 flex-col gap-3">
      <CardGrid>
        {REPAIRS.map((repair) => (
          <OptimisationCard
            key={repair.tool}
            icon={repair.icon}
            title={repair.name}
            description={repair.description}
            footer={
              <Button
                className="w-full"
                disabled={repairing !== null}
                onClick={() => setConfirming(repair)}
              >
                {repairing === repair.tool ? "Running…" : "Run"}
              </Button>
            }
          />
        ))}
        <OptimisationCard
          icon={Wrench}
          title="Windows troubleshooters"
          description="Windows' own guided fixes for sound, network, printers and more."
          footer={
            <Button
              variant="outline"
              className="w-full"
              onClick={() => {
                backend
                  .openLink("troubleshoot")
                  .catch((err: unknown) => toast.error(toBackendError(err).message));
              }}
            >
              Open troubleshooters
            </Button>
          }
        />
      </CardGrid>

      {repairing !== null && (
        <p className="panel px-4 py-3 text-[13px] text-muted-foreground">
          This can take many minutes. Windows' tool reports when it ends, and you can leave this
          page while it runs.
        </p>
      )}

      {finished.map(({ tool, name, outcome }) => (
        <section key={tool} data-stagger className="flex flex-col overflow-hidden panel">
          <header className="flex h-11 shrink-0 items-center justify-between gap-3 border-b px-4">
            <h2 className="truncate text-[13.5px] font-semibold">{name}</h2>
            <span className="flex items-center gap-3">
              <StatusLabel tone={outcome.succeeded ? "success" : "warning"}>
                {outcome.succeeded ? "Finished" : "Reported a problem"}
              </StatusLabel>
              <DataSourceBadge source="live" />
            </span>
          </header>
          <pre
            data-selectable
            className="max-h-64 overflow-auto p-4 font-mono text-[11.5px] leading-relaxed whitespace-pre-wrap text-muted-foreground"
          >
            {outcome.output || "The tool printed nothing."}
          </pre>
        </section>
      ))}

      {confirming && (
        <ConfirmDialog
          open
          onOpenChange={(next) => !next && setConfirming(null)}
          title={`${confirming.name}?`}
          description={confirming.order}
          consequences={[
            "Windows asks for permission once",
            "It can take many minutes, and the result shows when it ends",
            "It can replace damaged Windows files",
          ]}
          risk="moderate"
          confirmLabel="Run"
          onConfirm={() => {
            const { tool, name } = confirming;
            setConfirming(null);
            void runRepair(tool, name);
          }}
        />
      )}
    </div>
  );
}
