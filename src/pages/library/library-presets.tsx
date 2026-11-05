import { useState } from "react";
import { Link } from "react-router";
import {
  AppWindow,
  Cpu,
  Gamepad2,
  History,
  HatGlasses,
  Layers,
  type LucideIcon,
} from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { OptimisationRow } from "@/components/common/optimisation-card";
import { Button } from "@/components/ui/button";
import { refreshEngineState } from "@/hooks/use-system";
import { LIBRARY_PRESETS, presetProgress, type LibraryPreset } from "@/lib/library-presets";
import { backend, toBackendError } from "@/services/backend";
import type { TweakStatus } from "@/types/engine";

const ICONS: Record<string, LucideIcon> = {
  general: Layers,
  gaming: Gamepad2,
  privacy: HatGlasses,
  hardware: Cpu,
  apps: AppWindow,
};

const rowIcon = (Icon: LucideIcon) => (
  <Icon className="size-[22px] text-brand" strokeWidth={1.75} aria-hidden />
);

/** The Library's first tab: presets select for the normal review, and undo restores in bulk. */
export function LibraryPresets({
  statuses,
  select,
  disabled,
}: {
  statuses: TweakStatus[];
  select: (ids: string[]) => void;
  disabled: boolean;
}) {
  return (
    <div className="flex flex-col gap-2">
      {LIBRARY_PRESETS.map((preset) => (
        <PresetRow
          key={preset.id}
          preset={preset}
          statuses={statuses}
          select={select}
          disabled={disabled}
        />
      ))}
      <OptimisationRow
        leading={rowIcon(History)}
        title="History"
        description="Every change 2VX made, with undo"
        control={
          <Button asChild size="sm" variant="outline">
            <Link to="/restore">Restore Centre</Link>
          </Button>
        }
      />
    </div>
  );
}

function PresetRow({
  preset,
  statuses,
  select,
  disabled,
}: {
  preset: LibraryPreset;
  statuses: TweakStatus[];
  select: (ids: string[]) => void;
  disabled: boolean;
}) {
  const [confirming, setConfirming] = useState(false);
  const [busy, setBusy] = useState(false);
  const { tweaks, open, undoable } = presetProgress(preset, statuses);

  const undo = async () => {
    setBusy(true);
    try {
      const outcome = await backend.revertTweaks(undoable.map((t) => t.meta.id));
      if (outcome.error) toast.error(outcome.error);
      else toast.success(`${preset.name} restored`);
    } catch (err) {
      toast.error(toBackendError(err).message);
    } finally {
      refreshEngineState();
      setBusy(false);
    }
  };

  return (
    <>
      <OptimisationRow
        leading={rowIcon(ICONS[preset.id] ?? Layers)}
        title={preset.name}
        description={preset.summary}
        indicators={
          <span className="text-[13px] text-muted-foreground tabular">
            {tweaks.length - open.length} of {tweaks.length} in effect
          </span>
        }
        control={
          <div className="flex items-center gap-2">
            {undoable.length > 0 && (
              <Button
                size="sm"
                variant="ghost"
                disabled={disabled || busy}
                onClick={() => setConfirming(true)}
              >
                Undo {undoable.length}
              </Button>
            )}
            <Button
              size="sm"
              disabled={disabled || open.length === 0}
              onClick={() => select(open.map((t) => t.meta.id))}
            >
              Select {open.length}
            </Button>
          </div>
        }
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
