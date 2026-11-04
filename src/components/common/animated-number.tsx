import { useEffect, useRef, useState } from "react";
import { useUiStore } from "@/stores/ui-store";

/**
 * A number that counts to its value instead of snapping there: up from zero the first time,
 * then from wherever it was. `format` turns the in-between values into text.
 */
export function AnimatedNumber({
  value,
  format = (v) => String(Math.round(v)),
  duration = 600,
}: {
  value: number;
  format?: (v: number) => string;
  duration?: number;
}) {
  const reduce = useUiStore((s) => s.reduceMotion);
  const [shown, setShown] = useState(0);
  const current = useRef(0);

  useEffect(() => {
    if (reduce) {
      current.current = value;
      return;
    }
    const from = current.current;
    const started = performance.now();
    let frame = 0;
    const tick = (now: number) => {
      const t = Math.min((now - started) / duration, 1);
      const next = from + (value - from) * (1 - (1 - t) ** 4);
      current.current = next;
      setShown(next);
      if (t < 1) frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  }, [value, duration, reduce]);

  return <>{format(reduce ? value : shown)}</>;
}
