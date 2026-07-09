use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_language_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let current_locale = shell.config.locale().to_string();

    SettingPage::new(t!("language").to_string())
        .icon(IconName::Globe)
        .group(
            SettingGroup::new()
                .title(t!("language").to_string())
                .item(SettingItem::new(
                    t!("language").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        let locale = current_locale.clone();
                        move |_, _window, _cx| {
                            Button::new("language-dropdown")
                                .small()
                                .icon(IconName::Globe)
                                .label({
                                    if locale == "en" {
                                        t!("english").to_string()
                                    } else if locale == "zh-CN" {
                                        t!("chinese").to_string()
                                    } else {
                                        t!("follow_system").to_string()
                                    }
                                })
                                .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                    let view = view.clone();
                                    let current_locale = locale.clone();
                                    move |mut menu, window, _cx| {
                                        menu = menu
                                            .min_w(160.)
                                            .item(
                                                PopupMenuItem::new(t!("follow_system").to_string())
                                                    .checked(current_locale == "system")
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            this.set_display_language(
                                                                "system", window, cx,
                                                            )
                                                        },
                                                    )),
                                            )
                                            .separator()
                                            .item(
                                                PopupMenuItem::new(t!("english").to_string())
                                                    .checked(current_locale == "en")
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            this.set_display_language(
                                                                "en", window, cx,
                                                            )
                                                        },
                                                    )),
                                            )
                                            .item(
                                                PopupMenuItem::new(t!("chinese").to_string())
                                                    .checked(current_locale == "zh-CN")
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            this.set_display_language(
                                                                "zh-CN", window, cx,
                                                            )
                                                        },
                                                    )),
                                            );
                                        menu
                                    }
                                })
                                .into_any_element()
                        }
                    }),
                )),
        )
}
