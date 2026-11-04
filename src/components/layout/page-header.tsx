import { useContext, type ReactNode } from "react";
import { createPortal } from "react-dom";
import { HeaderSlot } from "./header-slot";

/** The page's title, with an optional line beside it and actions on the right. */
export function PageHeader({
  title,
  description,
  actions,
}: {
  title: string;
  description?: string;
  actions?: ReactNode;
}) {
  const slot = useContext(HeaderSlot);
  const header = (
    <>
      <div data-tauri-drag-region className="flex min-w-0 flex-1 items-baseline gap-3">
        <h1 data-tauri-drag-region className="shrink-0 text-page-title">
          {title}
        </h1>
        {description && (
          <p data-tauri-drag-region className="truncate text-[12.5px] text-muted-foreground">
            {description}
          </p>
        )}
      </div>
      {actions && <div className="flex shrink-0 items-center gap-2">{actions}</div>}
    </>
  );
  return slot ? createPortal(header, slot) : header;
}
