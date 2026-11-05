import { useState } from "react";
import { AppWindow, Cpu, Gamepad2, HatGlasses, Layers, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { PresetCard } from "@/components/common/preset-card";
import { Button } from "@/components/ui/button";
import { refreshEngineState } from "@/hooks/use-system";
import { restartNote } from "@/lib/engine-text";
import { presetProgress, presetStatus, TWEAK_PRESETS, type TweakPreset } from "@/lib/tweak-presets";
import { backend, toBackendError } from "@/services/backend";
import type { TweakStatus } from "@/types/engine";

const ICONS: Record<string, LucideIcon> = {
  general: Layers,
  gaming: Gamepad2,
  privacy: HatGlasses,
  hardware: Cpu,
  apps: AppWindow,
};

/** Presets select for the normal review, and undo restores in bulk. */
export function PresetsTab({
  statuses,
  select,
  disabled,
}: {
  statuses: TweakStatus[];
  select: (ids: string[]) => void;
  disabled: boolean;
}) {
  return (
    <div className="grid grid-cols-2 gap-3">
      {TWEAK_PRESETS.map((preset) => (
        <Preset
          key={preset.id}
          preset={preset}
          statuses={statuses}
          select={select}
          disabled={disabled}
        />
      ))}
    </div>
  );
}

function Preset({
  preset,
  statuses,
  select,
  disabled,
}: {
  preset: TweakPreset;
  statuses: TweakStatus[];
  select: (ids: string[]) => void;
  disabled: boolean;
}) {
  const [confirming, setConfirming] = useState(false);
  const [busy, setBusy] = useState(false);
  const { tweaks, open, undoable } = presetProgress(preset, statuses);
  const inEffect = tweaks.filter((t) => t.state === "applied").length;
  const restarts = tweaks.filter((t) => t.meta.restart !== "none").length;

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
        icon={ICONS[preset.id] ?? Layers}
        title={preset.name}
        summary={preset.summary}
        status={presetStatus(inEffect, tweaks.length)}
        facts={[
          { label: "Applied", value: `${inEffect} of ${tweaks.length}` },
          { label: "Need a restart or sign-out", value: restarts },
        ]}
      >
        {open.length > 0 && (
          <Button disabled={disabled} onClick={() => select(open.map((t) => t.meta.id))}>
            Select {open.length}
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
