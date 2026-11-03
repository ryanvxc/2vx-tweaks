import type { ReactNode } from "react";

export function PageHeader({
  title,
  description,
  actions,
}: {
  title: string;
  description?: string;
  actions?: ReactNode;
}) {
  return (
    <div className="mb-5 flex items-end justify-between gap-6">
      <div className="min-w-0">
        <h1 className="text-page-title">{title}</h1>
        {description && <p className="mt-0.5 text-[13.5px] text-muted-foreground">{description}</p>}
      </div>
      {actions && <div className="flex shrink-0 items-center gap-2">{actions}</div>}
    </div>
  );
}
