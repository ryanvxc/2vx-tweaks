import { ClipboardCopy, MonitorCog, Terminal } from "lucide-react";
import { toast } from "sonner";
import { SectionCard } from "@/components/common/section-card";
import { LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { useAppInfo, useSystemSnapshot, useTweakStatuses } from "@/hooks/use-system";
import { buildSystemReport } from "@/lib/system-report";

// Built-in Windows tools the user can start from the Run dialog (Win+R). 2VX doesn't
// launch them itself: the backend never starts processes (docs/security.md).
const WINDOWS_TOOLS = [
  { name: "Resource Monitor", run: "resmon" },
  { name: "Reliability Monitor", run: "perfmon /rel" },
  { name: "System Information", run: "msinfo32" },
  { name: "DirectX Diagnostic Tool", run: "dxdiag" },
  { name: "Disk Cleanup", run: "cleanmgr" },
  { name: "Event Viewer", run: "eventvwr.msc" },
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
      <PageHeader title="Tools" />
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
          </SectionCard>
        </div>
        <div className="col-span-5">
          <SectionCard title="Windows tools" description="Win+R, paste, Enter">
            <ul className="-mx-2 flex flex-col">
              {WINDOWS_TOOLS.map((t) => (
                <li
                  key={t.run}
                  className="flex items-center gap-3 rounded-md px-2 py-2 hover:bg-accent/40"
                >
                  <MonitorCog className="size-4 shrink-0 text-brand" />
                  <div className="min-w-0 flex-1 truncate text-[13px] font-medium">{t.name}</div>
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
      </div>
    </>
  );
}
