import { isDesktop } from "@/services/backend";
import { WindowControls } from "./window-controls";

/**
 * Custom title bar: only the window controls, since every page carries its own title. The whole bar
 * is a drag region; interactive children opt out because they are not the drag target.
 */
export function TitleBar() {
  return (
    <header data-tauri-drag-region className="flex h-9 shrink-0 items-center justify-end gap-3">
      {!isDesktop && (
        <span className="text-[11px] font-medium text-info">Browser preview: simulated data</span>
      )}
      <WindowControls />
    </header>
  );
}
