import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

/**
 * The card for anything the user can switch: icon and title on top, a two-line description,
 * a hairline, then indicators on the left and the control on the right. Clicking the body
 * (not the control) opens details.
 */
export function OptimisationCard({
  icon: Icon,
  title,
  description,
  badge,
  indicators,
  control,
  onOpen,
  openLabel,
  highlighted,
  muted,
  className,
}: {
  icon: LucideIcon;
  title: string;
  description: string;
  /** Top-right label, e.g. "New". */
  badge?: ReactNode;
  indicators?: ReactNode;
  control?: ReactNode;
  onOpen?: () => void;
  openLabel?: string;
  /** Selected for review. Applied tweaks stay neutral: the switch already says so. */
  highlighted?: boolean;
  /** Not applicable on this PC. */
  muted?: boolean;
  className?: string;
}) {
  return (
    <div
      className={cn(
        "group relative flex min-h-[136px] flex-col rounded-xl border bg-card px-4 pt-3.5 pb-3 transition-colors",
        "hover:border-white/12 hover:bg-raised",
        highlighted && "border-primary/50",
        muted && "opacity-60",
        className,
      )}
    >
      <div className="flex items-start gap-2.5">
        <Icon className="mt-px size-[18px] shrink-0 text-brand" aria-hidden />
        <div className="min-w-0 flex-1">
          {onOpen ? (
            <button
              type="button"
              onClick={onOpen}
              aria-label={openLabel ?? `Details for ${title}`}
              className="text-left text-card-title outline-none after:absolute after:inset-0 after:rounded-xl focus-visible:after:ring-2 focus-visible:after:ring-ring"
            >
              {title}
            </button>
          ) : (
            <div className="text-card-title">{title}</div>
          )}
        </div>
        {badge && <div className="relative shrink-0">{badge}</div>}
      </div>
      <p className="mt-1.5 line-clamp-2 text-[13px] leading-snug text-muted-foreground">
        {description}
      </p>
      <div className="mt-auto flex items-center justify-between gap-2 border-t border-dashed pt-2.5">
        {/* Raised above the stretched open-button so tooltips and the control stay usable. */}
        <div className="relative flex min-w-0 flex-wrap items-center gap-1">{indicators}</div>
        {control && <div className="relative shrink-0">{control}</div>}
      </div>
    </div>
  );
}

export function NewBadge() {
  return (
    <span className="inline-flex h-5 items-center rounded-full bg-success/15 px-2 text-[10.5px] font-semibold text-success">
      New
    </span>
  );
}

/** Grid that keeps cards readable from the 1080px minimum window to wide screens. */
export function CardGrid({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <div className={cn("grid grid-cols-2 gap-3 xl:grid-cols-3 2xl:grid-cols-4", className)}>
      {children}
    </div>
  );
}
