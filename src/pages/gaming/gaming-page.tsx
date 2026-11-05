import { useSearchParams } from "react-router";
import { EngineBanner } from "@/components/common/engine-banner";
import { RecommendedAction } from "@/components/common/recommended-action";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { PageHero } from "@/components/layout/page-hero";
import { useTweakStatuses } from "@/hooks/use-system";
import { GameModeConfigure } from "./game-mode-configure";
import { GameModeMain } from "./game-mode-main";

export function GamingPage() {
  const { data, error, refetch } = useTweakStatuses();
  const [params, setParams] = useSearchParams();
  const configuring = params.get("view") === "configure";

  return (
    <div className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]">
      <PageHero
        title="2VX Game Mode"
        subtitle="Put your game first while you play."
        action={<RecommendedAction />}
      />
      <EngineBanner />
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : !data ? (
        <LoadingRows rows={5} />
      ) : configuring ? (
        <GameModeConfigure statuses={data} onClose={() => setParams({})} />
      ) : (
        <GameModeMain statuses={data} onConfigure={() => setParams({ view: "configure" })} />
      )}
    </div>
  );
}
