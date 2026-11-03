import { Download, Loader2, RefreshCw, RotateCcw, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useUpdater, useUpdaterStore, type UpdaterState } from "@/hooks/use-updater";
import { cn } from "@/lib/utils";

/**
 * The sidebar's one-click route through the updater: check, download, then restart to apply.
 * Settings › Updates shows the same state with the release notes.
 */
export function UpdateButton({ collapsed }: { collapsed: boolean }) {
  const { state } = useUpdater();
  const { icon: Icon, label, busy, attention } = present(state);

  const button = (
    <button
      type="button"
      onClick={() => void advance(state)}
      disabled={busy}
      aria-label={collapsed ? label : undefined}
      className={cn(
        "relative flex h-8.5 w-full items-center gap-2.5 rounded-md px-2.5 text-[13px] font-medium text-sidebar-foreground transition-colors hover:bg-sidebar-accent/60 hover:text-sidebar-accent-foreground disabled:pointer-events-none",
        collapsed && "justify-center px-0",
      )}
    >
      <Icon
        className={cn(
          "size-4 shrink-0",
          attention ? "text-primary" : "text-muted-foreground",
          (state.kind === "checking" || state.kind === "installing") && "animate-spin",
        )}
      />
      {!collapsed && <span className="truncate tabular">{label}</span>}
      {attention && (
        <span
          className={cn(
            "size-1.5 rounded-full bg-primary",
            collapsed ? "absolute top-1.5 right-2.5" : "ml-auto",
          )}
        />
      )}
    </button>
  );

  if (!collapsed) return button;
  return (
    <Tooltip>
      <TooltipTrigger asChild>{button}</TooltipTrigger>
      <TooltipContent side="right">{label}</TooltipContent>
    </Tooltip>
  );
}

function present(state: UpdaterState): {
  icon: LucideIcon;
  label: string;
  busy: boolean;
  attention: boolean;
} {
  const idle = { icon: RefreshCw, busy: false, attention: false };
  switch (state.kind) {
    case "loading":
      return { ...idle, label: "Check for updates", busy: true };
    case "checking":
      return { icon: Loader2, label: "Checking…", busy: true, attention: false };
    case "available":
      return { icon: Download, label: "Update available", busy: false, attention: true };
    case "downloading": {
      const { downloaded, total } = state.progress;
      const label = total
        ? `Downloading ${Math.floor((downloaded / total) * 100)}%`
        : "Downloading…";
      return { icon: Download, label, busy: true, attention: false };
    }
    case "ready":
    case "restartRequired":
      return { icon: RotateCcw, label: "Restart to update", busy: false, attention: true };
    case "installing":
      return { icon: Loader2, label: "Installing…", busy: true, attention: false };
    case "error":
      return { ...idle, label: state.phase === "install" ? "Retry update" : "Check for updates" };
    default:
      return { ...idle, label: "Check for updates" };
  }
}

/** Moves the updater on by one click. Failures appear as toasts: the label has no room. */
async function advance(state: UpdaterState) {
  const updater = useUpdaterStore.getState();
  switch (state.kind) {
    case "unsupported":
      toast.info(state.reason);
      return;
    case "available":
      await updater.download();
      break;
    case "ready":
      await updater.install();
      break;
    case "restartRequired":
      await updater.restart();
      break;
    case "error":
      if (state.phase === "install") await updater.install();
      else if (state.phase === "download") await updater.download();
      else await updater.checkAndDownload();
      break;
    default:
      await updater.checkAndDownload();
  }

  const after = useUpdaterStore.getState().state;
  if (after.kind === "upToDate") toast.success("2VX Tweaks is up to date");
  if (after.kind === "error") toast.error(after.message);
}
