import { ClipboardCopy, MonitorCog, Terminal } from "lucide-react";
import { pageDescription } from "@/app/navigation";
import { toast } from "sonner";
import { PlannedFeatures } from "@/components/common/planned-features";
import { SectionCard } from "@/components/common/section-card";
import { LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { useAppInfo, useSystemSnapshot, useTweakStatuses } from "@/hooks/use-system";
import { buildSystemReport } from "@/lib/system-report";

// Built-in Windows tools the user can start from the Run dialog (Win+R). 2VX doesn't
// launch them itself: the backend never starts processes (docs/security.md).
const WINDOWS_TOOLS = [
  {
    name: "Resource Monitor",
    run: "resmon",
    text: "Live CPU, disk, network and memory per process.",
  },
  {
    name: "Reliability Monitor",
    run: "perfmon /rel",
    text: "Crashes, failed updates and warnings over time.",
  },
  { name: "System Information", run: "msinfo32", text: "Full hardware and driver inventory." },
  { name: "DirectX Diagnostic Tool", run: "dxdiag", text: "Graphics, sound and DirectX details." },
  { name: "Disk Cleanup", run: "cleanmgr", text: "Windows' own cleanup of system files." },
  { name: "Event Viewer", run: "eventvwr.msc", text: "Application and system logs." },
];

export function ToolsPage() {
  const app = useAppInfo();
  const snapshot = useSystemSnapshot();
  const tweaks = useTweakStatuses();
  const report =
    app.data && snapshot.data
      ? buildSystemReport(app.data, snapshot.data, tweaks.data ?? [])
      : null;

  const copy = async (text: string, what: string) => {
    try {
      await navigator.clipboard.writeText(text);
      toast.success(`${what} copied`);
    } catch {
      toast.error("Couldn't access the clipboard");
    }
  };

  return (
    <>
      <PageHeader title="Tools" description={pageDescription("/tools")} />
      <div className="grid grid-cols-12 gap-3">
        <div className="col-span-7">
          <SectionCard
            title="System report"
            source="live"
            actions={
              <Button size="xs" disabled={!report} onClick={() => report && copy(report, "Report")}>
                <ClipboardCopy /> Copy
              </Button>
            }
          >
            {!report ? (
              <LoadingRows rows={8} />
            ) : (
              <pre
                data-selectable
                className="max-h-[400px] overflow-auto rounded-lg border bg-background/50 p-3 font-mono text-[11.5px] leading-relaxed text-muted-foreground"
              >
                {report}
              </pre>
            )}
            <p className="mt-2 text-[11.5px] text-muted-foreground">
              The report stays on this PC until you paste it somewhere. It includes the PC name;
              remove it before sharing if you prefer.
            </p>
          </SectionCard>
        </div>
        <div className="col-span-5">
          <SectionCard title="Windows tools" description="Press Win+R, paste, Enter">
            <ul className="-mx-2 flex flex-col">
              {WINDOWS_TOOLS.map((t) => (
                <li
                  key={t.run}
                  className="flex items-center gap-3 rounded-md px-2 py-2 hover:bg-accent/40"
                >
                  <MonitorCog className="size-4 shrink-0 text-brand" />
                  <div className="min-w-0 flex-1">
                    <div className="text-[13px] font-medium">{t.name}</div>
                    <div className="truncate text-[11.5px] text-muted-foreground">{t.text}</div>
                  </div>
                  <Button
                    size="xs"
                    variant="outline"
                    className="font-mono"
                    aria-label={`Copy ${t.run}`}
                    onClick={() => copy(t.run, `"${t.run}"`)}
                  >
                    <Terminal /> {t.run}
                  </Button>
                </li>
              ))}
            </ul>
          </SectionCard>
        </div>
        <div className="col-span-12">
          <PlannedFeatures
            features={[
              "Ping and latency test to a server you choose",
              "DNS resolver comparison",
              "System File Checker run with live output (fixed command only)",
              "Save the system report to a file",
            ]}
          />
        </div>
      </div>
    </>
  );
}
