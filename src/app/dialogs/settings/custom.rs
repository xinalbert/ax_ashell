use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_custom_page(
    view: &gpui::Entity<AxShell>,
    shell: &AxShell,
    cx: &mut Context<AxShell>,
) -> SettingPage {
    let view_clone_for_custom = view.clone();
    let custom_theme_name_input = shell
        .custom_theme_inputs
        .get(crate::app::theme::custom_theme_name_input_key())
        .expect("custom theme name input missing")
        .clone();
    let custom_theme_save_path_input = shell.custom_theme_save_path_input.clone();
    let editing_mode = gpui_component::Theme::global(cx).mode;
    let custom_base_name = shell.resolved_custom_theme_base_name(editing_mode, cx);
    let mut custom_theme_meta_group = SettingGroup::new()
        .title(t!("settings_custom_theme").to_string())
        .description(t!("settings_custom_config_hint").to_string())
        .item(
            SettingItem::new(
                t!("custom_theme_name").to_string(),
                SettingField::render({
                    let input = custom_theme_name_input.clone();
                    move |_, _window, _cx| Input::new(&input).w(px(220.)).into_any_element()
                }),
            )
            .description(t!("custom_theme_saved_name_hint").to_string()),
        );
    custom_theme_meta_group = custom_theme_meta_group.item(
        SettingItem::new(
            t!("custom_theme_save_path").to_string(),
            SettingField::render({
                let input = custom_theme_save_path_input.clone();
                let view = view_clone_for_custom.clone();
                move |_, window, _cx| {
                    h_flex()
                        .w_full()
                        .gap_2()
                        .child(Input::new(&input).w(px(320.)))
                        .child(
                            Button::new("custom-theme-save-path-browse")
                                .small()
                                .label(t!("browse").to_string())
                                .on_click(window.listener_for(&view, |this, _, window, cx| {
                                    this.pick_custom_theme_save_path(window, cx);
                                })),
                        )
                        .into_any_element()
                }
            }),
        )
        .description(t!("custom_theme_save_path_hint").to_string()),
    );

    custom_theme_meta_group = custom_theme_meta_group.item(
        SettingItem::new(
            t!("custom_theme_base").to_string(),
            SettingField::render({
                let view = view_clone_for_custom.clone();
                let current_base_name = custom_base_name.clone();
                move |_, _window, _cx| {
                    super::fast_menu::fast_settings_menu_lazy(
                        "custom-theme-base-dropdown",
                        current_base_name.clone(),
                        Some(if editing_mode.is_dark() {
                            IconName::Moon
                        } else {
                            IconName::Sun
                        }),
                        px(220.),
                        Some(px(320.)),
                        {
                            let current_base_name = current_base_name.clone();
                            move |_, cx| {
                                gpui_component::ThemeRegistry::global(cx)
                                    .sorted_themes()
                                    .into_iter()
                                    .filter(|theme| theme.mode == editing_mode)
                                    .map(|theme| {
                                        let theme_name = theme.name.clone();
                                        let checked =
                                            theme_name.as_ref() == current_base_name.as_str();
                                        super::fast_menu::FastMenuItem::new(
                                            theme_name.to_string(),
                                            checked,
                                            move |this, window, cx| {
                                                this.set_custom_theme_base_preset(
                                                    editing_mode,
                                                    &theme_name,
                                                    window,
                                                    cx,
                                                );
                                            },
                                        )
                                    })
                                    .collect()
                            }
                        },
                        view.clone(),
                    )
                    .into_any_element()
                }
            }),
        )
        .description(t!("custom_theme_base_hint").to_string()),
    );

    custom_theme_meta_group = custom_theme_meta_group.item(SettingItem::new(
        t!("save").to_string(),
        SettingField::render({
            let view = view_clone_for_custom.clone();
            move |_, window, _cx| {
                h_flex()
                    .gap_2()
                    .child(
                        Button::new("custom-appearance-save")
                            .primary()
                            .label(t!("save").to_string())
                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                this.save_custom_appearance(window, cx);
                            })),
                    )
                    .child(
                        Button::new("custom-appearance-reset")
                            .ghost()
                            .label(t!("reset").to_string())
                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                this.reset_custom_appearance(window, cx);
                            })),
                    )
                    .child(
                        Button::new("custom-appearance-import")
                            .ghost()
                            .label(t!("import_theme").to_string())
                            .on_click(window.listener_for(&view, |this, _, window, cx| {
                                this.import_custom_theme_file(window, cx);
                            })),
                    )
                    .into_any_element()
            }
        }),
    ));

    let mut custom_theme_page = SettingPage::new(t!("settings_custom").to_string())
        .icon(IconName::Settings)
        .group(custom_theme_meta_group);

    let mut group = SettingGroup::new()
        .title(t!("settings_custom_theme_overrides").to_string())
        .description(t!("custom_theme_inherit_hint").to_string());

    for section in crate::app::theme::CUSTOM_THEME_SECTION_SPECS {
        let section_title = section.title.to_string();
        group = group.item(SettingItem::render(move |_, _window, _cx| {
            div()
                .pt_2()
                .text_sm()
                .font_weight(FontWeight::BOLD)
                .child(section_title.clone())
        }));

        for field in section.fields {
            let input_key = crate::app::theme::custom_theme_input_key(editing_mode, field.key);
            let input = shell
                .custom_theme_inputs
                .get(&input_key)
                .expect("custom theme input missing")
                .clone();
            let inherited_value = shell.custom_theme_inherited_field_value(editing_mode, field, cx);
            let width = px(180.);
            let description = format!(
                "{} {}; key: {}",
                t!("custom_theme_inherited_hint"),
                inherited_value,
                field.key,
            );

            group = group.item(
                SettingItem::new(
                    field.label.to_string(),
                    SettingField::render({
                        let input = input.clone();
                        move |_, _window, _cx| Input::new(&input).w(width).into_any_element()
                    }),
                )
                .description(description),
            );
        }
    }

    custom_theme_page = custom_theme_page.group(group);

    custom_theme_page
}
