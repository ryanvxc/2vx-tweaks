import { Link } from "react-router";
import { ArrowUpCircle, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import { useUpdater } from "@/hooks/use-updater";
import { useUiStore } from "@/stores/ui-store";

/**
 * Subtle Home notice when the feed offers a newer version. It only points to the Update
 * Centre, where the release notes are shown before anything downloads.
 */
export function UpdateNotice() {
  const { state } = useUpdater();
  const show = useUiStore((s) => s.showUpdateNotice);
  const dismissed = useUiStore((s) => s.dismissedUpdateVersion);
  const set = useUiStore((s) => s.set);

  if (!show) return null;
  const info =
    state.kind === "available" || state.kind === "ready" || state.kind === "downloading"
      ? state.info
      : null;
  if (!info || dismissed === info.version) return null;

  return (
    <div
      role="status"
      className="flex animate-rise items-center gap-3 rounded-xl border border-primary/25 bg-primary/8 px-4 py-2.5"
    >
      <ArrowUpCircle className="size-4 shrink-0 text-primary" />
      <div className="min-w-0 flex-1 text-[13px]">
        <span className="font-medium">2VX Tweaks {info.version} is available</span>
      </div>
      <Button asChild size="xs">
        <Link to="/settings?tab=updates">
          {state.kind === "ready" ? "Install" : "See what's new"}
        </Link>
      </Button>
      <Button
        size="icon-xs"
        variant="ghost"
        aria-label="Dismiss update notice"
        onClick={() => set({ dismissedUpdateVersion: info.version })}
      >
        <X />
      </Button>
    </div>
  );
}
