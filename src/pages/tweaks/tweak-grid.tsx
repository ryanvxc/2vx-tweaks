import { useMemo, useState } from "react";
import { useSearchParams } from "react-router";
import {
  AlertTriangle,
  CircleDashed,
  Layers,
  LogOut,
  PackageOpen,
  RotateCcw,
  SearchX,
  ShieldCheck,
  ShieldUser,
  Sparkles,
  Wand2,
  X,
} from "lucide-react";
import { toast } from "sonner";
import { ConfirmDialog } from "@/components/common/confirm-dialog";
import { FilterBar, type FilterGroup } from "@/components/common/filter-bar";
import { ImpactMeter, Indicator } from "@/components/common/indicator";
import { CardGrid, NewBadge, OptimisationCard } from "@/components/common/optimisation-card";
import { PageTabs, type PageTab } from "@/components/common/page-tabs";
import { EmptyState, ErrorState } from "@/components/common/states";
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
import {
  COMPATIBILITY,
  describeError,
  describeOutcome,
  isBlocked,
  RESTART_HINT,
} from "@/lib/engine-text";
import { CATEGORY_LABEL, isNewTweak, tweakIcon } from "@/lib/tweak-presentation";
import { backend, isDesktop, toBackendError } from "@/services/backend";
import type { Impact, RiskLevel, TweakCategory, TweakStatus } from "@/types/engine";
import { ChangeInspector } from "./change-inspector";
import { TweakDetails } from "./tweak-details";

type StateFilter = "applied" | "notApplied";
type PermissionFilter = "administrator" | "user";
type Sort = "recommended" | "name" | "impact" | "newest";

const IMPACT_ORDER: Record<Impact, number> = { low: 0, medium: 1, high: 2 };

/**
 * Tweaks as a card grid. The switch shows the state detected on this PC, never a stored
 * flag. Switching on stages a tweak for review (the dry-run plan in ChangeInspector);
 * switching off undoes exactly what 2VX recorded, after a confirmation.
 */
export function TweakGrid({ categories }: { categories: TweakCategory[] }) {
  const { data, error, loading, refetch } = useTweakStatuses();
  const engine = useEngineStatus();
  const app = useAppInfo();
  const [params, setParams] = useSearchParams();

  const [tab, setTab] = useState<TweakCategory | "all">("all");
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

  const all = useMemo(
    () => (data ?? []).filter((t) => categories.includes(t.meta.category)),
    [data, categories],
  );
  const names = Object.fromEntries((data ?? []).map((t) => [t.meta.id, t.meta.name]));
  // In preview the journal is absent by design; planning is refused with its own message.
  const changesDisabled = isDesktop && engine.data?.journalAvailable === false;
  const detailsId = params.get("tweak");
  const details = all.find((t) => t.meta.id === detailsId) ?? null;

  const tabs: PageTab<TweakCategory | "all">[] = [
    { value: "all", label: "All", icon: Layers, count: all.length },
    ...categories
      .filter((c) => all.some((t) => t.meta.category === c))
      .map((c) => ({
        value: c,
        label: CATEGORY_LABEL[c],
        icon: tweakIcon({ id: "", category: c }),
        count: all.filter((t) => t.meta.category === c).length,
      })),
  ];

  const q = search.trim().toLowerCase();
  const shown = all
    .filter((t) => tab === "all" || t.meta.category === tab)
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

  return (
    <div className="flex flex-col gap-4">
      <PageTabs label="Categories" tabs={tabs} value={tab} onChange={setTab} />
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
        end={
          recommendedOpen.length > 0 && (
            <Button
              size="sm"
              variant="outline"
              disabled={changesDisabled}
              onClick={() =>
                setStaged((s) => [...new Set([...s, ...recommendedOpen.map((t) => t.meta.id)])])
              }
            >
              <Wand2 /> Stage recommended ({recommendedOpen.length})
            </Button>
          )
        }
      />

      {error ? (
        <ErrorState message={error.message} onRetry={refetch} />
      ) : loading && !data ? (
        <CardGrid>
          {Array.from({ length: 6 }, (_, i) => (
            <Skeleton key={i} className="h-[132px] rounded-xl" />
          ))}
        </CardGrid>
      ) : all.length === 0 ? (
        <EmptyState
          icon={PackageOpen}
          title="No optimisations in this section yet"
          description="Each one is added only after its effect, risk and undo path are documented and tested."
          className="rounded-xl border bg-card"
        />
      ) : shown.length === 0 ? (
        <EmptyState icon={SearchX} title="No tweaks match" className="rounded-xl border bg-card" />
      ) : (
        <CardGrid>
          {shown.map((t) => (
            <TweakCard
              key={t.meta.id}
              tweak={t}
              staged={staged.includes(t.meta.id)}
              busy={busy === t.meta.id}
              disabled={changesDisabled}
              isNew={isNewTweak(t.meta, app.data?.version)}
              onSwitch={(on) => onSwitch(t, on)}
              onOpen={() => openDetails(t.meta.id)}
            />
          ))}
        </CardGrid>
      )}

      {staged.length > 0 && (
        <div className="sticky bottom-4 z-10 mx-auto flex items-center gap-3 rounded-xl border border-primary/30 bg-popover/95 py-2 pr-2 pl-4 shadow-2xl shadow-black/50 backdrop-blur">
          <Sparkles className="size-4 text-primary" />
          <span className="text-[13px]">
            <b className="font-semibold tabular">{staged.length}</b> tweak
            {staged.length === 1 ? "" : "s"} staged. Nothing changes until you review.
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
          description="2VX puts back the values it recorded before it made this change. A value something else has changed since is left as it is."
          consequences={undoing.settings.map((s) => `${s.targetLabel}: restore the recorded value`)}
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
  onSwitch,
  onOpen,
}: {
  tweak: TweakStatus;
  staged: boolean;
  busy: boolean;
  disabled: boolean;
  isNew: boolean;
  onSwitch: (on: boolean) => void;
  onOpen: () => void;
}) {
  const { meta, state, compatibility } = tweak;
  const blocked = isBlocked(compatibility);
  const applied = state === "applied";
  // Applied by something other than 2VX: there is no recorded before-value to restore.
  const lockedOn = applied && !tweak.undoable;
  const restart = RESTART_HINT[meta.restart];

  const switchControl = (
    <Switch
      checked={applied || staged}
      disabled={disabled || blocked || lockedOn || busy}
      onCheckedChange={onSwitch}
      aria-label={`${applied ? "Undo" : staged ? "Unstage" : "Turn on"} ${meta.name}`}
      className={staged ? "ring-2 ring-primary/30 ring-offset-2 ring-offset-card" : undefined}
    />
  );

  return (
    <OptimisationCard
      icon={tweakIcon(meta)}
      title={meta.name}
      description={meta.description}
      highlighted={applied || staged}
      muted={blocked}
      onOpen={onOpen}
      openLabel={`Details for ${meta.name}`}
      badge={isNew ? <NewBadge /> : undefined}
      indicators={
        <>
          <ImpactMeter impact={meta.impact} />
          {meta.warning && (
            <Indicator icon={AlertTriangle} tone="destructive" title="Can break features">
              {meta.warning}
            </Indicator>
          )}
          {meta.category === "privacy" && (
            <Indicator icon={ShieldCheck} tone="success" title="Privacy">
              Sends less data to Microsoft or websites.
            </Indicator>
          )}
          {meta.permission === "administrator" && (
            <Indicator icon={ShieldUser} tone="info" title="Asks for administrator permission">
              A machine-wide value. Windows shows one prompt when you apply or undo it.
            </Indicator>
          )}
          {restart && (
            <Indicator icon={LogOut} tone="neutral" title={restart}>
              {meta.restart === "reboot"
                ? "Windows reads this value when it next starts."
                : "Windows reads this value when you next sign in."}
            </Indicator>
          )}
          {blocked && (
            <Indicator icon={AlertTriangle} tone="neutral" title="Not for this PC">
              {COMPATIBILITY[compatibility]}
            </Indicator>
          )}
          {state === "partial" && (
            <Indicator icon={CircleDashed} tone="warning" title="Partly in effect">
              Some of its values match. Applying completes it.
            </Indicator>
          )}
          {applied && tweak.undoable && (
            <Indicator icon={RotateCcw} tone="brand" title="2VX can undo this">
              Switch it off to restore the values recorded before 2VX changed them.
            </Indicator>
          )}
        </>
      }
      control={
        lockedOn ? (
          <Tooltip>
            <TooltipTrigger asChild>
              <span tabIndex={0}>{switchControl}</span>
            </TooltipTrigger>
            <TooltipContent className="max-w-60">
              Already set on this PC, but not by 2VX, so there's no recorded value to go back to.
              Change it in Windows if you want it off.
            </TooltipContent>
          </Tooltip>
        ) : (
          switchControl
        )
      }
    />
  );
}
