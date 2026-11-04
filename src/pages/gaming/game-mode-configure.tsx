import { useState } from "react";
import {
  AlertTriangle,
  ArrowLeft,
  ArrowRight,
  RotateCw,
  Settings,
  Undo2,
  type LucideIcon,
} from "lucide-react";
import { Indicator } from "@/components/common/indicator";
import { Button } from "@/components/ui/button";
import { Switch } from "@/components/ui/switch";
import { COMPATIBILITY, isBlocked, RESTART_HINT } from "@/lib/engine-text";
import { DEFAULT_GAME_MODE_TWEAKS, GAME_MODE_TWEAKS, GAMING_PROFILES } from "@/lib/profiles";
import { tweakIcon } from "@/lib/tweak-presentation";
import { cn } from "@/lib/utils";
import { useUiStore } from "@/stores/ui-store";
import type { TweakStatus } from "@/types/engine";

const sameTweaks = (a: string[], b: string[]) =>
  a.length === b.length && a.every((id) => b.includes(id));

/** What Game Mode turns on. Saving changes the list only; the review still comes first. */
export function GameModeConfigure({
  statuses,
  onClose,
}: {
  statuses: TweakStatus[];
  onClose: () => void;
}) {
  const stored = useUiStore((s) => s.gameModeTweaks);
  const setUi = useUiStore((s) => s.set);
  const saved = stored ?? DEFAULT_GAME_MODE_TWEAKS;
  const [draft, setDraft] = useState(saved);
  const tweaks = GAME_MODE_TWEAKS.flatMap((id) => statuses.filter((t) => t.meta.id === id));

  const toggle = (id: string) =>
    setDraft((d) => (d.includes(id) ? d.filter((x) => x !== id) : [...d, id]));

  return (
    // Spacing follows the window height so all seven rows fit the smallest window.
    <div className="flex flex-1 flex-col gap-[clamp(0.25rem,calc(2vh-0.5rem),0.5rem)]">
      <button
        type="button"
        onClick={onClose}
        className="flex items-center gap-2 self-start py-0.5 text-[13px] text-muted-foreground transition-colors hover:text-foreground"
      >
        <ArrowLeft className="size-4" aria-hidden /> Go back
      </button>

      <div className="flex items-center gap-3 panel px-4 py-2.5">
        <Settings
          className="size-5 shrink-0 text-muted-foreground"
          strokeWidth={1.75}
          aria-hidden
        />
        <h2 className="min-w-0 flex-1 truncate text-section-title">Configure Game Mode</h2>
        <div className="flex shrink-0 items-center gap-1" role="group" aria-label="Presets">
          {GAMING_PROFILES.map((profile) => (
            <Button
              key={profile.id}
              size="sm"
              variant={sameTweaks(profile.tweakIds, draft) ? "secondary" : "ghost"}
              aria-pressed={sameTweaks(profile.tweakIds, draft)}
              title={profile.summary}
              onClick={() => setDraft(profile.tweakIds)}
            >
              {profile.name}
            </Button>
          ))}
        </div>
      </div>

      <ul className="flex flex-col gap-[clamp(0.25rem,calc(2vh-0.5rem),0.5rem)]">
        {tweaks.map((t) => (
          <Row
            key={t.meta.id}
            tweak={t}
            icon={tweakIcon(t.meta)}
            included={draft.includes(t.meta.id)}
            onToggle={() => toggle(t.meta.id)}
          />
        ))}
      </ul>

      <div className="sticky bottom-0 mt-auto flex gap-3 bg-background pt-1">
        <Button
          className="h-10 flex-1"
          disabled={sameTweaks(draft, saved)}
          onClick={() => {
            setUi({ gameModeTweaks: draft });
            onClose();
          }}
        >
          Save <ArrowRight />
        </Button>
        <Button
          variant="secondary"
          className="h-10 flex-1"
          disabled={sameTweaks(draft, DEFAULT_GAME_MODE_TWEAKS)}
          onClick={() => setDraft(DEFAULT_GAME_MODE_TWEAKS)}
        >
          Revert to Default <Undo2 />
        </Button>
      </div>
    </div>
  );
}

function Row({
  tweak,
  icon: Icon,
  included,
  onToggle,
}: {
  tweak: TweakStatus;
  icon: LucideIcon;
  included: boolean;
  onToggle: () => void;
}) {
  const { meta, state, compatibility } = tweak;
  const blocked = isBlocked(compatibility);
  const restart = state === "applied" ? null : RESTART_HINT[meta.restart];

  return (
    <li
      className={cn(
        "flex items-center gap-3.5 panel px-4 py-[clamp(0.3125rem,calc(1.5vh-0.25rem),0.5rem)]",
        blocked && "opacity-60",
      )}
    >
      <Icon className="size-5 shrink-0 text-brand" strokeWidth={1.75} aria-hidden />
      <div className="min-w-0 flex-1">
        <div className="truncate text-[13.5px] font-medium">{meta.name}</div>
        <p className="truncate text-[12.5px] text-muted-foreground">{meta.description}</p>
      </div>
      {blocked && (
        <Indicator icon={AlertTriangle} tone="neutral" title="Not for this PC">
          {COMPATIBILITY[compatibility]}
        </Indicator>
      )}
      {restart && !blocked && <Indicator icon={RotateCw} tone="neutral" title={restart} />}
      {meta.warning && !blocked && (
        <Indicator icon={AlertTriangle} tone="destructive" title="Can break features">
          {meta.warning}
        </Indicator>
      )}
      <Switch
        aria-label={`Include ${meta.name}`}
        checked={included && !blocked}
        disabled={blocked}
        onCheckedChange={onToggle}
      />
    </li>
  );
}
