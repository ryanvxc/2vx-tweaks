import { useSearchParams } from "react-router";
import { EngineBanner } from "@/components/common/engine-banner";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { useTweakStatuses } from "@/hooks/use-system";
import { GameModeConfigure } from "./game-mode-configure";
import { GameModeMain } from "./game-mode-main";

export function GamingPage() {
  const { data, error, refetch } = useTweakStatuses();
  const [params, setParams] = useSearchParams();
  const configuring = params.get("view") === "configure";

  return (
    <>
      <PageHeader title="Game Mode" />
      <div className="flex flex-1 flex-col gap-3">
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
    </>
  );
}
