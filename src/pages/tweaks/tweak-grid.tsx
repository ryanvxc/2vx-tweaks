import { createElement, useEffect, useMemo, useRef, useState } from "react";
import { useSearchParams } from "react-router";
import {
  AlertTriangle,
  Check,
  CircleDashed,
  LogOut,
  PackageOpen,
  SearchX,
  Sparkles,
  X,
} from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { DataSourceBadge } from "@/components/common/data-source-badge";
import { FilterBar, type FilterGroup } from "@/components/common/filter-bar";
import { ImpactMeter, Indicator } from "@/components/common/indicator";
import {
  CardGrid,
  NewBadge,
  OptimisationCard,
  OptimisationRow,
} from "@/components/common/optimisation-card";
import { PageTabs } from "@/components/common/page-tabs";
import { RecommendedAction } from "@/components/common/recommended-action";
import { EmptyState, ErrorState } from "@/components/common/states";
import { PageHero } from "@/components/layout/page-hero";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { Switch } from "@/components/ui/switch";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import {
  useAppInfo,
  useEngineStatus,
  useTweakStatuses,
  refreshEngineState,
} from "@/hooks/use-system";
import { useStaggerIn } from "@/hooks/use-motion";
import { useTabParam } from "@/hooks/use-tab-param";
import {
  COMPATIBILITY,
  describeError,
  describeOutcome,
  isBlocked,
  RESTART_HINT,
} from "@/lib/engine-text";
import {
  groupTweaks,
  isNewTweak,
  tweakIcon,
  tweakPlacement,
  type PlacedTab,
} from "@/lib/tweak-presentation";
import { play } from "@/lib/motion";
import { backend, isDesktop, toBackendError } from "@/services/backend";
import type { Impact, RiskLevel, TweakCategory, TweakStatus } from "@/types/engine";
import { ChangeInspector } from "./change-inspector";
import { TweakDetails } from "./tweak-details";
import type { RowStyle, TweakView } from "./tweak-views";

type StateFilter = "applied" | "notApplied";
type PermissionFilter = "administrator" | "user";
type Sort = "recommended" | "name" | "impact" | "newest";

const IMPACT_ORDER: Record<Impact, number> = { low: 0, medium: 1, high: 2 };
const NO_TABS: { value: string }[] = [];

/**
 * Tweaks as a card grid. The switch shows the state detected on this PC, never a stored
 * flag. Switching on selects a tweak for review (the dry-run plan in ChangeInspector);
 * switching off undoes exactly what 2VX recorded, after a confirmation.
 *
 * A `view` makes it a whole page: title, tabs, search and filters over the tweaks placed on that
 * page. A search looks through every tab of the page, because its point is not having to know the
 * tab. Without one it's a plain grid of `categories`.
 */
export function TweakGrid({
  view,
  categories,
}: {
  view?: TweakView;
  categories?: TweakCategory[];
}) {
  const { data, error, loading, refetch } = useTweakStatuses();
  const engine = useEngineStatus();
  const app = useAppInfo();
  const [params, setParams] = useSearchParams();

  const [tab, setTab] = useTabParam(view?.tabs ?? NO_TABS, view?.defaultTab ?? "");
  const [search, setSearch] = useState("");
  const [stateFilter, setStateFilter] = useState<StateFilter[]>([]);
  const [impactFilter, setImpactFilter] = useState<Impact[]>([]);
  const [riskFilter, setRiskFilter] = useState<RiskLevel[]>([]);
  const [permFilter, setPermFilter] = useState<PermissionFilter[]>([]);
  const [sort, setSort] = useState<Sort>("recommended");

  const [staged, setStaged] = useState<string[]>([]);
  const [reviewing, setReviewing] = useState(false);
  const [undoing, setUndoing] = useState<TweakStatus | null>(null);
  const [busy, setBusy] = useState<string | null>(null);
  const root = useStaggerIn<HTMLDivElement>(data ? tab : null);

  const route = view?.route;
  const all = useMemo(
    () =>
      (data ?? []).filter((t) =>
        route ? tweakPlacement(t.meta).route === route : categories?.includes(t.meta.category),
      ),
    [data, route, categories],
  );
  const names = Object.fromEntries((data ?? []).map((t) => [t.meta.id, t.meta.name]));
  const warnings = Object.fromEntries((data ?? []).map((t) => [t.meta.id, t.meta.warning]));
  // In preview the journal is absent by design; planning is refused with its own message.
  const changesDisabled = isDesktop && engine.data?.journalAvailable === false;
  const detailsId = params.get("tweak");
  const details = all.find((t) => t.meta.id === detailsId) ?? null;

  const q = search.trim().toLowerCase();
  const shown = all
    .filter((t) => !view || q !== "" || tweakPlacement(t.meta).tab === tab)
    .filter(
      (t) =>
        !q || t.meta.name.toLowerCase().includes(q) || t.meta.description.toLowerCase().includes(q),
    )
    .filter(
      (t) =>
        stateFilter.length === 0 ||
        stateFilter.includes(t.state === "applied" ? "applied" : "notApplied"),
    )
    .filter((t) => impactFilter.length === 0 || impactFilter.includes(t.meta.impact))
    .filter((t) => riskFilter.length === 0 || riskFilter.includes(t.meta.risk))
    .filter((t) => permFilter.length === 0 || permFilter.includes(t.meta.permission))
    .sort((a, b) => {
      switch (sort) {
        case "name":
          return a.meta.name.localeCompare(b.meta.name);
        case "impact":
          return IMPACT_ORDER[b.meta.impact] - IMPACT_ORDER[a.meta.impact];
        case "newest":
          return b.meta.addedIn.localeCompare(a.meta.addedIn, undefined, { numeric: true });
        default:
          return Number(b.meta.recommended) - Number(a.meta.recommended);
      }
    });

  const filters: FilterGroup[] = [
    {
      id: "state",
      label: "State on this PC",
      options: [
        { value: "applied", label: "In effect" },
        { value: "notApplied", label: "Not in effect" },
      ],
      selected: stateFilter,
      onChange: (v) => setStateFilter(v as StateFilter[]),
    },
    {
      id: "impact",
      label: "Expected effect",
      options: [
        { value: "low", label: "Small" },
        { value: "medium", label: "Noticeable" },
        { value: "high", label: "Large" },
      ],
      selected: impactFilter,
      onChange: (v) => setImpactFilter(v as Impact[]),
    },
    {
      id: "risk",
      label: "Risk",
      options: [
        { value: "low", label: "Low" },
        { value: "moderate", label: "Moderate" },
        { value: "high", label: "High" },
      ],
      selected: riskFilter,
      onChange: (v) => setRiskFilter(v as RiskLevel[]),
    },
    {
      id: "permission",
      label: "Permission",
      options: [
        { value: "administrator", label: "Needs administrator" },
        { value: "user", label: "Your account only" },
      ],
      selected: permFilter,
      onChange: (v) => setPermFilter(v as PermissionFilter[]),
    },
  ];

  const stageable = (t: TweakStatus) => t.state !== "applied" && !isBlocked(t.compatibility);
  const recommendedOpen = all.filter((t) => t.meta.recommended && stageable(t));

  const onSwitch = (t: TweakStatus, on: boolean) => {
    const id = t.meta.id;
    if (on) setStaged((s) => (s.includes(id) ? s : [...s, id]));
    else if (staged.includes(id)) setStaged((s) => s.filter((x) => x !== id));
    else setUndoing(t);
  };

  const undo = (t: TweakStatus) => {
    setBusy(t.meta.id);
    backend
      .revertTweak(t.meta.id)
      .then(
        (outcome) => {
          const m = describeOutcome(outcome, "revert");
          (m.tone === "success" ? toast.success : toast.warning)(m.title, {
            description: m.description,
          });
        },
        (err: unknown) => {
          const m = describeError(toBackendError(err));
          toast.error(m.title, { description: m.description });
        },
      )
      .finally(() => {
        setBusy(null);
        refreshEngineState();
      });
  };

  const openDetails = (id: string | null) =>
    setParams(
      (p) => {
        const next = new URLSearchParams(p);
        if (id) next.set("tweak", id);
        else next.delete("tweak");
        return next;
      },
      { replace: true },
    );

  // When most of the catalogue is new the badge is on every card and says nothing.
  const newCount = (data ?? []).filter((t) => isNewTweak(t.meta, app.data?.version)).length;
  const markNew = newCount * 2 < (data?.length ?? 0);

  const card = (t: TweakStatus) => (
    <TweakCard
      key={t.meta.id}
      tweak={t}
      staged={staged.includes(t.meta.id)}
      busy={busy === t.meta.id}
      disabled={changesDisabled}
      isNew={markNew && isNewTweak(t.meta, app.data?.version)}
      onSwitch={(on) => onSwitch(t, on)}
      row={q === "" ? view?.rows?.[tab] : undefined}
      onOpen={() => openDetails(t.meta.id)}
    />
  );

  const body = view?.body?.(tab);
  const groups = groupTweaks(body ? [] : shown, {
    tab: view && q === "" && !body ? (tab as PlacedTab) : null,
    // Sections keep their written order unless the user asked for another.
    catalogue: sort === "recommended",
    unavailable: (t) => isBlocked(t.compatibility),
  });
  const headings = view?.headings === "always" || groups.length > 1;

  return (
    <div
      ref={root}
      className="flex flex-1 flex-col gap-3 font-nav [--font-heading:var(--font-nav)]"
    >
      {view && (
        <>
          <PageHero
            title={view.title}
            subtitle={view.subtitle}
            action={
              <RecommendedAction
                otherwise={{ to: "/gaming", label: "Open Game Mode" }}
                disabled={changesDisabled}
                onSelect={
                  recommendedOpen.length > 0
                    ? () =>
                        setStaged((s) => [
                          ...new Set([...s, ...recommendedOpen.map((t) => t.meta.id)]),
                        ])
                    : undefined
                }
              />
            }
          />
          <PageTabs label="Topics" tabs={view.tabs} value={tab} onChange={setTab} filled />
          {view.banner}
          {!body && (
            <FilterBar
              search={search}
              onSearch={setSearch}
              placeholder="Search tweaks"
              filters={filters}
              sort={{
                value: sort,
                onChange: (v) => setSort(v as Sort),
                options: [
                  { value: "recommended", label: "Recommended first" },
                  { value: "name", label: "Name" },
                  { value: "impact", label: "Expected effect" },
                  { value: "newest", label: "Newest" },
                ],
              }}
              end={view.end ? view.end(tab) : <DataSourceBadge source="live" />}
            />
          )}
          {q === "" && !body && view.intro?.(tab)}
        </>
      )}

      {body ? (
        body
      ) : error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <CardGrid>
          {Array.from({ length: 6 }, (_, i) => (
            <Skeleton key={i} className="h-[168px] rounded-xl" />
          ))}
        </CardGrid>
      ) : all.length === 0 ? (
        <EmptyState icon={PackageOpen} title="Nothing here yet" className="flex-1 panel" />
      ) : shown.length === 0 ? (
        <EmptyState icon={SearchX} title="No tweaks match" className="flex-1 panel" />
      ) : (
        <div className="flex flex-col gap-6">
          {groups.map((group) => (
            <section key={group.title ?? "all"} className="flex flex-col gap-3">
              {group.title && headings && (
                <h2 className="text-[19px] leading-tight">{group.title}</h2>
              )}
              {q === "" && view?.rows?.[tab] ? (
                <div className="flex flex-col gap-2">{group.tweaks.map(card)}</div>
              ) : (
                // A catalogue keeps its column width; a short fixed list spreads across the row.
                <CardGrid
                  className={view ? undefined : "grid-cols-[repeat(auto-fit,minmax(19rem,1fr))]"}
                >
                  {group.tweaks.map(card)}
                </CardGrid>
              )}
            </section>
          ))}
        </div>
      )}

      {staged.length > 0 && (
        <div className="sticky bottom-3 z-10 mx-auto mt-auto flex animate-sheet-up items-center gap-3 rounded-xl border border-primary/30 bg-popover/95 py-2 pr-2 pl-4 shadow-2xl shadow-black/50 backdrop-blur">
          <Sparkles className="size-4 text-primary" />
          <span className="text-[13px] text-muted-foreground">
            <b
              key={staged.length}
              className="inline-block animate-pop font-semibold text-foreground tabular"
            >
              {staged.length}
            </b>{" "}
            selected
          </span>
          <Button size="sm" variant="ghost" onClick={() => setStaged([])}>
            <X /> Clear
          </Button>
          <Button size="sm" disabled={changesDisabled} onClick={() => setReviewing(true)}>
            Review {staged.length} change{staged.length === 1 ? "" : "s"}
          </Button>
        </div>
      )}

      <ChangeInspector
        open={reviewing}
        onOpenChange={(open) => {
          setReviewing(open);
          if (!open) setStaged([]);
        }}
        tweakIds={staged}
        names={names}
        warnings={warnings}
      />
      <TweakDetails
        tweak={details}
        onOpenChange={(open) => !open && openDetails(null)}
        staged={details ? staged.includes(details.meta.id) : false}
        onStage={(on) => details && onSwitch(details, on)}
        disabled={changesDisabled}
      />
      {undoing && (
        <ConfirmDialog
          open
          onOpenChange={(open) => !open && setUndoing(null)}
          title={`Undo: ${undoing.meta.name}`}
          description="Restores the recorded values."
          consequences={undoing.settings.map((s) => s.targetLabel)}
          risk={undoing.meta.risk}
          confirmLabel="Undo"
          onConfirm={() => {
            const t = undoing;
            setUndoing(null);
            undo(t);
          }}
        />
      )}
    </div>
  );
}

function TweakCard({
  tweak,
  staged,
  busy,
  disabled,
  isNew,
  row,
  onSwitch,
  onOpen,
}: {
  tweak: TweakStatus;
  staged: boolean;
  busy: boolean;
  disabled: boolean;
  isNew: boolean;
  /** Draw it as a row instead of a card. */
  row?: RowStyle;
  onSwitch: (on: boolean) => void;
  onOpen: () => void;
}) {
  const { meta, state, compatibility } = tweak;
  const blocked = isBlocked(compatibility);
  const applied = state === "applied";
  // Applied by something other than 2VX: there is no recorded before-value to restore.
  const lockedOn = applied && !tweak.undoable;
  const restart = RESTART_HINT[meta.restart];

  // The switch alone confirms an apply, so the card marks the moment it takes.
  const card = useRef<HTMLDivElement>(null);
  const wasApplied = useRef(applied);
  useEffect(() => {
    if (applied && !wasApplied.current) {
      play(
        card.current,
        [
          {
            borderColor: "var(--success)",
            backgroundColor: "color-mix(in oklch, var(--success) 14%, var(--card))",
          },
          {},
        ],
        { duration: 1100 },
      );
    }
    wasApplied.current = applied;
  }, [applied]);

  const switchControl = (
    <Switch
      size="lg"
      checked={applied || staged}
      disabled={disabled || blocked || lockedOn || busy}
      onCheckedChange={onSwitch}
      aria-label={`${applied ? "Undo" : staged ? "Deselect" : "Turn on"} ${meta.name}`}
      className={staged ? "ring-2 ring-primary/30 ring-offset-2 ring-offset-card" : undefined}
    />
  );

  const indicators = (
    <>
      <ImpactMeter impact={meta.impact} />
      {(meta.warning || restart || blocked || state === "partial") && (
        <span aria-hidden className="mx-0.5 h-4 w-px bg-hairline" />
      )}
      {meta.warning && (
        <Indicator boxed icon={AlertTriangle} tone="destructive" title="Can break features">
          {meta.warning}
        </Indicator>
      )}
      {restart && <Indicator boxed icon={LogOut} tone="neutral" title={restart} />}
      {blocked && (
        <Indicator boxed icon={AlertTriangle} tone="neutral" title="Not for this PC">
          {COMPATIBILITY[compatibility]}
        </Indicator>
      )}
      {state === "partial" && (
        <Indicator boxed icon={CircleDashed} tone="warning" title="Partly in effect" />
      )}
    </>
  );
  const control = lockedOn ? (
    <Tooltip>
      <TooltipTrigger asChild>
        <span tabIndex={0}>{switchControl}</span>
      </TooltipTrigger>
      <TooltipContent>Set outside 2VX, so there's nothing to undo.</TooltipContent>
    </Tooltip>
  ) : (
    switchControl
  );

  if (row) {
    return (
      <OptimisationRow
        ref={card}
        leading={
          row === "letter" ? (
            <span className="flex size-9 items-center justify-center rounded-lg bg-brand/12 text-[15px] font-semibold text-brand">
              {meta.name.charAt(0)}
            </span>
          ) : applied ? (
            <Check className="size-5 text-success" strokeWidth={2.5} aria-label="In effect" />
          ) : (
            createElement(tweakIcon(meta), {
              className: "size-[22px] text-brand",
              strokeWidth: 1.75,
              "aria-hidden": true,
            })
          )
        }
        title={meta.name}
        description={row === "letter" ? undefined : meta.description}
        highlighted={staged}
        muted={blocked}
        onOpen={onOpen}
        openLabel={`Details for ${meta.name}`}
        indicators={indicators}
        control={control}
      />
    );
  }

  return (
    <OptimisationCard
      ref={card}
      icon={tweakIcon(meta)}
      title={meta.name}
      description={meta.description}
      highlighted={staged}
      muted={blocked}
      onOpen={onOpen}
      openLabel={`Details for ${meta.name}`}
      badge={isNew ? <NewBadge /> : undefined}
      indicators={indicators}
      control={control}
    />
  );
}
