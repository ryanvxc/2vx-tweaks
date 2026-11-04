import { ClipboardCopy, Copy, MonitorCog } from "lucide-react";
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
      <div className="grid flex-1 grid-cols-12 gap-3">
        <SectionCard
          title="System report"
          source="live"
          className="col-span-7"
          contentClassName="relative min-h-[280px]"
          actions={
            <Button
              size="xs"
              variant="ghost"
              disabled={!report}
              onClick={() => report && copy(report, "Report")}
            >
              <ClipboardCopy /> Copy
            </Button>
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

        <SectionCard
          title="Windows tools"
          description="Win+R, paste, Enter"
          className="col-span-5"
          contentClassName="flex flex-col"
        >
          {/* Rows share the height, so the list ends level with the report beside it. */}
          <ul className="-mx-2 flex flex-1 flex-col">
            {WINDOWS_TOOLS.map((t) => (
              <li key={t.run} className="flex flex-1">
                <button
                  type="button"
                  aria-label={`Copy ${t.run}`}
                  onClick={() => copy(t.run, `"${t.run}"`)}
                  className="group flex min-h-9 w-full items-center gap-3 rounded-md px-2 text-left transition-colors outline-none hover:bg-accent/60 focus-visible:ring-2 focus-visible:ring-ring"
                >
                  <MonitorCog className="size-4 shrink-0 text-brand" aria-hidden />
                  <span className="min-w-0 flex-1 truncate text-[13px] font-medium">{t.name}</span>
                  <code className="font-mono text-[11.5px] text-muted-foreground">{t.run}</code>
                  <Copy
                    className="size-3.5 shrink-0 text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 group-focus-visible:opacity-100"
                    aria-hidden
                  />
                </button>
              </li>
            ))}
          </ul>
        </SectionCard>
      </div>
    </>
  );
}
