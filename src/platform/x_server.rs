pub(crate) fn default_app_path() -> String {
    #[cfg(target_os = "macos")]
    {
        return macos_default_app_path();
    }
    #[cfg(target_os = "windows")]
    {
        let mut candidates = Vec::new();
        if let Ok(program_files) = std::env::var("ProgramFiles") {
            candidates.push(
                std::path::PathBuf::from(&program_files)
                    .join("VcXsrv")
                    .join("vcxsrv.exe"),
            );
            candidates.push(
                std::path::PathBuf::from(&program_files)
                    .join("Xming")
                    .join("Xming.exe"),
            );
        }
        if let Ok(program_files_x86) = std::env::var("ProgramFiles(x86)") {
            candidates.push(
                std::path::PathBuf::from(&program_files_x86)
                    .join("VcXsrv")
                    .join("vcxsrv.exe"),
            );
            candidates.push(
                std::path::PathBuf::from(&program_files_x86)
                    .join("Xming")
                    .join("Xming.exe"),
            );
        }
        return candidates
            .into_iter()
            .find(|path| path.exists())
            .unwrap_or_else(|| std::path::PathBuf::from(r"C:\Program Files\VcXsrv\vcxsrv.exe"))
            .to_string_lossy()
            .to_string();
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        String::new()
    }
}

pub(crate) fn local_x_server_available(path: &str) -> bool {
    if display_from_env().is_some() {
        return true;
    }

    #[cfg(target_os = "macos")]
    {
        return std::path::Path::new(path.trim()).exists();
    }
    #[cfg(target_os = "windows")]
    {
        return std::path::Path::new(path.trim()).exists();
    }
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    {
        let _ = path;
        false
    }
}

#[cfg(target_os = "macos")]
const MACOS_MACXSERVER_APP_PATH: &str = "/Applications/MacXServer.app";
#[cfg(target_os = "macos")]
const MACOS_XQUARTZ_APP_PATH: &str = "/Applications/Utilities/XQuartz.app";
const MACXSERVER_APP_NAME: &str = "macxserver.app";
const MACXSERVER_DISPLAY: &str = "127.0.0.1:0";

#[cfg(target_os = "macos")]
fn macos_default_app_path() -> String {
    for candidate in [MACOS_MACXSERVER_APP_PATH, MACOS_XQUARTZ_APP_PATH] {
        if std::path::Path::new(candidate).exists() {
            return candidate.to_string();
        }
    }
    MACOS_XQUARTZ_APP_PATH.to_string()
}

pub(crate) fn is_macxserver_app_path(path: &str) -> bool {
    std::path::Path::new(path.trim())
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.eq_ignore_ascii_case(MACXSERVER_APP_NAME))
}

pub(crate) fn default_display() -> String {
    display_from_env().unwrap_or_else(default_display_fallback)
}

fn display_from_env() -> Option<String> {
    std::env::var("DISPLAY")
        .ok()
        .map(|display| display.trim().to_string())
        .filter(|display| !display.is_empty())
}

fn default_display_fallback() -> String {
    if cfg!(target_os = "windows") {
        "127.0.0.1:0".to_string()
    } else {
        ":0".to_string()
    }
}

pub(crate) fn resolve_display(_path: &str, _launch_local_x_server: bool) -> String {
    #[cfg(target_os = "macos")]
    {
        if is_macxserver_app_path(_path) {
            return MACXSERVER_DISPLAY.to_string();
        }
    }

    let display = default_display();

    #[cfg(target_os = "windows")]
    {
        if _launch_local_x_server
            && !matches!(windows_x_server_kind(_path), WindowsXServerKind::Other)
        {
            return select_available_windows_display(&display);
        }
    }

    display
}

#[cfg(target_os = "windows")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WindowsXServerKind {
    VcXsrv,
    Xming,
    Other,
}

#[cfg(target_os = "windows")]
impl WindowsXServerKind {
    pub(crate) fn label(self) -> &'static str {
        match self {
            Self::VcXsrv => "VcXsrv",
            Self::Xming => "Xming",
            Self::Other => "custom X server",
        }
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn windows_x_server_kind(path: &str) -> WindowsXServerKind {
    let file_name = std::path::Path::new(path.trim())
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or_default();
    if file_name.eq_ignore_ascii_case("vcxsrv.exe") {
        WindowsXServerKind::VcXsrv
    } else if file_name.eq_ignore_ascii_case("xming.exe") {
        WindowsXServerKind::Xming
    } else {
        WindowsXServerKind::Other
    }
}

#[cfg(target_os = "windows")]
pub(crate) fn launch_args(path: &str, display: &str) -> Vec<String> {
    let display = windows_display_arg(display);
    match windows_x_server_kind(path) {
        WindowsXServerKind::VcXsrv => vec![
            display,
            "-multiwindow".to_string(),
            "-clipboard".to_string(),
            "-ac".to_string(),
        ],
        WindowsXServerKind::Xming => vec![
            display,
            "-multiwindow".to_string(),
            "-clipboard".to_string(),
            "-ac".to_string(),
        ],
        WindowsXServerKind::Other => Vec::new(),
    }
}

#[cfg(target_os = "windows")]
fn select_available_windows_display(preferred_display: &str) -> String {
    if !display_uses_localhost(preferred_display) {
        return preferred_display.to_string();
    }

    let Some(start_display) = display_number(preferred_display) else {
        return preferred_display.to_string();
    };

    for offset in 0..64u16 {
        let Some(display_number) = start_display.checked_add(offset) else {
            break;
        };
        let Some(port) = 6000u16.checked_add(display_number) else {
            break;
        };
        if windows_local_port_available(port) {
            return display_with_number(preferred_display, display_number);
        }
    }

    preferred_display.to_string()
}

#[cfg(target_os = "windows")]
fn windows_local_port_available(port: u16) -> bool {
    std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, port)).is_ok()
}

#[cfg(target_os = "windows")]
fn windows_display_arg(display: &str) -> String {
    format!(":{}", display_number(display).unwrap_or(0))
}

#[cfg(target_os = "windows")]
fn display_number(display: &str) -> Option<u16> {
    let (_, rest) = display.rsplit_once(':')?;
    let number = rest.split('.').next().unwrap_or(rest);
    number.parse::<u16>().ok()
}

#[cfg(target_os = "windows")]
fn display_uses_localhost(display: &str) -> bool {
    if display.starts_with(':') {
        return true;
    }
    let Some((host, _)) = display.rsplit_once(':') else {
        return false;
    };
    let host = host.trim_start_matches("tcp/").trim();
    host.is_empty() || host.eq_ignore_ascii_case("localhost") || host == "127.0.0.1"
}

#[cfg(target_os = "windows")]
fn display_with_number(display: &str, display_number: u16) -> String {
    let screen_suffix = display
        .rsplit_once(':')
        .and_then(|(_, rest)| rest.split_once('.').map(|(_, screen)| format!(".{screen}")))
        .unwrap_or_default();

    if display.starts_with(':') {
        format!(":{display_number}{screen_suffix}")
    } else if let Some((host, _)) = display.rsplit_once(':') {
        format!("{host}:{display_number}{screen_suffix}")
    } else {
        format!("127.0.0.1:{display_number}{screen_suffix}")
    }
}

#[cfg(all(test, target_os = "windows"))]
mod tests {
    use super::{
        WindowsXServerKind, display_number, display_with_number, launch_args, windows_display_arg,
        windows_x_server_kind,
    };

    #[test]
    fn windows_display_helpers_parse_and_replace_display_numbers() {
        assert_eq!(display_number("127.0.0.1:0"), Some(0));
        assert_eq!(display_number(":12.0"), Some(12));
        assert_eq!(display_with_number("127.0.0.1:0", 3), "127.0.0.1:3");
        assert_eq!(display_with_number(":12.1", 7), ":7.1");
        assert_eq!(windows_display_arg("127.0.0.1:5"), ":5");
    }

    #[test]
    fn windows_server_kind_and_launch_args_follow_selected_display() {
        assert_eq!(
            windows_x_server_kind(r"C:\Program Files\VcXsrv\vcxsrv.exe"),
            WindowsXServerKind::VcXsrv
        );
        assert_eq!(
            windows_x_server_kind(r"C:\Program Files (x86)\Xming\Xming.exe"),
            WindowsXServerKind::Xming
        );
        assert_eq!(
            windows_x_server_kind(r"C:\Tools\other-x-server.exe"),
            WindowsXServerKind::Other
        );
        assert_eq!(
            launch_args(r"C:\Program Files\VcXsrv\vcxsrv.exe", "127.0.0.1:2",),
            vec![
                ":2".to_string(),
                "-multiwindow".to_string(),
                "-clipboard".to_string(),
                "-ac".to_string(),
            ]
        );
        assert_eq!(
            launch_args(r"C:\Program Files (x86)\Xming\Xming.exe", "127.0.0.1:3",),
            vec![
                ":3".to_string(),
                "-multiwindow".to_string(),
                "-clipboard".to_string(),
                "-ac".to_string(),
            ]
        );
        assert!(launch_args(r"C:\Tools\other-x-server.exe", "127.0.0.1:0").is_empty());
    }
}

#[cfg(test)]
mod shared_tests {
    use super::is_macxserver_app_path;

    #[test]
    fn macxserver_app_path_is_detected_by_bundle_name() {
        assert!(is_macxserver_app_path("/Applications/MacXServer.app"));
        assert!(is_macxserver_app_path("/Applications/macxserver.app"));
        assert!(!is_macxserver_app_path(
            "/Applications/Utilities/XQuartz.app"
        ));
        assert!(!is_macxserver_app_path("/Applications/MacXCapture.app"));
    }
}

#[cfg(all(test, target_os = "macos"))]
mod macos_tests {
    use super::{MACXSERVER_DISPLAY, resolve_display};

    #[test]
    fn macxserver_path_forces_tcp_display_zero() {
        assert_eq!(
            resolve_display("/Applications/MacXServer.app", true),
            MACXSERVER_DISPLAY
        );
    }
}
