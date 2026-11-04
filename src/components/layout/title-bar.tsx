import type { Ref } from "react";
import { isDesktop } from "@/services/backend";
import { WindowControls } from "./window-controls";

/**
 * Custom title bar, which also carries the page's title (`PageHeader`). The whole bar is a drag
 * region (double-click maximises); interactive children opt out because they are not the drag target.
 */
export function TitleBar({ slot }: { slot: Ref<HTMLDivElement> }) {
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
        <WindowControls />
      </div>
    </header>
  );
}
