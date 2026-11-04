import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

/** Settings rows under a heading. Several share one `panel` on the Settings page. */
export function SettingsGroup({
  title,
  description,
  children,
  id,
}: {
  title?: string;
  description?: string;
  children: ReactNode;
  id?: string;
}) {
  return (
    <section id={id}>
      {(title || description) && (
        <div className="px-4 pt-3 pb-2">
          {title && <h2 className="text-section-title">{title}</h2>}
          {description && <p className="mt-0.5 text-[13px] text-muted-foreground">{description}</p>}
        </div>
      )}
      <div className="divide-y border-t">{children}</div>
    </section>
  );
}

/** One setting: icon, title and explanation on the left, its control on the right. */
export function SettingsRow({
  icon: Icon,
  title,
  description,
  control,
  children,
  htmlFor,
  className,
}: {
  icon: LucideIcon;
  title: string;
  description?: ReactNode;
  control?: ReactNode;
  /** Extra content below the row text (details, progress). */
  children?: ReactNode;
  /** Associates the title with a form control for screen readers and clicks. */
  htmlFor?: string;
  className?: string;
}) {
  const Title = htmlFor ? "label" : "div";
  return (
    <div
      className={cn(
        // Taller with the window, up to a point: a toggle in a 120 px row reads as a gap.
        "flex min-h-[clamp(3.25rem,10.5vh,5rem)] gap-3 px-4 py-2.5",
        children ? "items-start" : "items-center",
        className,
      )}
    >
      <Icon className="size-[18px] shrink-0 text-brand" aria-hidden />
      <div className="min-w-0 flex-1">
        <Title htmlFor={htmlFor} className="block text-card-title">
          {title}
        </Title>
        {description && (
          <div className="mt-0.5 text-[13px] leading-snug text-muted-foreground">{description}</div>
        )}
        {children && <div className="mt-2.5">{children}</div>}
      </div>
      {control && <div className="flex shrink-0 items-center gap-2 self-center">{control}</div>}
    </div>
  );
}
