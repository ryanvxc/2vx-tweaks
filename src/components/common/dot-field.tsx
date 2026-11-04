import { useEffect, useRef } from "react";
import { useUiStore } from "@/stores/ui-store";

const STEP = 4;
const DOT = 2;
const FRAME_MS = 1000 / 30;
const RISE_MS = 620;
const FALL_MS = 260;

/** The same dot always twinkles the same way. */
function seed(col: number, row: number) {
  const s = Math.sin(col * 127.1 + row * 311.7) * 43758.5453;
  return s - Math.floor(s);
}

/**
 * A field of small squares that rises from the bottom edge and twinkles. Drawn only while
 * `active` and for the moment after, so a resting card costs nothing. The colour is the
 * element's `color`, so a text token sets it.
 */
export function DotField({ active, className }: { active: boolean; className?: string }) {
  const canvas = useRef<HTMLCanvasElement>(null);
  const reduce = useUiStore((s) => s.reduceMotion);
  // Kept between hovers, so leaving halfway falls from where it had reached.
  const level = useRef(0);

  useEffect(() => {
    if (!active && level.current === 0) return;
    const el = canvas.current;
    const ctx = el?.getContext("2d");
    if (!el || !ctx) return;

    const box = el.getBoundingClientRect();
    const ratio = window.devicePixelRatio || 1;
    el.width = Math.round(box.width * ratio);
    el.height = Math.round(box.height * ratio);
    ctx.setTransform(ratio, 0, 0, ratio, 0, 0);
    ctx.fillStyle = getComputedStyle(el).color;
    const cols = Math.ceil(box.width / STEP);
    const rows = Math.ceil(box.height / STEP);

    const draw = (time: number) => {
      ctx.clearRect(0, 0, box.width, box.height);
      const front = level.current * 1.35;
      for (let row = 0; row < rows; row++) {
        const up = 1 - (row * STEP) / box.height;
        const reveal = Math.min((front - up) / 0.4, 1);
        if (reveal <= 0) continue;
        // Sparser and fainter towards the top of the card.
        const fade = (1 - up) ** 1.5;
        for (let col = 0; col < cols; col++) {
          const s = seed(col, row);
          if (s < 0.18 + up * 0.5) continue;
          const twinkle = 0.45 + 0.55 * Math.sin(time * 0.0016 * (0.6 + s) + s * 40) ** 2;
          // Held back so the text over the brightest rows stays readable.
          ctx.globalAlpha = reveal * fade * twinkle * 0.6;
          ctx.fillRect(col * STEP, row * STEP, DOT, DOT);
        }
      }
      ctx.globalAlpha = 1;
    };

    if (reduce) {
      level.current = active ? 1 : 0;
      draw(0);
      return;
    }

    let frame = 0;
    let last = performance.now();
    const tick = (now: number) => {
      const elapsed = now - last;
      if (elapsed >= FRAME_MS) {
        last = now;
        level.current = active
          ? Math.min(level.current + elapsed / RISE_MS, 1)
          : Math.max(level.current - elapsed / FALL_MS, 0);
        draw(now);
        if (!active && level.current === 0) return;
      }
      frame = requestAnimationFrame(tick);
    };
    frame = requestAnimationFrame(tick);
    return () => cancelAnimationFrame(frame);
  }, [active, reduce]);

  return <canvas ref={canvas} aria-hidden className={className} />;
}
