use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{GetTokenInformation, TOKEN_ELEVATION, TOKEN_QUERY, TokenElevation};
use windows::Win32::System::Threading::{GetCurrentProcess, OpenProcessToken};

/// Whether the current process runs with an elevated (administrator) token.
///
/// Returns `false` if the token cannot be queried; callers must treat that as
/// "not elevated" and request elevation explicitly when an operation needs it.
pub fn is_elevated() -> bool {
    // SAFETY: GetCurrentProcess returns a pseudo-handle that needs no closing.
    // The token handle is closed on every path after a successful open, and the
    // output buffer is a correctly sized, properly aligned TOKEN_ELEVATION.
    unsafe {
        let mut token = HANDLE::default();
        if OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut token).is_err() {
            return false;
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut returned = 0u32;
        let result = GetTokenInformation(
            token,
            TokenElevation,
            Some(&mut elevation as *mut _ as *mut core::ffi::c_void),
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut returned,
        );
        let _ = CloseHandle(token);

        result.is_ok() && elevation.TokenIsElevated != 0
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn elevation_query_does_not_panic() {
        // The value depends on how the test runner was launched; we only check it resolves.
        let _ = super::is_elevated();
    }
}
