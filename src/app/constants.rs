pub(crate) const DEFAULT_COLS: u16 = 100;
pub(crate) const DEFAULT_ROWS: u16 = 30;
pub(crate) const SIDEBAR_WIDTH: f32 = 306.0;
pub(crate) const COLLAPSED_SIDEBAR_WIDTH: f32 = 52.0;
pub(crate) const WORKSPACE_TAB_MAX_WIDTH: f32 = 220.0;

#[allow(dead_code)]
pub(crate) const TAB_BAR_HEIGHT: f32 = 52.0;
#[allow(dead_code)]
pub(crate) const TERMINAL_PADDING_X: f32 = 32.0;
#[allow(dead_code)]
pub(crate) const TERMINAL_PADDING_Y: f32 = 32.0;

pub(crate) const TERMINAL_KEY_CONTEXT: &str = "AxShellTerminal";
pub(crate) const REPOSITORY_URL: &str = "https://github.com/xinalbert/axshell";
pub(crate) const ISSUES_URL: &str = "https://github.com/xinalbert/axshell/issues";

pub(crate) fn public_version_label() -> String {
    public_version_label_from_metadata(
        option_env!("AXSHELL_PUBLIC_VERSION"),
        env!("CARGO_PKG_VERSION"),
    )
}

fn public_version_label_from_metadata(
    injected_public_version: Option<&str>,
    cargo_version: &str,
) -> String {
    if let Some(version) = injected_public_version
        .map(str::trim)
        .filter(|version| !version.is_empty())
    {
        return version.to_string();
    }

    format_public_version(cargo_version)
}

fn format_public_version(version: &str) -> String {
    let version = version.split('+').next().unwrap_or(version);
    let (core, suffix) = version
        .split_once('-')
        .map_or((version, None), |(core, suffix)| (core, Some(suffix)));

    let mut parts = core.split('.');
    let (Some(year), Some(month), Some(day), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return version.to_string();
    };

    let (Ok(year), Ok(month), Ok(day)) = (
        year.parse::<u32>(),
        month.parse::<u32>(),
        day.parse::<u32>(),
    ) else {
        return version.to_string();
    };

    let mut public = format!("{year:04}.{month:02}.{day:02}");
    if let Some(suffix) = suffix.filter(|suffix| !suffix.is_empty()) {
        public.push('.');
        public.push_str(suffix);
    }
    public
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn injected_release_public_version_takes_precedence() {
        let label = public_version_label_from_metadata(Some(" 2026.07.11.1 "), "2026.7.6");

        assert_eq!(label, "2026.07.11.1");
    }

    #[test]
    fn empty_injected_version_falls_back_to_cargo_version() {
        let label = public_version_label_from_metadata(Some(" "), "2026.7.6-2");

        assert_eq!(label, "2026.07.06.2");
    }

    #[test]
    fn cargo_date_version_is_formatted_for_public_display() {
        assert_eq!(format_public_version("2026.7.6"), "2026.07.06");
        assert_eq!(format_public_version("2026.7.6-1"), "2026.07.06.1");
    }

    #[test]
    fn public_version_label_reads_injected_build_metadata() {
        if let Some(version) = option_env!("AXSHELL_PUBLIC_VERSION") {
            assert_eq!(public_version_label(), version.trim());
        }
    }
}
