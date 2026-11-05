import { useState, type ReactNode } from "react";
import { Link } from "react-router";
import { Megaphone } from "lucide-react";
import patchNotesArt from "@/assets/home/patch-notes.webp";
import recentChangesArt from "@/assets/home/recent-changes.webp";
import updateCentreArt from "@/assets/home/system-health.webp";
import { DataSourceBadge, type DataSource } from "@/components/common/data-source-badge";
import { Dialog, DialogContent, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { useAppInfo, useSnapshots, useTweaks } from "@/hooks/use-system";
import { useUpdater } from "@/hooks/use-updater";
import { latestNotes } from "@/lib/changelog";
import { formatTimestamp } from "@/lib/engine-text";
import { updateStatus } from "./update-status";

const CARD =
  "group relative isolate flex min-h-48 flex-col justify-end overflow-hidden panel p-4 text-left transition-colors duration-300 outline-none hover:border-foreground/20 focus-visible:ring-2 focus-visible:ring-ring";

export function UpdatesSection() {
  const [reading, setReading] = useState(false);
  const app = useAppInfo();
  const { state: update, lastCheckedAt } = useUpdater();
  const snapshots = useSnapshots();
  const tweaks = useTweaks();

  const notes = latestNotes();
  const recent = snapshots.data?.[0];
  const names = Object.fromEntries((tweaks.data ?? []).map((t) => [t.id, t.name]));

  return (
    <section className="flex flex-1 flex-col gap-2 pt-1">
      <div data-stagger className="flex h-6 items-center gap-2">
        <Megaphone className="size-4 shrink-0" strokeWidth={1.75} aria-hidden />
        <h2 className="text-section-title">Updates</h2>
      </div>

      <div className="grid flex-1 grid-cols-3 gap-3">
        <button
          type="button"
          data-stagger
          className={CARD}
          onClick={() => setReading(true)}
          aria-haspopup="dialog"
        >
          <Face art={patchNotesArt} title="Patch Notes">
            What's new in 2VX {app.data?.version ?? notes?.version}
          </Face>
        </button>

        <Link to="/restore" data-stagger className={CARD}>
          <Face art={recentChangesArt} title="Recent changes" source="live">
            {recent ? (
              <>
                {recent.kind === "apply" ? "" : "Undid "}
                {recent.tweakIds.map((id) => names[id] ?? id).join(", ")}
                <br />
                {formatTimestamp(recent.createdAtMs)}
              </>
            ) : (
              "No changes yet."
            )}
          </Face>
        </Link>

        <Link to="/settings?tab=updates" data-stagger className={CARD}>
          <Face art={updateCentreArt} title="Update Centre" source="live">
            {updateStatus(update)}
            {lastCheckedAt && (
              <>
                <br />
                Checked {formatTimestamp(lastCheckedAt)}
              </>
            )}
          </Face>
        </Link>
      </div>

      <Dialog open={reading} onOpenChange={setReading}>
        <DialogContent className="flex max-h-[calc(100vh-4rem)] flex-col sm:max-w-xl">
          <DialogHeader>
            <DialogTitle>Patch Notes {notes?.version}</DialogTitle>
          </DialogHeader>
          <div className="-mr-3 flex min-h-0 flex-col gap-4 overflow-y-auto pr-3">
            {notes?.sections.map((section) => (
              <section key={section.title} className="flex flex-col gap-1.5">
                <h3 className="text-section-title">{section.title}</h3>
                <ul className="flex list-disc flex-col gap-1 pl-4 text-[13px] leading-relaxed text-muted-foreground marker:text-border">
                  {section.items.map((item) => (
                    <li key={item}>{item}</li>
                  ))}
                </ul>
              </section>
            ))}
          </div>
        </DialogContent>
      </Dialog>
    </section>
  );
}

function Face({
  art,
  title,
  source,
  children,
}: {
  art: string;
  title: string;
  source?: DataSource;
  children: ReactNode;
}) {
  return (
    <>
      <img
        src={art}
        alt=""
        draggable={false}
        className="absolute inset-0 -z-20 size-full object-cover transition-[scale] duration-700 ease-arrive group-hover:scale-105"
      />
      <span
        aria-hidden
        className="absolute inset-0 -z-10 bg-linear-to-t from-background from-10% via-background/60 via-45% to-transparent to-75%"
      />
      <div className="flex items-end gap-2">
        <div className="min-w-0 flex-1">
          <div className="text-[1.0625rem] font-semibold tracking-tight">{title}</div>
          <p className="line-clamp-2 text-[13px] leading-snug text-foreground/70">{children}</p>
        </div>
        {source && <DataSourceBadge source={source} />}
      </div>
    </>
  );
}
