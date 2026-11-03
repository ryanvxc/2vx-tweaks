import { EngineBanner } from "@/components/common/engine-banner";
import { PageHeader } from "@/components/layout/page-header";
import { ErrorState } from "@/components/common/states";
import { UpdateNotice } from "@/components/common/update-notice";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useSystemSnapshot } from "@/hooks/use-system";
import { useUiStore } from "@/stores/ui-store";
import { HardwareCard } from "./hardware-card";
import { HealthPanel } from "./health-panel";
import { HomeStats, Quickstart, RecommendedTweaks } from "./home-panels";
import { RecentActivity } from "./recent-activity";
import { UtilisationChart } from "./utilisation-chart";

/** Summaries only, each linking to its primary location. Sized to fit the default window. */
export function DashboardPage() {
  const snapshot = useSystemSnapshot();
  const metrics = useLiveMetrics();
  const showQuickstart = useUiStore((s) => s.showQuickstart);
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

      <div className="flex flex-col gap-3">
        <EngineBanner previewNotice={false} />
        <UpdateNotice />
        <HomeStats />

        {metrics.error && metrics.samples.length === 0 && (
          <ErrorState title="Live metrics unavailable" message={metrics.error.message} />
        )}

        <div className="grid grid-cols-12 gap-3">
          <div className="col-span-8">
            <UtilisationChart samples={metrics.samples} snapshot={snapshot.data} />
          </div>
          <div className="col-span-4">
            <HealthPanel snapshot={snapshot.data} latest={metrics.latest} />
          </div>
        </div>

        <div className="grid grid-cols-12 gap-3">
          <div className="col-span-4">
            <RecommendedTweaks />
          </div>
          <div className="col-span-4">
            <HardwareCard
              snapshot={snapshot.data}
              error={snapshot.error}
              onRetry={snapshot.refetch}
            />
          </div>
          <div className="col-span-4">
            <RecentActivity />
          </div>
        </div>

        {showQuickstart && <Quickstart />}
      </div>
    </>
  );
}
