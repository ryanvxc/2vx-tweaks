import { knownGame } from "@/lib/known-games";
import type { RunningProgram } from "@/types/system";

/**
 * A game the user added. The list stays in the UI and is compared with the running programs
 * here, so a process name never reaches a backend command.
 */
export interface SavedGame {
  name: string;
  /** File name only, such as game.exe. It's matched, never launched. */
  exe: string;
}

export interface DetectedGame {
  name: string;
  runningSeconds: number;
}

const FILE_NAME = /^[^\\/:*?"<>|]+\.exe$/i;

const longestRunning = (games: DetectedGame[]) =>
  games.reduce<DetectedGame | null>(
    (best, game) => (best && best.runningSeconds >= game.runningSeconds ? best : game),
    null,
  );

/** Games the user added come first, because they chose those names. Then what a library lists. */
export function detectGame(programs: RunningProgram[], games: SavedGame[]): DetectedGame | null {
  const running = new Map(programs.map((p) => [p.fileName.toLowerCase(), p]));
  const added = games.flatMap((game) => {
    const program = running.get(game.exe.toLowerCase());
    return program ? [{ name: game.name, runningSeconds: program.runningSeconds }] : [];
  });
  const library = programs.flatMap((p) =>
    p.libraryGame ? [{ name: p.libraryGame, runningSeconds: p.runningSeconds }] : [],
  );
  const known = programs.flatMap((p) => {
    const name = knownGame(p.fileName);
    return name ? [{ name, runningSeconds: p.runningSeconds }] : [];
  });
  return longestRunning(added) ?? longestRunning(library) ?? longestRunning(known);
}

/** What's wrong with a new game, or `null` when it can be added. */
export function gameProblem(game: SavedGame, games: SavedGame[]): string | null {
  if (game.name.trim() === "") return "Enter a name.";
  if (!FILE_NAME.test(game.exe.trim())) return "Enter a file name ending in .exe.";
  const exe = game.exe.trim().toLowerCase();
  return games.some((g) => g.exe.toLowerCase() === exe) ? "Already added." : null;
}
