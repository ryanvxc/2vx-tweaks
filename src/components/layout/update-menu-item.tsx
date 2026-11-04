import { Download, Loader2, RefreshCw, RotateCcw, type LucideIcon } from "lucide-react";
import { toast } from "sonner";
import { DropdownMenuItem } from "@/components/ui/dropdown-menu";
import {
  updateNeedsUser,
  useUpdater,
  useUpdaterStore,
  type UpdaterState,
} from "@/hooks/use-updater";
import { cn } from "@/lib/utils";

/**
 * The account menu's one-click route through the updater: check, download, then restart to
 * apply. The menu stays open on a click so the label can show progress. Settings › Updates
 * shows the same state with the release notes.
 */
export function UpdateMenuItem({ className }: { className?: string }) {
  const { state } = useUpdater();
  const { icon: Icon, label, busy } = present(state);
  const attention = updateNeedsUser(state);

  return (
    <DropdownMenuItem
      disabled={busy}
      className={cn("group", className)}
      onSelect={(e) => {
        e.preventDefault();
        void advance(state);
      }}
    >
      <Icon
        strokeWidth={1.75}
        aria-hidden
        className={cn(
          "size-5 shrink-0 transition-[rotate] duration-300",
          Icon === RefreshCw && "group-hover:rotate-90",
          attention && "text-primary",
          (state.kind === "checking" || state.kind === "installing") && "animate-spin",
        )}
      />
      <span className="truncate tabular">{label}</span>
      {attention && (
        <span className="relative ml-auto size-1.5 rounded-full bg-primary">
          <span className="absolute inset-0 animate-ping-soft rounded-full bg-primary" />
        </span>
      )}
    </DropdownMenuItem>
  );
}

function present(state: UpdaterState): {
  icon: LucideIcon;
  label: string;
  busy: boolean;
} {
  const idle = { icon: RefreshCw, busy: false };
  switch (state.kind) {
    case "loading":
      return { ...idle, label: "Check for updates", busy: true };
    case "checking":
      return { icon: Loader2, label: "Checking…", busy: true };
    case "available":
      return { icon: Download, label: "Update available", busy: false };
    case "downloading": {
      const { downloaded, total } = state.progress;
      const label = total
        ? `Downloading ${Math.floor((downloaded / total) * 100)}%`
        : "Downloading…";
      return { icon: Download, label, busy: true };
    }
    case "ready":
    case "restartRequired":
      return { icon: RotateCcw, label: "Restart to update", busy: false };
    case "installing":
      return { icon: Loader2, label: "Installing…", busy: true };
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
