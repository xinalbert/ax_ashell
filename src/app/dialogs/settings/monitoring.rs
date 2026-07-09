use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_monitoring_page(
    view: &gpui::Entity<AxShell>,
    shell: &AxShell,
) -> SettingPage {
    let show_monitoring_dashboard = shell.config.show_monitoring_dashboard();
    let monitoring_position = shell.config.monitoring_position().to_string();

    SettingPage::new(t!("settings_monitoring").to_string())
        .icon(IconName::PanelLeftOpen)
        .group(
            SettingGroup::new()
                .title(t!("settings_monitoring").to_string())
                .item(
                    SettingItem::new(
                        t!("show_monitoring_dashboard").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("show-monitoring-dashboard")
                                    .small()
                                    .checked(show_monitoring_dashboard)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_show_monitoring_dashboard(*checked);
                                        let _ = this.config.save();
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("show_monitoring_dashboard_hint").to_string()),
                )
                .item(SettingItem::new(
                    t!("monitoring_position").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        let pos = monitoring_position.clone();
                        move |_, _window, _cx| {
                            Button::new("monitoring-position-dropdown")
                                .small()
                                .icon(IconName::PanelLeftOpen)
                                .label(if pos == "Sidebar" {
                                    t!("position_sidebar").to_string()
                                } else {
                                    t!("position_bottom").to_string()
                                })
                                .disabled(!show_monitoring_dashboard)
                                .dropdown_menu_with_anchor(Anchor::BottomRight, {
                                    let view = view.clone();
                                    let pos = pos.clone();
                                    move |mut menu, window, _cx| {
                                        menu = menu
                                            .min_w(160.)
                                            .item(
                                                PopupMenuItem::new(
                                                    t!("position_bottom").to_string(),
                                                )
                                                .checked(pos == "Bottom")
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _window, cx| {
                                                        this.config
                                                            .set_monitoring_position("Bottom");
                                                        let _ = this.config.save();
                                                        cx.notify();
                                                    },
                                                )),
                                            )
                                            .item(
                                                PopupMenuItem::new(
                                                    t!("position_sidebar").to_string(),
                                                )
                                                .checked(pos == "Sidebar")
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _window, cx| {
                                                        this.config
                                                            .set_monitoring_position("Sidebar");
                                                        let _ = this.config.save();
                                                        cx.notify();
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
