import { useLocation, useSearchParams } from "react-router";
import { ALL_NAV_ITEMS } from "@/app/navigation";
import { isDesktop } from "@/services/backend";
import { WindowControls } from "./window-controls";

/**
 * Custom title bar. The whole bar is a drag region (double-click maximises);
 * interactive children opt out automatically because they are not the drag target.
 * Privilege status lives in the sidebar footer.
 */
export function TitleBar() {
  const { pathname } = useLocation();
  const [params] = useSearchParams();
  const current = ALL_NAV_ITEMS.find((i) => i.to === pathname);
  const tab = current?.tabs?.find((t) => t.value === params.get("tab")) ?? current?.tabs?.[0];
  const trail = current && current.to !== "/" ? [current.label, tab?.label] : [];

  return (
    <header
      data-tauri-drag-region
      className="flex h-(--titlebar-height) shrink-0 items-center justify-between pl-8"
    >
      <div data-tauri-drag-region className="text-[11.5px] font-medium text-muted-foreground/70">
        {["2VX Tweaks", ...trail].filter(Boolean).join(" › ")}
      </div>

      <div className="flex h-full items-center gap-3">
        {!isDesktop && (
          <span className="rounded-full border border-info/30 bg-info/10 px-2 py-0.5 text-[11px] font-medium text-info">
            Browser preview: simulated data
          </span>
        )}
        <WindowControls />
      </div>
    </header>
  );
}
