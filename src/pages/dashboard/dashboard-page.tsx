import { EngineBanner } from "@/components/common/engine-banner";
import { Notice } from "@/components/common/notice";
import { ErrorState } from "@/components/common/states";
import { UpdateNotice } from "@/components/common/update-notice";
import { useLiveMetrics } from "@/hooks/use-live-metrics";
import { useStaggerIn } from "@/hooks/use-motion";
import { useAccount, useSystemSnapshot } from "@/hooks/use-system";
import { evaluateHealth } from "@/lib/health";
import { HomeAction, HomeCards } from "./home-panels";
import { Quickstart } from "./quickstart";
import { UpdatesSection } from "./updates-section";
import { UtilisationChart } from "./utilisation-chart";

/**
 * Every card opens the page that owns its feature. The page is taller than the window on purpose:
 * Updates sits below the fold and shows just enough at the bottom to say there's more.
 */
export function DashboardPage() {
  const snapshot = useSystemSnapshot();
  const metrics = useLiveMetrics();
  const account = useAccount();
  const page = useStaggerIn<HTMLDivElement>("home");
  const s = snapshot.data;
  const os = s?.os;
  const name = account.data?.email.split("@")[0];
  // Below the fold now, so anything wrong comes up here instead of waiting to be found.
  const problems = s ? evaluateHealth(s, metrics.latest).filter((c) => c.status !== "good") : [];

  return (
    <div
      ref={page}
      className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]"
    >
      <div className="flex items-center justify-between gap-6 pb-1">
        <div className="min-w-0">
          <h1 className="truncate text-[1.75rem] leading-tight font-semibold tracking-tight">
            Welcome back{name ? `, ${name}` : ""}!
          </h1>
          <p data-selectable className="truncate text-[14px] text-muted-foreground">
            {s
              ? `${s.hostname ?? "This PC"} · ${os?.productName}${os?.displayVersion ? ` ${os.displayVersion}` : ""} · build ${os?.build}${os?.ubr != null ? `.${os.ubr}` : ""}`
              : " "}
          </p>
        </div>
        <HomeAction />
      </div>

      <EngineBanner previewNotice={false} />
      <UpdateNotice />
      {problems.map((c) => (
        <Notice key={c.id} tone="warning" title={c.label}>
          {c.detail}
        </Notice>
      ))}
      <HomeCards />

      {metrics.error && metrics.samples.length === 0 && (
        <ErrorState title="Live metrics unavailable" message={metrics.error.message} />
      )}

      {/* Stops short of the window's height at the default size, so Updates shows at the bottom. */}
      <div className="grid h-[clamp(20rem,calc(100vh-20rem),34rem)] shrink-0 grid-cols-3 gap-3">
        <div data-stagger className="col-span-2 min-h-0">
          <UtilisationChart samples={metrics.samples} snapshot={s} />
        </div>
        <Quickstart />
      </div>

      <UpdatesSection snapshot={s} latest={metrics.latest} />
    </div>
  );
}
