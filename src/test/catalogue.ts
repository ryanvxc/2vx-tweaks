/**
 * The Rust catalogue as tests read it: it's the source of truth, and this catches a tweak added
 * there and not here. A retired tweak keeps its source so undo still reaches it, and `RETIRED` in
 * `tweaks/mod.rs` is what takes it out of the catalogue.
 */
const ROOT = "/src-tauri/src/engine/tweaks/";
const SOURCES = import.meta.glob<string>("/src-tauri/src/engine/tweaks/**/*.rs", {
  query: "?raw",
  import: "default",
  eager: true,
});

/** The Visuals table, as written. */
export const VISUALS_SOURCE = SOURCES[`${ROOT}library/visuals.rs`]!;

const retired = /const RETIRED: &\[SettingsTweak\] = &\[([\s\S]*?)\n\];/.exec(
  SOURCES[`${ROOT}mod.rs`]!,
)![1]!;
const RETIRED_FILES = new Set(
  [...retired.matchAll(/([a-z0-9_:]+)::TWEAK/g)].map(
    (m) => `${ROOT}${m[1]!.replaceAll("::", "/")}.rs`,
  ),
);

/** Every source that can still offer a tweak, for checking an ID exists. */
export const OFFERED_SOURCE = Object.entries(SOURCES)
  .filter(([path]) => !RETIRED_FILES.has(path) && !path.endsWith("/retired.rs"))
  .map(([, source]) => source)
  .join("\n");

/** Tweaks with a file of their own that are still offered. */
export const HANDWRITTEN = Object.entries(SOURCES)
  .filter(([path]) => !RETIRED_FILES.has(path))
  .flatMap(([, source]) =>
    [...source.matchAll(/^\s+id: "([a-z]+\.[a-z0-9-]+)",/gm)].map((m) => m[1]!),
  );

/** Tweaks in the library tables that are still offered. */
export const LIBRARY = Object.entries(SOURCES)
  .filter(([path]) => path.includes("/library/") && !path.endsWith("/retired.rs"))
  .flatMap(([, source]) =>
    [...source.matchAll(/^\s+"([a-z]+\.[a-z0-9-]+)",\r?$/gm)].map((m) => m[1]!),
  );
