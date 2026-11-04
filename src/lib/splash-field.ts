/**
 * Physics for the loading screen: a dot-matrix 2VX logo that forms as the app loads and
 * scatters around the cursor. No DOM here, so it can be tested without a canvas.
 */

// The logo's shapes, from src/assets/logo.svg (a 1024 unit viewBox).
const SQUARE = { centre: 512, half: 448, radius: 208 };
const CHEVRON_HALF_WIDTH = 46;
const CHEVRONS = [
  [
    [300, 330],
    [470, 512],
    [300, 694],
  ],
  [
    [560, 330],
    [730, 512],
    [560, 694],
  ],
] as const;

export const DOT_SPACING = 4;
const DUST = 110;

// A joinsAt above this never joins the logo, whatever the progress.
const NEVER = 2;
// The last dot joins before progress ends, so the logo is whole while the screen holds.
const LAST_JOIN = 0.8;

const PULL_HOME = 120;
const PULL_FREE = 5;
const DAMPING = 16;
const POINTER_RADIUS = 120;
const POINTER_FORCE = 9000;
export const PULSE_SPEED = 640;
const PULSE_BAND = 46;
const PULSE_FORCE = 14000;
export const PULSE_LIFE = 0.9;

/** Tones 0 to 5 run violet to magenta, then the two chevrons. */
export const GRADIENT_TONES = 6;
export const TONE_CHEVRON = 6;
export const TONE_CHEVRON_SOFT = 7;
export const TONES = 8;

export interface Point {
  x: number;
  y: number;
}

/** Where the logo forms. */
export interface Anchor extends Point {
  size: number;
}

export interface Particle extends Point {
  vx: number;
  vy: number;
  homeX: number;
  homeY: number;
  freeX: number;
  freeY: number;
  /** Loading progress at which this dot joins the logo. */
  joinsAt: number;
  tone: number;
  phase: number;
}

export interface Pulse extends Point {
  age: number;
}

export interface Field {
  anchor: Anchor;
  particles: Particle[];
  pulses: Pulse[];
  time: number;
  /** The exit has begun, so nothing heads home any more. */
  scattered: boolean;
}

function insideSquare(u: number, v: number) {
  const { centre, half, radius } = SQUARE;
  const qx = Math.abs(u - centre) - (half - radius);
  const qy = Math.abs(v - centre) - (half - radius);
  const outside = Math.hypot(Math.max(qx, 0), Math.max(qy, 0));
  return outside + Math.min(Math.max(qx, qy), 0) <= radius;
}

function distanceToSegment(u: number, v: number, a: readonly number[], b: readonly number[]) {
  const [ax = 0, ay = 0] = a;
  const [bx = 0, by = 0] = b;
  const dx = bx - ax;
  const dy = by - ay;
  const t = Math.min(1, Math.max(0, ((u - ax) * dx + (v - ay) * dy) / (dx * dx + dy * dy)));
  return Math.hypot(u - ax - t * dx, v - ay - t * dy);
}

function insideChevron(u: number, v: number, [a, b, c]: (typeof CHEVRONS)[number]) {
  return (
    Math.min(distanceToSegment(u, v, a, b), distanceToSegment(u, v, b, c)) <= CHEVRON_HALF_WIDTH
  );
}

function toneAt(u: number, v: number) {
  const [first, second] = CHEVRONS;
  if (insideChevron(u, v, first)) return TONE_CHEVRON;
  if (insideChevron(u, v, second)) return TONE_CHEVRON_SOFT;
  // The logo's gradient runs corner to corner.
  const { centre, half } = SQUARE;
  const along = (u + v - 2 * (centre - half)) / (4 * half);
  return Math.min(GRADIENT_TONES - 1, Math.floor(along * GRADIENT_TONES));
}

export function createField(
  width: number,
  height: number,
  anchor: Anchor,
  random: () => number = Math.random,
): Field {
  const { centre, half } = SQUARE;
  const cells = Math.round(anchor.size / DOT_SPACING);
  const left = anchor.x - anchor.size / 2;
  const top = anchor.y - anchor.size / 2;

  const dot = (homeX: number, homeY: number, joinsAt: number, tone: number): Particle => {
    const freeX = random() * width;
    const freeY = random() * height;
    return {
      x: freeX,
      y: freeY,
      vx: 0,
      vy: 0,
      homeX,
      homeY,
      freeX,
      freeY,
      joinsAt,
      tone,
      phase: random() * Math.PI * 2,
    };
  };

  const particles: Particle[] = [];
  for (let row = 0; row < cells; row++) {
    for (let col = 0; col < cells; col++) {
      const u = centre - half + ((col + 0.5) / cells) * 2 * half;
      const v = centre - half + ((row + 0.5) / cells) * 2 * half;
      if (!insideSquare(u, v)) continue;
      const homeX = left + (col + 0.5) * DOT_SPACING;
      const homeY = top + (row + 0.5) * DOT_SPACING;
      particles.push(dot(homeX, homeY, random() * LAST_JOIN, toneAt(u, v)));
    }
  }
  for (let i = 0; i < DUST; i++) {
    particles.push(dot(0, 0, NEVER, Math.floor(random() * GRADIENT_TONES)));
  }
  return { anchor, particles, pulses: [], time: 0, scattered: false };
}

export function addPulse(field: Field, at: Point) {
  field.pulses.push({ x: at.x, y: at.y, age: 0 });
}

/** Sends every dot outward for the exit. */
export function scatter(field: Field) {
  const { x, y } = field.anchor;
  field.scattered = true;
  for (const p of field.particles) {
    const dx = p.x - x;
    const dy = p.y - y;
    const distance = Math.hypot(dx, dy) || 1;
    const speed = 1400 + (p.phase / (Math.PI * 2)) * 2000;
    p.vx += (dx / distance) * speed;
    p.vy += (dy / distance) * speed;
  }
}

export function stepField(
  field: Field,
  dt: number,
  { progress, pointer }: { progress: number; pointer: Point | null },
) {
  field.time += dt;
  for (const pulse of field.pulses) pulse.age += dt;
  field.pulses = field.pulses.filter((pulse) => pulse.age < PULSE_LIFE);
  const friction = Math.exp(-DAMPING * dt);

  for (const p of field.particles) {
    const joined = !field.scattered && p.joinsAt <= progress;
    // Free dots drift a little, so the field never looks frozen.
    const drift = field.time * 0.5 + p.phase;
    const targetX = joined ? p.homeX : p.freeX + Math.sin(drift) * 14;
    const targetY = joined ? p.homeY : p.freeY + Math.cos(drift * 0.8) * 14;
    const pull = joined ? PULL_HOME : PULL_FREE;
    let ax = (targetX - p.x) * pull;
    let ay = (targetY - p.y) * pull;

    if (pointer) {
      const dx = p.x - pointer.x;
      const dy = p.y - pointer.y;
      const distance = Math.hypot(dx, dy) || 1;
      if (distance < POINTER_RADIUS) {
        const strength = (1 - distance / POINTER_RADIUS) ** 2 * POINTER_FORCE;
        ax += (dx / distance) * strength;
        ay += (dy / distance) * strength;
      }
    }

    for (const pulse of field.pulses) {
      const dx = p.x - pulse.x;
      const dy = p.y - pulse.y;
      const distance = Math.hypot(dx, dy) || 1;
      const offBand = Math.abs(distance - pulse.age * PULSE_SPEED);
      if (offBand < PULSE_BAND) {
        const strength = (1 - offBand / PULSE_BAND) * (1 - pulse.age / PULSE_LIFE) * PULSE_FORCE;
        ax += (dx / distance) * strength;
        ay += (dy / distance) * strength;
      }
    }

    p.vx = (p.vx + ax * dt) * friction;
    p.vy = (p.vy + ay * dt) * friction;
    p.x += p.vx * dt;
    p.y += p.vy * dt;
  }
}
