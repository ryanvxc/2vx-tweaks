import { useDeferredValue, useEffect, useMemo, useRef, useState } from "react";
import { useSearchParams } from "react-router";
import {
  AlertTriangle,
  Check,
  CircleDashed,
  CircleHelp,
  CircleX,
  Cpu,
  FlaskConical,
  Gpu,
  LogOut,
  PackageOpen,
  RotateCcw,
  SearchX,
  ShieldAlert,
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
  CardSkeletons,
  NewBadge,
  OptimisationCard,
} from "@/components/common/optimisation-card";
import { PageTabs } from "@/components/common/page-tabs";
import { RecommendedAction } from "@/components/common/recommended-action";
import { EmptyState, ErrorState } from "@/components/common/states";
import { PageHero } from "@/components/layout/page-hero";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import {
  useAppInfo,
  useEngineStatus,
  useSnapshots,
  useSystemSnapshot,
  useTweakStatuses,
  refreshEngineState,
} from "@/hooks/use-system";
import { useStaggerIn } from "@/hooks/use-motion";
import { useReveal } from "@/hooks/use-reveal";
import { useTabParam } from "@/hooks/use-tab-param";
import {
  BLOCKED_LABEL,
  COMPATIBILITY,
  describeError,
  describeOutcome,
  EVIDENCE,
  HARDWARE_LABEL,
  isBlocked,
  RESTART_HINT,
} from "@/lib/engine-text";
import { tweakHistory, type TweakHistory } from "@/lib/tweak-history";
import {
  capGroups,
  cardTitle,
  CATEGORY_LABEL,
  groupTweaks,
  isNewTweak,
  searchRank,
  tweakIcon,
  tweakPlacement,
  UNAVAILABLE_SECTION,
  type PlacedTab,
} from "@/lib/tweak-presentation";
import { play } from "@/lib/motion";
import { backend, isDesktop, toBackendError } from "@/services/backend";
import type {
  Evidence,
  HardwareScope,
  Impact,
  RiskLevel,
  TweakCategory,
  TweakState,
  TweakStatus,
} from "@/types/engine";
import { ChangeInspector } from "./change-inspector";
import { TweakDetails } from "./tweak-details";
import type { TweakView } from "./tweak-views";

type StateFilter = "applied" | "notApplied";
type FitFilter = "fits" | "blocked";
type PermissionFilter = "administrator" | "user";
type RestartFilter = "restart" | "none";
type Sort = "recommended" | "name" | "impact" | "status" | "newest" | "recent";

const IMPACT_ORDER: Record<Impact, number> = { low: 0, medium: 1, high: 2 };
const STATE_ORDER: Record<TweakState, number> = {
  applied: 0,
  partial: 1,
  notApplied: 2,
  unknown: 3,
};
/** Cards drawn at a time. A tab can hold hundreds, and each is a switch and several tooltips. */
const CARD_BATCH = 30;
const NO_TABS: { value: string }[] = [];
const NO_HISTORY = new Map<string, TweakHistory>();

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
  const [fitFilter, setFitFilter] = useState<FitFilter[]>([]);
  const [hardwareFilter, setHardwareFilter] = useState<HardwareScope[]>([]);
  const [impactFilter, setImpactFilter] = useState<Impact[]>([]);
  const [riskFilter, setRiskFilter] = useState<RiskLevel[]>([]);
  const [permFilter, setPermFilter] = useState<PermissionFilter[]>([]);
  const [restartFilter, setRestartFilter] = useState<RestartFilter[]>([]);
  const [evidenceFilter, setEvidenceFilter] = useState<Evidence[]>([]);
  const [categoryFilter, setCategoryFilter] = useState<TweakCategory[]>([]);
  const [sort, setSort] = useState<Sort>("recommended");

  const [staged, setStaged] = useState<string[]>([]);
  const [reviewing, setReviewing] = useState(false);
  const [undoing, setUndoing] = useState<TweakStatus | null>(null);
  const [busy, setBusy] = useState<string | null>(null);
  const root = useStaggerIn<HTMLDivElement>(data ? tab : null);

  const snapshots = useSnapshots();
  const uptime = useSystemSnapshot().data?.uptimeSecs ?? null;
  const history = useMemo(
    () => (snapshots.data ? tweakHistory(snapshots.data, uptime) : NO_HISTORY),
    [snapshots.data, uptime],
  );

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

  // Filtering 1,200 tweaks on every key would make typing lag behind.
  const q = useDeferredValue(search).trim().toLowerCase();
  const onTab = useMemo(
    () => (view ? all.filter((t) => tweakPlacement(t.meta).tab === tab) : all),
    [all, view, tab],
  );
  const shown = useMemo(() => {
    const ranks = new Map<string, number>();
    if (q) {
      for (const t of all) {
        const found = searchRank(
          t.meta,
          q,
          t.settings.map((s) => s.targetLabel),
        );
        if (found !== null) ranks.set(t.meta.id, found);
      }
    }
    const rank = (t: TweakStatus) => ranks.get(t.meta.id) ?? 9;
    return (q ? all : onTab)
      .filter((t) => !q || ranks.has(t.meta.id))
      .filter(
        (t) =>
          stateFilter.length === 0 ||
          stateFilter.includes(t.state === "applied" ? "applied" : "notApplied"),
      )
      .filter(
        (t) =>
          fitFilter.length === 0 ||
          fitFilter.includes(isBlocked(t.compatibility) ? "blocked" : "fits"),
      )
      .filter((t) => hardwareFilter.length === 0 || hardwareFilter.includes(t.meta.hardware))
      .filter((t) => impactFilter.length === 0 || impactFilter.includes(t.meta.impact))
      .filter((t) => riskFilter.length === 0 || riskFilter.includes(t.meta.risk))
      .filter((t) => permFilter.length === 0 || permFilter.includes(t.meta.permission))
      .filter(
        (t) =>
          restartFilter.length === 0 ||
          restartFilter.includes(t.meta.restart === "none" ? "none" : "restart"),
      )
      .filter((t) => evidenceFilter.length === 0 || evidenceFilter.includes(t.meta.evidence))
      .filter((t) => !q || categoryFilter.length === 0 || categoryFilter.includes(t.meta.category))
      .sort((a, b) => {
        switch (sort) {
          case "name":
            return a.meta.name.localeCompare(b.meta.name);
          case "impact":
            return IMPACT_ORDER[b.meta.impact] - IMPACT_ORDER[a.meta.impact];
          case "status":
            return STATE_ORDER[a.state] - STATE_ORDER[b.state];
          case "newest":
            return b.meta.addedIn.localeCompare(a.meta.addedIn, undefined, { numeric: true });
          case "recent":
            return (
              (history.get(b.meta.id)?.lastUsedMs ?? 0) - (history.get(a.meta.id)?.lastUsedMs ?? 0)
            );
          default:
            return (
              (q ? rank(a) - rank(b) : 0) || Number(b.meta.recommended) - Number(a.meta.recommended)
            );
        }
      });
  }, [
    all,
    onTab,
    q,
    stateFilter,
    fitFilter,
    hardwareFilter,
    impactFilter,
    riskFilter,
    permFilter,
    restartFilter,
    evidenceFilter,
    categoryFilter,
    sort,
    history,
  ]);
  // A different tab, search, filter or sort is a different list, drawn again from its first batch.
  const listKey = JSON.stringify([
    tab,
    q,
    sort,
    stateFilter,
    fitFilter,
    hardwareFilter,
    impactFilter,
    riskFilter,
    permFilter,
    restartFilter,
    evidenceFilter,
    categoryFilter,
  ]);

  const hardware = [...new Set(all.map((t) => t.meta.hardware))].filter(
    (h): h is Exclude<HardwareScope, "any"> => h !== "any",
  );
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
      id: "fit",
      label: "This PC",
      options: [
        { value: "fits", label: "Works here" },
        { value: "blocked", label: "Unavailable here" },
      ],
      selected: fitFilter,
      onChange: (v) => setFitFilter(v as FitFilter[]),
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
    {
      id: "restart",
      label: "Restart",
      options: [
        { value: "none", label: "No restart" },
        { value: "restart", label: "Needs a restart or sign-out" },
      ],
      selected: restartFilter,
      onChange: (v: string[]) => setRestartFilter(v as RestartFilter[]),
    },
    {
      id: "evidence",
      label: "Evidence",
      options: (Object.keys(EVIDENCE) as Evidence[]).map((value) => ({
        value,
        label: EVIDENCE[value].label,
      })),
      selected: evidenceFilter,
      onChange: (v: string[]) => setEvidenceFilter(v as Evidence[]),
    },
    ...(hardware.length > 0
      ? [
          {
            id: "hardware",
            label: "Hardware",
            options: hardware.map((value) => ({ value, label: HARDWARE_LABEL[value] })),
            selected: hardwareFilter,
            onChange: (v: string[]) => setHardwareFilter(v as HardwareScope[]),
          },
        ]
      : []),
    // Tabs pick the topic until a search looks through all of them.
    ...(q
      ? [
          {
            id: "category",
            label: "Category",
            options: [...new Set(all.map((t) => t.meta.category))].map((value) => ({
              value,
              label: CATEGORY_LABEL[value],
            })),
            selected: categoryFilter,
            onChange: (v: string[]) => setCategoryFilter(v as TweakCategory[]),
          },
        ]
      : []),
  ];

  // An unread setting can't be backed up, so it can't be selected either.
  const stageable = (t: TweakStatus) =>
    t.state !== "applied" && t.state !== "unknown" && !isBlocked(t.compatibility);
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
      history={history.get(t.meta.id)}
      onSwitch={(on) => onSwitch(t, on)}
      onOpen={() => openDetails(t.meta.id)}
    />
  );

  const body =
    q !== ""
      ? undefined
      : view?.body?.(tab, {
          statuses: data ?? [],
          select: (ids) => setStaged((s) => [...new Set([...s, ...ids])]),
          disabled: changesDisabled,
        });
  const groups = groupTweaks(body ? [] : shown, {
    tab: view && q === "" && !body ? (tab as PlacedTab) : null,
    // Sections keep their written order unless the user asked for another.
    catalogue: sort === "recommended",
    unavailable: (t) => isBlocked(t.compatibility),
  });
  const headings = view?.headings === "always" || groups.length > 1;
  const { limit, end } = useReveal(CARD_BATCH, shown.length, listKey);
  const visible = capGroups(groups, limit);
  // A tab without its own search row has nothing to count.
  const inEffect = onTab.filter((t) => t.state === "applied").length;

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
                disabled={changesDisabled}
                select={{
                  count: recommendedOpen.length,
                  onSelect: () =>
                    setStaged((s) => [
                      ...new Set([...s, ...recommendedOpen.map((t) => t.meta.id)]),
                    ]),
                }}
              />
            }
          />
          <PageTabs label={view.title} tabs={view.tabs} value={tab} onChange={setTab} filled />
          {view.banner}
          {!body && (
            <FilterBar
              search={search}
              onSearch={setSearch}
              placeholder="Search"
              filters={filters}
              sort={{
                value: sort,
                onChange: (v) => setSort(v as Sort),
                options: [
                  { value: "recommended", label: q ? "Best match" : "Recommended" },
                  { value: "name", label: "Name" },
                  { value: "impact", label: "Expected effect" },
                  { value: "status", label: "Status" },
                  { value: "newest", label: "Newest" },
                  { value: "recent", label: "Recently used" },
                ],
              }}
              end={
                <>
                  {view.end?.(tab)}
                  {data && (
                    <DataSourceBadge
                      source="live"
                      label={`${inEffect.toLocaleString()} of ${onTab.length.toLocaleString()} in effect`}
                    />
                  )}
                </>
              }
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
        <CardSkeletons />
      ) : all.length === 0 ? (
        <EmptyState icon={PackageOpen} title="Nothing here yet" className="flex-1 panel" />
      ) : shown.length === 0 ? (
        <EmptyState icon={SearchX} title="No tweaks match" className="flex-1 panel" />
      ) : (
        <div className="flex flex-col gap-4">
          {visible.map((group, i) => {
            // The count is the whole section's, however much of it has been drawn yet.
            const whole = groups[i]!.tweaks;
            return (
              <section key={group.title ?? "all"} className="flex flex-col gap-2.5">
                {group.title && headings && (
                  <div className="mt-1 flex items-baseline justify-between gap-3 pr-1">
                    <h2 className="text-[19px] leading-tight">{group.title}</h2>
                    <span className="text-[12.5px] text-muted-foreground tabular">
                      {group.title === UNAVAILABLE_SECTION
                        ? whole.length
                        : `${whole.filter((t) => t.state === "applied").length} of ${whole.length} in effect`}
                    </span>
                  </div>
                )}
                {/* A catalogue keeps its column width; a short fixed list spreads across the row. */}
                <CardGrid
                  className={view ? undefined : "grid-cols-[repeat(auto-fit,minmax(19rem,1fr))]"}
                >
                  {group.tweaks.map(card)}
                </CardGrid>
              </section>
            );
          })}
          {limit < shown.length && <div ref={end} aria-hidden className="h-px" />}
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
  history,
  onSwitch,
  onOpen,
}: {
  tweak: TweakStatus;
  staged: boolean;
  busy: boolean;
  disabled: boolean;
  isNew: boolean;
  history: TweakHistory | undefined;
  onSwitch: (on: boolean) => void;
  onOpen: () => void;
}) {
  const { meta, state, compatibility } = tweak;
  const blocked = isBlocked(compatibility);
  const applied = state === "applied";
  // Applied by something other than 2VX: there is no recorded before-value to restore.
  const lockedOn = applied && !tweak.undoable;
  const unread = state === "unknown";
  const restart = RESTART_HINT[meta.restart];
  // Applied since this boot, by a tweak that needs one: the change isn't finished yet.
  const restartPending = applied && meta.restart !== "none" && history?.sinceBoot === true;

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
      disabled={disabled || unread || busy}
      onCheckedChange={onSwitch}
      aria-label={`${applied ? "Undo" : staged ? "Deselect" : "Turn on"} ${meta.name}`}
      className={staged ? "ring-2 ring-primary/30 ring-offset-2 ring-offset-card" : undefined}
    />
  );

  const indicators = (
    <>
      <ImpactMeter impact={meta.impact} />
      {meta.risk === "high" && (
        <Indicator boxed icon={ShieldAlert} tone="destructive" title="High risk" />
      )}
      {meta.hardware !== "any" && (
        <Indicator
          boxed
          icon={meta.hardware.endsWith("Gpu") ? Gpu : Cpu}
          tone="info"
          title={`${HARDWARE_LABEL[meta.hardware]} only`}
        />
      )}
      {meta.evidence === "community" && (
        <Indicator boxed icon={FlaskConical} tone="neutral" title={EVIDENCE.community.label}>
          {EVIDENCE.community.note}
        </Indicator>
      )}
      {meta.warning && (
        <Indicator boxed icon={AlertTriangle} tone="destructive" title="Can break features">
          {meta.warning}
        </Indicator>
      )}
      {restartPending ? (
        <Indicator
          boxed
          icon={RotateCcw}
          tone="warning"
          title={meta.restart === "signOut" ? "Sign out to finish" : "Restart to finish"}
        />
      ) : (
        restart && <Indicator boxed icon={LogOut} tone="neutral" title={restart} />
      )}
      {history?.failed && (
        <Indicator boxed icon={CircleX} tone="destructive" title="Last change failed">
          {history.failed}
        </Indicator>
      )}
      {state === "partial" && (
        <Indicator boxed icon={CircleDashed} tone="warning" title="Partly in effect" />
      )}
      {unread && (
        <Indicator boxed icon={CircleHelp} tone="warning" title="Couldn't read this setting">
          {tweak.settings.find((s) => s.error)?.error}
        </Indicator>
      )}
    </>
  );
  // Only something that can be switched gets a switch: anything else says why it can't.
  const control = blocked ? (
    <Tooltip>
      <TooltipTrigger asChild>
        <span tabIndex={0} className="text-[12.5px] whitespace-nowrap text-muted-foreground">
          {BLOCKED_LABEL[compatibility]}
        </span>
      </TooltipTrigger>
      <TooltipContent>{COMPATIBILITY[compatibility]}</TooltipContent>
    </Tooltip>
  ) : lockedOn ? (
    <Tooltip>
      <TooltipTrigger asChild>
        <span
          tabIndex={0}
          className="flex items-center gap-1.5 text-[12.5px] whitespace-nowrap text-muted-foreground"
        >
          <Check className="size-3.5 text-success" strokeWidth={2.5} aria-hidden />
          Already set
        </span>
      </TooltipTrigger>
      <TooltipContent>Set outside 2VX, so there's nothing to undo.</TooltipContent>
    </Tooltip>
  ) : (
    switchControl
  );

  return (
    <OptimisationCard
      ref={card}
      icon={tweakIcon(meta)}
      title={cardTitle(meta)}
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
