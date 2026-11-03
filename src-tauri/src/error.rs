use serde::{Serialize, Serializer, ser::SerializeStruct};

/// The single error type returned by every Tauri command.
///
/// Serialised to the frontend as `{ code, message }` so the UI can branch on a
/// stable `code` instead of parsing human-readable text.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid input: {0}")]
    InvalidInput(String),

    #[error("not supported on this system: {0}")]
    Unsupported(String),

    #[error("administrator privileges are required: {0}")]
    ElevationRequired(String),

    #[error("system query failed: {0}")]
    System(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("internal error: {0}")]
    Internal(String),
}

impl AppError {
    pub fn code(&self) -> &'static str {
        match self {
            AppError::InvalidInput(_) => "invalid_input",
            AppError::Unsupported(_) => "unsupported",
            AppError::ElevationRequired(_) => "elevation_required",
            AppError::System(_) => "system",
            AppError::Io(_) => "io",
            AppError::Internal(_) => "internal",
        }
    }
}

impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut s = serializer.serialize_struct("AppError", 2)?;
        s.serialize_field("code", self.code())?;
        s.serialize_field("message", &self.to_string())?;
        s.end()
    }
}

pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serialises_with_stable_code() {
        let json = serde_json::to_value(AppError::InvalidInput("bad level".into())).unwrap();
        assert_eq!(json["code"], "invalid_input");
        assert_eq!(json["message"], "invalid input: bad level");
    }
}
