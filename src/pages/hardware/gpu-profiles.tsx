import { useState } from "react";
import { ArrowRight, Gpu, Laptop, Monitor, MonitorCog, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { PresetCard } from "@/components/common/preset-card";
import { Button } from "@/components/ui/button";
import { refreshEngineState, useEngineStatus, useTweakStatuses } from "@/hooks/use-system";
import { restartNote } from "@/lib/engine-text";
import {
  GPU_PROFILES,
  POWER_PROFILES,
  presetFits,
  presetProgress,
  presetStatus,
  type TweakPreset,
} from "@/lib/tweak-presets";
import { backend, isDesktop, toBackendError } from "@/services/backend";
import type { TweakStatus } from "@/types/engine";
import { ChangeInspector } from "../tweaks/change-inspector";

const ICONS: Record<string, LucideIcon> = {
  "gpu-nvidia": Gpu,
  "gpu-windows": MonitorCog,
  "power-desktop": Monitor,
  "power-laptop": Laptop,
};

export const GpuProfiles = () => <ProfileCards presets={GPU_PROFILES} />;
export const PowerProfiles = () => <ProfileCards presets={POWER_PROFILES} />;

/** Named lists of existing tweaks, applied through the normal review. */
function ProfileCards({ presets }: { presets: TweakPreset[] }) {
  const { data } = useTweakStatuses();
  const engine = useEngineStatus();
  if (!data) return null;
  const disabled = isDesktop && engine.data?.journalAvailable === false;
  const cards = presets.filter((p) => presetFits(p, data));
  if (cards.length === 0) return null;
  return (
    <div className="grid grid-cols-2 gap-3">
      {cards.map((preset) => (
        <Profile key={preset.id} preset={preset} statuses={data} disabled={disabled} />
      ))}
    </div>
  );
}

function Profile({
  preset,
  statuses,
  disabled,
}: {
  preset: TweakPreset;
  statuses: TweakStatus[];
  disabled: boolean;
}) {
  const [reviewing, setReviewing] = useState(false);
  const [confirming, setConfirming] = useState(false);
  const [busy, setBusy] = useState(false);
  const { tweaks, open, undoable } = presetProgress(preset, statuses);
  const inEffect = tweaks.length - open.length;

  const undo = async () => {
    setBusy(true);
    try {
      const outcome = await backend.revertTweaks(undoable.map((t) => t.meta.id));
      if (outcome.error) toast.error(outcome.error);
      else {
        toast.success(`${preset.name} restored`, {
          description: restartNote(outcome.restart) ?? undefined,
        });
      }
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      refreshEngineState();
      setBusy(false);
    }
  };

  return (
    <>
      <PresetCard
        icon={ICONS[preset.id] ?? Gpu}
        title={preset.name}
        summary={preset.summary}
        status={presetStatus(inEffect, tweaks.length)}
        facts={[{ label: "Applied", value: `${inEffect} of ${tweaks.length}` }]}
      >
        {open.length > 0 && (
          <Button disabled={disabled} onClick={() => setReviewing(true)}>
            Apply <ArrowRight />
          </Button>
        )}
        {undoable.length > 0 && (
          <Button
            variant="secondary"
            disabled={disabled || busy}
            onClick={() => setConfirming(true)}
          >
            Undo {undoable.length}
          </Button>
        )}
      </PresetCard>

      <ChangeInspector
        open={reviewing}
        onOpenChange={setReviewing}
        tweakIds={open.map((t) => t.meta.id)}
        names={Object.fromEntries(tweaks.map((t) => [t.meta.id, t.meta.name]))}
        warnings={Object.fromEntries(tweaks.map((t) => [t.meta.id, t.meta.warning]))}
      />
      {confirming && (
        <ConfirmDialog
          open
          onOpenChange={(next) => !next && setConfirming(false)}
          title={`Undo ${preset.name}`}
          description={`Restores the recorded values of ${undoable.length} tweak${undoable.length === 1 ? "" : "s"}.`}
          consequences={undoable.map((t) => t.meta.name)}
          risk="low"
          confirmLabel="Undo"
          onConfirm={() => {
            setConfirming(false);
            void undo();
          }}
        />
      )}
    </>
  );
}
