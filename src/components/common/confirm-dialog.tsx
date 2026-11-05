import { useState, type ReactNode } from "react";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";
import { cn } from "@/lib/utils";
import type { RiskLevel } from "@/types/engine";
import { RiskBadge } from "./risk-badge";

/**
 * Confirmation for any action that changes the system. High-risk actions require
 * ticking an explicit acknowledgement before the confirm button enables.
 */
export function ConfirmDialog({
  open,
  onOpenChange,
  title,
  description,
  consequences,
  risk,
  confirmLabel = "Apply",
  onConfirm,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  title: string;
  description: ReactNode;
  /** Bullet list of what will change. Required: users must see consequences first. */
  consequences: string[];
  risk: RiskLevel;
  confirmLabel?: string;
  onConfirm: () => void;
}) {
  const [acknowledged, setAcknowledged] = useState(false);
  const needsAck = risk === "high";

  return (
    <AlertDialog
      open={open}
      onOpenChange={(o) => {
        if (!o) setAcknowledged(false);
        onOpenChange(o);
      }}
    >
      <AlertDialogContent>
        <AlertDialogHeader>
          {risk !== "moderate" && (
            <div className="mb-1">
              <RiskBadge risk={risk} />
            </div>
          )}
          <AlertDialogTitle>{title}</AlertDialogTitle>
          <AlertDialogDescription>{description}</AlertDialogDescription>
        </AlertDialogHeader>

        <ul className="flex list-disc flex-col gap-1 rounded-md border bg-muted/50 py-3 pr-3 pl-7 text-[13px] text-muted-foreground">
          {consequences.map((c) => (
            <li key={c}>{c}</li>
          ))}
        </ul>

        {needsAck && (
          <label className="flex cursor-pointer items-start gap-2.5 text-[13px]">
            <input
              type="checkbox"
              className="mt-0.5 size-4 accent-primary"
              checked={acknowledged}
              onChange={(e) => setAcknowledged(e.target.checked)}
            />
            <span>I understand this change may disrupt my system and I have a restore point.</span>
          </label>
        )}

        <AlertDialogFooter>
          <AlertDialogCancel>Cancel</AlertDialogCancel>
          <AlertDialogAction
            disabled={needsAck && !acknowledged}
            className={cn(risk === "high" && "bg-destructive text-white hover:bg-destructive/90")}
            onClick={onConfirm}
          >
            {confirmLabel}
          </AlertDialogAction>
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
  );
}
