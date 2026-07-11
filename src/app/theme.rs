use anyhow::{Context as _, Result, anyhow};
use gpui::{App, Context, Entity, Hsla, SharedString, Window, px};
use gpui_component::{
    ActiveTheme as _, Theme, ThemeConfig, ThemeMode, ThemeRegistry, ThemeSet, input::InputState,
    try_parse_color,
};
use serde_json::{Map as JsonMap, Value as JsonValue};

use crate::{
    AxShell,
    config::{ConfigStore, CustomThemeModeConfig},
};

pub(crate) const EMBEDDED_THEME_JSONS: &[&str] = &[
    include_str!("../../assets/themes/matrix.json"),
    include_str!("../../assets/themes/tokyonight.json"),
    include_str!("../../assets/themes/gruvbox.json"),
    include_str!("../../assets/themes/solarized.json"),
    include_str!("../../assets/themes/phygerr.json"),
];

use std::{
    borrow::Cow,
    fs,
    path::{Path, PathBuf},
    rc::Rc,
    sync::atomic::{AtomicBool, Ordering},
};

pub(crate) static USING_SYSTEM_MAPLE: AtomicBool = AtomicBool::new(false);

const CUSTOM_THEME_NAME_INPUT_KEY: &str = "custom_theme.theme_name";
const CUSTOM_THEME_FILE_PREFIX: &str = "custom-";
const IMPORTED_THEME_FILE_PREFIX: &str = "imported-";
const CUSTOM_LIGHT_SUFFIX: &str = "[Custom Light]";
const CUSTOM_DARK_SUFFIX: &str = "[Custom Dark]";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum CustomThemeFieldDomain {
    ThemeColor,
    HighlightColor,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CustomThemeFieldSpec {
    pub key: &'static str,
    pub label: &'static str,
    pub placeholder: &'static str,
    pub domain: CustomThemeFieldDomain,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct CustomThemeSectionSpec {
    pub title: &'static str,
    pub fields: &'static [CustomThemeFieldSpec],
}

pub(crate) const CUSTOM_THEME_CORE_FIELDS: &[CustomThemeFieldSpec] = &[
    CustomThemeFieldSpec {
        key: "background",
        label: "Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "foreground",
        label: "Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "border",
        label: "Border",
        placeholder: "#374151",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "primary.background",
        label: "Primary Background",
        placeholder: "#4F8CFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "primary.foreground",
        label: "Primary Foreground",
        placeholder: "#FFFFFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "secondary.background",
        label: "Secondary Background",
        placeholder: "#1F2937",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "secondary.foreground",
        label: "Secondary Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "accent.background",
        label: "Accent Background",
        placeholder: "#1D4ED8",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "accent.foreground",
        label: "Accent Foreground",
        placeholder: "#FFFFFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "selection.background",
        label: "Selection Background",
        placeholder: "#2563EB66",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "ring",
        label: "Focus Ring",
        placeholder: "#60A5FA",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
];

pub(crate) const CUSTOM_THEME_SURFACE_FIELDS: &[CustomThemeFieldSpec] = &[
    CustomThemeFieldSpec {
        key: "popover.background",
        label: "Popover Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "popover.foreground",
        label: "Popover Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "sidebar.background",
        label: "Sidebar Background",
        placeholder: "#0F172A",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "sidebar.foreground",
        label: "Sidebar Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "sidebar.primary.background",
        label: "Sidebar Primary Background",
        placeholder: "#4F8CFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "sidebar.primary.foreground",
        label: "Sidebar Primary Foreground",
        placeholder: "#FFFFFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "tab.active.background",
        label: "Active Tab Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "tab.active.foreground",
        label: "Active Tab Foreground",
        placeholder: "#F9FAFB",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "tab.foreground",
        label: "Tab Foreground",
        placeholder: "#CBD5E1",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "table.head.background",
        label: "Table Head Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "table.head.foreground",
        label: "Table Head Foreground",
        placeholder: "#CBD5E1",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
];

pub(crate) const CUSTOM_THEME_SEMANTIC_FIELDS: &[CustomThemeFieldSpec] = &[
    CustomThemeFieldSpec {
        key: "danger.background",
        label: "Danger Background",
        placeholder: "#EF4444",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "danger.foreground",
        label: "Danger Foreground",
        placeholder: "#FFFFFF",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "info.background",
        label: "Info Background",
        placeholder: "#06B6D4",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "success.background",
        label: "Success Background",
        placeholder: "#22C55E",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "warning.background",
        label: "Warning Background",
        placeholder: "#F59E0B",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.red",
        label: "Base Red",
        placeholder: "#EF4444",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.green",
        label: "Base Green",
        placeholder: "#22C55E",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.blue",
        label: "Base Blue",
        placeholder: "#3B82F6",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.yellow",
        label: "Base Yellow",
        placeholder: "#F59E0B",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.magenta",
        label: "Base Magenta",
        placeholder: "#A855F7",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
    CustomThemeFieldSpec {
        key: "base.cyan",
        label: "Base Cyan",
        placeholder: "#06B6D4",
        domain: CustomThemeFieldDomain::ThemeColor,
    },
];

pub(crate) const CUSTOM_THEME_EDITOR_FIELDS: &[CustomThemeFieldSpec] = &[
    CustomThemeFieldSpec {
        key: "editor.background",
        label: "Editor Background",
        placeholder: "#111827",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "editor.foreground",
        label: "Editor Foreground",
        placeholder: "#E5E7EB",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "editor.active_line.background",
        label: "Active Line Background",
        placeholder: "#1F2937",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "editor.line_number",
        label: "Line Number",
        placeholder: "#64748B",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "editor.active_line_number",
        label: "Active Line Number",
        placeholder: "#F8FAFC",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.keyword.color",
        label: "Syntax Keyword",
        placeholder: "#F472B6",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.string.color",
        label: "Syntax String",
        placeholder: "#86EFAC",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.function.color",
        label: "Syntax Function",
        placeholder: "#60A5FA",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.type.color",
        label: "Syntax Type",
        placeholder: "#C084FC",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
    CustomThemeFieldSpec {
        key: "syntax.comment.color",
        label: "Syntax Comment",
        placeholder: "#64748B",
        domain: CustomThemeFieldDomain::HighlightColor,
    },
];

pub(crate) const CUSTOM_THEME_SECTION_SPECS: &[CustomThemeSectionSpec] = &[
    CustomThemeSectionSpec {
        title: "Core",
        fields: CUSTOM_THEME_CORE_FIELDS,
    },
    CustomThemeSectionSpec {
        title: "Surfaces",
        fields: CUSTOM_THEME_SURFACE_FIELDS,
    },
    CustomThemeSectionSpec {
        title: "Semantic & Base Palette",
        fields: CUSTOM_THEME_SEMANTIC_FIELDS,
    },
    CustomThemeSectionSpec {
        title: "Editor & Syntax",
        fields: CUSTOM_THEME_EDITOR_FIELDS,
    },
];

pub(crate) fn custom_theme_name_input_key() -> &'static str {
    CUSTOM_THEME_NAME_INPUT_KEY
}

pub(crate) fn custom_theme_modes() -> [ThemeMode; 2] {
    [ThemeMode::Light, ThemeMode::Dark]
}

pub(crate) fn custom_theme_input_key(mode: ThemeMode, key: &str) -> String {
    format!(
        "custom_theme.{}.{}",
        if mode.is_dark() { "dark" } else { "light" },
        key
    )
}

pub(crate) fn custom_theme_registry_name(theme_name: &str, mode: ThemeMode) -> String {
    let theme_name = normalized_custom_theme_name(theme_name);
    if mode.is_dark() {
        format!("{theme_name} {CUSTOM_DARK_SUFFIX}")
    } else {
        format!("{theme_name} {CUSTOM_LIGHT_SUFFIX}")
    }
}

pub(crate) fn load_fonts(cx: &mut App) -> Result<()> {
    let has_system_maple = cx
        .text_system()
        .all_font_names()
        .contains(&"Maple Mono NF CN".to_string());
    if has_system_maple {
        USING_SYSTEM_MAPLE.store(true, Ordering::Relaxed);
    } else {
        let regular = Cow::Borrowed(
            include_bytes!("../../assets/fonts/MapleMono-NF-CN-Regular.ttf").as_slice(),
        );
        let bold =
            Cow::Borrowed(include_bytes!("../../assets/fonts/MapleMono-NF-CN-Bold.ttf").as_slice());
        cx.text_system()
            .add_fonts(vec![regular, bold])
            .context("load Maple Mono NF CN fonts")?;
    }
    set_theme_font_names(cx.global_mut::<Theme>(), ".SystemUIFont");
    Ok(())
}

pub(crate) fn load_embedded_themes(cx: &mut App) {
    let registry = ThemeRegistry::global_mut(cx);
    for theme_json in EMBEDDED_THEME_JSONS {
        if let Err(err) = registry.load_themes_from_str(theme_json) {
            tracing::warn!(
                component = "theme",
                operation = "load_embedded",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to load embedded theme"
            );
        }
    }
}

pub(crate) fn load_user_themes(cx: &mut App) {
    let Ok(themes_dir) = ConfigStore::theme_dir_path() else {
        tracing::warn!(
            component = "theme",
            operation = "resolve_user_theme_dir",
            "Failed to resolve user theme directory"
        );
        return;
    };

    if let Err(err) = fs::create_dir_all(&themes_dir) {
        tracing::warn!(
            component = "theme",
            operation = "create_user_theme_dir",
            theme_path = %crate::diagnostics::mask_path(&themes_dir.to_string_lossy()),
            error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
            "Failed to create user theme directory"
        );
        return;
    }

    let registry = ThemeRegistry::global_mut(cx);
    if let Ok(entries) = fs::read_dir(&themes_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }
            match fs::read_to_string(&path) {
                Ok(content) => {
                    if let Err(err) = registry.load_themes_from_str(&content) {
                        tracing::warn!(
                            component = "theme",
                            operation = "load_user_theme",
                            theme_path = %crate::diagnostics::mask_path(&path.to_string_lossy()),
                            error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                            "Failed to load user theme"
                        );
                    }
                }
                Err(err) => {
                    tracing::warn!(
                        component = "theme",
                        operation = "read_user_theme",
                        theme_path = %crate::diagnostics::mask_path(&path.to_string_lossy()),
                        error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                        "Failed to read user theme"
                    );
                }
            }
        }
    }

    // gpui-component's directory watcher reloads only default themes plus files
    // from this directory. AxShell's built-in themes are embedded resources, so
    // the watcher would drop them from the registry and profile selection would
    // fall back to Default Dark/Light.
}

pub(crate) fn set_theme_font_names(theme: &mut Theme, ui_font_family: &str) {
    theme.font_family = ui_font_family.into();
    theme.mono_font_family = ui_font_family.into();
}

fn normalized_custom_theme_name(theme_name: &str) -> String {
    let theme_name = theme_name.trim();
    if theme_name.is_empty() {
        "Custom Theme".to_string()
    } else {
        theme_name.to_string()
    }
}

fn theme_name_slug(theme_name: &str, fallback: &str) -> String {
    let mut slug = String::new();
    for ch in theme_name.trim().chars() {
        if ch.is_ascii_alphanumeric() {
            slug.push(ch.to_ascii_lowercase());
        } else if !slug.ends_with('-') {
            slug.push('-');
        }
    }
    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        fallback.to_string()
    } else {
        slug.to_string()
    }
}

fn custom_theme_file_name(theme_name: &str) -> String {
    let slug = theme_name_slug(&normalized_custom_theme_name(theme_name), "custom-theme");
    format!("{CUSTOM_THEME_FILE_PREFIX}{slug}.json")
}

fn custom_theme_file_path(theme_dir: &Path, theme_name: &str, save_path: &str) -> PathBuf {
    let file_name = custom_theme_file_name(theme_name);
    let save_path = save_path.trim();
    if save_path.is_empty() {
        return theme_dir.join(file_name);
    }

    let path = expand_user_path(save_path);
    if path
        .extension()
        .and_then(|ext| ext.to_str())
        .is_some_and(|ext| ext.eq_ignore_ascii_case("json"))
    {
        path
    } else {
        path.join(file_name)
    }
}

fn imported_theme_source_name(theme_set: &ThemeSet, source_path: &Path) -> String {
    let theme_set_name = theme_set.name.as_ref().trim();
    if !theme_set_name.is_empty() {
        return theme_set_name.to_string();
    }

    source_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::trim)
        .filter(|stem| !stem.is_empty())
        .unwrap_or("Theme")
        .to_string()
}

fn imported_theme_profile_name(theme_set: &ThemeSet, source_path: &Path) -> String {
    format!(
        "Imported {}",
        imported_theme_source_name(theme_set, source_path)
    )
}

fn imported_theme_file_name(theme_set: &ThemeSet, source_path: &Path) -> String {
    let slug = theme_name_slug(&imported_theme_source_name(theme_set, source_path), "theme");
    format!("{IMPORTED_THEME_FILE_PREFIX}{slug}.json")
}

fn unique_imported_theme_file_path(
    theme_dir: &Path,
    theme_set: &ThemeSet,
    source_path: &Path,
) -> PathBuf {
    let file_name = imported_theme_file_name(theme_set, source_path);
    let stem = file_name.trim_end_matches(".json");
    let mut path = theme_dir.join(&file_name);
    let mut suffix = 2;
    while path.exists() {
        path = theme_dir.join(format!("{stem}-{suffix}.json"));
        suffix += 1;
    }
    path
}

fn write_imported_theme_file(
    theme_dir: &Path,
    theme_set: &ThemeSet,
    source_path: &Path,
    content: &str,
) -> Result<PathBuf> {
    fs::create_dir_all(theme_dir)
        .with_context(|| format!("failed to create {}", theme_dir.display()))?;
    let path = unique_imported_theme_file_path(theme_dir, theme_set, source_path);
    fs::write(&path, content).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(path)
}

fn imported_theme_names(
    theme_set: &ThemeSet,
    fallback_light_theme_name: &str,
    fallback_dark_theme_name: &str,
) -> Result<(String, String)> {
    if theme_set.themes.is_empty() {
        return Err(anyhow!("imported theme file does not contain any themes"));
    }

    let light_theme_name = theme_set
        .themes
        .iter()
        .find(|theme| !theme.mode.is_dark())
        .map(|theme| theme.name.to_string())
        .unwrap_or_else(|| fallback_light_theme_name.to_string());
    let dark_theme_name = theme_set
        .themes
        .iter()
        .find(|theme| theme.mode.is_dark())
        .map(|theme| theme.name.to_string())
        .unwrap_or_else(|| fallback_dark_theme_name.to_string());

    Ok((light_theme_name, dark_theme_name))
}

fn expand_user_path(path: &str) -> PathBuf {
    if path == "~" || path.starts_with("~/") {
        if let Some(dirs) = directories::BaseDirs::new() {
            let rest = path.strip_prefix("~/").unwrap_or("");
            return dirs.home_dir().join(rest);
        }
    }
    PathBuf::from(path)
}

fn custom_theme_field_specs() -> impl Iterator<Item = &'static CustomThemeFieldSpec> {
    CUSTOM_THEME_SECTION_SPECS
        .iter()
        .flat_map(|section| section.fields.iter())
}

fn find_custom_theme_field(key: &str) -> Option<&'static CustomThemeFieldSpec> {
    custom_theme_field_specs().find(|field| field.key == key)
}

fn is_highlight_status_key(key: &str) -> bool {
    matches!(key, "error" | "warning" | "info" | "success" | "hint")
        || key.starts_with("error.")
        || key.starts_with("warning.")
        || key.starts_with("info.")
        || key.starts_with("success.")
        || key.starts_with("hint.")
}

fn set_syntax_override(
    highlight_object: &mut JsonMap<String, JsonValue>,
    key: &str,
    value: &str,
) -> Result<()> {
    let Some(rest) = key.strip_prefix("syntax.") else {
        return Err(anyhow!("invalid syntax override key: {key}"));
    };
    let Some(token) = rest.strip_suffix(".color") else {
        return Err(anyhow!("unsupported syntax override key: {key}"));
    };

    let syntax = highlight_object
        .entry("syntax".to_string())
        .or_insert_with(|| JsonValue::Object(JsonMap::new()));
    let syntax = syntax
        .as_object_mut()
        .ok_or_else(|| anyhow!("syntax highlight section is not an object"))?;
    let style = syntax
        .entry(token.to_string())
        .or_insert_with(|| JsonValue::Object(JsonMap::new()));
    let style = style
        .as_object_mut()
        .ok_or_else(|| anyhow!("syntax style entry is not an object"))?;
    style.insert("color".to_string(), JsonValue::String(value.to_string()));
    Ok(())
}

fn build_custom_theme_config(
    base_theme: &ThemeConfig,
    mode_config: &CustomThemeModeConfig,
    generated_name: &str,
    mode: ThemeMode,
) -> Result<ThemeConfig> {
    let mut value = serde_json::to_value(base_theme.clone()).context("serialize base theme")?;
    let object = value
        .as_object_mut()
        .ok_or_else(|| anyhow!("serialized theme config is not an object"))?;

    object.insert("is_default".to_string(), JsonValue::Bool(false));
    object.insert(
        "name".to_string(),
        JsonValue::String(generated_name.to_string()),
    );
    object.insert(
        "mode".to_string(),
        JsonValue::String(if mode.is_dark() { "dark" } else { "light" }.to_string()),
    );

    let mut colors = match object.remove("colors") {
        Some(JsonValue::Object(colors)) => colors,
        Some(_) => return Err(anyhow!("theme colors is not an object")),
        None => JsonMap::new(),
    };
    let mut highlight = match object.remove("highlight") {
        Some(JsonValue::Object(highlight)) => highlight,
        Some(_) => return Err(anyhow!("theme highlight is not an object")),
        None => JsonMap::new(),
    };

    for (key, raw_value) in &mode_config.overrides {
        let value = raw_value.trim();
        if value.is_empty() {
            continue;
        }
        match find_custom_theme_field(key).map(|field| field.domain) {
            Some(CustomThemeFieldDomain::ThemeColor) | None => {
                colors.insert(key.clone(), JsonValue::String(value.to_string()));
            }
            Some(CustomThemeFieldDomain::HighlightColor) => {
                if key.starts_with("syntax.") {
                    set_syntax_override(&mut highlight, key, value)?;
                } else if key.starts_with("editor.") || is_highlight_status_key(key) {
                    highlight.insert(key.clone(), JsonValue::String(value.to_string()));
                }
            }
        }
    }

    object.insert("colors".to_string(), JsonValue::Object(colors));
    object.insert("highlight".to_string(), JsonValue::Object(highlight));

    serde_json::from_value(value).context("deserialize generated custom theme")
}

fn custom_theme_inherited_field_value_from_theme_value(
    theme_value: &JsonValue,
    field: &CustomThemeFieldSpec,
) -> String {
    let inherited = match field.domain {
        CustomThemeFieldDomain::ThemeColor => theme_value
            .get("colors")
            .and_then(|colors| colors.get(field.key))
            .and_then(JsonValue::as_str),
        CustomThemeFieldDomain::HighlightColor => {
            theme_value.get("highlight").and_then(|highlight| {
                if let Some(token) = field
                    .key
                    .strip_prefix("syntax.")
                    .and_then(|rest| rest.strip_suffix(".color"))
                {
                    highlight
                        .get("syntax")
                        .and_then(|syntax| syntax.get(token))
                        .and_then(|style| style.get("color"))
                        .and_then(JsonValue::as_str)
                } else {
                    highlight.get(field.key).and_then(JsonValue::as_str)
                }
            })
        }
    };

    inherited
        .map(str::to_string)
        .unwrap_or_else(|| field.placeholder.to_string())
}

fn custom_theme_inherited_field_value(
    base_theme: &ThemeConfig,
    field: &CustomThemeFieldSpec,
) -> String {
    let theme_value = serde_json::to_value(base_theme).unwrap_or(JsonValue::Null);
    custom_theme_inherited_field_value_from_theme_value(&theme_value, field)
}

fn ensure_embedded_themes_registered(cx: &mut App) {
    if ThemeRegistry::global(cx)
        .themes()
        .contains_key(&SharedString::from("Gruvbox Dark"))
    {
        return;
    }

    load_embedded_themes(cx);
}

fn resolve_base_theme(config: &ConfigStore, mode: ThemeMode, cx: &App) -> Rc<ThemeConfig> {
    let base_name = config.custom_theme_base_name(mode);
    if let Some(theme) = ThemeRegistry::global(cx)
        .themes()
        .get(&SharedString::from(base_name.clone()))
        .filter(|theme| theme.mode == mode)
    {
        return theme.clone();
    }

    if mode.is_dark() {
        ThemeRegistry::global(cx).default_dark_theme().clone()
    } else {
        ThemeRegistry::global(cx).default_light_theme().clone()
    }
}

fn build_custom_theme_set(
    config: &ConfigStore,
    cx: &App,
) -> Result<(ThemeSet, ThemeConfig, ThemeConfig)> {
    let draft = config.custom_theme_draft();
    let custom_name = normalized_custom_theme_name(&draft.theme_name);
    let light_name = custom_theme_registry_name(&custom_name, ThemeMode::Light);
    let dark_name = custom_theme_registry_name(&custom_name, ThemeMode::Dark);

    let light = build_custom_theme_config(
        &resolve_base_theme(config, ThemeMode::Light, cx),
        &draft.light,
        &light_name,
        ThemeMode::Light,
    )?;
    let dark = build_custom_theme_config(
        &resolve_base_theme(config, ThemeMode::Dark, cx),
        &draft.dark,
        &dark_name,
        ThemeMode::Dark,
    )?;

    Ok((
        ThemeSet {
            name: custom_name.clone().into(),
            author: Some("ax_shell".into()),
            url: None,
            themes: vec![light.clone(), dark.clone()],
        },
        light,
        dark,
    ))
}

fn write_custom_theme_file(config: &ConfigStore, theme_set: &ThemeSet) -> Result<PathBuf> {
    let theme_dir = config
        .theme_dir()
        .or_else(|| ConfigStore::theme_dir_path().ok())
        .ok_or_else(|| anyhow!("could not resolve local theme dir"))?;
    let path = custom_theme_file_path(
        &theme_dir,
        theme_set.name.as_ref(),
        config.custom_theme_save_path(),
    );
    let parent = path
        .parent()
        .ok_or_else(|| anyhow!("custom theme path has no parent: {}", path.display()))?;
    fs::create_dir_all(parent).with_context(|| format!("failed to create {}", parent.display()))?;
    let content = serde_json::to_string_pretty(theme_set).context("serialize custom theme file")?;
    fs::write(&path, content).with_context(|| format!("failed to write {}", path.display()))?;
    Ok(path)
}

fn adjust_font_brightness(color: Hsla, factor: f32) -> Hsla {
    if (factor - 1.0).abs() <= f32::EPSILON {
        return color;
    }

    Hsla {
        l: (color.l * factor).clamp(0.02, 0.98),
        ..color
    }
}

fn apply_ui_font_brightness(theme: &mut Theme, factor: f32) {
    if (factor - 1.0).abs() <= f32::EPSILON {
        return;
    }

    macro_rules! adjust {
        ($($field:ident),+ $(,)?) => {
            $(
                theme.$field = adjust_font_brightness(theme.$field, factor);
            )+
        };
    }

    adjust!(
        accent_foreground,
        button_primary_foreground,
        danger_foreground,
        description_list_label_foreground,
        foreground,
        group_box_foreground,
        info_foreground,
        link,
        link_active,
        link_hover,
        muted_foreground,
        popover_foreground,
        primary_foreground,
        secondary_foreground,
        sidebar_accent_foreground,
        sidebar_foreground,
        sidebar_primary_foreground,
        success_foreground,
        tab_active_foreground,
        tab_foreground,
        table_head_foreground,
        table_foot_foreground,
        warning_foreground,
    );
}

impl AxShell {
    fn current_custom_theme_draft_name(&self) -> String {
        self.config.custom_theme_draft().theme_name
    }

    fn current_custom_theme_registry_name(&self, mode: ThemeMode) -> SharedString {
        custom_theme_registry_name(&self.current_custom_theme_draft_name(), mode).into()
    }

    fn is_current_custom_theme_name(&self, name: &SharedString, mode: ThemeMode) -> bool {
        let generated = self.current_custom_theme_registry_name(mode);
        name == &generated || name.as_ref() == self.config.custom_theme_name()
    }

    pub(crate) fn resolved_custom_theme_base_name(&self, mode: ThemeMode, cx: &App) -> String {
        resolve_base_theme(&self.config, mode, cx).name.to_string()
    }

    pub(crate) fn custom_theme_inherited_field_value(
        &self,
        mode: ThemeMode,
        field: &CustomThemeFieldSpec,
        cx: &App,
    ) -> String {
        let base_theme = resolve_base_theme(&self.config, mode, cx);
        custom_theme_inherited_field_value(&base_theme, field)
    }

    fn set_input_placeholder(
        input: &Entity<InputState>,
        value: impl Into<SharedString>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        input.update(cx, |state, cx| state.set_placeholder(value, window, cx));
    }

    pub(crate) fn sync_custom_theme_inputs_from_draft(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let draft = self.config.custom_theme_draft();
        if let Some(input) = self.custom_theme_inputs.get(CUSTOM_THEME_NAME_INPUT_KEY) {
            Self::set_input_value(input, draft.theme_name.clone(), window, cx);
        }
        Self::set_input_value(
            &self.custom_theme_save_path_input,
            self.config.custom_theme_save_path().to_string(),
            window,
            cx,
        );

        for mode in custom_theme_modes() {
            let mode_config = if mode.is_dark() {
                &draft.dark
            } else {
                &draft.light
            };
            let base_theme = resolve_base_theme(&self.config, mode, cx);
            for field in custom_theme_field_specs() {
                let input_key = custom_theme_input_key(mode, field.key);
                let value = mode_config
                    .overrides
                    .get(field.key)
                    .cloned()
                    .unwrap_or_default();
                if let Some(input) = self.custom_theme_inputs.get(&input_key) {
                    let placeholder = custom_theme_inherited_field_value(&base_theme, field);
                    Self::set_input_placeholder(input, placeholder, window, cx);
                    Self::set_input_value(input, value, window, cx);
                }
            }
        }
    }

    fn resolve_selected_theme(
        &self,
        name: &SharedString,
        mode: ThemeMode,
        cx: &App,
    ) -> Rc<ThemeConfig> {
        if self.is_current_custom_theme_name(name, mode) {
            if let Ok((_, light, dark)) = build_custom_theme_set(&self.config, cx) {
                return Rc::new(if mode.is_dark() { dark } else { light });
            }
        }

        Self::resolve_registered_theme(name, mode, cx)
    }

    fn resolve_registered_theme(name: &SharedString, mode: ThemeMode, cx: &App) -> Rc<ThemeConfig> {
        if let Some(theme) = ThemeRegistry::global(cx)
            .themes()
            .get(name)
            .filter(|theme| theme.mode == mode)
        {
            return theme.clone();
        }

        if mode.is_dark() {
            ThemeRegistry::global(cx).default_dark_theme().clone()
        } else {
            ThemeRegistry::global(cx).default_light_theme().clone()
        }
    }

    pub(crate) fn switch_theme_mode(
        &mut self,
        mode: ThemeMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.appearance.follow_system_theme = false;
        self.appearance.theme_mode = mode;
        self.apply_theme_preferences(window, cx);
        self.status = format!("theme mode: {}", cx.theme().mode.name()).into();
        self.persist_theme_preferences();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn apply_theme_profile(
        &mut self,
        id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        ensure_embedded_themes_registered(cx);
        let Some(profile) = self.config.set_active_theme_profile(&id) else {
            self.status = format!("theme profile not found: {id}").into();
            cx.notify();
            return;
        };

        self.appearance.light_theme_name = if profile.light_theme_name.trim().is_empty() {
            ThemeRegistry::global(cx).default_light_theme().name.clone()
        } else {
            profile.light_theme_name.clone().into()
        };
        self.appearance.dark_theme_name = if profile.dark_theme_name.trim().is_empty() {
            ThemeRegistry::global(cx).default_dark_theme().name.clone()
        } else {
            profile.dark_theme_name.clone().into()
        };
        let (light_theme, dark_theme) = if profile.custom_theme.is_some() {
            (
                self.resolve_selected_theme(
                    &self.appearance.light_theme_name,
                    ThemeMode::Light,
                    cx,
                ),
                self.resolve_selected_theme(&self.appearance.dark_theme_name, ThemeMode::Dark, cx),
            )
        } else {
            (
                Self::resolve_registered_theme(
                    &self.appearance.light_theme_name,
                    ThemeMode::Light,
                    cx,
                ),
                Self::resolve_registered_theme(
                    &self.appearance.dark_theme_name,
                    ThemeMode::Dark,
                    cx,
                ),
            )
        };
        self.apply_theme_configs(light_theme, dark_theme, window, cx);
        self.sync_custom_theme_inputs_from_draft(window, cx);
        self.persist_theme_preferences();
        self.status = format!("theme profile: {}", profile.name).into();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn set_custom_theme_base_preset(
        &mut self,
        mode: ThemeMode,
        name: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config.set_custom_theme_base_name(mode, name);
        if let Err(err) = self.config.save() {
            tracing::error!(
                component = "theme",
                operation = "save_custom_theme_base",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to save custom theme base"
            );
            self.status = format!("failed to save custom theme base: {err:#}").into();
        } else {
            self.status = format!("custom {} base: {name}", mode.name()).into();
        }
        self.sync_custom_theme_inputs_from_draft(window, cx);
        if let Err(err) = self.apply_custom_theme_preview(window, cx) {
            tracing::debug!(
                component = "theme",
                operation = "preview_custom_theme_base",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Skipping custom theme base preview until inputs are valid"
            );
        }
        cx.notify();
    }

    pub(crate) fn preview_custom_theme_input(
        &mut self,
        input: &Entity<InputState>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        let Some(changed) = self.update_custom_theme_draft_from_input(input, cx) else {
            return false;
        };
        if !changed {
            return true;
        }

        if let Err(err) = self.apply_custom_theme_preview(window, cx) {
            tracing::debug!(
                component = "theme",
                operation = "preview_custom_theme",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Skipping custom theme preview until inputs are valid"
            );
        }
        true
    }

    fn update_custom_theme_draft_from_input(
        &mut self,
        input: &Entity<InputState>,
        cx: &mut Context<Self>,
    ) -> Option<bool> {
        if let Some(name_input) = self.custom_theme_inputs.get(CUSTOM_THEME_NAME_INPUT_KEY)
            && input == name_input
        {
            let name = name_input.read(cx).value().trim().to_string();
            self.config.set_custom_theme_draft_name(&name);
            return Some(true);
        }

        for mode in custom_theme_modes() {
            for field in custom_theme_field_specs() {
                let input_key = custom_theme_input_key(mode, field.key);
                let Some(field_input) = self.custom_theme_inputs.get(&input_key) else {
                    continue;
                };
                if input != field_input {
                    continue;
                }

                let value = field_input.read(cx).value().trim().to_string();
                match field.domain {
                    CustomThemeFieldDomain::ThemeColor | CustomThemeFieldDomain::HighlightColor => {
                        if !value.is_empty() && try_parse_color(&value).is_err() {
                            return Some(false);
                        }
                        self.config
                            .set_custom_theme_override(mode, field.key, &value);
                    }
                }
                return Some(true);
            }
        }

        None
    }

    fn apply_custom_theme_preview(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Result<()> {
        let (_, light, dark) = build_custom_theme_set(&self.config, cx)?;
        self.appearance.light_theme_name = light.name.clone();
        self.appearance.dark_theme_name = dark.name.clone();

        // ThemeRegistry only inserts a name once, so preview must use this fresh pair directly.
        self.apply_theme_configs(Rc::new(light), Rc::new(dark), window, cx);
        self.status = "custom theme preview".into();
        window.refresh();
        cx.notify();
        Ok(())
    }

    pub(crate) fn pick_custom_theme_save_path(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let start_dir = self
            .config
            .theme_dir()
            .or_else(|| ConfigStore::theme_dir_path().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        let folder_dialog = rfd::AsyncFileDialog::new()
            .set_directory(start_dir)
            .pick_folder();

        cx.spawn_in(window, async move |this, mut cx| {
            if let Some(folder) = folder_dialog.await {
                let _ = gpui::AsyncWindowContext::update(&mut cx, |window, cx| {
                    let _ = this.update(cx, |this, cx| {
                        Self::set_input_value(
                            &this.custom_theme_save_path_input,
                            folder.path().to_string_lossy().to_string(),
                            window,
                            cx,
                        );
                    });
                });
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    pub(crate) fn import_custom_theme_file(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let start_dir = self
            .config
            .theme_dir()
            .or_else(|| ConfigStore::theme_dir_path().ok())
            .unwrap_or_else(|| PathBuf::from("."));
        let file_dialog = rfd::AsyncFileDialog::new()
            .set_directory(start_dir)
            .add_filter("Theme JSON", &["json"])
            .pick_file();

        cx.spawn_in(window, async move |this, mut cx| {
            if let Some(file) = file_dialog.await {
                let source_path = file.path().to_path_buf();
                let import_result = fs::read_to_string(&source_path)
                    .with_context(|| format!("failed to read {}", source_path.display()))
                    .map(|content| (source_path, content));

                let _ = gpui::AsyncWindowContext::update(&mut cx, |window, cx| {
                    let _ = this.update(cx, |this, cx| match import_result {
                        Ok((source_path, content)) => {
                            this.finish_import_custom_theme_file(&source_path, content, window, cx);
                        }
                        Err(err) => {
                            this.status = format!("failed to import theme file: {err:#}").into();
                            cx.notify();
                        }
                    });
                });
            }
            Ok::<(), anyhow::Error>(())
        })
        .detach();
    }

    fn finish_import_custom_theme_file(
        &mut self,
        source_path: &Path,
        content: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let theme_set = match serde_json::from_str::<ThemeSet>(&content) {
            Ok(theme_set) => theme_set,
            Err(err) => {
                self.status = format!("failed to parse theme file: {err:#}").into();
                cx.notify();
                return;
            }
        };

        let fallback_light = if self.appearance.light_theme_name.as_ref().trim().is_empty() {
            ThemeRegistry::global(cx)
                .default_light_theme()
                .name
                .to_string()
        } else {
            self.appearance.light_theme_name.to_string()
        };
        let fallback_dark = if self.appearance.dark_theme_name.as_ref().trim().is_empty() {
            ThemeRegistry::global(cx)
                .default_dark_theme()
                .name
                .to_string()
        } else {
            self.appearance.dark_theme_name.to_string()
        };
        let (light_theme_name, dark_theme_name) =
            match imported_theme_names(&theme_set, &fallback_light, &fallback_dark) {
                Ok(names) => names,
                Err(err) => {
                    self.status = format!("failed to import theme file: {err:#}").into();
                    cx.notify();
                    return;
                }
            };

        if let Err(err) = ThemeRegistry::global_mut(cx).load_themes_from_str(&content) {
            self.status = format!("failed to register imported theme: {err:#}").into();
            cx.notify();
            return;
        }

        let theme_dir = match self
            .config
            .theme_dir()
            .or_else(|| ConfigStore::theme_dir_path().ok())
        {
            Some(theme_dir) => theme_dir,
            None => {
                self.status = "failed to resolve local theme directory".into();
                cx.notify();
                return;
            }
        };
        let saved_path =
            match write_imported_theme_file(&theme_dir, &theme_set, source_path, &content) {
                Ok(path) => path,
                Err(err) => {
                    self.status = format!("failed to save imported theme file: {err:#}").into();
                    cx.notify();
                    return;
                }
            };

        let profile_name = imported_theme_profile_name(&theme_set, source_path);
        let profile = self.config.activate_imported_theme_profile(
            profile_name,
            light_theme_name.clone(),
            dark_theme_name.clone(),
        );
        self.appearance.light_theme_name = light_theme_name.into();
        self.appearance.dark_theme_name = dark_theme_name.into();
        self.sync_custom_theme_inputs_from_draft(window, cx);
        self.apply_theme_preferences(window, cx);
        self.persist_theme_preferences();
        self.status = format!(
            "theme imported: {} ({})",
            profile.name,
            crate::diagnostics::mask_path(saved_path.to_string_lossy().as_ref())
        )
        .into();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn set_follow_system_theme(
        &mut self,
        follow: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.appearance.follow_system_theme = follow;
        if follow {
            self.status = "theme mode: system".into();
        } else {
            self.status = format!("theme mode: {}", cx.theme().mode.name()).into();
        }
        self.apply_theme_preferences(window, cx);
        self.persist_theme_preferences();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn set_display_language(
        &mut self,
        locale: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.config.set_locale(locale);
        let mut active_locale = locale.to_string();
        if active_locale == "system" {
            active_locale = sys_locale::get_locale().unwrap_or_else(|| "en".to_string());
            if active_locale.starts_with("zh") {
                active_locale = "zh-CN".to_string();
            } else {
                active_locale = "en".to_string();
            }
        }
        rust_i18n::set_locale(&active_locale);
        gpui_component::set_locale(&active_locale);
        self.config.save_logged("set_display_language");
        window.refresh();
        cx.notify();
    }

    pub(crate) fn apply_theme_preferences(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        ensure_embedded_themes_registered(cx);
        let light_theme =
            self.resolve_selected_theme(&self.appearance.light_theme_name, ThemeMode::Light, cx);
        let dark_theme =
            self.resolve_selected_theme(&self.appearance.dark_theme_name, ThemeMode::Dark, cx);

        self.apply_theme_configs(light_theme, dark_theme, window, cx);
    }

    pub(crate) fn apply_theme_preferences_for_system(&mut self, cx: &mut Context<Self>) {
        ensure_embedded_themes_registered(cx);
        let light_theme =
            self.resolve_selected_theme(&self.appearance.light_theme_name, ThemeMode::Light, cx);
        let dark_theme =
            self.resolve_selected_theme(&self.appearance.dark_theme_name, ThemeMode::Dark, cx);
        let active_mode = cx.window_appearance().into();

        self.apply_theme_configs_for_mode(light_theme, dark_theme, active_mode, cx);
    }

    fn apply_theme_configs(
        &self,
        light_theme: Rc<ThemeConfig>,
        dark_theme: Rc<ThemeConfig>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let active_mode = if self.appearance.follow_system_theme {
            window.appearance().into()
        } else {
            self.appearance.theme_mode
        };
        self.apply_theme_configs_for_mode(light_theme, dark_theme, active_mode, cx);
        window.refresh();
    }

    fn apply_theme_configs_for_mode(
        &self,
        light_theme: Rc<ThemeConfig>,
        dark_theme: Rc<ThemeConfig>,
        active_mode: ThemeMode,
        cx: &mut Context<Self>,
    ) {
        let active_theme = if active_mode.is_dark() {
            dark_theme.clone()
        } else {
            light_theme.clone()
        };

        let theme = Theme::global_mut(cx);
        theme.light_theme = light_theme;
        theme.dark_theme = dark_theme;
        theme.apply_config(&active_theme);
        apply_ui_font_brightness(theme, self.appearance.ui_font_brightness);
        theme.font_size = px(self.appearance.ui_font_size);
        set_theme_font_names(theme, &self.appearance.ui_font_family);
    }

    pub(crate) fn save_custom_appearance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let previous_draft = self.config.custom_theme_draft();
        let theme_name = self
            .custom_theme_inputs
            .get(CUSTOM_THEME_NAME_INPUT_KEY)
            .expect("custom theme name input missing")
            .read(cx)
            .value()
            .trim()
            .to_string();
        self.config.set_custom_theme_draft_name(&theme_name);
        let save_path = self
            .custom_theme_save_path_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        self.config.set_custom_theme_save_path(&save_path);

        for mode in custom_theme_modes() {
            for field in custom_theme_field_specs() {
                let input_key = custom_theme_input_key(mode, field.key);
                let value = self
                    .custom_theme_inputs
                    .get(&input_key)
                    .expect("custom theme input missing")
                    .read(cx)
                    .value()
                    .trim()
                    .to_string();

                match field.domain {
                    CustomThemeFieldDomain::ThemeColor | CustomThemeFieldDomain::HighlightColor => {
                        if !value.is_empty() && try_parse_color(&value).is_err() {
                            self.status = format!(
                                "invalid color for {}: use hex like #RRGGBB or #RRGGBBAA",
                                field.label
                            )
                            .into();
                            cx.notify();
                            return;
                        }
                        self.config
                            .set_custom_theme_override(mode, field.key, &value);
                    }
                }
            }
        }

        let (theme_set, _, _) = match build_custom_theme_set(&self.config, cx) {
            Ok(themes) => themes,
            Err(err) => {
                self.status = format!("failed to build custom theme: {err:#}").into();
                cx.notify();
                return;
            }
        };

        let saved_path = match write_custom_theme_file(&self.config, &theme_set) {
            Ok(path) => path,
            Err(err) => {
                self.status = format!("failed to save custom theme file: {err:#}").into();
                cx.notify();
                return;
            }
        };

        match serde_json::to_string_pretty(&theme_set) {
            Ok(theme_json) => {
                if let Err(err) = ThemeRegistry::global_mut(cx).load_themes_from_str(&theme_json) {
                    tracing::warn!(
                        component = "theme",
                        operation = "register_custom_theme",
                        error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                        "Failed to register custom theme"
                    );
                }
            }
            Err(err) => {
                tracing::warn!(
                    component = "theme",
                    operation = "serialize_custom_theme",
                    error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                    "Failed to serialize custom theme"
                );
            }
        }

        let previous_light =
            custom_theme_registry_name(&previous_draft.theme_name, ThemeMode::Light);
        let previous_dark = custom_theme_registry_name(&previous_draft.theme_name, ThemeMode::Dark);
        let current_light =
            custom_theme_registry_name(&self.current_custom_theme_draft_name(), ThemeMode::Light);
        let current_dark =
            custom_theme_registry_name(&self.current_custom_theme_draft_name(), ThemeMode::Dark);
        self.config
            .promote_active_theme_profile_to_custom(current_light.clone(), current_dark.clone());

        if self.appearance.light_theme_name.as_ref() == previous_draft.theme_name
            || self.appearance.light_theme_name.as_ref() == previous_light
        {
            self.appearance.light_theme_name = current_light.clone().into();
        }
        if self.appearance.dark_theme_name.as_ref() == previous_draft.theme_name
            || self.appearance.dark_theme_name.as_ref() == previous_dark
        {
            self.appearance.dark_theme_name = current_dark.clone().into();
        }

        if Theme::global(cx).mode.is_dark() {
            self.appearance.dark_theme_name = current_dark.into();
        } else {
            self.appearance.light_theme_name = current_light.into();
        }

        self.apply_theme_preferences(window, cx);
        self.persist_theme_preferences();
        self.status = format!(
            "custom theme saved: {}",
            crate::diagnostics::mask_path(saved_path.to_string_lossy().as_ref())
        )
        .into();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn reset_custom_appearance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.config.reset_custom_theme_draft();
        if let Err(err) = self.config.save() {
            tracing::error!(
                component = "theme",
                operation = "reset_custom_theme",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Failed to reset custom theme draft"
            );
            self.status = format!("failed to reset custom theme draft: {err:#}").into();
            cx.notify();
            return;
        }

        self.sync_custom_theme_inputs_from_draft(window, cx);
        if let Err(err) = self.apply_custom_theme_preview(window, cx) {
            tracing::debug!(
                component = "theme",
                operation = "preview_custom_theme_reset",
                error = %crate::diagnostics::sanitize_error(&format!("{err:#}")),
                "Skipping custom theme reset preview until inputs are valid"
            );
        }

        self.status = "custom theme editor reset".into();
        cx.notify();
    }

    pub(crate) fn persist_theme_preferences(&mut self) {
        let theme_mode_str = match self.appearance.theme_mode {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };
        self.config.set_theme_preferences(
            self.appearance.follow_system_theme,
            theme_mode_str,
            self.appearance.light_theme_name.to_string(),
            self.appearance.dark_theme_name.to_string(),
        );
        self.config.save_logged("save_theme_preferences");
    }
}

#[cfg(test)]
mod import_theme_tests {
    use super::{
        CUSTOM_THEME_CORE_FIELDS, custom_theme_inherited_field_value_from_theme_value,
        imported_theme_names, unique_imported_theme_file_path,
    };
    use gpui_component::{ThemeMode, ThemeSet};
    use std::{
        fs,
        time::{SystemTime, UNIX_EPOCH},
    };

    #[test]
    fn imported_theme_names_use_file_modes() {
        let theme_set: ThemeSet =
            serde_json::from_str(include_str!("../../assets/themes/solarized.json"))
                .expect("solarized theme parses");

        let (light, dark) =
            imported_theme_names(&theme_set, "Fallback Light", "Fallback Dark").unwrap();

        assert_eq!(light, "Solarized Light");
        assert_eq!(dark, "Solarized Dark");
    }

    #[test]
    fn imported_theme_names_fill_missing_mode_from_fallback() {
        let theme_set: ThemeSet = serde_json::from_str(
            r##"{
                "name": "Dark Only",
                "themes": [
                    {
                        "name": "Only Dark",
                        "mode": "dark",
                        "colors": {}
                    }
                ]
            }"##,
        )
        .expect("dark-only theme parses");

        let (light, dark) =
            imported_theme_names(&theme_set, "Fallback Light", "Fallback Dark").unwrap();

        assert_eq!(light, "Fallback Light");
        assert_eq!(dark, "Only Dark");
    }

    #[test]
    fn embedded_single_mode_theme_families_include_light_companions() {
        let tokyo: ThemeSet =
            serde_json::from_str(include_str!("../../assets/themes/tokyonight.json"))
                .expect("tokyo theme parses");
        let matrix: ThemeSet =
            serde_json::from_str(include_str!("../../assets/themes/matrix.json"))
                .expect("matrix theme parses");

        for name in ["Tokyo Night Light", "Tokyo Storm Light", "Tokyo Moon Light"] {
            assert!(
                tokyo
                    .themes
                    .iter()
                    .any(|theme| theme.name.as_ref() == name && theme.mode == ThemeMode::Light),
                "{name} light companion exists"
            );
        }
        assert!(
            matrix.themes.iter().any(
                |theme| theme.name.as_ref() == "Matrix Light" && theme.mode == ThemeMode::Light
            )
        );
    }

    #[test]
    fn imported_theme_file_path_is_unique() {
        let theme_set: ThemeSet =
            serde_json::from_str(include_str!("../../assets/themes/solarized.json"))
                .expect("solarized theme parses");
        let suffix = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let theme_dir = std::env::temp_dir().join(format!("ax-shell-import-theme-test-{suffix}"));
        fs::create_dir_all(&theme_dir).expect("create temp theme dir");

        let first =
            unique_imported_theme_file_path(&theme_dir, &theme_set, std::path::Path::new("x.json"));
        fs::write(&first, "{}").expect("write first imported theme placeholder");
        let second =
            unique_imported_theme_file_path(&theme_dir, &theme_set, std::path::Path::new("x.json"));

        assert_eq!(
            first.file_name().and_then(|name| name.to_str()),
            Some("imported-solarized.json")
        );
        assert_eq!(
            second.file_name().and_then(|name| name.to_str()),
            Some("imported-solarized-2.json")
        );

        fs::remove_dir_all(&theme_dir).expect("remove temp theme dir");
    }

    #[test]
    fn inherited_field_value_comes_from_selected_base_theme() {
        let theme_set: ThemeSet =
            serde_json::from_str(include_str!("../../assets/themes/solarized.json"))
                .expect("solarized theme parses");
        let light = theme_set
            .themes
            .iter()
            .find(|theme| theme.mode == ThemeMode::Light)
            .expect("solarized light exists");
        let theme_value = serde_json::to_value(light).expect("serialize solarized light");
        let background = CUSTOM_THEME_CORE_FIELDS
            .iter()
            .find(|field| field.key == "background")
            .expect("background field exists");

        let inherited =
            custom_theme_inherited_field_value_from_theme_value(&theme_value, background);

        assert_eq!(inherited, "#FDF6E3");
    }
}
