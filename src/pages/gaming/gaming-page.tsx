import { useState } from "react";
import { AlertTriangle, ArrowRight, Check, LogOut, type LucideIcon } from "lucide-react";
import { EngineBanner } from "@/components/common/engine-banner";
import { Indicator } from "@/components/common/indicator";
import { StatusLabel } from "@/components/common/notice";
import { ErrorState, LoadingRows } from "@/components/common/states";
import { PageHeader } from "@/components/layout/page-header";
import { Button } from "@/components/ui/button";
import { useEngineStatus, useTweakStatuses } from "@/hooks/use-system";
import { COMPATIBILITY, isBlocked, RESTART_HINT } from "@/lib/engine-text";
import { activeProfile, GAMING_PROFILES, profileProgress, type Profile } from "@/lib/profiles";
import { tweakIcon } from "@/lib/tweak-presentation";
import { cn } from "@/lib/utils";
import { isDesktop } from "@/services/backend";
import type { TweakStatus } from "@/types/engine";
import { ChangeInspector } from "../tweaks/change-inspector";
import { TweakGrid } from "../tweaks/tweak-grid";

export function GamingPage() {
  const { data, error, loading, refetch } = useTweakStatuses();
  const statuses = data ?? [];
  const active = activeProfile(statuses);

  return (
    <>
      <PageHeader title="Game Mode" />
      <div className="flex flex-1 flex-col gap-3">
        <EngineBanner />

        {error ? (
          <ErrorState message={error.message} onRetry={refetch} />
        ) : loading && !data ? (
          <LoadingRows rows={5} />
        ) : (
          <div className="grid flex-1 grid-cols-3 gap-3">
            {GAMING_PROFILES.map((p) => (
              <ProfileColumn
                key={p.id}
                profile={p}
                statuses={statuses}
                active={p.id === active?.id}
              />
            ))}
          </div>
        )}

        <section className="flex flex-col gap-2">
          <h2 className="px-1 text-section-title">Gaming settings</h2>
          <TweakGrid categories={["gaming"]} />
        </section>
      </div>
    </>
  );
}

/** One profile and what it changes. Anything can be left out before the review. */
function ProfileColumn({
  profile,
  statuses,
  active,
}: {
  profile: Profile;
  statuses: TweakStatus[];
  active: boolean;
}) {
  const engine = useEngineStatus();
  const [left, setLeft] = useState<string[]>([]);
  const [reviewing, setReviewing] = useState(false);
  const { tweaks, usable, inEffect } = profileProgress(profile, statuses);
  const open = tweaks.filter(
    (t) => t.state !== "applied" && !isBlocked(t.compatibility) && !left.includes(t.meta.id),
  );
  const names = Object.fromEntries(statuses.map((t) => [t.meta.id, t.meta.name]));
  const warnings = Object.fromEntries(statuses.map((t) => [t.meta.id, t.meta.warning]));
  const changesDisabled = isDesktop && engine.data?.journalAvailable === false;

  return (
    <section className="flex flex-col overflow-hidden panel">
      <div className="border-b px-4 py-2.5">
        <div className="flex items-baseline justify-between gap-2">
          <h2 className="truncate text-section-title">{profile.name}</h2>
          {active ? (
            <StatusLabel tone="success">Active</StatusLabel>
          ) : (
            <span className="text-[12px] whitespace-nowrap text-muted-foreground tabular">
              {inEffect} of {usable.length} on
            </span>
          )}
        </div>
        <p className="truncate text-[12.5px] text-muted-foreground">{profile.summary}</p>
      </div>
      <ul className="flex-1 px-4 py-1">
        {tweaks.map((t) => (
          <ProfileRow
            key={t.meta.id}
            tweak={t}
            icon={tweakIcon(t.meta)}
            included={!left.includes(t.meta.id)}
            onToggle={() =>
              setLeft((l) =>
                l.includes(t.meta.id) ? l.filter((x) => x !== t.meta.id) : [...l, t.meta.id],
              )
            }
          />
        ))}
      </ul>
      <div className="p-3 pt-1">
        <Button
          size="sm"
          variant="secondary"
          className="w-full"
          disabled={open.length === 0 || changesDisabled}
          aria-label={`Review ${profile.name}`}
          onClick={() => setReviewing(true)}
        >
          {open.length === 0
            ? "Nothing to change"
            : `Review ${open.length} change${open.length === 1 ? "" : "s"}`}
          {open.length > 0 && <ArrowRight />}
        </Button>
      </div>
      <ChangeInspector
        open={reviewing}
        onOpenChange={setReviewing}
        tweakIds={open.map((t) => t.meta.id)}
        names={names}
        warnings={warnings}
      />
    </section>
  );
}

function ProfileRow({
  tweak,
  icon: Icon,
  included,
  onToggle,
}: {
  tweak: TweakStatus;
  icon: LucideIcon;
  included: boolean;
  onToggle: () => void;
}) {
  const { meta, state, compatibility } = tweak;
  const applied = state === "applied";
  const blocked = isBlocked(compatibility);
  const restart = applied ? null : RESTART_HINT[meta.restart];

  return (
    <li className={cn("flex h-9 items-center gap-2.5", blocked && "opacity-60")}>
      {applied ? (
        <Check className="size-4 shrink-0 text-success" aria-label="In effect" />
      ) : (
        <input
          type="checkbox"
          aria-label={`Include ${meta.name}`}
          className="size-4 shrink-0 accent-primary disabled:opacity-40"
          checked={included && !blocked}
          disabled={blocked}
          onChange={onToggle}
        />
      )}
      <Icon className="size-4 shrink-0 text-brand" aria-hidden />
      <div className="min-w-0 flex-1 truncate text-[13px] font-medium" title={meta.name}>
        {meta.name}
      </div>
      {blocked && (
        <Indicator icon={AlertTriangle} tone="neutral" title="Not for this PC">
          {COMPATIBILITY[compatibility]}
        </Indicator>
      )}
      {restart && !blocked && <Indicator icon={LogOut} tone="neutral" title={restart} />}
      {meta.warning && !blocked && (
        <Indicator icon={AlertTriangle} tone="destructive" title="Can break features">
          {meta.warning}
        </Indicator>
      )}
    </li>
  );
}
