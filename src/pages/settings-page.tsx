import type { ReactNode } from "react";
import { Link } from "react-router";
import { toast } from "sonner";
import { PageHeader } from "@/components/layout/page-header";
import { PlannedFeatures } from "@/components/common/planned-features";
import { SectionCard } from "@/components/common/section-card";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { StatusLabel } from "@/components/common/notice";
import { useAppInfo, useEngineStatus } from "@/hooks/use-system";
import { isDesktop } from "@/services/backend";

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
      <PageHeader title="Settings" description="Preferences and engine status" />
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
        <EngineCard />
        <PlannedFeatures
          features={[
            "Theme and accent colour",
            "Notification preferences",
            "Change journal export",
            "System restore point creation",
            "In-app log viewer",
            "Signed updates (not decided yet; the app makes no network requests today)",
          ]}
        />
      </div>
    </>
  );
}

function EngineCard() {
  const { data: status, error, refetch } = useEngineStatus();
  return (
    <SectionCard
      title="Optimisation engine"
      source="live"
      actions={
        <Button asChild size="xs" variant="outline">
          <Link to="/history">Open History</Link>
        </Button>
      }
    >
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : !status ? (
        <LoadingRows rows={3} />
      ) : (
        <dl className="grid grid-cols-[160px_1fr] gap-y-2.5 text-[13px]">
          <Item label="Tweaks available">{status.tweakCount}</Item>
          <Item label="Change journal">
            {!isDesktop ? (
              <StatusLabel tone="info">Not in browser preview</StatusLabel>
            ) : status.journalAvailable ? (
              <StatusLabel tone="success">Available</StatusLabel>
            ) : (
              <span className="flex flex-col gap-1">
                <StatusLabel tone="destructive" className="self-start">
                  Unavailable: changes are disabled
                </StatusLabel>
                <span data-selectable className="text-[12px] text-muted-foreground">
                  {status.journalError}
                </span>
              </span>
            )}
          </Item>
          <Item label="Interrupted batches">
            {status.interruptedSnapshots === 0 ? (
              "None"
            ) : (
              <StatusLabel tone="warning">
                {status.interruptedSnapshots} to review in History
              </StatusLabel>
            )}
          </Item>
        </dl>
      )}
    </SectionCard>
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
