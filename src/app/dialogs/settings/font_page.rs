use super::*;

use std::cell::RefCell;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem};

thread_local! {
    static SETTINGS_FONT_NAMES_CACHE: RefCell<Option<Vec<String>>> = const { RefCell::new(None) };
    static TERMINAL_FONT_NAMES_CACHE: RefCell<Vec<(u32, Vec<String>)>> = const { RefCell::new(Vec::new()) };
}

fn settings_font_names(cx: &mut gpui::App) -> Vec<String> {
    SETTINGS_FONT_NAMES_CACHE.with(|cache| {
        if let Some(names) = cache.borrow().as_ref() {
            return names.clone();
        }

        let names = cx.text_system().all_font_names();
        *cache.borrow_mut() = Some(names.clone());
        names
    })
}

fn terminal_font_names(window: &mut Window, cx: &mut gpui::App, font_size: f32) -> Vec<String> {
    let cache_key = font_size.to_bits();
    if let Some(names) = TERMINAL_FONT_NAMES_CACHE.with(|cache| {
        cache
            .borrow()
            .iter()
            .find_map(|(key, names)| (*key == cache_key).then(|| names.clone()))
    }) {
        return names;
    }

    let mut names = settings_font_names(cx);
    names.retain(|name| {
        !crate::app::theme::BUILT_IN_TERMINAL_FONT_FAMILIES.contains(&name.as_str())
            && crate::terminal::element::terminal_font_is_monospace(
                window,
                name.clone().into(),
                px(font_size),
            )
    });
    names.sort_unstable();
    names.dedup();
    TERMINAL_FONT_NAMES_CACHE.with(|cache| {
        let mut cache = cache.borrow_mut();
        cache.retain(|(key, _)| *key != cache_key);
        cache.push((cache_key, names.clone()));
        if cache.len() > 4 {
            cache.remove(0);
        }
    });
    names
}

fn font_family_label(family: &str) -> String {
    if crate::app::theme::BUILT_IN_FONT_FAMILIES.contains(&family) {
        format!("{} ({})", family, t!("software_builtin"))
    } else {
        family.to_string()
    }
}

fn remove_built_in_font_names(names: &mut Vec<String>) {
    names.retain(|name| !crate::app::theme::BUILT_IN_FONT_FAMILIES.contains(&name.as_str()));
}

pub(super) fn settings_font_group(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingGroup {
    let ui_font_size = shell.appearance.ui_font_size;
    let terminal_font_size = shell.appearance.terminal_font_size;
    let ui_font_family = shell.appearance.ui_font_family.to_string();
    let terminal_font_family = shell.appearance.terminal_font_family.to_string();
    let cursor_style = shell.appearance.cursor_style;

    SettingGroup::new()
        .title(t!("settings_group_font").to_string())
        .item(SettingItem::new(
            t!("ui_font_size").to_string(),
            SettingField::render({
                let view = view.clone();
                move |_, window, _cx| {
                    h_flex()
                        .items_center()
                        .gap_3()
                        .child(
                            Button::new("ui-font-size-down")
                                .small()
                                .label("-")
                                .on_click(window.listener_for(&view, |this, _, _, cx| {
                                    this.change_ui_font_size(-1.0, cx)
                                })),
                        )
                        .child(
                            div()
                                .min_w(px(64.))
                                .text_center()
                                .child(format!("{ui_font_size:.0}px")),
                        )
                        .child(Button::new("ui-font-size-up").small().label("+").on_click(
                            window.listener_for(&view, |this, _, _, cx| {
                                this.change_ui_font_size(1.0, cx)
                            }),
                        ))
                        .into_any_element()
                }
            }),
        ))
        .item(SettingItem::new(
            t!("terminal_font_size").to_string(),
            SettingField::render({
                let view = view.clone();
                move |_, window, _cx| {
                    h_flex()
                        .items_center()
                        .gap_3()
                        .child(
                            Button::new("terminal-font-size-down")
                                .small()
                                .label("-")
                                .on_click(window.listener_for(&view, |this, _, _, cx| {
                                    this.change_terminal_font_size(-1.0, cx)
                                })),
                        )
                        .child(
                            div()
                                .min_w(px(64.))
                                .text_center()
                                .child(format!("{terminal_font_size:.0}px")),
                        )
                        .child(
                            Button::new("terminal-font-size-up")
                                .small()
                                .label("+")
                                .on_click(window.listener_for(&view, |this, _, _, cx| {
                                    this.change_terminal_font_size(1.0, cx)
                                })),
                        )
                        .into_any_element()
                }
            }),
        ))
        .item(SettingItem::new(
            t!("ui_font_family").to_string(),
            SettingField::render({
                let view = view.clone();
                let current = ui_font_family.clone();
                move |_, _window, _cx| {
                    let label = if current == *".SystemUIFont" || current.is_empty() {
                        t!("system_default").to_string()
                    } else {
                        font_family_label(&current)
                    };
                    super::fast_menu::fast_settings_menu_lazy(
                        "ui-font-dropdown",
                        label,
                        Some(IconName::ChevronsUpDown),
                        px(200.),
                        Some(px(320.)),
                        {
                            let current = current.clone();
                            move |_window, cx| {
                                let mut names = settings_font_names(cx);
                                remove_built_in_font_names(&mut names);
                                let mut items = vec![super::fast_menu::FastMenuItem::new(
                                    t!("system_default").to_string(),
                                    current == *".SystemUIFont" || current.is_empty(),
                                    |this, window, cx| {
                                        this.change_ui_font_family(".SystemUIFont", window, cx);
                                    },
                                )];
                                for &family in crate::app::theme::BUILT_IN_FONT_FAMILIES {
                                    let checked = current == family;
                                    items.push(super::fast_menu::FastMenuItem::new(
                                        font_family_label(family),
                                        checked,
                                        move |this, window, cx| {
                                            this.change_ui_font_family(family, window, cx);
                                        },
                                    ));
                                }
                                for name in names {
                                    let checked = name == current;
                                    items.push(super::fast_menu::FastMenuItem::new(
                                        name.clone(),
                                        checked,
                                        move |this, window, cx| {
                                            this.change_ui_font_family(&name, window, cx);
                                        },
                                    ));
                                }
                                items
                            }
                        },
                        view.clone(),
                    )
                    .into_any_element()
                }
            }),
        ))
        .item(SettingItem::new(
            t!("terminal_font_family").to_string(),
            SettingField::render({
                let view = view.clone();
                let current = terminal_font_family.clone();
                move |_, _window, _cx| {
                    let label = font_family_label(&current);
                    super::fast_menu::fast_settings_menu_lazy(
                        "terminal-font-dropdown",
                        label,
                        Some(IconName::ChevronsUpDown),
                        px(200.),
                        Some(px(320.)),
                        {
                            let current = current.clone();
                            move |window, cx| {
                                let mut names = terminal_font_names(window, cx, terminal_font_size);
                                remove_built_in_font_names(&mut names);
                                let mut items = Vec::new();
                                for &family in crate::app::theme::BUILT_IN_TERMINAL_FONT_FAMILIES {
                                    let checked = current == family;
                                    items.push(super::fast_menu::FastMenuItem::new(
                                        font_family_label(family),
                                        checked,
                                        move |this, _window, cx| {
                                            this.change_terminal_font_family(family, cx);
                                        },
                                    ));
                                }
                                for name in names {
                                    let checked = name == current;
                                    items.push(super::fast_menu::FastMenuItem::new(
                                        name.clone(),
                                        checked,
                                        move |this, _window, cx| {
                                            this.change_terminal_font_family(&name, cx);
                                        },
                                    ));
                                }
                                items
                            }
                        },
                        view.clone(),
                    )
                    .into_any_element()
                }
            }),
        ))
        .item(SettingItem::new(
            t!("cursor_style").to_string(),
            SettingField::render({
                let view = view.clone();
                move |_, _window, _cx| {
                    use crate::config::CursorStyle;

                    let label = match cursor_style {
                        CursorStyle::Default => t!("cursor_style_default").to_string(),
                        CursorStyle::Blink => t!("cursor_style_blink").to_string(),
                        CursorStyle::Beam => t!("cursor_style_beam").to_string(),
                        CursorStyle::BeamBlink => t!("cursor_style_beam_blink").to_string(),
                    };
                    let mut items = Vec::new();
                    for style in [
                        CursorStyle::Default,
                        CursorStyle::Blink,
                        CursorStyle::Beam,
                        CursorStyle::BeamBlink,
                    ] {
                        let checked = style == cursor_style;
                        let label = match style {
                            CursorStyle::Default => t!("cursor_style_default").to_string(),
                            CursorStyle::Blink => t!("cursor_style_blink").to_string(),
                            CursorStyle::Beam => t!("cursor_style_beam").to_string(),
                            CursorStyle::BeamBlink => t!("cursor_style_beam_blink").to_string(),
                        };
                        items.push(super::fast_menu::FastMenuItem::new(
                            label,
                            checked,
                            move |this, _window, cx| {
                                this.change_cursor_style(style, cx);
                            },
                        ));
                    }
                    super::fast_menu::fast_settings_menu(
                        "cursor-style-dropdown",
                        label,
                        Some(IconName::ChevronsUpDown),
                        px(160.),
                        Some(px(320.)),
                        items,
                        view.clone(),
                    )
                    .into_any_element()
                }
            }),
        ))
}

#[cfg(test)]
mod tests {
    use super::remove_built_in_font_names;

    #[test]
    fn built_in_fonts_are_removed_from_system_candidates() {
        let mut names = vec![
            "System Mono".to_string(),
            "JetBrains Mono".to_string(),
            "Iosevka Term".to_string(),
            "Maple Mono NF CN".to_string(),
            "Monaspace Neon Var".to_string(),
        ];

        remove_built_in_font_names(&mut names);

        assert_eq!(names, vec!["System Mono"]);
    }
}
