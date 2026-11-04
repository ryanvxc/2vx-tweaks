import { useState } from "react";
import { AlertTriangle, ArrowRight, ChevronDown, PackageOpen, ShieldCheck } from "lucide-react";
import { StatusLabel } from "@/components/common/notice";
import { RiskBadge } from "@/components/common/risk-badge";
import { SectionCard } from "@/components/common/section-card";
import { EmptyState, ErrorState, LoadingRows } from "@/components/common/states";
import { Button } from "@/components/ui/button";
import { useEngineStatus, useTweakStatuses } from "@/hooks/use-system";
import { COMPATIBILITY, isBlocked, RESTART_HINT, TWEAK_STATE } from "@/lib/engine-text";
import { cn } from "@/lib/utils";
import { isDesktop } from "@/services/backend";
import type { TweakCategory, TweakStatus } from "@/types/engine";
import { ChangeInspector } from "./change-inspector";

/**
 * Tweaks for the given categories with their state as read from this PC right now. The
 * state comes from detection, never from a stored "enabled" flag.
 */
export function TweakList({ categories }: { categories: TweakCategory[] }) {
  const { data, error, loading, refetch } = useTweakStatuses();
  const engine = useEngineStatus();
  const [selected, setSelected] = useState<string[]>([]);
  const [reviewing, setReviewing] = useState(false);

  const tweaks = (data ?? []).filter((t) => categories.includes(t.meta.category));
  const names = Object.fromEntries((data ?? []).map((t) => [t.meta.id, t.meta.name]));
  // In preview the journal is absent by design; planning is refused with its own message.
  const changesDisabled = isDesktop && engine.data?.journalAvailable === false;

  const toggle = (id: string) =>
    setSelected((s) => (s.includes(id) ? s.filter((x) => x !== id) : [...s, id]));

  return (
    <SectionCard
      title="Available optimisations"
      description="State read from this PC"
      source="live"
      actions={
        tweaks.length > 0 && (
          <Button
            size="xs"
            disabled={selected.length === 0 || changesDisabled}
            onClick={() => setReviewing(true)}
          >
            Review {selected.length || ""} change{selected.length === 1 ? "" : "s"}
          </Button>
        )
      }
    >
      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <LoadingRows />
      ) : tweaks.length === 0 ? (
        <EmptyState
          icon={PackageOpen}
          title="No optimisations in this section yet"
          description="Each one is added only after its effect, risk and undo path are documented and tested."
        />
      ) : (
        <ul className="-mx-2 flex flex-col">
          {tweaks.map((t) => (
            <TweakRow
              key={t.meta.id}
              tweak={t}
              selected={selected.includes(t.meta.id)}
              onToggle={() => toggle(t.meta.id)}
            />
          ))}
        </ul>
      )}
      <ChangeInspector
        open={reviewing}
        onOpenChange={(open) => {
          setReviewing(open);
          if (!open) setSelected([]);
        }}
        tweakIds={selected}
        names={names}
      />
    </SectionCard>
  );
}

function TweakRow({
  tweak,
  selected,
  onToggle,
}: {
  tweak: TweakStatus;
  selected: boolean;
  onToggle: () => void;
}) {
  const [expanded, setExpanded] = useState(false);
  const { meta, state, compatibility } = tweak;
  const blocked = isBlocked(compatibility);
  // Already-applied tweaks are undone from History, where the exact before-values live.
  const selectable = !blocked && state !== "applied";
  const stateInfo = TWEAK_STATE[state];
  const note = COMPATIBILITY[compatibility];
  const restart = RESTART_HINT[meta.restart];
  const detailsId = `tweak-details-${meta.id}`;

  return (
    <li className={cn("rounded-md px-2 py-2.5", selected && "bg-accent/50")}>
      <div className="flex items-start gap-3">
        <input
          type="checkbox"
          aria-label={`Select ${meta.name}`}
          className="mt-0.5 size-4 shrink-0 accent-primary disabled:opacity-40"
          checked={selected}
          disabled={!selectable}
          onChange={onToggle}
        />
        <div className="min-w-0 flex-1">
          <div className="flex flex-wrap items-center gap-2">
            <span className="text-[13px] leading-snug font-medium">{meta.name}</span>
            {meta.permission === "administrator" && (
              <ShieldCheck
                className="size-3.5 text-muted-foreground"
                aria-label="Needs administrator permission"
              />
            )}
          </div>
          <div className="text-[12px] leading-snug text-muted-foreground">{meta.description}</div>
          {meta.warning && (
            <div className="mt-0.5 flex items-start gap-1.5 text-[12px] leading-snug text-destructive">
              <AlertTriangle className="mt-px size-3.5 shrink-0" />
              {meta.warning}
            </div>
          )}
          {(note || restart) && (
            <div className="mt-1 text-[11.5px] text-muted-foreground">
              {[note, restart].filter(Boolean).join(" · ")}
            </div>
          )}
        </div>
        <div className="flex shrink-0 items-center gap-2">
          <StatusLabel tone={stateInfo.tone}>{stateInfo.label}</StatusLabel>
          <RiskBadge risk={meta.risk} />
          <Button
            size="icon"
            variant="ghost"
            className="size-6"
            aria-expanded={expanded}
            aria-controls={detailsId}
            aria-label={`${expanded ? "Hide" : "Show"} details for ${meta.name}`}
            onClick={() => setExpanded((e) => !e)}
          >
            <ChevronDown className={cn("transition-transform", expanded && "rotate-180")} />
          </Button>
        </div>
      </div>

      {expanded && (
        <div id={detailsId} className="mt-2.5 ml-7 flex flex-col gap-2.5">
          <p className="text-[12.5px] leading-relaxed text-muted-foreground">{meta.details}</p>
          <ul className="divide-y rounded-md border">
            {tweak.settings.map((s) => (
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
          <div className="text-[12px] text-muted-foreground">
            Source:{" "}
            {meta.references.map((r) => (
              <code key={r} data-selectable className="block font-mono text-[11px] break-all">
                {r}
              </code>
            ))}
          </div>
        </div>
      )}
    </li>
  );
}
