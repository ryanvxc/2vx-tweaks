import { EngineBanner } from "@/components/common/engine-banner";
import { PageHeader } from "@/components/layout/page-header";
import { ErrorState } from "@/components/common/states";
import { UpdateNotice } from "@/components/common/update-notice";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useSystemSnapshot } from "@/hooks/use-system";
import { HardwareCard } from "./hardware-card";
import { HealthPanel } from "./health-panel";
import { HomeStats, RecommendedTweaks } from "./home-panels";
import { RecentActivity } from "./recent-activity";
import { UtilisationChart } from "./utilisation-chart";

/** Summaries only, each linking to its primary location. The chart takes the spare height. */
export function DashboardPage() {
  const snapshot = useSystemSnapshot();
  const metrics = useLiveMetrics();
  const os = snapshot.data?.os;

  return (
    <>
      <PageHeader
        title="Welcome back"
        description={
          snapshot.data
            ? `${snapshot.data.hostname ?? "This PC"} · ${os?.productName}${os?.displayVersion ? ` ${os.displayVersion}` : ""} · build ${os?.build}${os?.ubr != null ? `.${os.ubr}` : ""}`
            : undefined
        }
      />

      <div className="flex flex-1 flex-col gap-3">
        <EngineBanner previewNotice={false} />
        <UpdateNotice />
        <HomeStats />

        {metrics.error && metrics.samples.length === 0 && (
          <ErrorState title="Live metrics unavailable" message={metrics.error.message} />
        )}

        <div className="grid flex-1 grid-cols-12 gap-3">
          <div className="col-span-8">
            <UtilisationChart samples={metrics.samples} snapshot={snapshot.data} />
          </div>
          <div className="col-span-4">
            <HealthPanel snapshot={snapshot.data} latest={metrics.latest} />
          </div>
        </div>

        {/* A row, not a grid: Recommended drops out when there's nothing to suggest. */}
        <div className="flex gap-3 *:min-w-0 *:flex-1">
          <RecommendedTweaks />
          <HardwareCard
            snapshot={snapshot.data}
            error={snapshot.error}
            onRetry={snapshot.refetch}
          />
          <RecentActivity />
        </div>
      </div>
    </>
  );
}
