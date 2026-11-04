import { useState, type FormEvent, type ReactNode } from "react";
import { ArrowRight, Info, X } from "lucide-react";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip";
import { gameProblem } from "@/lib/game-mode";
import { useUiStore } from "@/stores/ui-store";

/**
 * For games outside a known library. The file name is only compared with what's running, so a
 * mistake here can't change anything.
 */
export function AddGameDialog({
  open,
  onOpenChange,
}: {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}) {
  const games = useUiStore((s) => s.games);
  const setUi = useUiStore((s) => s.set);
  const [name, setName] = useState("");
  const [exe, setExe] = useState("");
  const [problem, setProblem] = useState<string | null>(null);

  const reset = () => {
    setName("");
    setExe("");
    setProblem(null);
  };

  const create = (e: FormEvent) => {
    e.preventDefault();
    const game = { name: name.trim(), exe: exe.trim() };
    const found = gameProblem(game, games);
    setProblem(found);
    if (found) return;
    setUi({ games: [...games, game] });
    reset();
    onOpenChange(false);
  };

  return (
    <Dialog
      open={open}
      onOpenChange={(next) => {
        if (!next) reset();
        onOpenChange(next);
      }}
    >
      <DialogContent className="sm:max-w-md">
        <form noValidate onSubmit={create} className="flex flex-col gap-4">
          <DialogHeader className="border-b pb-3">
            <DialogTitle>Add a Game</DialogTitle>
            <DialogDescription className="sr-only">
              Name a game and give its program file.
            </DialogDescription>
          </DialogHeader>

          {games.length > 0 && (
            <ul className="-mx-1 max-h-28 divide-y overflow-y-auto border-b">
              {games.map((game) => (
                <li key={game.exe} className="flex items-center gap-3 px-1 py-1.5">
                  <span className="min-w-0 flex-1 truncate text-[13px] font-medium">
                    {game.name}
                  </span>
                  <span className="truncate text-[12px] text-muted-foreground">{game.exe}</span>
                  <Button
                    type="button"
                    size="icon-xs"
                    variant="ghost"
                    aria-label={`Remove ${game.name}`}
                    onClick={() => setUi({ games: games.filter((g) => g !== game) })}
                  >
                    <X />
                  </Button>
                </li>
              ))}
            </ul>
          )}

          <Field label="Name" hint="How it appears on this page.">
            <Input
              // Otherwise focus lands on the info icon and opens its tooltip.
              autoFocus
              aria-label="Name"
              autoComplete="off"
              spellCheck={false}
              value={name}
              onChange={(e) => setName(e.target.value)}
            />
          </Field>
          <Field
            label="Program file (including .exe)"
            hint="Find it in Task Manager under Details, such as game.exe."
          >
            <Input
              aria-label="Program file"
              autoComplete="off"
              spellCheck={false}
              value={exe}
              onChange={(e) => setExe(e.target.value)}
              aria-invalid={problem !== null}
            />
          </Field>
          {problem && <p className="text-[12.5px] text-destructive">{problem}</p>}

          <div className="border-t pt-4">
            <Button
              type="submit"
              className="h-10 w-full"
              disabled={name.trim() === "" || exe.trim() === ""}
            >
              Create <ArrowRight />
            </Button>
          </div>
        </form>
      </DialogContent>
    </Dialog>
  );
}

function Field({ label, hint, children }: { label: string; hint: string; children: ReactNode }) {
  return (
    <div className="flex flex-col gap-1.5">
      <div className="flex items-center justify-between gap-2">
        <span className="text-[13px] font-medium text-muted-foreground">{label}</span>
        <Tooltip>
          <TooltipTrigger asChild>
            <span tabIndex={0} aria-label={hint} className="text-muted-foreground">
              <Info className="size-4" strokeWidth={1.75} aria-hidden />
            </span>
          </TooltipTrigger>
          <TooltipContent>{hint}</TooltipContent>
        </Tooltip>
      </div>
      {children}
    </div>
  );
}
