import { ShieldAlert, ShieldCheck } from "lucide-react";
import { useLocation } from "react-router";
import { ALL_NAV_ITEMS } from "@/app/navigation";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { useAppInfo } from "@/hooks/use-system";
import { isDesktop } from "@/services/backend";
import { WindowControls } from "./window-controls";

/**
 * Custom title bar. The whole bar is a drag region (double-click maximises);
 * interactive children opt out automatically because they are not the drag target.
 */
export function TitleBar() {
  const { pathname } = useLocation();
  const current = ALL_NAV_ITEMS.find((i) => i.to === pathname);
  const { data: app } = useAppInfo();

  return (
    <header
      data-tauri-drag-region
      className="flex h-(--titlebar-height) shrink-0 items-center justify-between border-b pl-6"
    >
      <div data-tauri-drag-region className="text-xs font-medium text-muted-foreground">
        {current?.label ?? "2VX Tweaks"}
      </div>

      <div className="flex h-full items-center gap-3">
        {!isDesktop && (
          <span className="rounded-full border border-info/30 bg-info/10 px-2 py-0.5 text-[11px] font-medium text-info">
            Browser preview: simulated data
          </span>
        )}
        {app && (
          <Tooltip>
            <TooltipTrigger asChild>
              <span className="flex cursor-default items-center gap-1.5 text-[11px] text-muted-foreground">
                {app.isElevated ? (
                  <ShieldAlert className="size-3.5 text-warning" />
                ) : (
                  <ShieldCheck className="size-3.5 text-success" />
                )}
                {app.isElevated ? "Administrator" : "Standard user"}
              </span>
            </TooltipTrigger>
            <TooltipContent side="bottom" className="max-w-64">
              {app.isElevated
                ? "Running elevated. 2VX only needs administrator rights for system-wide changes."
                : "Running without administrator rights. You'll be asked to elevate only when an action needs it."}
            </TooltipContent>
          </Tooltip>
        )}
        <WindowControls />
      </div>
    </header>
  );
}
