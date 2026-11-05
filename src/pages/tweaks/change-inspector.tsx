import { useEffect, useRef, useState } from "react";
import { AlertTriangle, ArrowRight, Loader2, ShieldCheck } from "lucide-react";
import { announceRestart } from "@/stores/restart-store";
import { MessageNotice, Notice, StatusLabel } from "@/components/common/notice";
import { RiskBadge } from "@/components/common/risk-badge";
import { LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { refreshEngineState } from "@/hooks/use-system";
import {
  CHANGE_STATUS,
  COMPATIBILITY,
  describeError,
  describeOutcome,
  restartNote,
} from "@/lib/engine-text";
import { backend, toBackendError } from "@/services/backend";
import type { OperationOutcome, Plan, SkippedTweak } from "@/types/engine";
import type { BackendError } from "@/types/system";

// Just longer than the dialog's exit in components/ui/dialog.tsx, so the loading rows never show as it leaves.
const EXIT_MS = 180;

type Step =
  | { kind: "planning" }
  | { kind: "planned"; plan: Plan }
  | { kind: "applying"; plan: Plan }
  | { kind: "done"; outcome: OperationOutcome }
  | { kind: "error"; error: BackendError; replan: boolean };

/**
 * The confirmation step for applying tweaks. It shows the engine's dry-run plan (every
 * value with its current and new data) and applies exactly that plan, nothing else.
 */
export function ChangeInspector({
  open,
  onOpenChange,
  tweakIds,
  names,
  warnings = {},
  onApplied,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  tweakIds: string[];
  /** Tweak ID → display name. */
  names: Record<string, string>;
  /** Tweak ID → the features it can break. */
  warnings?: Record<string, string | null>;
  /** After a clean apply, with the batch it made. */
  onApplied?: (snapshotId: number) => void;
}) {
  const [step, setStep] = useState<Step>({ kind: "planning" });
  const [acknowledged, setAcknowledged] = useState(false);
  // Ignores results that arrive after the dialog closed or re-planned.
  const run = useRef(0);

  // Only sets state from the promise, so it can run from the effect below.
  const requestPlan = (ids: string[]) => {
    const id = ++run.current;
    backend.planTweaks(ids).then(
      (plan) => id === run.current && setStep({ kind: "planned", plan }),
      (err: unknown) =>
        id === run.current && setStep({ kind: "error", error: toBackendError(err), replan: false }),
    );
  };

  const replan = () => {
    setStep({ kind: "planning" });
    setAcknowledged(false);
    requestPlan(tweakIds);
  };

  const close = () => {
    const id = ++run.current;
    setAcknowledged(false);
    onOpenChange(false);
    // After the dialog has faded out, so it doesn't flash the loading rows on the way.
    setTimeout(() => id === run.current && setStep({ kind: "planning" }), EXIT_MS);
  };

  // Plans once per opening; the selection is fixed while the dialog is open.
  useEffect(() => {
    if (open) requestPlan(tweakIds);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [open]);

  const apply = (plan: Plan) => {
    const id = ++run.current;
    setStep({ kind: "applying", plan });
    backend.applyPlan(plan.id).then(
      (outcome) => {
        refreshEngineState();
        if (id !== run.current) return;
        const { snapshotId } = outcome;
        if (outcome.status === "completed" && snapshotId != null) {
          onApplied?.(snapshotId);
          announceRestart(outcome.restart);
          // The switch already shows a clean apply. It can't show what was left out.
          if (!outcome.skipped?.length) {
            close();
            return;
          }
        }
        // Anything else has to be read, so it stays open.
        setStep({ kind: "done", outcome });
      },
      (err: unknown) => {
        const error = toBackendError(err);
        refreshEngineState();
        // A conflict means the plan is stale: offer a fresh review instead of a retry.
        if (id === run.current)
          setStep({ kind: "error", error, replan: error.code === "conflict" });
      },
    );
  };

  const busy = step.kind === "planning" || step.kind === "applying";

  return (
    <Dialog open={open} onOpenChange={(o) => !o && !busy && close()}>
      <DialogContent className="max-h-[calc(100vh-4rem)] grid-rows-[auto_minmax(0,1fr)_auto] sm:max-w-2xl">
        <DialogHeader>
          <DialogTitle>Review changes</DialogTitle>
          <DialogDescription className="sr-only">The values that will change.</DialogDescription>
        </DialogHeader>

        <div className="-mx-6 min-h-0 overflow-y-auto px-6">
          {step.kind === "planning" && <LoadingRows rows={4} />}
          {(step.kind === "planned" || step.kind === "applying") && (
            <PlanView plan={step.plan} names={names} warnings={warnings} />
          )}
          {step.kind === "done" && <OutcomeView outcome={step.outcome} names={names} />}
          {step.kind === "error" && <MessageNotice {...describeError(step.error)} />}
        </div>

        <DialogFooter className="items-center">
          {(step.kind === "planned" || step.kind === "applying") &&
            step.plan.highestRisk === "high" && (
              <label className="mr-auto flex cursor-pointer items-center gap-2 text-[12.5px]">
                <input
                  type="checkbox"
                  className="size-4 accent-primary"
                  checked={acknowledged}
                  onChange={(e) => setAcknowledged(e.target.checked)}
                />
                I understand this may disrupt my system.
              </label>
            )}
          {step.kind === "done" || step.kind === "error" ? (
            <>
              {step.kind === "error" && step.replan && (
                <Button variant="outline" onClick={replan}>
                  Review again
                </Button>
              )}
              <Button onClick={close}>Close</Button>
            </>
          ) : (
            <>
              <Button variant="outline" disabled={busy} onClick={close}>
                Cancel
              </Button>
              {step.kind !== "planning" && (
                <Button
                  disabled={
                    step.kind === "applying" ||
                    step.plan.changes.length === 0 ||
                    (step.plan.highestRisk === "high" && !acknowledged)
                  }
                  onClick={() => apply(step.plan)}
                >
                  {step.kind === "applying" ? (
                    <Loader2 className="animate-spin" />
                  ) : (
                    step.plan.needsElevation && <ShieldCheck />
                  )}
                  {applyLabel(step.plan)}
                </Button>
              )}
            </>
          )}
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}

function applyLabel(plan: Plan) {
  const n = plan.changes.length;
  if (n === 0) return "Nothing to apply";
  return `Apply ${n} change${n === 1 ? "" : "s"}`;
}

/** What wasn't applied and why. Near the top, where a long list of changes can't bury it. */
function SkippedList({
  skipped,
  names,
}: {
  skipped: SkippedTweak[];
  names: Record<string, string>;
}) {
  const n = skipped.length;
  if (n === 0) return null;
  return (
    <Notice tone="warning" title={`Skipped ${n} ${n === 1 ? "tweak" : "tweaks"}`}>
      <ul className="flex max-h-36 flex-col gap-1 overflow-y-auto pr-1">
        {skipped.map((s) => (
          <li key={s.tweakId}>
            <span className="font-medium text-foreground">{names[s.tweakId] ?? s.tweakId}</span>
            {": "}
            {s.detail ?? COMPATIBILITY[s.reason] ?? s.reason}
          </li>
        ))}
      </ul>
    </Notice>
  );
}

function PlanView({
  plan,
  names,
  warnings,
}: {
  plan: Plan;
  names: Record<string, string>;
  warnings: Record<string, string | null>;
}) {
  const restart = restartNote(plan.restart);
  return (
    <div className="flex flex-col gap-3">
      {plan.changes.length === 0 && (
        <Notice
          tone="neutral"
          title={plan.skipped.length > 0 ? "Nothing can be applied" : "Nothing to change"}
        />
      )}
      <SkippedList skipped={plan.skipped} names={names} />
      {plan.changes.length > 0 && (
        <>
          <div className="flex flex-wrap items-center gap-2 text-[12.5px] text-muted-foreground">
            {plan.highestRisk && <RiskBadge risk={plan.highestRisk} />}
            {restart && <StatusLabel tone="info">{restart}</StatusLabel>}
          </div>
          {plan.needsElevation && (
            <Notice tone="info" title="Windows will ask for administrator permission once" />
          )}
          {plan.tweakIds.map((id) => (
            <section key={id} className="rounded-md border">
              <h3 className="border-b px-3 py-2 text-[13px] font-medium">{names[id] ?? id}</h3>
              {warnings[id] && (
                <p className="flex items-start gap-2 border-b px-3 py-2 text-[12.5px] leading-snug">
                  <AlertTriangle className="mt-px size-3.5 shrink-0 text-destructive" />
                  {warnings[id]}
                </p>
              )}
              <ul className="divide-y">
                {plan.changes
                  .filter((c) => c.tweakId === id)
                  .map((c) => (
                    <li key={c.targetLabel} className="flex flex-col gap-1 px-3 py-2">
                      <code data-selectable className="font-mono text-[11.5px] break-all">
                        {c.targetLabel}
                      </code>
                      <div className="flex items-center gap-2 text-[12px]">
                        <span className="text-muted-foreground">{c.fromLabel}</span>
                        <ArrowRight className="size-3.5 text-muted-foreground" />
                        <span className="font-medium">{c.toLabel}</span>
                      </div>
                    </li>
                  ))}
              </ul>
            </section>
          ))}
        </>
      )}
      {plan.alreadyApplied.length > 0 && (
        <p className="text-[12.5px] text-muted-foreground">
          Already in place: {plan.alreadyApplied.map((id) => names[id] ?? id).join(", ")}.
        </p>
      )}
    </div>
  );
}

function OutcomeView({
  outcome,
  names,
}: {
  outcome: OperationOutcome;
  names: Record<string, string>;
}) {
  return (
    <div className="flex flex-col gap-3">
      <MessageNotice {...describeOutcome(outcome, "apply")} />
      <SkippedList skipped={outcome.skipped ?? []} names={names} />
      {outcome.changes.length > 0 && (
        <ul className="divide-y rounded-md border">
          {outcome.changes.map((c) => (
            <li key={c.id} className="flex items-start gap-3 px-3 py-2">
              <div className="min-w-0 flex-1">
                <div className="text-[12px] text-muted-foreground">
                  {names[c.tweakId] ?? c.tweakId}
                </div>
                <code data-selectable className="font-mono text-[11.5px] break-all">
                  {c.targetLabel}
                </code>
                <div className="text-[12px]">
                  {c.beforeLabel} → {c.afterLabel}
                </div>
                {c.error && <div className="text-[12px] text-destructive">{c.error}</div>}
              </div>
              <StatusLabel tone={CHANGE_STATUS[c.status].tone}>
                {CHANGE_STATUS[c.status].label}
              </StatusLabel>
            </li>
          ))}
        </ul>
      )}
    </div>
  );
}
