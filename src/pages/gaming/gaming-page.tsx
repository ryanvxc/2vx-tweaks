import { useState } from "react";
import { Link } from "react-router";
import { AlertTriangle, ArrowRight, Check, type LucideIcon } from "lucide-react";
import { pageDescription } from "@/app/navigation";
import { EngineBanner } from "@/components/common/engine-banner";
import { StatusPill } from "@/components/common/notice";
import { PlannedFeatures } from "@/components/common/planned-features";
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
  const [chosen, setChosen] = useState<string | null>(null);
  const selected = GAMING_PROFILES.find((p) => p.id === chosen) ?? active ?? GAMING_PROFILES[0]!;

  return (
    <>
      <PageHeader title="Gaming" description={pageDescription("/gaming")} />
      <div className="flex flex-col gap-4">
        <EngineBanner />

        {error ? (
          <ErrorState message={error.message} onRetry={refetch} />
        ) : loading && !data ? (
          <LoadingRows rows={5} />
        ) : (
          <>
            <div role="radiogroup" aria-label="Profiles" className="grid grid-cols-3 gap-3">
              {GAMING_PROFILES.map((p) => (
                <ProfileCard
                  key={p.id}
                  profile={p}
                  statuses={statuses}
                  selected={p.id === selected.id}
                  active={p.id === active?.id}
                  onSelect={() => setChosen(p.id)}
                />
              ))}
            </div>
            {/* Keyed so leaving a tweak out of one profile doesn't carry into the next. */}
            <ProfilePanel key={selected.id} profile={selected} statuses={statuses} />
          </>
        )}

        <section className="flex flex-col gap-2.5">
          <h2 className="text-section-title">Gaming settings</h2>
          <TweakGrid categories={["gaming"]} />
          <p className="text-[12.5px] text-muted-foreground">
            Xbox services are under{" "}
            <Link to="/tweaks?tab=services" className="text-brand hover:underline">
              Tweaks › Services
            </Link>
            , because switching them off breaks Game Pass.
          </p>
        </section>

        <PlannedFeatures
          features={[
            "Windows Game Mode, Game Bar and capture settings",
            "Hardware-accelerated GPU scheduling (where supported)",
            "Per-game profiles applied on launch, reverted on exit",
            "Before/after frame-time measurement, not promised FPS",
          ]}
        />
      </div>
    </>
  );
}

function ProfileCard({
  profile,
  statuses,
  selected,
  active,
  onSelect,
}: {
  profile: Profile;
  statuses: TweakStatus[];
  selected: boolean;
  active: boolean;
  onSelect: () => void;
}) {
  const { usable, inEffect } = profileProgress(profile, statuses);
  return (
    <button
      type="button"
      role="radio"
      aria-checked={selected}
      onClick={onSelect}
      className={cn(
        "flex flex-col gap-1.5 rounded-xl border bg-card px-4 py-3.5 text-left transition-colors outline-none hover:bg-raised focus-visible:ring-2 focus-visible:ring-ring",
        selected && "border-primary/50 bg-raised",
      )}
    >
      <span className="flex items-center justify-between gap-2">
        <span className="text-card-title">{profile.name}</span>
        {active ? (
          <StatusPill tone="success">Active</StatusPill>
        ) : (
          <span className="text-[12px] text-muted-foreground tabular">
            {inEffect} of {usable.length} on
          </span>
        )}
      </span>
      <span className="text-[13px] leading-snug text-muted-foreground">{profile.summary}</span>
    </button>
  );
}

/** What the selected profile changes. Anything can be left out before the review. */
function ProfilePanel({ profile, statuses }: { profile: Profile; statuses: TweakStatus[] }) {
  const engine = useEngineStatus();
  const [left, setLeft] = useState<string[]>([]);
  const [reviewing, setReviewing] = useState(false);
  const { tweaks } = profileProgress(profile, statuses);
  const open = tweaks.filter(
    (t) => t.state !== "applied" && !isBlocked(t.compatibility) && !left.includes(t.meta.id),
  );
  const names = Object.fromEntries(statuses.map((t) => [t.meta.id, t.meta.name]));
  const warnings = Object.fromEntries(statuses.map((t) => [t.meta.id, t.meta.warning]));
  const changesDisabled = isDesktop && engine.data?.journalAvailable === false;
  const restarts = open.some((t) => t.meta.restart !== "none");

  return (
    <div className="overflow-hidden rounded-xl border bg-card">
      <div className="flex items-center justify-between gap-4 border-b px-4 py-3">
        <h2 className="text-section-title">What {profile.name} changes</h2>
        <div className="flex items-center gap-3">
          {restarts && (
            <span className="text-[12.5px] text-muted-foreground">Some need a restart</span>
          )}
          <Button
            size="sm"
            disabled={open.length === 0 || changesDisabled}
            onClick={() => setReviewing(true)}
          >
            {open.length === 0
              ? "Nothing to change"
              : `Review ${open.length} change${open.length === 1 ? "" : "s"}`}
            {open.length > 0 && <ArrowRight />}
          </Button>
        </div>
      </div>
      <ul className="grid grid-cols-2 gap-x-6 px-4 py-1.5">
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
      <ChangeInspector
        open={reviewing}
        onOpenChange={setReviewing}
        tweakIds={open.map((t) => t.meta.id)}
        names={names}
        warnings={warnings}
      />
    </div>
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
  const restart = RESTART_HINT[meta.restart];

  return (
    <li className={cn("flex items-start gap-3 py-2.5", blocked && "opacity-60")}>
      {applied ? (
        <Check className="mt-0.5 size-4 shrink-0 text-success" aria-label="In effect" />
      ) : (
        <input
          type="checkbox"
          aria-label={`Include ${meta.name}`}
          className="mt-0.5 size-4 shrink-0 accent-primary disabled:opacity-40"
          checked={included && !blocked}
          disabled={blocked}
          onChange={onToggle}
        />
      )}
      <Icon className="mt-0.5 size-4 shrink-0 text-brand" aria-hidden />
      <div className="min-w-0 flex-1">
        <div className="text-[13px] leading-snug font-medium">{meta.name}</div>
        <div className="text-[12.5px] leading-snug text-muted-foreground">
          {blocked
            ? COMPATIBILITY[compatibility]
            : applied
              ? "In effect"
              : (restart ?? meta.description)}
        </div>
        {meta.warning && !blocked && (
          <div className="mt-0.5 flex items-start gap-1.5 text-[12.5px] leading-snug">
            <AlertTriangle className="mt-px size-3.5 shrink-0 text-destructive" />
            {meta.warning}
          </div>
        )}
      </div>
    </li>
  );
}
