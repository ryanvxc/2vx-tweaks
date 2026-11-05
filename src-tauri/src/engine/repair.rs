//! Windows' two repair tools, the one place 2VX starts a program for its output (decision P0-8c in
//! `docs/security.md`). Both commands are constants, nothing the app knows reaches them, and they
//! always run from the real System32. `tests/policy.rs` holds this file to exactly these two.

use std::os::windows::process::CommandExt;
use std::process::Command;

use serde::{Deserialize, Serialize};

use crate::engine::broker::BrokerSession;
use crate::error::{AppError, AppResult};
use crate::links;
use crate::system::privilege;

/// `CREATE_NO_WINDOW`: the tool's output comes back to the page instead.
const NO_WINDOW: u32 = 0x0800_0000;
/// Keeps the reply well inside the broker's frame limit. The end of a run holds the verdict.
const MAX_OUTPUT_CHARS: usize = 16_000;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum RepairTool {
    SystemFiles,
    WindowsImage,
}

impl RepairTool {
    pub const fn command(self) -> (&'static str, &'static [&'static str]) {
        match self {
            Self::SystemFiles => ("sfc.exe", &["/scannow"]),
            Self::WindowsImage => ("dism.exe", &["/Online", "/Cleanup-Image", "/RestoreHealth"]),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepairOutcome {
    pub tool: RepairTool,
    pub succeeded: bool,
    /// What the tool printed, ending where it ended.
    pub output: String,
}

/// `sfc` writes UTF-16 when its output isn't a console, and `dism` writes the console code page.
fn decode(bytes: &[u8]) -> String {
    let wide = bytes.iter().filter(|&&b| b == 0).count() * 4 > bytes.len();
    if wide {
        let units: Vec<u16> = bytes
            .as_chunks::<2>()
            .0
            .iter()
            .map(|pair| u16::from_le_bytes(*pair))
            .collect();
        String::from_utf16_lossy(&units)
    } else {
        String::from_utf8_lossy(bytes).into_owned()
    }
}

/// Keeps the last state of each line, because both tools redraw a progress bar with `\r`.
fn tidy(text: &str) -> String {
    let lines: Vec<&str> = text
        .split('\n')
        .map(|line| line.trim_end_matches('\r'))
        .map(|line| line.rsplit('\r').next().unwrap_or(line).trim_end())
        .filter(|line| !line.is_empty())
        .collect();
    let joined = lines.join("\n");
    let extra = joined.chars().count().saturating_sub(MAX_OUTPUT_CHARS);
    joined.chars().skip(extra).collect()
}

/// Runs in this process, which must be elevated. The broker's side of [`run`].
pub fn run_here(tool: RepairTool) -> AppResult<RepairOutcome> {
    let (file, arguments) = tool.command();
    let output = Command::new(links::system_directory()?.join(file))
        .args(arguments)
        .creation_flags(NO_WINDOW)
        .output()
        .map_err(|e| AppError::System(format!("couldn't start {file}: {e}")))?;
    Ok(RepairOutcome {
        tool,
        succeeded: output.status.success(),
        output: tidy(&decode(&output.stdout)),
    })
}

/// One UAC prompt, unless the app is already elevated. Either tool can take many minutes.
pub fn run(tool: RepairTool) -> AppResult<RepairOutcome> {
    if privilege::is_elevated() {
        run_here(tool)
    } else {
        BrokerSession::launch()?.repair(tool)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wide_output_is_read_as_wide() {
        let wide: Vec<u8> = "Verification 100%"
            .encode_utf16()
            .flat_map(u16::to_le_bytes)
            .collect();
        assert_eq!(decode(&wide), "Verification 100%");
        assert_eq!(
            decode(b"The operation completed."),
            "The operation completed."
        );
    }

    #[test]
    fn progress_redraws_leave_only_their_last_state() {
        let text = "Begin\r\n  10%\r  55%\r 100% done\r\n\r\nFinished\r\n";
        assert_eq!(tidy(text), "Begin\n 100% done\nFinished");
    }

    #[test]
    fn long_output_keeps_its_end() {
        let text = format!("{}END", "x".repeat(MAX_OUTPUT_CHARS * 2));
        let kept = tidy(&text);
        assert_eq!(kept.chars().count(), MAX_OUTPUT_CHARS);
        assert!(kept.ends_with("END"));
    }
}
