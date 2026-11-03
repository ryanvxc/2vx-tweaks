use std::path::{Path, PathBuf};

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

/// Kept in Tauri state for the lifetime of the app. Dropping the guard flushes
/// buffered log lines, so it must outlive every other component.
pub struct LogState {
    pub guard: WorkerGuard,
    pub log_dir: PathBuf,
}

/// Log files rotate daily; older files beyond this count are removed by the appender.
const MAX_LOG_FILES: usize = 14;

/// Initialises structured logging to a rolling file in `log_dir` and, in debug
/// builds, to stdout. Filter with `RUST_LOG` (defaults to `info`).
pub fn init(log_dir: &Path) -> std::io::Result<WorkerGuard> {
    std::fs::create_dir_all(log_dir)?;

    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("2vx")
        .filename_suffix("log")
        .max_log_files(MAX_LOG_FILES)
        .build(log_dir)
        .map_err(std::io::Error::other)?;
    let (file_writer, guard) = tracing_appender::non_blocking(file_appender);

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let file_layer = fmt::layer()
        .with_writer(file_writer)
        .with_ansi(false)
        .with_target(true);

    let stdout_layer = cfg!(debug_assertions).then(|| fmt::layer().with_target(true));

    // `try_init` so tests or a second init don't panic.
    let _ = tracing_subscriber::registry()
        .with(filter)
        .with(file_layer)
        .with(stdout_layer)
        .try_init();

    Ok(guard)
}

/// Upper bound on a single frontend log message, to stop the UI flooding the log file.
pub const MAX_FRONTEND_MESSAGE_LEN: usize = 2_000;

/// Strips control characters (except tab) and truncates on a char boundary.
pub fn sanitise_message(input: &str) -> String {
    input
        .chars()
        .filter(|c| !c.is_control() || *c == '\t')
        .take(MAX_FRONTEND_MESSAGE_LEN)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitise_strips_newlines_and_truncates() {
        assert_eq!(sanitise_message("a\nb\r\x1bc\td"), "abc\td");
        let long = "é".repeat(MAX_FRONTEND_MESSAGE_LEN + 50);
        assert_eq!(
            sanitise_message(&long).chars().count(),
            MAX_FRONTEND_MESSAGE_LEN
        );
    }

    #[test]
    fn init_creates_log_directory() {
        let dir = tempfile::tempdir().unwrap();
        let log_dir = dir.path().join("nested").join("logs");
        let _guard = init(&log_dir).unwrap();
        assert!(log_dir.is_dir());
    }
}
