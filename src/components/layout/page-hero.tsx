import type { ReactNode } from "react";

/** A title in the page itself, for the pages that use the thin title bar. */
export function PageHero({
  title,
  subtitle,
  action,
}: {
  title: string;
  subtitle: string;
  action?: ReactNode;
}) {
  return (
    <div className="flex items-center justify-between gap-6 pb-1">
      <div className="min-w-0">
        <h1 className="truncate text-[1.75rem] leading-tight font-semibold tracking-tight">
          {title}
        </h1>
        <p data-selectable className="truncate text-[14px] text-muted-foreground">
          {subtitle}
        </p>
      </div>
      {action}
    </div>
  );
}
