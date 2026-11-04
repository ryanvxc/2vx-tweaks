import { describe, expect, it } from "vitest";
import type { RunningProgram } from "@/types/system";
import { detectGame, gameProblem } from "./game-mode";

const program = (
  fileName: string,
  runningSeconds: number,
  libraryGame: string | null = null,
): RunningProgram => ({ fileName, runningSeconds, libraryGame });

describe("detectGame", () => {
  it("finds nothing when no game runs", () => {
    expect(detectGame([program("explorer.exe", 900)], [])).toBeNull();
    expect(detectGame([], [{ name: "Game", exe: "game.exe" }])).toBeNull();
  });

  it("matches an added game by file name, ignoring case", () => {
    const games = [{ name: "My Game", exe: "GAME.exe" }];
    expect(detectGame([program("game.EXE", 60)], games)).toEqual({
      name: "My Game",
      runningSeconds: 60,
    });
  });

  it("finds a game in a library without being told about it", () => {
    expect(detectGame([program("cs2.exe", 30, "Counter-Strike")], [])).toEqual({
      name: "Counter-Strike",
      runningSeconds: 30,
    });
  });

  it("prefers an added game, and the longest running of several", () => {
    const programs = [
      program("library.exe", 999, "Library Game"),
      program("a.exe", 10),
      program("b.exe", 20),
    ];
    const games = [
      { name: "A", exe: "a.exe" },
      { name: "B", exe: "b.exe" },
    ];
    expect(detectGame(programs, games)?.name).toBe("B");
  });
});

describe("gameProblem", () => {
  const games = [{ name: "Game", exe: "game.exe" }];

  it("accepts a name and a file name", () => {
    expect(gameProblem({ name: "New", exe: " new game.exe " }, games)).toBeNull();
  });

  it.each([
    ["", "new.exe", "Enter a name."],
    ["New", "new", "Enter a file name ending in .exe."],
    ["New", "C:\\Games\\new.exe", "Enter a file name ending in .exe."],
    ["New", "../new.exe", "Enter a file name ending in .exe."],
    ["New", "GAME.EXE", "Already added."],
  ])("refuses %j %j", (name, exe, problem) => {
    expect(gameProblem({ name, exe }, games)).toBe(problem);
  });
});
