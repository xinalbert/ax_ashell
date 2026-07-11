use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_general_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let current_locale = shell.config.locale().to_string();
    let settings_close_shortcut_confirms = shell.config.settings_close_shortcut_confirms();

    SettingPage::new(t!("settings_general").to_string())
        .icon(IconName::Settings)
        .group(
            SettingGroup::new()
                .title(t!("settings_language").to_string())
                .item(SettingItem::new(
                    t!("language").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        let locale = current_locale.clone();
                        move |_, _window, _cx| {
                            let label = if locale == "en" {
                                t!("english").to_string()
                            } else if locale == "zh-CN" {
                                t!("chinese").to_string()
                            } else {
                                t!("follow_system").to_string()
                            };
                            super::fast_menu::fast_settings_menu(
                                "language-dropdown",
                                label,
                                Some(IconName::Globe),
                                px(160.),
                                None,
                                vec![
                                    super::fast_menu::FastMenuItem::new(
                                        t!("follow_system").to_string(),
                                        locale == "system",
                                        |this, window, cx| {
                                            this.set_display_language("system", window, cx)
                                        },
                                    ),
                                    super::fast_menu::FastMenuItem::new(
                                        t!("english").to_string(),
                                        locale == "en",
                                        |this, window, cx| {
                                            this.set_display_language("en", window, cx)
                                        },
                                    ),
                                    super::fast_menu::FastMenuItem::new(
                                        t!("chinese").to_string(),
                                        locale == "zh-CN",
                                        |this, window, cx| {
                                            this.set_display_language("zh-CN", window, cx)
                                        },
                                    ),
                                ],
                                view.clone(),
                            )
                            .into_any_element()
                        }
                    }),
                )),
        )
        .group(
            SettingGroup::new()
                .title(t!("settings_behavior").to_string())
                .item(
                    SettingItem::new(
                        t!("settings_close_shortcut_confirms").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("settings-close-shortcut-confirms")
                                    .small()
                                    .checked(settings_close_shortcut_confirms)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_settings_close_shortcut_confirms(*checked);
                                        this.config.save_logged("set_settings_close_shortcut");
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("settings_close_shortcut_confirms_hint").to_string()),
                ),
        )
}
