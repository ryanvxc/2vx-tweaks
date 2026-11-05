import { useEffect } from "react";
import { Undo2, ShieldCheck, CircuitBoard, FileClock, type LucideIcon } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { useMakeRestorePoint } from "@/hooks/use-restore-point";
import { useRestorePoints, useSnapshots } from "@/hooks/use-system";
import { useUiStore } from "@/stores/ui-store";

const PROMISES: { icon: LucideIcon; text: string }[] = [
  {
    icon: Undo2,
    text: "Every change is saved first, and you can undo it from the Restore Centre.",
  },
  {
    icon: ShieldCheck,
    text: "2VX never turns off Defender, Firewall, Windows Update, UAC or SmartScreen.",
  },
  { icon: CircuitBoard, text: "2VX never changes your BIOS. It only shows what to check." },
];

/** Shown once, to someone who hasn't changed anything yet. */
export function FirstRunDialog() {
  const seen = useUiStore((s) => s.firstRunSeen);
  const setUi = useUiStore((s) => s.set);
  const { data: snapshots } = useSnapshots();
  const { data: restore } = useRestorePoints();
  const { making, make } = useMakeRestorePoint();

  // Someone with history is already past this, so an update doesn't greet them.
  const returning = (snapshots?.length ?? 0) > 0;
  useEffect(() => {
    if (returning && !seen) setUi({ firstRunSeen: true });
  }, [returning, seen, setUi]);

  if (seen || returning || !snapshots) return null;
  const close = () => setUi({ firstRunSeen: true });
  const canMake = restore?.protection === "on" || restore?.protection === "unknown";

  return (
    <Dialog open onOpenChange={(open) => !open && close()}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Before you start</DialogTitle>
          <DialogDescription>
            A restore point gives you a way back for the whole PC.
          </DialogDescription>
        </DialogHeader>
        <ul className="flex flex-col gap-2.5 text-[13px]">
          {PROMISES.map(({ icon: Icon, text }) => (
            <li key={text} className="flex items-start gap-2.5">
              <Icon className="mt-0.5 size-4 shrink-0 text-brand" aria-hidden />
              <span>{text}</span>
            </li>
          ))}
        </ul>
        <DialogFooter>
          <Button variant="ghost" onClick={close}>
            Not now
          </Button>
          <Button
            disabled={making || !canMake}
            onClick={() => {
              void make().finally(close);
            }}
          >
            <FileClock /> {making ? "Making…" : "Make a restore point"}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
