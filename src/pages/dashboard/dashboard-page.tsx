import { PageHeader } from "@/components/layout/page-header";
import { ErrorState } from "@/components/common/states";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useSystemSnapshot } from "@/hooks/use-system";
import { HardwareCard } from "./hardware-card";
import { HealthPanel } from "./health-panel";
import { LiveStats } from "./live-stats";
import { RecentActivity, Recommendations } from "./recommendations";
import { SystemSummary } from "./system-summary";
import { UtilisationChart } from "./utilisation-chart";

export function DashboardPage() {
  const snapshot = useSystemSnapshot();
  const metrics = useLiveMetrics();

  return (
    <>
      <PageHeader title="Dashboard" description="Your PC at a glance" />

      <div className="flex flex-col gap-4">
        <SystemSummary snapshot={snapshot.data} />

        {metrics.error && metrics.samples.length === 0 && (
          <ErrorState title="Live metrics unavailable" message={metrics.error.message} />
        )}
        <LiveStats samples={metrics.samples} snapshot={snapshot.data} />

        <div className="grid grid-cols-12 gap-4">
          <div className="col-span-8">
            <UtilisationChart samples={metrics.samples} />
          </div>
          <div className="col-span-4">
            <HealthPanel snapshot={snapshot.data} latest={metrics.latest} />
          </div>
        </div>

        <div className="grid grid-cols-12 gap-4">
          <div className="col-span-7">
            <HardwareCard
              snapshot={snapshot.data}
              error={snapshot.error}
              onRetry={snapshot.refetch}
            />
          </div>
          <div className="col-span-5 flex flex-col gap-4">
            <Recommendations />
            <RecentActivity />
          </div>
        </div>
      </div>
    </>
  );
}
