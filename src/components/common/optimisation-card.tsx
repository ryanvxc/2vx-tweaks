import type { ReactNode, Ref } from "react";
import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

/**
 * The card for anything the user can switch: icon and title on top, a short description, then
 * indicators on the left and the control on the right. Clicking the body (not the control)
 * opens details.
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
  ref,
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
  ref?: Ref<HTMLDivElement>;
}) {
  return (
    <div
      ref={ref}
      data-stagger
      className={cn(
        "group relative flex min-h-[168px] flex-col panel px-4 pt-4 pb-3 transition-colors",
        "hover:bg-raised",
        highlighted && "border-primary/50",
        muted && "opacity-60",
        className,
      )}
    >
      <div className="flex items-center gap-2.5">
        <Icon
          className="size-[18px] shrink-0 text-brand transition-transform duration-200 group-hover:scale-110"
          aria-hidden
        />
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
      <p className="mt-2.5 line-clamp-3 text-[13px] leading-normal text-muted-foreground">
        {description}
      </p>
      <div className="mt-auto flex items-center justify-between gap-2 pt-2">
        {/* Raised above the stretched open-button so tooltips and the control stay usable. */}
        <div className="relative -ml-1 flex min-w-0 flex-wrap items-center gap-0.5">
          {indicators}
        </div>
        {control && <div className="relative shrink-0">{control}</div>}
      </div>
    </div>
  );
}

export function NewBadge() {
  return <span className="text-[11px] font-medium text-success">New</span>;
}

/** Fills the width at any window size, adding columns rather than stretching cards. */
export function CardGrid({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <div className={cn("grid grid-cols-[repeat(auto-fill,minmax(19rem,1fr))] gap-3", className)}>
      {children}
    </div>
  );
}
