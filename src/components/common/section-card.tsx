import type { ReactNode } from "react";
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { cn } from "@/lib/utils";
import { DataSourceBadge, type DataSource } from "./data-source-badge";

/** The standard content card: title, optional description, source badge and actions. */
export function SectionCard({
  title,
  description,
  source,
  actions,
  children,
  className,
  headerClassName,
  contentClassName,
}: {
  title: ReactNode;
  description?: string;
  source?: DataSource;
  actions?: ReactNode;
  children: ReactNode;
  className?: string;
  headerClassName?: string;
  contentClassName?: string;
}) {
  return (
    <Card className={className}>
      <CardHeader className={headerClassName}>
        <CardTitle className="text-section-title">{title}</CardTitle>
        {description && <CardDescription className="text-[12px]">{description}</CardDescription>}
        {(source || actions) && (
          <CardAction className="flex min-h-5 items-center gap-3">
            {actions}
            {source && <DataSourceBadge source={source} />}
          </CardAction>
        )}
      </CardHeader>
      <CardContent className={cn("flex-1", contentClassName)}>{children}</CardContent>
    </Card>
  );
}
