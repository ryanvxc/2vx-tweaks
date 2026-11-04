//! The local account: one email address in one file in this user's app data folder.
//!
//! There's no server, password or sign-in behind it. The address names the account card in the
//! sidebar and nothing else: it never leaves the PC and never reaches the logs.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, AppResult};

const FILE_NAME: &str = "account.json";
const MAX_EMAIL_LEN: usize = 254;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub email: String,
}

fn is_email(email: &str) -> bool {
    let plain = !email.chars().any(|c| c.is_whitespace() || c.is_control());
    let Some((local, domain)) = email.split_once('@') else {
        return false;
    };
    plain
        && email.len() <= MAX_EMAIL_LEN
        && !local.is_empty()
        && !domain.contains('@')
        && domain.contains('.')
        && domain.split('.').all(|label| !label.is_empty())
}

/// The saved account. A missing or damaged file is simply no account: nothing depends on it.
pub fn read(dir: &Path) -> Option<Account> {
    let text = fs::read_to_string(dir.join(FILE_NAME)).ok()?;
    let account: Account = serde_json::from_str(&text).ok()?;
    is_email(&account.email).then_some(account)
}

pub fn write(dir: &Path, email: &str) -> AppResult<Account> {
    let email = email.trim();
    if !is_email(email) {
        return Err(AppError::InvalidInput("not an email address".into()));
    }
    let account = Account {
        email: email.to_owned(),
    };
    let json = serde_json::to_string(&account).map_err(|e| AppError::Internal(e.to_string()))?;
    fs::create_dir_all(dir)?;
    // Written beside the real file first, so a crash can't leave half an address behind.
    let temp = dir.join(format!("{FILE_NAME}.tmp"));
    fs::write(&temp, json)?;
    fs::rename(&temp, dir.join(FILE_NAME))?;
    Ok(account)
}

pub fn remove(dir: &Path) -> AppResult<()> {
    match fs::remove_file(dir.join(FILE_NAME)) {
        Err(e) if e.kind() != std::io::ErrorKind::NotFound => Err(e.into()),
        _ => Ok(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saves_reads_and_removes_an_account() {
        let dir = tempfile::tempdir().unwrap();
        assert_eq!(read(dir.path()), None);

        let saved = write(dir.path(), "  someone@example.com ").unwrap();
        assert_eq!(saved.email, "someone@example.com");
        assert_eq!(read(dir.path()), Some(saved));

        remove(dir.path()).unwrap();
        assert_eq!(read(dir.path()), None);
        remove(dir.path()).unwrap();
    }

    #[test]
    fn replaces_an_existing_account() {
        let dir = tempfile::tempdir().unwrap();
        write(dir.path(), "first@example.com").unwrap();
        write(dir.path(), "second@example.com").unwrap();
        assert_eq!(read(dir.path()).unwrap().email, "second@example.com");
    }

    #[test]
    fn refuses_anything_that_is_not_an_email() {
        let dir = tempfile::tempdir().unwrap();
        let long = format!("{}@example.com", "a".repeat(250));
        for bad in [
            "",
            "someone",
            "@example.com",
            "someone@",
            "someone@example",
            "someone@.com",
            "someone@example.",
            "a@b@example.com",
            "some one@example.com",
            "some\none@example.com",
            long.as_str(),
        ] {
            assert!(write(dir.path(), bad).is_err(), "accepted {bad:?}");
        }
        assert_eq!(read(dir.path()), None);
    }

    #[test]
    fn treats_a_damaged_file_as_no_account() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join(FILE_NAME), "not json").unwrap();
        assert_eq!(read(dir.path()), None);
        fs::write(dir.path().join(FILE_NAME), r#"{"email":"nope"}"#).unwrap();
        assert_eq!(read(dir.path()), None);
    }
}
