import { useState } from "react";
import { ArrowRight, ChevronRight, Scale, Zap, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { StatusLabel } from "@/components/common/notice";
import { PresetCard } from "@/components/common/preset-card";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useBackendQuery } from "@/hooks/use-backend-query";
import { refreshEngineState, useTweakStatuses } from "@/hooks/use-system";
import { profileProgress } from "@/lib/profiles";
import {
  activePreset,
  SERVICE_PRESETS,
  undoableIn,
  type ServicePreset,
} from "@/lib/service-presets";
import { presetStatus } from "@/lib/tweak-presets";
import { cn } from "@/lib/utils";
import { backend, toBackendError } from "@/services/backend";
import type { TweakStatus } from "@/types/engine";
import type { ServiceStartType } from "@/types/system";
import { ChangeInspector } from "../tweaks/change-inspector";

const ICONS: Record<string, LucideIcon> = { basic: Scale, advanced: Zap };

/** Above the list of services: two named lists of the same tweaks. */
export function ServicePresets() {
  const { data } = useTweakStatuses();
  if (!data) return null;
  return (
    <div className="grid grid-cols-2 gap-3">
      {SERVICE_PRESETS.map((preset) => (
        <Preset key={preset.id} preset={preset} statuses={data} />
      ))}
    </div>
  );
}

function Preset({ preset, statuses }: { preset: ServicePreset; statuses: TweakStatus[] }) {
  const [reviewing, setReviewing] = useState(false);
  const [reverting, setReverting] = useState(false);
  const progress = profileProgress(preset, statuses);
  const active = activePreset(statuses)?.id === preset.id;
  // Advanced holds everything in Basic, so Basic isn't off while Advanced is on.
  const included = !active && progress.active;
  const undoable = undoableIn(progress);
  const canRevert = active || (progress.inEffect > 0 && undoable.length > 0 && !included);

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
    <>
      <PresetCard
        icon={ICONS[preset.id] ?? Scale}
        title={preset.name}
        summary={preset.summary}
        status={presetStatus(progress.inEffect, progress.usable.length)}
        facts={[{ label: "Applied", value: `${progress.inEffect} of ${progress.usable.length}` }]}
      >
        {canRevert ? (
          <Button
            variant="secondary"
            disabled={undoable.length === 0 || reverting}
            title={undoable.length === 0 ? "Changed outside 2VX, so there's no record to undo" : ""}
            onClick={() => void revert()}
          >
            {reverting ? "Reverting…" : "Revert"}
          </Button>
        ) : (
          <Button disabled={included} onClick={() => setReviewing(true)}>
            Apply <ArrowRight />
          </Button>
        )}
      </PresetCard>

      <ChangeInspector
        open={reviewing}
        onOpenChange={setReviewing}
        tweakIds={progress.usable.map((t) => t.meta.id)}
        names={Object.fromEntries(progress.tweaks.map((t) => [t.meta.id, t.meta.name]))}
        warnings={Object.fromEntries(progress.tweaks.map((t) => [t.meta.id, t.meta.warning]))}
      />
    </>
  );
}

const START_TYPE: Record<ServiceStartType, string> = {
  automatic: "Automatic",
  automaticDelayed: "Automatic (delayed)",
  manual: "Manual",
  disabled: "Disabled",
  unknown: "Unknown",
};

/** The services the tweaks below act on, as Windows reports them right now. Read-only. */
export function ServiceReadout() {
  const [open, setOpen] = useState(false);
  const { data, error, loading, refetch } = useBackendQuery(
    "service-states",
    backend.getServiceStates,
  );
  const rows = [...(data ?? [])].sort((a, b) => a.displayName.localeCompare(b.displayName));

  return (
    <section className="flex flex-col gap-2">
      <button
        type="button"
        aria-expanded={open}
        onClick={() => setOpen((o) => !o)}
        className="flex h-11 items-center gap-2.5 panel px-4 text-left text-[14.5px] font-semibold transition-colors outline-none hover:bg-raised focus-visible:ring-2 focus-visible:ring-ring"
      >
        <ChevronRight
          className={cn(
            "size-4 text-muted-foreground transition-transform duration-150",
            open && "rotate-90",
          )}
          aria-hidden
        />
        <span className="flex-1 truncate">Services on this PC</span>
        {data && (
          <span className="text-[12.5px] font-normal text-muted-foreground tabular">
            {rows.length}
          </span>
        )}
      </button>
      {open &&
        (error ? (
          <ErrorState message={error.message} onRetry={refetch} />
        ) : loading && !data ? (
          <LoadingRows rows={4} />
        ) : (
          <div className="max-h-64 overflow-y-auto panel">
            <ul className="flex flex-col">
              {rows.map((s) => (
                <li
                  key={s.name}
                  className="flex h-9 items-center gap-4 px-4 text-[12.5px] not-last:border-b not-last:border-border"
                >
                  <span className="w-56 shrink-0 truncate font-medium" title={s.name}>
                    {s.displayName}
                  </span>
                  <span className="w-20 shrink-0">
                    {s.running === null ? (
                      <StatusLabel tone="neutral">Unknown</StatusLabel>
                    ) : s.running ? (
                      <StatusLabel tone="success">Running</StatusLabel>
                    ) : (
                      <StatusLabel tone="neutral">Stopped</StatusLabel>
                    )}
                  </span>
                  <span className="w-32 shrink-0 truncate text-muted-foreground">
                    {START_TYPE[s.startType]}
                  </span>
                  <span
                    className="min-w-0 flex-1 truncate text-muted-foreground"
                    title={s.description ?? undefined}
                  >
                    {s.description ?? ""}
                  </span>
                </li>
              ))}
            </ul>
          </div>
        ))}
    </section>
  );
}
