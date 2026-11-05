import type { ReactNode, Ref } from "react";
import type { LucideIcon } from "lucide-react";
import { Skeleton } from "@/components/ui/skeleton";
import { cn } from "@/lib/utils";

/**
 * The card for anything the user can switch: icon and title on top, a short description, then
 * indicators on the left and the control on the right, or one full-width action. Clicking the
 * body (not the control) opens details.
 */
export function OptimisationCard({
  icon: Icon,
  leading,
  title,
  titleHint,
  description,
  badge,
  indicators,
  control,
  footer,
  onOpen,
  openLabel,
  highlighted,
  muted,
  className,
  ref,
}: {
  icon?: LucideIcon;
  /** In place of `icon`, for something with a real logo. */
  leading?: ReactNode;
  title: string;
  /** Shown on hover when the title isn't the whole story, such as a path. */
  titleHint?: string;
  description: string;
  /** Top-right label, e.g. "New". */
  badge?: ReactNode;
  indicators?: ReactNode;
  control?: ReactNode;
  /** In place of the indicators and control. */
  footer?: ReactNode;
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
        "group relative flex h-44 flex-col panel px-5 pt-5 pb-3.5 transition-colors",
        "hover:bg-raised",
        highlighted && "border-primary/50",
        muted && "opacity-60",
        className,
      )}
    >
      <div className={cn("flex items-center gap-3", badge && "pr-10")}>
        {leading ??
          (Icon && (
            <Icon
              className="size-[22px] shrink-0 text-brand transition-transform duration-200 group-hover:scale-110"
              strokeWidth={1.75}
              aria-hidden
            />
          ))}
        <div className="min-w-0 flex-1">
          {onOpen ? (
            <button
              type="button"
              onClick={onOpen}
              aria-label={openLabel ?? `Details for ${title}`}
              title={title}
              className="block w-full truncate text-left text-[14.5px] font-semibold outline-none after:absolute after:inset-0 after:rounded-xl focus-visible:after:ring-2 focus-visible:after:ring-ring"
            >
              {title}
            </button>
          ) : (
            <div
              data-selectable
              title={titleHint ?? title}
              className="truncate text-[14.5px] font-semibold"
            >
              {title}
            </div>
          )}
        </div>
      </div>
      {badge && <div className="absolute top-3 right-3">{badge}</div>}
      <p className="mt-2.5 line-clamp-2 text-[13px] leading-normal text-muted-foreground">
        {description}
      </p>
      {footer ? (
        <div className="relative mt-auto">{footer}</div>
      ) : (
        <div className="mt-auto flex items-center justify-between gap-2 border-t border-dashed pt-3">
          {/* Raised above the stretched open-button so tooltips and the control stay usable. */}
          <div className="relative flex min-w-0 items-center gap-1.5">{indicators}</div>
          {control && <div className="relative flex shrink-0 justify-end">{control}</div>}
        </div>
      )}
    </div>
  );
}

/**
 * A full-width row, for something that isn't a tweak, such as a tool to open. `flat` drops its own
 * panel so it can sit in a shared one, split by hairlines.
 */
export function OptimisationRow({
  leading,
  title,
  description,
  indicators,
  control,
  onOpen,
  openLabel,
  highlighted,
  muted,
  flat,
  ref,
}: {
  leading: ReactNode;
  title: string;
  description?: string;
  indicators?: ReactNode;
  control?: ReactNode;
  onOpen?: () => void;
  openLabel?: string;
  highlighted?: boolean;
  muted?: boolean;
  flat?: boolean;
  ref?: Ref<HTMLDivElement>;
}) {
  return (
    <div
      ref={ref}
      data-stagger
      className={cn(
        "group relative flex items-center transition-colors hover:bg-raised",
        flat ? "h-14 gap-3 px-4" : "h-[68px] gap-4 panel px-5",
        highlighted && (flat ? "bg-primary/8" : "border-primary/50"),
        muted && "opacity-60",
      )}
    >
      <span className={cn("flex shrink-0 items-center justify-center", flat ? "size-7" : "size-9")}>
        {leading}
      </span>
      <div className="min-w-0 flex-1">
        {onOpen ? (
          <button
            type="button"
            onClick={onOpen}
            aria-label={openLabel ?? `Details for ${title}`}
            title={title}
            className="block w-full truncate text-left text-[14.5px] font-semibold outline-none after:absolute after:inset-0 after:rounded-xl focus-visible:after:ring-2 focus-visible:after:ring-ring"
          >
            {title}
          </button>
        ) : (
          <div className="truncate text-[14.5px] font-semibold">{title}</div>
        )}
        {description && <p className="truncate text-[13px] text-muted-foreground">{description}</p>}
      </div>
      {indicators && <div className="relative flex items-center gap-1.5">{indicators}</div>}
      {/* A fixed slot, so a label in one row doesn't push that row's indicators out of line. */}
      {control && <div className="relative flex w-32 shrink-0 justify-end">{control}</div>}
    </div>
  );
}

export function NewBadge() {
  return (
    <span className="rounded-md border border-success/25 bg-success/10 px-2 py-0.5 text-[11px] font-medium text-success">
      New
    </span>
  );
}

/** Fills the width at any window size, adding columns rather than stretching cards. */
export function CardGrid({ children, className }: { children: ReactNode; className?: string }) {
  return (
    <div className={cn("grid grid-cols-[repeat(auto-fill,minmax(19rem,1fr))] gap-3", className)}>
      {children}
    </div>
  );
}

export function CardSkeletons({ count = 6 }: { count?: number }) {
  return (
    <CardGrid>
      {Array.from({ length: count }, (_, i) => (
        <Skeleton key={i} className="h-44 rounded-xl" />
      ))}
    </CardGrid>
  );
}
