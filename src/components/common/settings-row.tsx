import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

/** Settings rows under a heading, split by hairlines. */
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
        <div className="pt-3 pb-1">
          {title && <h2 className="text-[13px] font-medium text-muted-foreground">{title}</h2>}
          {description && <p className="mt-0.5 text-[13px] text-muted-foreground">{description}</p>}
        </div>
      )}
      <div className="divide-y border-b">{children}</div>
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
        "flex min-h-[4.25rem] gap-3.5 py-2.5",
        children ? "items-start" : "items-center",
        className,
      )}
    >
      <span className="flex size-10 shrink-0 items-center justify-center panel rounded-lg">
        <Icon className="size-[18px] text-brand" strokeWidth={1.75} aria-hidden />
      </span>
      <div className="min-w-0 flex-1">
        <Title htmlFor={htmlFor} className="block text-[14.5px] font-semibold">
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
