use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_appearance_page(
    view: &gpui::Entity<AxShell>,
    shell: &AxShell,
) -> SettingPage {
    let follow_system_theme = shell.appearance.follow_system_theme;
    let theme_mode_is_dark = shell.appearance.theme_mode.is_dark();
    let active_theme_profile_id = shell.config.active_theme_profile_id().to_string();
    let active_theme_profile_name = shell
        .config
        .active_theme_profile()
        .map(|profile| profile.name.clone())
        .unwrap_or_else(|| t!("theme_profile_custom").to_string());
    let theme_profiles = shell.config.theme_profiles().to_vec();
    let ui_font_brightness = shell.appearance.ui_font_brightness;
    let terminal_font_brightness = shell.appearance.terminal_font_brightness;
    let title_bar_style = shell.config.effective_title_bar_style();

    SettingPage::new(t!("settings_appearance").to_string())
        .icon(IconName::Palette)
        .default_open(true)
        .group(
            SettingGroup::new()
                .title(t!("settings_group_appearance").to_string())
                .item(SettingItem::new(
                    t!("theme_mode").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, _window, _cx| {
                            let label = if follow_system_theme {
                                t!("follow_system").to_string()
                            } else if theme_mode_is_dark {
                                t!("use_dark_mode").to_string()
                            } else {
                                t!("use_light_mode").to_string()
                            };
                            let icon = if follow_system_theme {
                                IconName::Sun
                            } else if theme_mode_is_dark {
                                IconName::Moon
                            } else {
                                IconName::Sun
                            };
                            super::fast_menu::fast_settings_menu(
                                "theme-mode-dropdown",
                                label,
                                Some(icon),
                                px(160.),
                                None,
                                vec![
                                    super::fast_menu::FastMenuItem::new(
                                        t!("follow_system").to_string(),
                                        follow_system_theme,
                                        |this, window, cx| {
                                            this.set_follow_system_theme(true, window, cx)
                                        },
                                    ),
                                    super::fast_menu::FastMenuItem::new(
                                        t!("use_light_mode").to_string(),
                                        !follow_system_theme && !theme_mode_is_dark,
                                        |this, window, cx| {
                                            this.switch_theme_mode(
                                                gpui_component::ThemeMode::Light,
                                                window,
                                                cx,
                                            )
                                        },
                                    ),
                                    super::fast_menu::FastMenuItem::new(
                                        t!("use_dark_mode").to_string(),
                                        !follow_system_theme && theme_mode_is_dark,
                                        |this, window, cx| {
                                            this.switch_theme_mode(
                                                gpui_component::ThemeMode::Dark,
                                                window,
                                                cx,
                                            )
                                        },
                                    ),
                                ],
                                view.clone(),
                            )
                            .into_any_element()
                        }
                    }),
                ))
                .item(
                    SettingItem::new(
                        t!("theme").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            let current_profile_id = active_theme_profile_id.clone();
                            let current_profile_name = active_theme_profile_name.clone();
                            let theme_profiles = theme_profiles.clone();
                            move |_, _window, _cx| {
                                let items = theme_profiles
                                    .iter()
                                    .cloned()
                                    .map(|profile| {
                                        let profile_id = profile.id.clone();
                                        super::fast_menu::FastMenuItem::new(
                                            profile.name,
                                            profile_id == current_profile_id,
                                            move |this, window, cx| {
                                                this.apply_theme_profile(
                                                    profile_id.clone(),
                                                    window,
                                                    cx,
                                                );
                                            },
                                        )
                                    })
                                    .collect();
                                super::fast_menu::fast_settings_menu(
                                    "theme-profile-dropdown",
                                    current_profile_name.clone(),
                                    Some(IconName::Palette),
                                    px(220.),
                                    Some(px(320.)),
                                    items,
                                    view.clone(),
                                )
                                .into_any_element()
                            }
                        }),
                    )
                    .description(t!("theme_profile_hint").to_string()),
                )
                .item(
                    SettingItem::new(
                        t!("ui_font_brightness").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                h_flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        Button::new("ui-font-brightness-down")
                                            .small()
                                            .label("-")
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, window, cx| {
                                                    this.change_ui_font_brightness(
                                                        -0.05, window, cx,
                                                    )
                                                },
                                            )),
                                    )
                                    .child(
                                        div()
                                            .min_w(px(64.))
                                            .text_center()
                                            .child(format!("{ui_font_brightness:.2}")),
                                    )
                                    .child(
                                        Button::new("ui-font-brightness-up")
                                            .small()
                                            .label("+")
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, window, cx| {
                                                    this.change_ui_font_brightness(0.05, window, cx)
                                                },
                                            )),
                                    )
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("font_brightness_hint").to_string()),
                )
                .item(
                    SettingItem::new(
                        t!("terminal_font_brightness").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                h_flex()
                                    .items_center()
                                    .gap_3()
                                    .child(
                                        Button::new("terminal-font-brightness-down")
                                            .small()
                                            .label("-")
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, window, cx| {
                                                    this.change_terminal_font_brightness(
                                                        -0.05, window, cx,
                                                    )
                                                },
                                            )),
                                    )
                                    .child(
                                        div()
                                            .min_w(px(64.))
                                            .text_center()
                                            .child(format!("{terminal_font_brightness:.2}")),
                                    )
                                    .child(
                                        Button::new("terminal-font-brightness-up")
                                            .small()
                                            .label("+")
                                            .on_click(window.listener_for(
                                                &view,
                                                |this, _, window, cx| {
                                                    this.change_terminal_font_brightness(
                                                        0.05, window, cx,
                                                    )
                                                },
                                            )),
                                    )
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("font_brightness_hint").to_string()),
                )
                .item(SettingItem::new(
                    format!("{}{}", t!("title_bar_style"), t!("restart_hint")),
                    SettingField::render({
                        let view = view.clone();
                        move |_, _window, _cx| {
                            let supports_integrated = cfg!(target_os = "macos");
                            let label = match title_bar_style {
                                crate::config::TitleBarStyle::Native => {
                                    t!("title_bar_native").to_string()
                                }
                                crate::config::TitleBarStyle::Integrated => {
                                    t!("title_bar_integrated").to_string()
                                }
                            };
                            let mut items = vec![super::fast_menu::FastMenuItem::new(
                                t!("title_bar_native").to_string(),
                                title_bar_style == crate::config::TitleBarStyle::Native,
                                |this, _window, cx| {
                                    this.config
                                        .set_title_bar_style(crate::config::TitleBarStyle::Native);
                                    this.config.save_logged("set_native_title_bar");
                                    cx.notify();
                                },
                            )];
                            if supports_integrated {
                                items.push(super::fast_menu::FastMenuItem::new(
                                    t!("title_bar_integrated").to_string(),
                                    title_bar_style == crate::config::TitleBarStyle::Integrated,
                                    |this, _window, cx| {
                                        this.config.set_title_bar_style(
                                            crate::config::TitleBarStyle::Integrated,
                                        );
                                        this.config.save_logged("set_integrated_title_bar");
                                        cx.notify();
                                    },
                                ));
                            }
                            super::fast_menu::fast_settings_menu(
                                "title-bar-style-dropdown",
                                label,
                                None,
                                px(160.),
                                None,
                                items,
                                view.clone(),
                            )
                            .into_any_element()
                        }
                    }),
                )),
        )
        .group(super::font_page::settings_font_group(view, shell))
}
