use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_monitoring_page(
    view: &gpui::Entity<AxShell>,
    shell: &AxShell,
) -> SettingPage {
    let show_monitoring_dashboard = shell.config.show_monitoring_dashboard();
    let monitoring_position = shell.config.monitoring_position().to_string();
    let deep_sleep_after_minutes = shell.config.deep_sleep_after_minutes();
    let rayon_threads_input = shell.rayon_threads_input.clone();

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
                                        this.config.save_logged("set_monitoring_visibility");
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
                            super::fast_menu::fast_settings_menu_disabled(
                                "monitoring-position-dropdown",
                                if pos == "Sidebar" {
                                    t!("position_sidebar").to_string()
                                } else {
                                    t!("position_bottom").to_string()
                                },
                                Some(IconName::PanelLeftOpen),
                                px(160.),
                                None,
                                !show_monitoring_dashboard,
                                vec![
                                    super::fast_menu::FastMenuItem::new(
                                        t!("position_bottom").to_string(),
                                        pos == "Bottom",
                                        |this, _window, cx| {
                                            this.config.set_monitoring_position("Bottom");
                                            this.config.save_logged("set_monitoring_bottom");
                                            cx.notify();
                                        },
                                    ),
                                    super::fast_menu::FastMenuItem::new(
                                        t!("position_sidebar").to_string(),
                                        pos == "Sidebar",
                                        |this, _window, cx| {
                                            this.config.set_monitoring_position("Sidebar");
                                            this.config.save_logged("set_monitoring_sidebar");
                                            cx.notify();
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
                .title(t!("settings_resource_usage").to_string())
                .item(
                    SettingItem::new(
                        t!("deep_sleep_after_unfocused").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, _window, _cx| {
                                let label = match deep_sleep_after_minutes {
                                    0 => t!("deep_sleep_disabled").to_string(),
                                    1 => t!("deep_sleep_after_1_minute").to_string(),
                                    5 => t!("deep_sleep_after_5_minutes").to_string(),
                                    15 => t!("deep_sleep_after_15_minutes").to_string(),
                                    _ => t!("deep_sleep_after_30_minutes").to_string(),
                                };
                                let mut items = Vec::new();
                                for (minutes, label) in [
                                    (0, t!("deep_sleep_disabled").to_string()),
                                    (1, t!("deep_sleep_after_1_minute").to_string()),
                                    (5, t!("deep_sleep_after_5_minutes").to_string()),
                                    (15, t!("deep_sleep_after_15_minutes").to_string()),
                                    (30, t!("deep_sleep_after_30_minutes").to_string()),
                                ] {
                                    items.push(super::fast_menu::FastMenuItem::new(
                                        label,
                                        deep_sleep_after_minutes == minutes,
                                        move |this, _window, cx| {
                                            this.config.set_deep_sleep_after_minutes(minutes);
                                            this.config.save_logged("set_deep_sleep_delay");
                                            cx.notify();
                                        },
                                    ));
                                }
                                super::fast_menu::fast_settings_menu(
                                    "deep-sleep-after-unfocused",
                                    label,
                                    None,
                                    px(180.),
                                    None,
                                    items,
                                    view.clone(),
                                )
                                .into_any_element()
                            }
                        }),
                    )
                    .description(t!("deep_sleep_after_unfocused_hint").to_string()),
                )
                .item(
                    SettingItem::new(
                        t!("rayon_threads").to_string(),
                        SettingField::render({
                            move |_, _window, _cx| {
                                Input::new(&rayon_threads_input)
                                    .w(px(180.))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("rayon_threads_hint").to_string()),
                ),
        )
}
