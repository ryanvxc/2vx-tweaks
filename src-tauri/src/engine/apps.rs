//! Built-in Windows apps the user can remove and bring back.
//!
//! Only the packages in [`APPS`] can be touched, by ID. Everything goes through WinRT's
//! `PackageManager` for the current user, so no shell is involved and no other account is
//! affected. Reinstalling re-registers the copy Windows keeps; once that's gone the app has to
//! come from Microsoft Store.

use std::path::{Path, PathBuf};

use serde::Serialize;
use windows::ApplicationModel::Package;
use windows::Management::Deployment::{DeploymentOptions, PackageManager, PackageVolume};
use windows::core::HSTRING;
use windows_collections::IIterable;

use crate::error::{AppError, AppResult};

/// Logos are small; anything bigger isn't worth sending through the bridge.
const MAX_ICON_BYTES: u64 = 64 * 1024;
/// Sharpest first: the row shows these at 32px, and displays are often scaled.
const ICON_VARIANTS: &[&str] = &[
    ".scale-200",
    ".scale-150",
    ".scale-125",
    ".scale-100",
    ".targetsize-48",
    ".targetsize-44",
    "",
];

struct App {
    id: &'static str,
    name: &'static str,
    description: &'static str,
    family: &'static str,
}

const fn app(
    id: &'static str,
    name: &'static str,
    description: &'static str,
    family: &'static str,
) -> App {
    App {
        id,
        name,
        description,
        family,
    }
}

/// Leaves out what Windows or other apps rely on: Microsoft Store, App Installer, Windows
/// Security, Edge, the Xbox sign-in components and every framework package.
const APPS: &[App] = &[
    app(
        "3d-viewer",
        "3D Viewer",
        "View 3D models",
        "Microsoft.Microsoft3DViewer_8wekyb3d8bbwe",
    ),
    app(
        "alarms",
        "Alarms & Clock",
        "Alarms, timers and world clocks",
        "Microsoft.WindowsAlarms_8wekyb3d8bbwe",
    ),
    app(
        "camera",
        "Camera",
        "Take photos and videos",
        "Microsoft.WindowsCamera_8wekyb3d8bbwe",
    ),
    app(
        "clipchamp",
        "Clipchamp",
        "Video editor",
        "Clipchamp.Clipchamp_yxz26nhyzhsrt",
    ),
    app(
        "copilot",
        "Copilot",
        "Microsoft's AI assistant",
        "Microsoft.Copilot_8wekyb3d8bbwe",
    ),
    app(
        "dev-home",
        "Dev Home",
        "Developer dashboard",
        "Microsoft.Windows.DevHome_8wekyb3d8bbwe",
    ),
    app(
        "family",
        "Family Safety",
        "Screen time and content limits",
        "MicrosoftCorporationII.MicrosoftFamily_8wekyb3d8bbwe",
    ),
    app(
        "feedback-hub",
        "Feedback Hub",
        "Send feedback to Microsoft",
        "Microsoft.WindowsFeedbackHub_8wekyb3d8bbwe",
    ),
    app(
        "get-help",
        "Get Help",
        "Microsoft support",
        "Microsoft.GetHelp_8wekyb3d8bbwe",
    ),
    app(
        "mail-calendar",
        "Mail and Calendar",
        "Classic mail and calendar apps",
        "microsoft.windowscommunicationsapps_8wekyb3d8bbwe",
    ),
    app(
        "maps",
        "Maps",
        "Maps and directions",
        "Microsoft.WindowsMaps_8wekyb3d8bbwe",
    ),
    app(
        "media-player",
        "Media Player",
        "Music and video player, once Groove Music",
        "Microsoft.ZuneMusic_8wekyb3d8bbwe",
    ),
    app(
        "office-hub",
        "Microsoft 365",
        "Hub for Office apps and files",
        "Microsoft.MicrosoftOfficeHub_8wekyb3d8bbwe",
    ),
    app(
        "journal",
        "Microsoft Journal",
        "Handwritten notes",
        "Microsoft.MicrosoftJournal_8wekyb3d8bbwe",
    ),
    app(
        "loop",
        "Microsoft Loop",
        "Shared workspaces",
        "Microsoft.MicrosoftLoop_8wekyb3d8bbwe",
    ),
    app(
        "mahjong",
        "Microsoft Mahjong",
        "Tile game",
        "Microsoft.MicrosoftMahjong_8wekyb3d8bbwe",
    ),
    app(
        "minesweeper",
        "Microsoft Minesweeper",
        "Puzzle game",
        "Microsoft.MicrosoftMinesweeper_8wekyb3d8bbwe",
    ),
    app(
        "solitaire",
        "Microsoft Solitaire Collection",
        "Card games",
        "Microsoft.MicrosoftSolitaireCollection_8wekyb3d8bbwe",
    ),
    app(
        "teams",
        "Microsoft Teams",
        "Chat and meetings",
        "MSTeams_8wekyb3d8bbwe",
    ),
    app(
        "todo",
        "Microsoft To Do",
        "Lists and tasks",
        "Microsoft.Todos_8wekyb3d8bbwe",
    ),
    app(
        "movies-tv",
        "Movies & TV",
        "Video player and store",
        "Microsoft.ZuneVideo_8wekyb3d8bbwe",
    ),
    app(
        "news",
        "News",
        "News feed",
        "Microsoft.BingNews_8wekyb3d8bbwe",
    ),
    app(
        "onenote",
        "OneNote",
        "Notebooks",
        "Microsoft.Office.OneNote_8wekyb3d8bbwe",
    ),
    app(
        "outlook",
        "Outlook",
        "The new Outlook for Windows",
        "Microsoft.OutlookForWindows_8wekyb3d8bbwe",
    ),
    app(
        "paint",
        "Paint",
        "Simple image editor",
        "Microsoft.Paint_8wekyb3d8bbwe",
    ),
    app(
        "people",
        "People",
        "Contacts",
        "Microsoft.People_8wekyb3d8bbwe",
    ),
    app(
        "phone-link",
        "Phone Link",
        "Connect your phone",
        "Microsoft.YourPhone_8wekyb3d8bbwe",
    ),
    app(
        "photos",
        "Photos",
        "View and edit photos",
        "Microsoft.Windows.Photos_8wekyb3d8bbwe",
    ),
    app(
        "power-automate",
        "Power Automate",
        "Automate desktop tasks",
        "Microsoft.PowerAutomateDesktop_8wekyb3d8bbwe",
    ),
    app(
        "quick-assist",
        "Quick Assist",
        "Remote help",
        "MicrosoftCorporationII.QuickAssist_8wekyb3d8bbwe",
    ),
    app(
        "snipping-tool",
        "Snipping Tool",
        "Screenshots and screen recording",
        "Microsoft.ScreenSketch_8wekyb3d8bbwe",
    ),
    app(
        "sound-recorder",
        "Sound Recorder",
        "Record audio",
        "Microsoft.WindowsSoundRecorder_8wekyb3d8bbwe",
    ),
    app(
        "sticky-notes",
        "Sticky Notes",
        "Desktop notes",
        "Microsoft.MicrosoftStickyNotes_8wekyb3d8bbwe",
    ),
    app(
        "tips",
        "Tips",
        "Windows tips",
        "Microsoft.Getstarted_8wekyb3d8bbwe",
    ),
    app(
        "weather",
        "Weather",
        "Forecasts",
        "Microsoft.BingWeather_8wekyb3d8bbwe",
    ),
    app(
        "xbox",
        "Xbox",
        "Xbox app and Game Pass",
        "Microsoft.GamingApp_8wekyb3d8bbwe",
    ),
    app(
        "xbox-game-bar",
        "Xbox Game Bar",
        "In-game overlay and capture",
        "Microsoft.XboxGamingOverlay_8wekyb3d8bbwe",
    ),
];

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RemovableApp {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub installed: bool,
    /// The logo from the installed package, as a data URL. `None` when it isn't installed.
    pub icon: Option<String>,
}

fn find(id: &str) -> AppResult<&'static App> {
    APPS.iter()
        .find(|a| a.id == id)
        .ok_or_else(|| AppError::InvalidInput(format!("unknown app `{id}`")))
}

fn system_error(what: &str, e: windows::core::Error) -> AppError {
    AppError::System(format!("{what}: {}", e.message()))
}

fn manager() -> AppResult<PackageManager> {
    PackageManager::new().map_err(|e| system_error("package manager unavailable", e))
}

/// The current user's package from this family. An empty SID means "the calling user".
fn installed(manager: &PackageManager, family: &str) -> Option<Package> {
    manager
        .FindPackagesByUserSecurityIdPackageFamilyName(&HSTRING::new(), &HSTRING::from(family))
        .ok()?
        .into_iter()
        .next()
}

pub fn list() -> AppResult<Vec<RemovableApp>> {
    let manager = manager()?;
    Ok(APPS
        .iter()
        .map(|app| {
            let package = installed(&manager, app.family);
            RemovableApp {
                id: app.id,
                name: app.name,
                description: app.description,
                icon: package.as_ref().and_then(icon_of),
                installed: package.is_some(),
            }
        })
        .collect())
}

/// Removes the app for the current user. Returns its name.
pub fn uninstall(id: &str) -> AppResult<&'static str> {
    let app = find(id)?;
    let manager = manager()?;
    let package = installed(&manager, app.family)
        .ok_or_else(|| AppError::Conflict(format!("{} isn't installed", app.name)))?;
    let full_name = package
        .Id()
        .and_then(|id| id.FullName())
        .map_err(|e| system_error("couldn't read the package", e))?;
    let finished = manager
        .RemovePackageAsync(&full_name)
        .and_then(|op| op.join());
    finished.map_err(|e| system_error(&format!("Windows wouldn't remove {}", app.name), e))?;
    if installed(&manager, app.family).is_some() {
        return Err(AppError::System(format!("{} is still installed", app.name)));
    }
    Ok(app.name)
}

/// Registers the copy Windows keeps. Fails plainly when there isn't one.
pub fn reinstall(id: &str) -> AppResult<&'static str> {
    let app = find(id)?;
    let manager = manager()?;
    let finished = manager
        .RegisterPackageByFamilyNameAndOptionalPackagesAsync(
            &HSTRING::from(app.family),
            None::<&IIterable<HSTRING>>,
            DeploymentOptions::None,
            None::<&PackageVolume>,
            None::<&IIterable<HSTRING>>,
        )
        .and_then(|op| op.join());
    if finished.is_err() || installed(&manager, app.family).is_none() {
        return Err(AppError::Conflict(format!(
            "Windows has no copy of {} on this PC. Get it from Microsoft Store.",
            app.name
        )));
    }
    Ok(app.name)
}

/// The package's own logo. Read from the package on this PC, never from anywhere else.
///
/// Windows points at the scale it picked, often inside a resource pack beside the main package
/// that may not hold it, so the main package's own scales are the fallback.
fn icon_of(package: &Package) -> Option<String> {
    let shown = logo_path(&package.Logo().ok()?.AbsoluteUri().ok()?.to_string())?;
    let root = PathBuf::from(package.InstalledPath().ok()?.to_string());
    let packages = root.parent()?;
    let inside: PathBuf = shown
        .strip_prefix(packages)
        .ok()?
        .components()
        .skip(1)
        .collect();
    let paths = std::iter::once(shown.clone()).chain(candidates(&root.join(inside)));
    paths
        .filter(|path| path.starts_with(packages))
        .find_map(|path| {
            if std::fs::metadata(&path).ok()?.len() > MAX_ICON_BYTES {
                return None;
            }
            let bytes = std::fs::read(&path).ok()?;
            bytes
                .starts_with(b"\x89PNG")
                .then(|| format!("data:image/png;base64,{}", base64(&bytes)))
        })
}

/// `file:///C:/Program%20Files/...` to a Windows path.
fn logo_path(uri: &str) -> Option<PathBuf> {
    let rest = uri.strip_prefix("file:///")?;
    let bytes = rest.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let escaped = (bytes[i] == b'%')
            .then(|| rest.get(i + 1..i + 3))
            .flatten()
            .and_then(|hex| u8::from_str_radix(hex, 16).ok());
        match escaped {
            Some(byte) => {
                out.push(byte);
                i += 3;
            }
            None => {
                out.push(if bytes[i] == b'/' { b'\\' } else { bytes[i] });
                i += 1;
            }
        }
    }
    String::from_utf8(out).ok().map(PathBuf::from)
}

/// `Logo.scale-100.png` and its siblings, whichever scale `shown` happens to name.
fn candidates(shown: &Path) -> Vec<PathBuf> {
    let name = shown.file_name().unwrap_or_default().to_string_lossy();
    let stem = name.split('.').next().unwrap_or_default();
    ICON_VARIANTS
        .iter()
        .map(|variant| shown.with_file_name(format!("{stem}{variant}.png")))
        .collect()
}

fn base64(bytes: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = u32::from(chunk[0]) << 16
            | u32::from(*chunk.get(1).unwrap_or(&0)) << 8
            | u32::from(*chunk.get(2).unwrap_or(&0));
        for (i, shift) in [18, 12, 6, 0].into_iter().enumerate() {
            out.push(if i <= chunk.len() {
                TABLE[(n >> shift & 63) as usize] as char
            } else {
                '='
            });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ids_and_families_are_unique() {
        let mut ids: Vec<_> = APPS.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), APPS.len());
        let mut families: Vec<_> = APPS.iter().map(|a| a.family).collect();
        families.sort_unstable();
        families.dedup();
        assert_eq!(families.len(), APPS.len());
    }

    #[test]
    fn nothing_windows_depends_on_is_listed() {
        let protected = [
            "Microsoft.WindowsStore",
            "Microsoft.DesktopAppInstaller",
            "Microsoft.SecHealthUI",
            "Microsoft.StorePurchaseApp",
            "Microsoft.MicrosoftEdge",
            "Microsoft.XboxIdentityProvider",
            "Microsoft.Xbox.TCUI",
            "Microsoft.VCLibs",
            "Microsoft.NET",
            "Microsoft.UI.Xaml",
            "Microsoft.WindowsAppRuntime",
        ];
        for app in APPS {
            assert!(
                !protected.iter().any(|p| app.family.starts_with(p)),
                "{} is protected",
                app.family
            );
        }
    }

    #[test]
    fn unknown_apps_are_refused() {
        assert!(matches!(
            uninstall("Microsoft.SecHealthUI_8wekyb3d8bbwe"),
            Err(AppError::InvalidInput(_))
        ));
        assert!(matches!(reinstall("nope"), Err(AppError::InvalidInput(_))));
    }

    #[test]
    fn logo_uris_become_paths() {
        assert_eq!(
            logo_path("file:///C:/Program%20Files/WindowsApps/App/Assets/Logo.png"),
            Some(PathBuf::from(
                r"C:\Program Files\WindowsApps\App\Assets\Logo.png"
            ))
        );
        assert_eq!(logo_path("https://example.com/logo.png"), None);
    }

    #[test]
    fn logo_scales_are_tried_by_their_plain_name() {
        let found = candidates(Path::new(r"C:\Apps\Paint\Assets\Logo.scale-100.png"));
        assert_eq!(
            found.first(),
            Some(&PathBuf::from(r"C:\Apps\Paint\Assets\Logo.scale-200.png"))
        );
        assert!(found.contains(&PathBuf::from(r"C:\Apps\Paint\Assets\Logo.png")));
    }

    #[test]
    fn base64_matches_the_standard_alphabet() {
        assert_eq!(base64(b""), "");
        assert_eq!(base64(b"f"), "Zg==");
        assert_eq!(base64(b"fo"), "Zm8=");
        assert_eq!(base64(b"foo"), "Zm9v");
        assert_eq!(base64(b"foobar"), "Zm9vYmFy");
    }

    #[test]
    #[cfg(windows)]
    fn lists_every_app_on_a_real_system() {
        assert_eq!(list().unwrap().len(), APPS.len());
    }
}
