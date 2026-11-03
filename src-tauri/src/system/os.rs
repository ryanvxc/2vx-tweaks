use serde::Serialize;

/// First build number of Windows 11. Builds below this are Windows 10.
pub const WINDOWS_11_FIRST_BUILD: u32 = 22_000;
/// Windows 10 RTM (1507). The minimum build the app supports.
pub const WINDOWS_10_FIRST_BUILD: u32 = 10_240;

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OsInfo {
    /// Marketing name, e.g. "Windows 11 Home".
    pub product_name: String,
    /// Feature-update label, e.g. "24H2". `None` on very old Windows 10 builds.
    pub display_version: Option<String>,
    pub build: u32,
    /// Update build revision (the number after the dot in 26100.2605).
    pub ubr: Option<u32>,
    pub edition_id: Option<String>,
    pub is_windows_11: bool,
    pub architecture: String,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum WindowsGeneration {
    Windows10,
    Windows11,
}

pub fn generation_for_build(build: u32) -> WindowsGeneration {
    if build >= WINDOWS_11_FIRST_BUILD {
        WindowsGeneration::Windows11
    } else {
        WindowsGeneration::Windows10
    }
}

/// `ProductName` in the registry still reads "Windows 10 …" on Windows 11 (Microsoft never
/// updated it), so the generation is derived from the build number instead.
pub fn normalise_product_name(raw: &str, build: u32) -> String {
    match generation_for_build(build) {
        WindowsGeneration::Windows11 if raw.starts_with("Windows 10") => {
            raw.replacen("Windows 10", "Windows 11", 1)
        }
        _ => raw.to_owned(),
    }
}

const CURRENT_VERSION_KEY: &str = r"SOFTWARE\Microsoft\Windows NT\CurrentVersion";

/// Reads OS version details from `HKLM\SOFTWARE\Microsoft\Windows NT\CurrentVersion`.
/// Read-only; accessible to standard users.
pub fn read_os_info() -> OsInfo {
    let key = windows_registry::LOCAL_MACHINE
        .open(CURRENT_VERSION_KEY)
        .ok();
    let get_string = |name: &str| key.as_ref().and_then(|k| k.get_string(name).ok());

    let build = get_string("CurrentBuildNumber")
        .and_then(|b| b.parse::<u32>().ok())
        .or_else(|| {
            sysinfo::System::kernel_version()
                .ok()
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(0);

    let raw_name = get_string("ProductName").unwrap_or_else(|| "Windows".to_owned());

    OsInfo {
        product_name: normalise_product_name(&raw_name, build),
        display_version: get_string("DisplayVersion").or_else(|| get_string("ReleaseId")),
        build,
        ubr: key.as_ref().and_then(|k| k.get_u32("UBR").ok()),
        edition_id: get_string("EditionID"),
        is_windows_11: generation_for_build(build) == WindowsGeneration::Windows11,
        architecture: sysinfo::System::cpu_arch(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn product_name_is_corrected_on_windows_11() {
        assert_eq!(
            normalise_product_name("Windows 10 Pro", 26_100),
            "Windows 11 Pro"
        );
        assert_eq!(
            normalise_product_name("Windows 10 Pro", 19_045),
            "Windows 10 Pro"
        );
        assert_eq!(
            normalise_product_name("Windows 11 Home", 22_631),
            "Windows 11 Home"
        );
    }

    #[test]
    fn generation_boundary() {
        assert_eq!(generation_for_build(21_999), WindowsGeneration::Windows10);
        assert_eq!(generation_for_build(22_000), WindowsGeneration::Windows11);
    }

    #[test]
    fn reads_real_os_info() {
        let info = read_os_info();
        assert!(
            info.build >= WINDOWS_10_FIRST_BUILD,
            "build = {}",
            info.build
        );
        assert!(info.product_name.starts_with("Windows"));
    }
}
