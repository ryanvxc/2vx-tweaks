import { useState } from "react";
import { ArrowRight, Scale, Zap, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { EngineBanner } from "@/components/common/engine-banner";
import { Notice, StatusLabel } from "@/components/common/notice";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { refreshEngineState, useTweakStatuses } from "@/hooks/use-system";
import { profileProgress } from "@/lib/profiles";
import {
  activePreset,
  SERVICE_PRESETS,
  undoableIn,
  type ServicePreset,
} from "@/lib/service-presets";
import { tweakIcon } from "@/lib/tweak-presentation";
import { cn } from "@/lib/utils";
import { backend, toBackendError } from "@/services/backend";
import type { TweakStatus } from "@/types/engine";
import { ChangeInspector } from "../tweaks/change-inspector";

const ICONS: Record<string, LucideIcon> = { basic: Scale, advanced: Zap };

function serviceName(tweakName: string): string {
  const name = tweakName.replace(/^Disable /, "");
  return name.charAt(0).toUpperCase() + name.slice(1);
}

export function ServicesTab() {
  const { data, error, loading, refetch } = useTweakStatuses();
  return (
    <div className="flex flex-1 flex-col gap-3">
      <EngineBanner />
      <Notice tone="info" title="Fewer processes doesn't mean a faster PC.">
        These presets only turn off services with a documented reason, and every change can be
        undone.
      </Notice>
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows rows={5} />
      ) : (
        <div className="grid flex-1 grid-cols-2 gap-3">
          {SERVICE_PRESETS.map((preset) => (
            <PresetCard key={preset.id} preset={preset} statuses={data ?? []} />
          ))}
        </div>
      )}
    </div>
  );
}

function PresetCard({ preset, statuses }: { preset: ServicePreset; statuses: TweakStatus[] }) {
  const [reviewing, setReviewing] = useState(false);
  const [reverting, setReverting] = useState(false);
  const Icon = ICONS[preset.id] ?? Scale;
  const progress = profileProgress(preset, statuses);
  const active = activePreset(statuses)?.id === preset.id;
  // Advanced holds everything in Basic, so Basic isn't off while Advanced is on.
  const included = !active && progress.active;
  const undoable = undoableIn(progress);

  const revert = async () => {
    setReverting(true);
    try {
      const outcome = await backend.revertTweaks(undoable);
      if (outcome.error) toast.error(outcome.error);
      else toast.success("Reverted. Restart to finish.");
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      refreshEngineState();
      setReverting(false);
    }
  };

  return (
    <article className="flex flex-col gap-5 panel p-6">
      <header className="flex items-center gap-3">
        <span className="flex size-10 items-center justify-center rounded-lg bg-brand/12 text-brand">
          <Icon className="size-5" aria-hidden />
        </span>
        <h2 className="flex-1 text-[17px] font-semibold">{preset.name}</h2>
        <StatusLabel tone={active || included ? "success" : "neutral"}>
          {active ? "Active" : included ? "Included" : "Inactive"}
        </StatusLabel>
      </header>

      <p data-selectable className="text-[13.5px] leading-relaxed text-muted-foreground">
        {preset.summary}
      </p>

      <ul
        aria-label={`What ${preset.name} turns off`}
        className="grid grid-cols-2 gap-x-4 gap-y-2 text-[12.5px]"
      >
        {progress.tweaks.map((t) => {
          const ItemIcon = tweakIcon(t.meta);
          return (
            <li
              key={t.meta.id}
              className="flex min-w-0 items-center gap-2"
              title={t.meta.warning ?? ""}
            >
              <ItemIcon
                className={cn(
                  "size-3.5 shrink-0",
                  t.state === "applied" ? "text-brand" : "text-muted-foreground",
                )}
                aria-hidden
              />
              <span className="truncate text-muted-foreground">{serviceName(t.meta.name)}</span>
            </li>
          );
        })}
      </ul>

      <dl className="mt-auto flex flex-col gap-3 text-[13px]">
        <div className="flex items-center justify-between">
          <dt className="text-muted-foreground">Functionality</dt>
          <dd
            className="flex gap-1.5"
            role="img"
            aria-label={`${preset.functionality} out of 5`}
            title="Our rating of how much of Windows keeps working"
          >
            {[1, 2, 3, 4, 5].map((n) => (
              <span
                key={n}
                className={cn(
                  "size-2.5 rounded-full",
                  n <= preset.functionality ? "bg-brand" : "bg-muted",
                )}
              />
            ))}
          </dd>
        </div>
        <div className="flex items-center justify-between">
          <dt className="text-muted-foreground">Background services</dt>
          <dd className="font-medium" title="Compared with the other preset. Not measured.">
            {preset.background}
          </dd>
        </div>
        <div className="flex items-center justify-between">
          <dt className="text-muted-foreground">In effect</dt>
          <dd className="flex items-center gap-3 tabular">
            {progress.inEffect} of {progress.usable.length}
            <DataSourceBadge source="live" />
          </dd>
        </div>
      </dl>

      {active || (progress.inEffect > 0 && undoable.length > 0 && !included) ? (
        <Button
          variant="secondary"
          className="h-11"
          disabled={undoable.length === 0 || reverting}
          title={undoable.length === 0 ? "Changed outside 2VX, so there's no record to undo" : ""}
          onClick={() => void revert()}
        >
          {reverting ? "Reverting…" : "Revert"}
        </Button>
      ) : (
        <Button className="h-11" disabled={included} onClick={() => setReviewing(true)}>
          Apply <ArrowRight />
        </Button>
      )}

      <ChangeInspector
        open={reviewing}
        onOpenChange={setReviewing}
        tweakIds={progress.usable.map((t) => t.meta.id)}
        names={Object.fromEntries(progress.tweaks.map((t) => [t.meta.id, t.meta.name]))}
        warnings={Object.fromEntries(progress.tweaks.map((t) => [t.meta.id, t.meta.warning]))}
      />
    </article>
  );
}
