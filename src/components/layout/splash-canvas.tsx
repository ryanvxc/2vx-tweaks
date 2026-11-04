import { useEffect, useRef, type PointerEvent, type RefObject } from "react";
import {
  addPulse,
  createField,
  GRADIENT_TONES,
  PULSE_LIFE,
  PULSE_SPEED,
  scatter,
  stepField,
  TONE_CHEVRON,
  TONE_CHEVRON_SOFT,
  TONES,
  type Field,
  type Point,
} from "@/lib/splash-field";

const JOINED_SIZE = 3;
const FREE_SIZE = 1.8;
const FREE_ALPHA = 0.4;

/** Canvas can't read Tailwind tokens, so it mixes the same CSS variables itself. */
function toneColours() {
  const style = getComputedStyle(document.documentElement);
  const brand = style.getPropertyValue("--brand").trim();
  const primary = style.getPropertyValue("--primary").trim();
  const foreground = style.getPropertyValue("--foreground").trim();
  const colours = Array.from({ length: TONES }, (_, tone) => {
    const share = Math.round((tone / (GRADIENT_TONES - 1)) * 100);
    return `color-mix(in oklch, ${brand}, ${primary} ${share}%)`;
  });
  colours[TONE_CHEVRON] = foreground;
  colours[TONE_CHEVRON_SOFT] = `color-mix(in oklch, ${foreground} 60%, ${primary})`;
  return colours;
}

function paint(
  ctx: CanvasRenderingContext2D,
  field: Field,
  colours: string[],
  progress: number,
  width: number,
  height: number,
  dpr: number,
) {
  ctx.clearRect(0, 0, width, height);
  // On whole device pixels, or the logo's grid blurs into a noisy texture.
  const snap = (v: number) => Math.round(v * dpr) / dpr;

  for (const pulse of field.pulses) {
    ctx.globalAlpha = (1 - pulse.age / PULSE_LIFE) * 0.45;
    ctx.strokeStyle = colours[0]!;
    ctx.lineWidth = 1.5;
    ctx.beginPath();
    ctx.arc(pulse.x, pulse.y, pulse.age * PULSE_SPEED, 0, Math.PI * 2);
    ctx.stroke();
  }

  // One path per colour: a fill call per dot would be far too slow at this count.
  for (let tone = 0; tone < TONES; tone++) {
    ctx.fillStyle = colours[tone]!;
    for (const joined of [false, true]) {
      const size = joined ? JOINED_SIZE : FREE_SIZE;
      ctx.globalAlpha = joined ? 1 : FREE_ALPHA;
      ctx.beginPath();
      for (const p of field.particles) {
        if (p.tone === tone && p.joinsAt <= progress === joined) {
          ctx.rect(snap(p.x - size / 2), snap(p.y - size / 2), size, size);
        }
      }
      ctx.fill();
    }
  }
  ctx.globalAlpha = 1;
}

/**
 * The animated layer of the loading screen. It only draws: what it shows comes from
 * `progress`, which the screen works out from what has really loaded.
 */
export function SplashCanvas({
  progress,
  leaving,
  anchor,
  onPlay,
}: {
  progress: RefObject<number>;
  leaving: boolean;
  /** The element the logo forms over. */
  anchor: RefObject<HTMLElement | null>;
  /** Called on every cursor move or click, so the screen can stay up while being played with. */
  onPlay: () => void;
}) {
  const canvas = useRef<HTMLCanvasElement>(null);
  const field = useRef<Field | null>(null);
  const pointer = useRef<Point | null>(null);

  useEffect(() => {
    const element = canvas.current;
    const ctx = element?.getContext("2d");
    const box = anchor.current;
    if (!element || !ctx || !box) return;
    const colours = toneColours();
    let width = 0;
    let height = 0;
    let dpr = 1;

    const layout = () => {
      const bounds = element.getBoundingClientRect();
      const mark = box.getBoundingClientRect();
      dpr = window.devicePixelRatio || 1;
      width = bounds.width;
      height = bounds.height;
      element.width = width * dpr;
      element.height = height * dpr;
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
      field.current = createField(width, height, {
        x: mark.left + mark.width / 2 - bounds.left,
        y: mark.top + mark.height / 2 - bounds.top,
        size: mark.width,
      });
    };
    layout();
    const resizing = new ResizeObserver(layout);
    resizing.observe(element);

    let frame = 0;
    let last = performance.now();
    let shown = 0;
    const draw = (now: number) => {
      // A hidden window pauses frames: don't let one huge step fling the dots away.
      const dt = Math.min((now - last) / 1000, 1 / 30);
      last = now;
      shown += (progress.current - shown) * (1 - Math.exp(-dt * 9));
      if (field.current) {
        stepField(field.current, dt, { progress: shown, pointer: pointer.current });
        paint(ctx, field.current, colours, shown, width, height, dpr);
      }
      frame = requestAnimationFrame(draw);
    };
    frame = requestAnimationFrame(draw);

    return () => {
      cancelAnimationFrame(frame);
      resizing.disconnect();
    };
  }, [anchor, progress]);

  useEffect(() => {
    if (leaving && field.current) scatter(field.current);
  }, [leaving]);

  const at = (e: PointerEvent<HTMLCanvasElement>): Point => {
    const bounds = e.currentTarget.getBoundingClientRect();
    return { x: e.clientX - bounds.left, y: e.clientY - bounds.top };
  };

  return (
    <canvas
      ref={canvas}
      aria-hidden
      className="absolute inset-0 size-full"
      onPointerMove={(e) => {
        pointer.current = at(e);
        onPlay();
      }}
      onPointerLeave={() => (pointer.current = null)}
      onPointerDown={(e) => {
        if (field.current) addPulse(field.current, at(e));
        onPlay();
      }}
    />
  );
}
