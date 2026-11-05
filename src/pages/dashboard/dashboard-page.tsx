import { useState } from "react";
import { EngineBanner } from "@/components/common/engine-banner";
import { RecommendedAction } from "@/components/common/recommended-action";
import { ErrorState } from "@/components/common/states";
import { UpdateNotice } from "@/components/common/update-notice";
import { PageHero } from "@/components/layout/page-hero";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useStaggerIn } from "@/hooks/use-motion";
import { useAccount, useAppInfo, useSystemSnapshot } from "@/hooks/use-system";
import { evaluateHealth } from "@/lib/health";
import { HealthDialog } from "./health-panel";
import { HomeCards } from "./home-panels";
import { Quickstart } from "./quickstart";
import { UpdatesSection } from "./updates-section";
import { UtilisationChart } from "./utilisation-chart";

/**
 * Every card opens the page that owns its feature, or the health list that has no page of its own. The page is taller than the window on purpose:
 * Updates sits below the fold and shows just enough at the bottom to say there's more.
 */
export function DashboardPage() {
  const snapshot = useSystemSnapshot();
  const metrics = useLiveMetrics();
  const account = useAccount();
  const app = useAppInfo();
  const [healthOpen, setHealthOpen] = useState(false);
  const page = useStaggerIn<HTMLDivElement>("home");
  const s = snapshot.data;
  const os = s?.os;
  const name = account.data?.email.split("@")[0];
  const checks = s ? evaluateHealth(s, metrics.latest) : [];

  return (
    <div
      ref={page}
      className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]"
    >
      <PageHero
        title={`Welcome back${name ? `, ${name}` : ""}!`}
        subtitle={
          s
            ? `${s.hostname ?? "This PC"} · ${os?.productName}${os?.displayVersion ? ` ${os.displayVersion}` : ""} · build ${os?.build}${os?.ubr != null ? `.${os.ubr}` : ""}${app.data ? ` · ${app.data.isElevated ? "Administrator" : "Standard user"}` : ""}`
            : " "
        }
        action={<RecommendedAction />}
      />

      <EngineBanner previewNotice={false} />
      <UpdateNotice />
      <HomeCards checks={checks} onOpenHealth={() => setHealthOpen(true)} />

      {metrics.error && metrics.samples.length === 0 && (
        <ErrorState title="Live metrics unavailable" message={metrics.error.message} />
      )}

      {/* Short enough that Updates shows at the bottom. Quickstart scrolls inside this height. */}
      <div className="grid h-84 shrink-0 grid-cols-3 gap-3">
        <div data-stagger className="col-span-2 min-h-0">
          <UtilisationChart samples={metrics.samples} snapshot={s} />
        </div>
        <Quickstart />
      </div>

      <UpdatesSection />
      <HealthDialog open={healthOpen} onOpenChange={setHealthOpen} checks={checks} />
    </div>
  );
}
