use std::path::PathBuf;

use tauri::{AppHandle, Manager};

use crate::account::{self, Account};
use crate::error::{AppError, AppResult};

// %LOCALAPPDATA%\com.twovx.tweaks: per-user and not shared with other accounts.
fn data_dir(app: &AppHandle) -> AppResult<PathBuf> {
    app.path()
        .app_local_data_dir()
        .map_err(|e| AppError::Internal(e.to_string()))
}

#[tauri::command]
pub fn get_account(app: AppHandle) -> AppResult<Option<Account>> {
    Ok(account::read(&data_dir(&app)?))
}

#[tauri::command]
pub fn set_account(app: AppHandle, email: String) -> AppResult<Account> {
    account::write(&data_dir(&app)?, &email)
}

#[tauri::command]
pub fn clear_account(app: AppHandle) -> AppResult<()> {
    account::remove(&data_dir(&app)?)
}
