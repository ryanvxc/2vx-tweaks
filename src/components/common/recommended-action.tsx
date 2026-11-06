import { useEffect, useRef } from "react";
import { Link } from "react-router";
import { Sparkles } from "lucide-react";
import { toast } from "sonner";
import { useTweakStatuses } from "@/hooks/use-system";
import { burstAt } from "@/lib/dot-burst";
import { openRecommended } from "@/lib/quickstart";
import { tweakHref } from "@/lib/tweak-presentation";

const ACTION =
  "glow-action flex h-11 items-center gap-2.5 rounded-lg px-5 text-[14px] font-medium outline-none focus-visible:ring-2 focus-visible:ring-ring disabled:opacity-50";

/**
 * The one action a page pushes: the recommended tweaks still to apply. `select` is for a page that
 * lists them, and picks them where they are; anywhere else it links to the first. With nothing left
 * there's no button: this slot isn't a place to advertise something else.
 */
export function RecommendedAction({
  select,
  disabled,
}: {
  select?: { count: number; onSelect: () => void };
  disabled?: boolean;
}) {
  const { data } = useTweakStatuses();
  const left = data && openRecommended(data).length;
  const box = useRef<HTMLDivElement>(null);
  // Where the button was: it goes the moment the last one lands.
  const spot = useRef({ x: 0, y: 0 });
  const was = useRef(left);

  useEffect(() => {
    if (box.current) {
      const b = box.current.getBoundingClientRect();
      spot.current = { x: b.left + b.width / 2, y: b.top + b.height / 2 };
    }
    if (left === 0 && was.current) {
      burstAt(spot.current.x, spot.current.y, { tone: "success", count: 90, reach: 240 });
      toast.success("All recommended tweaks are on");
    }
    was.current = left;
  });

  if (!data) return null;
  const open = openRecommended(data);
  const count = select ? select.count : open.length;
  if (count === 0) return null;
  const content = (
    <>
      <Sparkles
        className="size-4 text-brand transition-[rotate,scale] duration-300 group-hover:scale-110 group-hover:rotate-12"
        aria-hidden
      />
      {select
        ? `Select ${count} recommended`
        : `${count} recommended tweak${count === 1 ? "" : "s"}`}
    </>
  );

  return (
    <div ref={box} className="group relative isolate shrink-0">
      <span
        aria-hidden
        className="absolute -inset-1 -z-10 rounded-xl bg-linear-to-r from-brand to-primary opacity-35 blur-md transition-opacity duration-300 group-hover:opacity-60"
      />
      {select ? (
        <button type="button" disabled={disabled} onClick={select.onSelect} className={ACTION}>
          {content}
        </button>
      ) : (
        <Link to={tweakHref(open[0]!.meta).split("&tweak=")[0]!} className={ACTION}>
          {content}
        </Link>
      )}
    </div>
  );
}
