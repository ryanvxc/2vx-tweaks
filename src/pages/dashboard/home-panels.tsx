import { useState, type ReactNode } from "react";
import { Link } from "react-router";
import {
  ArchiveRestore,
  ChevronRight,
  Headphones,
  SlidersHorizontal,
  type LucideIcon,
} from "lucide-react";
import { AnimatedNumber } from "@/components/common/animated-number";
import { DataSourceBadge, type DataSource } from "@/components/common/data-source-badge";
import { DotField } from "@/components/common/dot-field";
import { Skeleton } from "@/components/ui/skeleton";
import { useSnapshots, useTweakStatuses } from "@/hooks/use-system";
import { savedBackups } from "@/lib/quickstart";
import { cn } from "@/lib/utils";

/** Written out in full so Tailwind can see each class. */
const TONES = {
  violet: {
    icon: "group-hover:text-chart-1 group-focus-visible:text-chart-1",
    chip: "border-chart-1/20 bg-chart-1/8 text-chart-1",
    dots: "text-chart-1",
    wash: "from-chart-1/30",
    edge: "hover:border-chart-1/55 focus-visible:border-chart-1/55",
  },
  green: {
    icon: "group-hover:text-chart-3 group-focus-visible:text-chart-3",
    chip: "border-chart-3/20 bg-chart-3/8 text-chart-3",
    dots: "text-chart-3",
    wash: "from-chart-3/30",
    edge: "hover:border-chart-3/55 focus-visible:border-chart-3/55",
  },
  pink: {
    icon: "group-hover:text-chart-5 group-focus-visible:text-chart-5",
    chip: "border-chart-5/20 bg-chart-5/8 text-chart-5",
    dots: "text-chart-5",
    wash: "from-chart-5/30",
    edge: "hover:border-chart-5/55 focus-visible:border-chart-5/55",
  },
};

/** Three cards, each read from the engine or opening the page that holds the answer. */
export function HomeCards() {
  const statuses = useTweakStatuses();
  const snapshots = useSnapshots();
  const applied = statuses.data?.filter((t) => t.state === "applied").length;
  const total = statuses.data?.length;
  const backups = snapshots.data && savedBackups(snapshots.data).length;

  return (
    <div className="grid grid-cols-3 gap-3">
      <HomeCard
        to="/restore"
        icon={ArchiveRestore}
        tone="violet"
        action="View backups"
        source="live"
        title={
          snapshots.error ? (
            "—"
          ) : backups !== undefined ? (
            <AnimatedNumber value={backups} />
          ) : undefined
        }
        text={
          snapshots.error ? "Journal unavailable" : backups === 1 ? "Backup saved" : "Backups saved"
        }
      />
      <HomeCard
        to="/tools"
        icon={Headphones}
        tone="green"
        action="Get help"
        title="Need help?"
        text="System report and Windows tools"
      />
      <HomeCard
        to="/tweaks"
        icon={SlidersHorizontal}
        tone="pink"
        action="Browse tweaks"
        source="live"
        title={
          applied !== undefined ? (
            <>
              <AnimatedNumber value={applied} /> of {total}
            </>
          ) : undefined
        }
        text="Tweaks in effect"
      />
    </div>
  );
}

function HomeCard({
  to,
  icon: Icon,
  tone,
  action,
  title,
  text,
  source,
}: {
  to: string;
  icon: LucideIcon;
  tone: keyof typeof TONES;
  action: string;
  title: ReactNode;
  text: string;
  source?: DataSource;
}) {
  const [active, setActive] = useState(false);
  const t = TONES[tone];

  return (
    <Link
      to={to}
      data-stagger
      onPointerEnter={() => setActive(true)}
      onPointerLeave={() => setActive(false)}
      onFocus={() => setActive(true)}
      onBlur={() => setActive(false)}
      className={cn(
        "group relative isolate flex h-31 flex-col justify-between overflow-hidden panel p-3.5 transition-colors duration-300 outline-none",
        t.edge,
      )}
    >
      <span
        aria-hidden
        className={cn(
          "absolute inset-0 -z-20 bg-linear-to-t to-transparent to-80% opacity-0 transition-opacity duration-500 group-hover:opacity-100 group-focus-visible:opacity-100",
          t.wash,
        )}
      />
      <DotField active={active} className={cn("absolute inset-0 -z-10 size-full", t.dots)} />

      <div className="flex items-start justify-between gap-2">
        <span
          className={cn(
            "flex size-8 shrink-0 items-center justify-center rounded-lg border bg-background/40 text-muted-foreground transition-colors duration-300",
            t.icon,
          )}
        >
          <Icon className="size-4" strokeWidth={1.75} aria-hidden />
        </span>
        <span
          className={cn(
            "flex h-8 items-center gap-1 rounded-lg border pr-2 pl-3 text-[13px] font-medium",
            t.chip,
          )}
        >
          {action}
          <ChevronRight
            className="size-3.5 transition-transform duration-200 group-hover:translate-x-0.5"
            aria-hidden
          />
        </span>
      </div>

      <div className="flex items-end gap-2">
        <div className="min-w-0 flex-1">
          {title === undefined ? (
            <Skeleton className="mb-1 h-7 w-16" />
          ) : (
            <div className="truncate text-[1.5rem] leading-tight font-semibold tracking-tight tabular">
              {title}
            </div>
          )}
          <div className="truncate text-[13px] text-muted-foreground transition-colors duration-300 group-hover:text-foreground/85 group-focus-visible:text-foreground/85">
            {text}
          </div>
        </div>
        {source && <DataSourceBadge source={source} />}
      </div>
    </Link>
  );
}
