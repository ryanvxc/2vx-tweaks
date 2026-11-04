import { describe, expect, it } from "vitest";
import { FACTS } from "./splash-facts";
import {
  addPulse,
  createField,
  PULSE_LIFE,
  scatter,
  stepField,
  TONE_CHEVRON,
  TONE_CHEVRON_SOFT,
  type Field,
  type Point,
} from "./splash-field";

const ANCHOR = { x: 400, y: 300, size: 168 };

function seeded() {
  let state = 7;
  return () => {
    state = (state * 16807) % 2147483647;
    return state / 2147483647;
  };
}

const field = () => createField(800, 600, ANCHOR, seeded());
const logoDots = (f: Field) => f.particles.filter((p) => p.joinsAt <= 1);

function run(f: Field, seconds: number, progress: number, pointer: Point | null = null) {
  for (let t = 0; t < seconds; t += 1 / 60) stepField(f, 1 / 60, { progress, pointer });
}

const offHome = (p: Field["particles"][number]) => Math.hypot(p.x - p.homeX, p.y - p.homeY);

describe("loading screen field", () => {
  it("builds the logo inside its square, with both chevrons", () => {
    const dots = logoDots(field());
    expect(dots.length).toBeGreaterThan(1000);
    for (const p of dots) {
      expect(Math.abs(p.homeX - ANCHOR.x)).toBeLessThanOrEqual(ANCHOR.size / 2);
      expect(Math.abs(p.homeY - ANCHOR.y)).toBeLessThanOrEqual(ANCHOR.size / 2);
    }
    expect(dots.filter((p) => p.tone === TONE_CHEVRON).length).toBeGreaterThan(50);
    expect(dots.filter((p) => p.tone === TONE_CHEVRON_SOFT).length).toBeGreaterThan(50);
  });

  it("forms the logo only as progress allows", () => {
    const f = field();
    run(f, 1, 0);
    expect(logoDots(f).filter((p) => offHome(p) < 2)).toHaveLength(0);

    run(f, 3, 1);
    expect(logoDots(f).every((p) => offHome(p) < 1)).toBe(true);
  });

  it("moves dots away from the cursor, then lets them settle back", () => {
    const f = field();
    run(f, 3, 1);
    const cursor = { x: ANCHOR.x, y: ANCHOR.y };
    run(f, 0.5, 1, cursor);
    expect(Math.max(...logoDots(f).map(offHome))).toBeGreaterThan(5);

    run(f, 3, 1);
    expect(logoDots(f).every((p) => offHome(p) < 1)).toBe(true);
  });

  it("pushes dots with a pulse that dies away", () => {
    const f = field();
    run(f, 3, 1);
    addPulse(f, { x: ANCHOR.x, y: ANCHOR.y });
    run(f, 0.1, 1);
    expect(Math.max(...logoDots(f).map(offHome))).toBeGreaterThan(5);

    run(f, PULSE_LIFE, 1);
    expect(f.pulses).toHaveLength(0);
  });

  it("sends everything away for the exit", () => {
    const f = field();
    run(f, 3, 1);
    scatter(f);
    run(f, 0.3, 1);
    expect(logoDots(f).filter((p) => offHome(p) < 5)).toHaveLength(0);
  });
});

describe("loading screen facts", () => {
  it("fit on two lines", () => {
    for (const fact of FACTS) expect(fact.length).toBeLessThanOrEqual(100);
  });
});
