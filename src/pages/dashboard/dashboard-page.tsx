import { PageHeader } from "@/components/layout/page-header";
import { ErrorState } from "@/components/common/states";
import { UpdateNotice } from "@/components/common/update-notice";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useSystemSnapshot } from "@/hooks/use-system";
import { evaluateHealth, summariseHealth } from "@/lib/health";
import { useUiStore } from "@/stores/ui-store";
import { HardwareCard } from "./hardware-card";
import { HealthPanel } from "./health-panel";
import { HomeStats, Quickstart, RecommendedTweaks } from "./home-panels";
import { LiveStats } from "./live-stats";
import { RecentActivity } from "./recent-activity";
import { UtilisationChart } from "./utilisation-chart";

export function DashboardPage() {
  const snapshot = useSystemSnapshot();
  const metrics = useLiveMetrics();
  const showQuickstart = useUiStore((s) => s.showQuickstart);
  const health = snapshot.data
    ? summariseHealth(evaluateHealth(snapshot.data, metrics.latest))
    : null;
  const os = snapshot.data?.os;

  return (
    <>
      <PageHeader
        title="Welcome back"
        description={
          snapshot.data
            ? `${snapshot.data.hostname ?? "This PC"} · ${os?.productName}${os?.displayVersion ? ` ${os.displayVersion}` : ""} · build ${os?.build}${os?.ubr != null ? `.${os.ubr}` : ""}`
            : "Reading this PC…"
        }
      />

      <div className="flex flex-col gap-4">
        <UpdateNotice />
        <HomeStats health={health} />

        {metrics.error && metrics.samples.length === 0 && (
          <ErrorState title="Live metrics unavailable" message={metrics.error.message} />
        )}

        <div className="grid grid-cols-12 gap-4">
          <div className={showQuickstart ? "col-span-8" : "col-span-12"}>
            <UtilisationChart samples={metrics.samples} snapshot={snapshot.data} />
          </div>
          {showQuickstart && (
            <div className="col-span-4">
              <Quickstart />
            </div>
          )}
        </div>

        <LiveStats samples={metrics.samples} snapshot={snapshot.data} />

        <div className="grid grid-cols-12 gap-4">
          <div className="col-span-7 flex flex-col gap-4">
            <RecommendedTweaks />
            <HardwareCard
              snapshot={snapshot.data}
              error={snapshot.error}
              onRetry={snapshot.refetch}
            />
          </div>
          <div className="col-span-5 flex flex-col gap-4">
            <HealthPanel snapshot={snapshot.data} latest={metrics.latest} />
            <RecentActivity />
          </div>
        </div>
      </div>
    </>
  );
}
