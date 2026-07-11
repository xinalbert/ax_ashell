use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem};

fn terminal_font_names(window: &mut Window, cx: &mut gpui::App, font_size: f32) -> Vec<String> {
    let mut names = cx.text_system().all_font_names();
    names.retain(|name| {
        crate::terminal::element::terminal_font_is_monospace(
            window,
            name.clone().into(),
            px(font_size),
        )
    });
    names.sort_unstable();
    names.dedup();
    names
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
                    let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE
                        .load(std::sync::atomic::Ordering::Relaxed);
                    let label = if current == *".SystemUIFont" || current.is_empty() {
                        t!("system_default").to_string()
                    } else if !using_system_maple && current == "Maple Mono NF CN" {
                        format!("Maple Mono NF CN ({})", t!("software_builtin"))
                    } else {
                        current.clone()
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
                                let mut names = cx.text_system().all_font_names();
                                let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE
                                    .load(std::sync::atomic::Ordering::Relaxed);
                                let mut items = vec![super::fast_menu::FastMenuItem::new(
                                    t!("system_default").to_string(),
                                    current == *".SystemUIFont" || current.is_empty(),
                                    |this, window, cx| {
                                        this.change_ui_font_family(".SystemUIFont", window, cx);
                                    },
                                )];
                                let maple_font = "Maple Mono NF CN".to_string();
                                if !using_system_maple && names.contains(&maple_font) {
                                    names.retain(|name| name != &maple_font);
                                    items.push(super::fast_menu::FastMenuItem::new(
                                        format!("{} ({})", maple_font, t!("software_builtin")),
                                        current == maple_font,
                                        |this, window, cx| {
                                            this.change_ui_font_family(
                                                "Maple Mono NF CN",
                                                window,
                                                cx,
                                            );
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
                    let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE
                        .load(std::sync::atomic::Ordering::Relaxed);
                    let label = if !using_system_maple && current == "Maple Mono NF CN" {
                        format!("Maple Mono NF CN ({})", t!("software_builtin"))
                    } else {
                        current.clone()
                    };
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
                                let mut items = Vec::new();
                                let maple_font = "Maple Mono NF CN".to_string();
                                let using_system_maple = crate::app::theme::USING_SYSTEM_MAPLE
                                    .load(std::sync::atomic::Ordering::Relaxed);
                                if !using_system_maple && names.contains(&maple_font) {
                                    names.retain(|name| name != &maple_font);
                                    items.push(super::fast_menu::FastMenuItem::new(
                                        format!("{} ({})", maple_font, t!("software_builtin")),
                                        current == maple_font,
                                        |this, _window, cx| {
                                            this.change_terminal_font_family(
                                                "Maple Mono NF CN",
                                                cx,
                                            );
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
