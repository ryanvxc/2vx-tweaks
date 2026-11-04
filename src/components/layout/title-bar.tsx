import type { Ref } from "react";
import { Search } from "lucide-react";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { isDesktop } from "@/services/backend";
import { WindowControls } from "./window-controls";

/**
 * Custom title bar, which also carries the page's title (`PageHeader`). The whole bar is a drag
 * region (double-click maximises); interactive children opt out because they are not the drag target.
 */
export function TitleBar({ slot, onSearch }: { slot: Ref<HTMLDivElement>; onSearch: () => void }) {
  return (
    <header
      data-tauri-drag-region
      className="flex h-(--titlebar-height) shrink-0 items-stretch justify-between"
    >
      <div
        ref={slot}
        data-tauri-drag-region
        className="flex min-w-0 flex-1 items-center gap-4 pr-4 pl-5"
      />
      <div className="flex h-9 shrink-0 items-center gap-3">
        {!isDesktop && (
          <span className="text-[11px] font-medium text-info">Browser preview: simulated data</span>
        )}
        <Tooltip>
          <TooltipTrigger asChild>
            <button
              type="button"
              onClick={onSearch}
              aria-label="Search"
              className="flex h-9 w-11 items-center justify-center text-muted-foreground transition-colors hover:bg-accent/60 hover:text-foreground"
            >
              <Search className="size-4" strokeWidth={1.5} />
            </button>
          </TooltipTrigger>
          <TooltipContent>Search (Ctrl K)</TooltipContent>
        </Tooltip>
        <WindowControls />
      </div>
    </header>
  );
}
