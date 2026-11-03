import type { ReactNode } from "react";
import type { LucideIcon } from "lucide-react";
import { cn } from "@/lib/utils";

/** A titled group of settings rows. */
export function SettingsGroup({
  title,
  description,
  children,
  id,
}: {
  title: string;
  description?: string;
  children: ReactNode;
  id?: string;
}) {
  return (
    <section id={id} className="scroll-mt-4">
      <div className="mb-2 px-1">
        <h2 className="text-section-title">{title}</h2>
        {description && <p className="mt-0.5 text-[12.5px] text-muted-foreground">{description}</p>}
      </div>
      <div className="divide-y overflow-hidden rounded-xl border bg-card">{children}</div>
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
    <div className={cn("flex items-start gap-3.5 px-4 py-3.5", className)}>
      <div className="flex size-8 shrink-0 items-center justify-center rounded-lg bg-brand/12 text-brand">
        <Icon className="size-4" aria-hidden />
      </div>
      <div className="min-w-0 flex-1">
        <Title htmlFor={htmlFor} className="block text-card-title">
          {title}
        </Title>
        {description && (
          <div className="mt-0.5 text-[12.5px] leading-snug text-muted-foreground">
            {description}
          </div>
        )}
        {children && <div className="mt-2.5">{children}</div>}
      </div>
      {control && <div className="flex shrink-0 items-center gap-2 self-center">{control}</div>}
    </div>
  );
}
