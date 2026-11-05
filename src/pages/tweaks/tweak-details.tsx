import { createElement, useState } from "react";
import { AlertTriangle, ArrowRight, ExternalLink } from "lucide-react";
import { StatusLabel } from "@/components/common/notice";
import { RiskBadge } from "@/components/common/risk-badge";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { COMPATIBILITY, EVIDENCE, isBlocked, RESTART_HINT, TWEAK_STATE } from "@/lib/engine-text";
import { CATEGORY_LABEL, tweakIcon } from "@/lib/tweak-presentation";
import type { Impact, TweakStatus } from "@/types/engine";

const IMPACT_LABEL: Record<Impact, string> = {
  low: "Small effect (estimate)",
  medium: "Noticeable effect (estimate)",
  high: "Large effect (estimate)",
};

/** The full explanation of one tweak: what changes, exact values, sources. */
export function TweakDetails({
  tweak,
  onOpenChange,
  staged,
  onStage,
  disabled,
}: {
  tweak: TweakStatus | null;
  onOpenChange: (open: boolean) => void;
  staged: boolean;
  onStage: (on: boolean) => void;
  disabled: boolean;
}) {
  // Closing clears `tweak`; the last one stays so the dialog has content to fade out with.
  const [last, setLast] = useState(tweak);
  if (tweak && tweak !== last) setLast(tweak);
  const shown = tweak ?? last;
  if (!shown) return null;
  const { meta, state, compatibility } = shown;
  const stateInfo = TWEAK_STATE[state];
  // The administrator pill below already says this.
  const note = compatibility === "needsElevation" ? null : COMPATIBILITY[compatibility];
  const restart = RESTART_HINT[meta.restart];
  const blocked = isBlocked(compatibility);
  const onPowerPlan = shown.settings.some((s) => s.target.hive === "powerPlan");

  return (
    <Dialog open={tweak !== null} onOpenChange={onOpenChange}>
      <DialogContent className="max-h-[calc(100vh-4rem)] grid-rows-[auto_minmax(0,1fr)_auto] sm:max-w-xl">
        <DialogHeader>
          <div className="flex items-center gap-2.5">
            <div className="flex size-9 items-center justify-center rounded-lg bg-brand/12 text-brand">
              {createElement(tweakIcon(meta), { className: "size-[18px]" })}
            </div>
            <div className="min-w-0">
              <DialogTitle className="text-[15px]">{meta.name}</DialogTitle>
              <div className="text-[12px] text-muted-foreground">
                {CATEGORY_LABEL[meta.category]}
              </div>
            </div>
          </div>
          <DialogDescription className="pt-1">{meta.description}</DialogDescription>
        </DialogHeader>

        <div className="-mx-6 flex min-h-0 flex-col gap-4 overflow-y-auto px-6">
          <div className="flex flex-wrap gap-1.5">
            <StatusLabel tone={stateInfo.tone}>{stateInfo.label}</StatusLabel>
            <RiskBadge risk={meta.risk} />
            <StatusLabel tone="neutral">{IMPACT_LABEL[meta.impact]}</StatusLabel>
            {meta.permission === "administrator" && <StatusLabel tone="info">Admin</StatusLabel>}
            {restart && <StatusLabel tone="neutral">{restart}</StatusLabel>}
            {meta.recommended && <StatusLabel tone="success">Recommended</StatusLabel>}
            {meta.section && (
              <StatusLabel tone="neutral">{EVIDENCE[meta.evidence].label}</StatusLabel>
            )}
          </div>
          {note && <p className="text-[12.5px] text-muted-foreground">{note}</p>}
          {meta.section && meta.evidence !== "documented" && (
            <p className="text-[12.5px] text-muted-foreground">{EVIDENCE[meta.evidence].note}</p>
          )}
          {onPowerPlan && (
            <p className="text-[12.5px] text-muted-foreground">
              Changes the active power plan. Undo restores the value on whichever plan is active
              then.
            </p>
          )}
          {meta.warning && (
            <p className="flex items-start gap-2 rounded-lg border border-destructive/30 bg-destructive/8 px-3 py-2 text-[12.5px] leading-snug">
              <AlertTriangle className="mt-px size-3.5 shrink-0 text-destructive" />
              {meta.warning}
            </p>
          )}

          {meta.details !== meta.description && (
            <p data-selectable className="text-[13px] leading-relaxed text-muted-foreground">
              {meta.details}
            </p>
          )}

          <section>
            <h3 className="mb-1.5 text-[12.5px] font-semibold">Values on this PC</h3>
            <ul className="divide-y rounded-lg border">
              {shown.settings.map((s) => (
                <li key={s.targetLabel} className="flex flex-col gap-1 px-3 py-2">
                  <code data-selectable className="font-mono text-[11.5px] break-all">
                    {s.targetLabel}
                  </code>
                  <div className="flex items-center gap-2 text-[12px]">
                    <span className="text-muted-foreground">Now {s.currentLabel}</span>
                    <ArrowRight className="size-3.5 text-muted-foreground" />
                    <span>Wants {s.desiredLabel}</span>
                  </div>
                  {s.error && <div className="text-[12px] text-warning">{s.error}</div>}
                </li>
              ))}
            </ul>
          </section>

          <section>
            <h3 className="mb-1.5 text-[12.5px] font-semibold">Source</h3>
            <ul className="flex flex-col gap-1">
              {meta.references.map((r) => (
                <li key={r} className="flex items-start gap-1.5">
                  <ExternalLink className="mt-0.5 size-3 shrink-0 text-muted-foreground" />
                  <code data-selectable className="font-mono text-[11px] break-all">
                    {r}
                  </code>
                </li>
              ))}
            </ul>
          </section>

          {state === "applied" && !shown.undoable && (
            <p className="text-[12.5px] text-muted-foreground">
              Set outside 2VX, so there's nothing to undo.
            </p>
          )}
        </div>

        <DialogFooter>
          <Button variant="outline" onClick={() => onOpenChange(false)}>
            Close
          </Button>
          {state !== "applied" && !blocked && (
            <Button disabled={disabled} onClick={() => onStage(!staged)}>
              {staged ? "Deselect" : "Select for review"}
            </Button>
          )}
          {state === "applied" && shown.undoable && (
            <Button
              variant="outline"
              disabled={disabled}
              onClick={() => {
                onOpenChange(false);
                onStage(false);
              }}
            >
              Undo…
            </Button>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
