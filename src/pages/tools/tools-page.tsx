import {
  Activity,
  ClipboardCopy,
  Cog,
  Copy,
  Cpu,
  Download,
  Eraser,
  Info,
  ListChecks,
  MonitorCog,
  Network,
  ScrollText,
  ShieldCheck,
  type LucideIcon,
} from "lucide-react";
import { toast } from "sonner";
import { OptimisationRow } from "@/components/common/optimisation-card";
import { SectionCard } from "@/components/common/section-card";
import { LoadingRows } from "@/components/common/states";
import { RecommendedAction } from "@/components/common/recommended-action";
import { PageHero } from "@/components/layout/page-hero";
import { Button } from "@/components/ui/button";
import { useAppInfo, useSystemSnapshot, useTweakStatuses } from "@/hooks/use-system";
import { buildSystemReport } from "@/lib/system-report";
import { backend, toBackendError } from "@/services/backend";
import type { WindowsTool } from "@/types/system";

// The backend opens these from its own fixed list: the frontend only names one.
const WINDOWS_TOOLS: { id: WindowsTool; name: string; icon: LucideIcon }[] = [
  { id: "resourceMonitor", name: "Resource Monitor", icon: Activity },
  { id: "reliabilityMonitor", name: "Reliability Monitor", icon: ShieldCheck },
  { id: "systemInformation", name: "System Information", icon: Info },
  { id: "directxDiagnostic", name: "DirectX Diagnostic", icon: MonitorCog },
  { id: "diskCleanup", name: "Disk Cleanup", icon: Eraser },
  { id: "eventViewer", name: "Event Viewer", icon: ScrollText },
  { id: "taskManager", name: "Task Manager", icon: ListChecks },
  { id: "deviceManager", name: "Device Manager", icon: Cpu },
  { id: "servicesConsole", name: "Services", icon: Cog },
  { id: "networkConnections", name: "Network Connections", icon: Network },
];

export function ToolsPage() {
  const app = useAppInfo();
  const snapshot = useSystemSnapshot();
  const tweaks = useTweakStatuses();
  const report =
    app.data && snapshot.data
      ? buildSystemReport(app.data, snapshot.data, tweaks.data ?? [])
      : null;
  const fullReport =
    app.data && snapshot.data
      ? buildSystemReport(app.data, snapshot.data, tweaks.data ?? [], new Date(), true)
      : null;

  const copy = async (text: string, what: string) => {
    try {
      await navigator.clipboard.writeText(text);
      toast.success(`${what} copied`);
    } catch {
      toast.error("Couldn't access the clipboard");
    }
  };

  const exportReport = () => {
    if (!report) return;
    backend.exportReport(report).then(
      (file) => file && toast.success(`Saved ${file}`),
      (err: unknown) => toast.error(toBackendError(err).message),
    );
  };

  const open = (tool: WindowsTool) => {
    backend.openTool(tool).catch((err: unknown) => toast.error(toBackendError(err).message));
  };

  return (
    <div className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]">
      <PageHero
        title="Resources"
        subtitle="System report and Windows tools"
        action={<RecommendedAction />}
      />
      <div className="grid flex-1 grid-cols-12 gap-3">
        <SectionCard
          title="System report"
          source="live"
          className="col-span-7"
          contentClassName="relative min-h-[280px]"
          actions={
            <>
              <Button
                size="xs"
                variant="ghost"
                disabled={!fullReport}
                onClick={() => fullReport && copy(fullReport, "Full list")}
              >
                <Copy /> Copy full list
              </Button>
              <Button
                size="xs"
                variant="ghost"
                disabled={!report}
                onClick={() => report && copy(report, "Report")}
              >
                <ClipboardCopy /> Copy
              </Button>
              <Button size="xs" variant="ghost" disabled={!report} onClick={exportReport}>
                <Download /> Export
              </Button>
            </>
          }
        >
          {!report ? (
            <LoadingRows rows={8} />
          ) : (
            <pre
              data-selectable
              className="absolute inset-x-4 inset-y-0 overflow-auto rounded-lg bg-background/50 p-3 font-mono text-[11.5px] leading-relaxed text-muted-foreground"
            >
              {report}
            </pre>
          )}
        </SectionCard>

        <div className="col-span-5 flex flex-col divide-y overflow-hidden panel [&>*]:h-auto [&>*]:flex-1">
          {WINDOWS_TOOLS.map((t) => (
            <OptimisationRow
              key={t.id}
              flat
              leading={<t.icon className="size-[18px] text-brand" strokeWidth={1.75} aria-hidden />}
              title={t.name}
              control={
                <Button size="xs" variant="ghost" onClick={() => open(t.id)}>
                  Open
                </Button>
              }
            />
          ))}
        </div>
      </div>
    </div>
  );
}
