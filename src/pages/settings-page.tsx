import type { ReactNode } from "react";
import { toast } from "sonner";
import { PageHeader } from "@/components/layout/page-header";
import { PlannedFeatures } from "@/components/common/planned-features";
import { SectionCard } from "@/components/common/section-card";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useAppInfo } from "@/hooks/use-system";

export function SettingsPage() {
  const { data: app, error, refetch } = useAppInfo();

  const copyLogPath = async () => {
    if (!app) return;
    try {
      await navigator.clipboard.writeText(app.logDir);
      toast.success("Log folder path copied");
    } catch {
      toast.error("Couldn't access the clipboard");
    }
  };

  return (
    <>
      <PageHeader title="Settings" description="Preferences, backups and restore history" />
      <div className="flex flex-col gap-4">
        <SectionCard title="About" source="live">
          {error ? (
            <ErrorState message={error.message} onRetry={refetch} />
          ) : !app ? (
            <LoadingRows rows={4} />
          ) : (
            <dl className="grid grid-cols-[160px_1fr] gap-y-2.5 text-[13px]">
              <Item label="Version">
                {app.version}
                {app.debugBuild && (
                  <span className="text-muted-foreground"> (development build)</span>
                )}
              </Item>
              <Item label="Privileges">{app.isElevated ? "Administrator" : "Standard user"}</Item>
              <Item label="Log folder">
                <span className="flex items-center gap-3">
                  <code
                    data-selectable
                    className="truncate font-mono text-[12px] text-muted-foreground"
                  >
                    {app.logDir}
                  </code>
                  <Button size="xs" variant="outline" onClick={copyLogPath}>
                    Copy
                  </Button>
                </span>
              </Item>
            </dl>
          )}
        </SectionCard>
        <PlannedFeatures
          phase="Phase 2"
          features={[
            "Theme and accent colour",
            "Notification preferences",
            "Backup management and export",
            "Restore history with per-change undo",
            "System restore point creation",
            "Update channel and update checks",
          ]}
        />
      </div>
    </>
  );
}

function Item({ label, children }: { label: string; children: ReactNode }) {
  return (
    <>
      <dt className="text-muted-foreground">{label}</dt>
      <dd className="min-w-0">{children}</dd>
    </>
  );
}
