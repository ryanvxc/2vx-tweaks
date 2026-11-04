import { Link } from "react-router";
import { Sparkles } from "lucide-react";
import { useTweakStatuses } from "@/hooks/use-system";
import { openRecommended } from "@/lib/quickstart";

const ACTION =
  "glow-action flex h-11 items-center gap-2.5 rounded-lg px-5 text-[14px] font-medium outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:opacity-50";

/**
 * The one action a page pushes: what's left to apply, or `otherwise` once nothing is. It links
 * to Tweaks, unless the page is Tweaks: `onSelect` then selects them where they are.
 */
export function RecommendedAction({
  otherwise,
  onSelect,
  disabled,
}: {
  otherwise: { to: string; label: string };
  onSelect?: () => void;
  disabled?: boolean;
}) {
  const { data } = useTweakStatuses();
  if (!data) return null;
  const open = openRecommended(data).length;
  const content = (
    <>
      <Sparkles
        className="size-4 text-brand transition-[rotate,scale] duration-300 group-hover:scale-110 group-hover:rotate-12"
        aria-hidden
      />
      {open > 0 ? `${open} recommended tweak${open === 1 ? "" : "s"}` : otherwise.label}
    </>
  );

  return (
    <div className="group relative isolate shrink-0">
      <span
        aria-hidden
        className="absolute -inset-1 -z-10 rounded-xl bg-linear-to-r from-brand to-primary opacity-35 blur-md transition-opacity duration-300 group-hover:opacity-70"
      />
      {open > 0 && onSelect ? (
        <button type="button" disabled={disabled} onClick={onSelect} className={ACTION}>
          {content}
        </button>
      ) : (
        <Link to={open > 0 ? "/tweaks" : otherwise.to} className={ACTION}>
          {content}
        </Link>
      )}
    </div>
  );
}
