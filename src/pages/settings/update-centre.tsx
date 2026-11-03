import type { ReactNode } from "react";
import {
  AlertTriangle,
  ArrowUpCircle,
  CheckCircle2,
  Download,
  Loader2,
  RefreshCw,
  RotateCcw,
  ShieldCheck,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { Progress } from "@/components/ui/progress";
import { useUpdater, type UpdaterState } from "@/hooks/use-updater";
import { formatBytes } from "@/lib/format";
import { formatTimestamp } from "@/lib/engine-text";
import { cn } from "@/lib/utils";
import type { UpdateInfo } from "@/services/updater";

/** Every updater state with its explanation and the one action that moves it on. */
export function UpdateCentre({ currentVersion }: { currentVersion: string | undefined }) {
  const updater = useUpdater();
  const { state } = updater;

  return (
    <div className="flex flex-col gap-3 px-4 py-4" aria-live="polite">
      <StatusLine state={state} currentVersion={currentVersion} lastChecked={updater.lastCheckedAt}>
        <Action state={state} updater={updater} />
      </StatusLine>

      {state.kind === "downloading" && (
        <div className="flex flex-col gap-1.5">
          <Progress
            value={
              state.progress.total ? (state.progress.downloaded / state.progress.total) * 100 : 5
            }
            aria-label="Download progress"
          />
          <div className="text-[11.5px] text-muted-foreground tabular">
            {formatBytes(state.progress.downloaded)}
            {state.progress.total ? ` of ${formatBytes(state.progress.total)}` : ""}
          </div>
        </div>
      )}

      {"info" in state && state.info && <ReleaseNotes info={state.info} />}

      <div className="flex items-start gap-2 text-[11.5px] leading-snug text-muted-foreground">
        <ShieldCheck className="mt-px size-3.5 shrink-0 text-success" />
        <span>
          Updates are signed. 2VX checks each download against its built-in public key and only
          accepts versions newer than the one installed, so a tampered or older file is refused.
          {updater.endpoint && (
            <>
              {" "}
              Channel <b className="font-medium text-foreground">{updater.channel}</b>:{" "}
              <code data-selectable className="font-mono break-all">
                {updater.endpoint}
              </code>
            </>
          )}
        </span>
      </div>
    </div>
  );
}

function StatusLine({
  state,
  currentVersion,
  lastChecked,
  children,
}: {
  state: UpdaterState;
  currentVersion: string | undefined;
  lastChecked: number | null;
  children: ReactNode;
}) {
  const { icon, tone, title, text } = describe(state, currentVersion, lastChecked);
  const Icon = icon;
  return (
    <div className="flex items-center gap-3">
      <Icon
        className={cn(
          "size-5 shrink-0",
          tone,
          (state.kind === "checking" || state.kind === "installing") && "animate-spin",
        )}
      />
      <div className="min-w-0 flex-1">
        <div className="text-card-title">{title}</div>
        {text && (
          <div data-selectable className="text-[12.5px] leading-snug text-muted-foreground">
            {text}
          </div>
        )}
      </div>
      <div className="flex shrink-0 items-center gap-2">{children}</div>
    </div>
  );
}

function describe(state: UpdaterState, current: string | undefined, lastChecked: number | null) {
  const version = current ? `You have version ${current}.` : "";
  const checked = lastChecked ? ` Last checked ${formatTimestamp(lastChecked)}.` : "";
  switch (state.kind) {
    case "loading":
      return { icon: Loader2, tone: "text-muted-foreground", title: "Loading…", text: version };
    case "unsupported":
      return {
        icon: AlertTriangle,
        tone: "text-muted-foreground",
        title: "Updates aren't available for this build",
        text: `${state.reason} ${version}`,
      };
    case "idle":
      return {
        icon: RefreshCw,
        tone: "text-muted-foreground",
        title: "Not checked yet",
        text: version,
      };
    case "checking":
      return { icon: Loader2, tone: "text-brand", title: "Checking for updates…", text: version };
    case "upToDate":
      return {
        icon: CheckCircle2,
        tone: "text-success",
        title: "2VX Tweaks is up to date",
        text: `${version}${checked}`,
      };
    case "available":
      return {
        icon: ArrowUpCircle,
        tone: "text-primary",
        title: `Version ${state.info.version} is available`,
        text: version,
      };
    case "downloading":
      return {
        icon: Download,
        tone: "text-brand",
        title: `Downloading ${state.info.version}…`,
        text: "",
      };
    case "ready":
      return {
        icon: CheckCircle2,
        tone: "text-success",
        title: `Version ${state.info.version} is downloaded and verified`,
        text: "Installing closes 2VX, runs the installer (Windows asks for administrator permission) and reopens the app.",
      };
    case "installing":
      return {
        icon: Loader2,
        tone: "text-brand",
        title: `Installing ${state.info.version}…`,
        text: "2VX will close while the installer runs.",
      };
    case "restartRequired":
      return {
        icon: RotateCcw,
        tone: "text-warning",
        title: "Restart to finish updating",
        text: `Version ${state.info.version} is installed. Restart 2VX to start using it.`,
      };
    case "error":
      return {
        icon: AlertTriangle,
        tone: "text-destructive",
        title:
          state.phase === "check"
            ? "Couldn't check for updates"
            : state.phase === "download"
              ? "The download didn't complete"
              : "The update couldn't be installed",
        text: `${state.message}. Nothing was changed.`,
      };
  }
}

function Action({
  state,
  updater,
}: {
  state: UpdaterState;
  updater: ReturnType<typeof useUpdater>;
}) {
  switch (state.kind) {
    case "idle":
    case "upToDate":
      return (
        <Button size="sm" variant="outline" onClick={() => void updater.check()}>
          <RefreshCw /> Check now
        </Button>
      );
    case "available":
      return (
        <>
          <Button size="sm" variant="ghost" onClick={() => void updater.check()}>
            Check again
          </Button>
          <Button size="sm" onClick={() => void updater.download()}>
            <Download /> Download {state.info.version}
          </Button>
        </>
      );
    case "ready":
      return (
        <Button size="sm" onClick={() => void updater.install()}>
          Install and restart
        </Button>
      );
    case "restartRequired":
      return (
        <Button size="sm" onClick={() => void updater.restart()}>
          <RotateCcw /> Restart now
        </Button>
      );
    case "error":
      return (
        <Button
          size="sm"
          variant="outline"
          onClick={() =>
            void (state.phase === "check"
              ? updater.check()
              : state.phase === "download"
                ? updater.download()
                : updater.install())
          }
        >
          <RefreshCw /> Try again
        </Button>
      );
    default:
      return null;
  }
}

function ReleaseNotes({ info }: { info: UpdateInfo }) {
  const lines = info.notes.split("\n").filter((l) => l.trim());
  return (
    <div className="rounded-lg border bg-background/40 px-4 py-3">
      <div className="mb-1.5 flex items-baseline justify-between gap-3">
        <div className="text-[12.5px] font-semibold">What's new in {info.version}</div>
        {info.date && (
          <div className="text-[11.5px] text-muted-foreground">
            {formatTimestamp(Date.parse(info.date))}
          </div>
        )}
      </div>
      {lines.length === 0 ? (
        <p className="text-[12.5px] text-muted-foreground">This release has no notes.</p>
      ) : (
        <div data-selectable className="flex max-h-56 flex-col gap-1 overflow-y-auto text-[12.5px]">
          {/* Notes are rendered as text, never HTML: the feed is not trusted with markup. */}
          {lines.map((line, i) =>
            /^#+\s/.test(line) ? (
              <div key={i} className="mt-1.5 font-medium first:mt-0">
                {line.replace(/^#+\s*/, "")}
              </div>
            ) : (
              <div key={i} className="flex gap-2 text-muted-foreground">
                {/^[-*]\s/.test(line) && <span className="text-brand">•</span>}
                <span>{line.replace(/^[-*]\s*/, "")}</span>
              </div>
            ),
          )}
        </div>
      )}
    </div>
  );
}
