use anyhow::{Context as _, Result};
use gpui::{App, Context, Hsla, SharedString, Window, px, rgb};
use gpui_component::{ActiveTheme as _, Colorize, Theme, ThemeMode, ThemeRegistry};

use crate::AxAshell;

pub(crate) const EMBEDDED_THEME_JSONS: &[&str] = &[
    include_str!("../../assets/themes/matrix.json"),
    include_str!("../../assets/themes/tokyonight.json"),
    include_str!("../../assets/themes/gruvbox.json"),
    include_str!("../../assets/themes/solarized.json"),
    include_str!("../../assets/themes/phygerr.json"),
];

use std::sync::atomic::{AtomicBool, Ordering};

pub(crate) static USING_SYSTEM_MAPLE: AtomicBool = AtomicBool::new(false);

pub(crate) fn load_fonts(cx: &mut App) -> Result<()> {
    let has_system_maple = cx
        .text_system()
        .all_font_names()
        .contains(&"Maple Mono NF CN".to_string());
    if has_system_maple {
        USING_SYSTEM_MAPLE.store(true, Ordering::Relaxed);
    } else {
        let regular = std::borrow::Cow::Borrowed(
            include_bytes!("../../assets/fonts/MapleMono-NF-CN-Regular.ttf").as_slice(),
        );
        let bold = std::borrow::Cow::Borrowed(
            include_bytes!("../../assets/fonts/MapleMono-NF-CN-Bold.ttf").as_slice(),
        );
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
            tracing::warn!("failed to load embedded theme: {err:#}");
        }
    }
}

pub(crate) fn set_theme_font_names(theme: &mut Theme, ui_font_family: &str) {
    theme.font_family = ui_font_family.into();
    theme.mono_font_family = ui_font_family.into();
}

fn parse_hex_color(value: &str) -> Option<Hsla> {
    let value = value.trim().trim_start_matches('#');
    if value.len() != 6 {
        return None;
    }
    u32::from_str_radix(value, 16)
        .ok()
        .map(|hex| Hsla::from(rgb(hex)))
}

fn contrast_text_for(background: Hsla) -> Hsla {
    Hsla {
        h: 0.0,
        s: 0.0,
        l: if background.l > 0.58 { 0.08 } else { 0.96 },
        a: 1.0,
    }
}

fn adjust_text_brightness(color: Hsla, factor: f32) -> Hsla {
    Hsla {
        l: (color.l * factor).clamp(0.02, 0.98),
        ..color
    }
}

fn apply_custom_theme_overrides(theme: &mut Theme, config: &crate::session::config::ConfigStore) {
    let is_dark = theme.mode.is_dark();

    if let Some(primary) = parse_hex_color(config.custom_primary_color()) {
        let primary_foreground = contrast_text_for(primary);
        theme.primary = primary;
        theme.button_primary = primary;
        theme.sidebar_primary = primary;
        theme.primary_hover = primary.mix_oklab(theme.background, 0.18);
        theme.primary_active = primary.mix_oklab(theme.background, 0.28);
        theme.button_primary_hover = theme.primary_hover;
        theme.button_primary_active = theme.primary_active;
        theme.primary_foreground = primary_foreground;
        theme.button_primary_foreground = primary_foreground;
        theme.sidebar_primary_foreground = primary_foreground;
        theme.link = primary;
        theme.link_hover = theme.primary_hover;
        theme.link_active = theme.primary_active;
        theme.ring = primary;
        theme.progress_bar = primary;
        theme.selection = primary.alpha(if is_dark { 0.32 } else { 0.20 });
    }

    if let Some(background) = parse_hex_color(config.custom_background_color()) {
        let surface = background;
        let raised = if is_dark {
            surface.mix_oklab(theme.foreground, 0.08)
        } else {
            surface.mix_oklab(theme.foreground, 0.04)
        };
        let subtle = if is_dark {
            surface.mix_oklab(theme.foreground, 0.14)
        } else {
            surface.mix_oklab(theme.foreground, 0.08)
        };
        let border = if is_dark {
            surface.mix_oklab(theme.foreground, 0.20)
        } else {
            surface.mix_oklab(theme.foreground, 0.14)
        };

        theme.background = surface;
        theme.sidebar = surface;
        theme.title_bar = surface;
        theme.tab_bar = raised;
        theme.tab_bar_segmented = raised;
        theme.popover = raised;
        theme.secondary = raised;
        theme.secondary_hover = subtle;
        theme.secondary_active = border;
        theme.muted = raised;
        theme.colors.list = surface;
        theme.list_even = raised;
        theme.list_head = raised;
        theme.list_hover = subtle;
        theme.table = surface;
        theme.table_even = raised;
        theme.table_head = raised;
        theme.table_foot = raised;
        theme.table_hover = subtle;
        theme.tab = surface;
        theme.tab_active = subtle;
        theme.accordion = surface;
        theme.accordion_hover = subtle;
        theme.group_box = surface;
        theme.sidebar_accent = subtle;
        theme.scrollbar = raised.alpha(if is_dark { 0.65 } else { 0.45 });
        theme.scrollbar_thumb = border.alpha(if is_dark { 0.85 } else { 0.60 });
        theme.scrollbar_thumb_hover = border;
        theme.border = border;
        theme.input = border;
        theme.sidebar_border = border;
        theme.table_row_border = border;
        theme.title_bar_border = border;
        theme.window_border = border;
    }

    let brightness = config.custom_font_brightness();
    if (brightness - 1.0).abs() > f32::EPSILON {
        theme.foreground = adjust_text_brightness(theme.foreground, brightness);
        theme.muted_foreground = adjust_text_brightness(theme.muted_foreground, brightness);
        theme.secondary_foreground = adjust_text_brightness(theme.secondary_foreground, brightness);
        theme.popover_foreground = adjust_text_brightness(theme.popover_foreground, brightness);
        theme.tab_foreground = adjust_text_brightness(theme.tab_foreground, brightness);
        theme.tab_active_foreground =
            adjust_text_brightness(theme.tab_active_foreground, brightness);
        theme.sidebar_foreground = adjust_text_brightness(theme.sidebar_foreground, brightness);
        theme.table_head_foreground =
            adjust_text_brightness(theme.table_head_foreground, brightness);
        theme.table_foot_foreground =
            adjust_text_brightness(theme.table_foot_foreground, brightness);
    }
}

impl AxAshell {
    fn custom_theme_name(&self) -> SharedString {
        self.config.custom_theme_name().into()
    }

    fn is_custom_theme_name(&self, name: &SharedString) -> bool {
        name.as_ref() == self.config.custom_theme_name()
    }

    pub(crate) fn switch_theme_mode(
        &mut self,
        mode: ThemeMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.follow_system_theme = false;
        self.theme_mode = mode;
        self.apply_theme_preferences(window, cx);
        self.status = format!("theme mode: {}", cx.theme().mode.name()).into();
        self.persist_theme_preferences();
        cx.notify();
    }

    pub(crate) fn apply_theme(
        &mut self,
        name: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(theme_config) = ThemeRegistry::global(cx).themes().get(&name).cloned() else {
            self.status = format!("theme not found: {name}").into();
            cx.notify();
            return;
        };

        if theme_config.mode.is_dark() {
            self.dark_theme_name = name.clone();
        } else {
            self.light_theme_name = name.clone();
        }
        self.apply_theme_preferences(window, cx);
        self.status = format!("theme: {name}").into();
        self.persist_theme_preferences();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn apply_custom_theme(
        &mut self,
        mode: ThemeMode,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let name = self.custom_theme_name();
        if mode.is_dark() {
            self.dark_theme_name = name.clone();
        } else {
            self.light_theme_name = name.clone();
        }
        self.apply_theme_preferences(window, cx);
        self.status = format!("theme: {name}").into();
        self.persist_theme_preferences();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn set_follow_system_theme(
        &mut self,
        follow: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.follow_system_theme = follow;
        if follow {
            self.status = "theme mode: system".into();
        } else {
            self.status = format!("theme mode: {}", cx.theme().mode.name()).into();
        }
        self.apply_theme_preferences(window, cx);
        self.persist_theme_preferences();
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
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save language preferences: {err:#}");
        }
        window.refresh();
        cx.notify();
    }

    pub(crate) fn apply_theme_preferences(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let light_is_custom = self.is_custom_theme_name(&self.light_theme_name);
        let dark_is_custom = self.is_custom_theme_name(&self.dark_theme_name);
        let light_theme = ThemeRegistry::global(cx)
            .themes()
            .get(&self.light_theme_name)
            .cloned()
            .unwrap_or_else(|| ThemeRegistry::global(cx).default_light_theme().clone());
        let dark_theme = ThemeRegistry::global(cx)
            .themes()
            .get(&self.dark_theme_name)
            .cloned()
            .unwrap_or_else(|| ThemeRegistry::global(cx).default_dark_theme().clone());
        let theme = Theme::global_mut(cx);
        theme.light_theme = light_theme;
        theme.dark_theme = dark_theme;
        theme.font_size = px(self.ui_font_size);
        set_theme_font_names(theme, &self.ui_font_family);

        if self.follow_system_theme {
            Theme::sync_system_appearance(Some(window), cx);
        } else {
            Theme::change(self.theme_mode, Some(window), cx);
        }

        let active_custom_theme = if Theme::global(cx).mode.is_dark() {
            dark_is_custom
        } else {
            light_is_custom
        };
        if active_custom_theme {
            apply_custom_theme_overrides(Theme::global_mut(cx), &self.config);
        }
    }

    pub(crate) fn save_custom_appearance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let theme_name = self
            .custom_theme_name_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let primary_color = self
            .custom_primary_color_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let background_color = self
            .custom_background_color_input
            .read(cx)
            .value()
            .trim()
            .to_string();
        let brightness_raw = self
            .custom_font_brightness_input
            .read(cx)
            .value()
            .trim()
            .to_string();

        if !primary_color.is_empty() && parse_hex_color(&primary_color).is_none() {
            self.status = "invalid primary color, use #RRGGBB".into();
            cx.notify();
            return;
        }
        if !background_color.is_empty() && parse_hex_color(&background_color).is_none() {
            self.status = "invalid background color, use #RRGGBB".into();
            cx.notify();
            return;
        }

        let brightness = match brightness_raw.parse::<f32>() {
            Ok(value) if (0.6..=1.6).contains(&value) => value,
            _ => {
                self.status = "invalid font brightness, use 0.60-1.60".into();
                cx.notify();
                return;
            }
        };

        self.config.set_custom_theme_name(&theme_name);
        self.config.set_custom_primary_color(&primary_color);
        self.config.set_custom_background_color(&background_color);
        self.config.set_custom_font_brightness(brightness);

        let custom_theme_name = self.custom_theme_name();
        if Theme::global(cx).mode.is_dark() {
            self.dark_theme_name = custom_theme_name;
        } else {
            self.light_theme_name = custom_theme_name;
        }
        self.persist_theme_preferences();
        if let Err(err) = self.config.save() {
            self.status = format!("failed to save custom appearance: {err:#}").into();
            cx.notify();
            return;
        }

        self.apply_theme_preferences(window, cx);
        self.status = "custom appearance saved".into();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn reset_custom_appearance(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.config.set_custom_theme_name("Custom Theme");
        self.config.set_custom_primary_color("");
        self.config.set_custom_background_color("");
        self.config.set_custom_font_brightness(1.0);
        if let Err(err) = self.config.save() {
            self.status = format!("failed to reset custom appearance: {err:#}").into();
            cx.notify();
            return;
        }

        self.custom_theme_name_input.update(cx, |input, cx| {
            input.set_value("Custom Theme", window, cx);
        });
        self.custom_primary_color_input.update(cx, |input, cx| {
            input.set_value("", window, cx);
        });
        self.custom_background_color_input.update(cx, |input, cx| {
            input.set_value("", window, cx);
        });
        self.custom_font_brightness_input.update(cx, |input, cx| {
            input.set_value("1.00", window, cx);
        });

        self.apply_theme_preferences(window, cx);
        self.status = "custom appearance reset".into();
        window.refresh();
        cx.notify();
    }

    pub(crate) fn persist_theme_preferences(&mut self) {
        let theme_mode_str = match self.theme_mode {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        };
        self.config.set_theme_preferences(
            self.follow_system_theme,
            theme_mode_str,
            self.light_theme_name.to_string(),
            self.dark_theme_name.to_string(),
        );
        if let Err(err) = self.config.save() {
            tracing::warn!("failed to save theme preferences: {err:#}");
        }
    }
}
