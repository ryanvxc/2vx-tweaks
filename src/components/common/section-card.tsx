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
  contentClassName,
}: {
  title: string;
  description?: string;
  source?: DataSource;
  actions?: ReactNode;
  children: ReactNode;
  className?: string;
  contentClassName?: string;
}) {
  return (
    <Card className={cn("gap-4", className)}>
      <CardHeader>
        <CardTitle className="text-section-title">{title}</CardTitle>
        {description && <CardDescription className="text-[12.5px]">{description}</CardDescription>}
        {(source || actions) && (
          <CardAction className="flex items-center gap-2">
            {actions}
            {source && <DataSourceBadge source={source} />}
          </CardAction>
        )}
      </CardHeader>
      <CardContent className={cn("flex-1", contentClassName)}>{children}</CardContent>
    </Card>
  );
}
