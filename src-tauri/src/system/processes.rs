//! Which programs are running, for Game Mode's detected game. Read-only.
//!
//! The command takes no arguments. The user's game list stays in the UI, which compares it with
//! this list, so a process name never reaches a command. Nothing is launched, stopped or changed,
//! and the list isn't logged.

use std::collections::HashMap;

use serde::Serialize;
use sysinfo::{ProcessRefreshKind, ProcessesToUpdate, System, UpdateKind};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningProgram {
    pub file_name: String,
    pub running_seconds: u64,
    /// The game's folder, when it runs from a game library. The rest of the path stays here.
    pub library_game: Option<String>,
}

/// Where game launchers install, in lowercase. Each holds one folder per game.
const LIBRARIES: [&str; 8] = [
    r"\steamapps\common\",
    r"\epic games\",
    r"\gog games\",
    r"\gog galaxy\games\",
    r"\xboxgames\",
    r"\ubisoft game launcher\games\",
    r"\ea games\",
    r"\riot games\",
];

/// Folders in those libraries that aren't games.
const NOT_GAMES: [&str; 5] = [
    "launcher",
    "directxredist",
    "riot client",
    "steamworks shared",
    "gamesave",
];

/// The game folder in an executable's path, if the path is inside a game library.
fn library_game(path: &str) -> Option<String> {
    // ASCII lowercasing keeps byte offsets, so positions found here are valid in `path`.
    let lower = path.to_ascii_lowercase();
    LIBRARIES.iter().find_map(|library| {
        let start = lower.find(library)? + library.len();
        let (folder, _rest) = path[start..].split_once('\\')?;
        let game = !folder.is_empty() && !NOT_GAMES.contains(&&*folder.to_ascii_lowercase());
        game.then(|| folder.to_owned())
    })
}

/// One entry per file name, keeping the instance that has run longest. Launchers and games often
/// start several processes, and only the whole session matters here.
fn merge(programs: impl IntoIterator<Item = RunningProgram>) -> Vec<RunningProgram> {
    let mut by_name: HashMap<String, RunningProgram> = HashMap::new();
    for program in programs {
        let key = program.file_name.to_ascii_lowercase();
        match by_name.get(&key) {
            Some(known) if known.running_seconds >= program.running_seconds => {}
            _ => {
                by_name.insert(key, program);
            }
        }
    }
    by_name.into_values().collect()
}

pub fn running_programs() -> Vec<RunningProgram> {
    let Ok(mut system) = System::new() else {
        return Vec::new();
    };
    system.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_exe(UpdateKind::Always),
    );
    merge(system.processes().values().filter_map(|process| {
        let file_name = process.name().to_string_lossy().into_owned();
        (!file_name.is_empty()).then(|| RunningProgram {
            file_name,
            running_seconds: process.run_time(),
            library_game: process
                .exe()
                .and_then(|path| library_game(&path.to_string_lossy())),
        })
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn program(file_name: &str, running_seconds: u64) -> RunningProgram {
        RunningProgram {
            file_name: file_name.to_owned(),
            running_seconds,
            library_game: None,
        }
    }

    #[test]
    fn finds_the_game_folder_in_each_library() {
        for (path, game) in [
            (
                r"D:\SteamLibrary\steamapps\common\Counter-Strike Global Offensive\game\bin\cs2.exe",
                "Counter-Strike Global Offensive",
            ),
            (
                r"C:\Program Files\Epic Games\Fortnite\Bin\f.exe",
                "Fortnite",
            ),
            (
                r"C:\GOG Games\Cyberpunk 2077\bin\game.exe",
                "Cyberpunk 2077",
            ),
            (
                r"C:\XboxGames\Forza Horizon 5\Content\f.exe",
                "Forza Horizon 5",
            ),
            (r"C:\Riot Games\VALORANT\live\v.exe", "VALORANT"),
        ] {
            assert_eq!(library_game(path).as_deref(), Some(game), "{path}");
        }
    }

    #[test]
    fn keeps_non_ascii_folder_names_intact() {
        assert_eq!(
            library_game(r"E:\STEAM\steamapps\common\Pokémon Ünite\ü.exe").as_deref(),
            Some("Pokémon Ünite"),
        );
    }

    #[test]
    fn ignores_launchers_and_everything_else() {
        for path in [
            r"C:\Windows\explorer.exe",
            r"C:\Program Files (x86)\Steam\steam.exe",
            r"C:\Program Files\Epic Games\Launcher\Portal\launcher.exe",
            r"C:\Riot Games\Riot Client\client.exe",
            r"D:\steamapps\common\Steamworks Shared\redist.exe",
            r"D:\steamapps\common\loose.exe",
        ] {
            assert_eq!(library_game(path), None, "{path}");
        }
    }

    #[test]
    fn merges_by_name_and_keeps_the_longest_running() {
        let merged = merge([
            program("game.exe", 5),
            program("Game.exe", 90),
            program("other.exe", 1),
        ]);
        assert_eq!(merged.len(), 2);
        let game = merged.iter().find(|p| p.running_seconds == 90);
        assert_eq!(game.unwrap().file_name, "Game.exe");
    }

    #[test]
    fn reads_this_process_from_the_real_system() {
        assert!(!running_programs().is_empty());
    }
}
