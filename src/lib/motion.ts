import { useUiStore } from "@/stores/ui-store";

// Matches --ease-arrive in globals.css. Web Animations can't read it from a theme token.
const ARRIVE = "cubic-bezier(0.16, 1, 0.3, 1)";

/**
 * A one-shot animation started from code, for moments CSS can't key off: the CSS kill switch
 * in globals.css doesn't reach these, so Reduce motion is checked here.
 */
export function play(
  el: Element | null | undefined,
  keyframes: Keyframe[],
  options: KeyframeAnimationOptions,
) {
  if (!el || typeof el.animate !== "function") return; // jsdom has no Web Animations
  if (useUiStore.getState().reduceMotion) return;
  el.animate(keyframes, { easing: ARRIVE, ...options });
}
