use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_terminal_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let right_click_copy_paste = shell.config.right_click_copy_paste();
    let keyword_highlight = shell.config.keyword_highlight();
    let local_shell_profiles = shell.config.local_shell_profiles().to_vec();
    let default_local_shell_profile = shell.config.default_local_shell_profile();
    let profile_name_input = shell.local_shell_profile_name_input.clone();
    let profile_program_input = shell.local_shell_profile_program_input.clone();
    let profile_args_input = shell.local_shell_profile_args_input.clone();

    SettingPage::new(t!("settings_terminal").to_string())
        .icon(IconName::SquareTerminal)
        .group(
            SettingGroup::new()
                .title(t!("settings_terminal").to_string())
                .item(
                    SettingItem::new(
                        t!("right_click_copy_paste").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            move |_, window, _cx| {
                                Switch::new("right-click-copy-paste")
                                    .small()
                                    .checked(right_click_copy_paste)
                                    .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                        this.config.set_right_click_copy_paste(*checked);
                                        this.config.save_logged("set_right_click_copy_paste");
                                        cx.notify();
                                    }))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("copy_paste_hint").to_string()),
                )
                .item(SettingItem::new(
                    t!("keyword_highlight").to_string(),
                    SettingField::render({
                        let view = view.clone();
                        move |_, window, _cx| {
                            Switch::new("keyword-highlight")
                                .small()
                                .checked(keyword_highlight)
                                .on_click(window.listener_for(&view, |this, checked, _, cx| {
                                    this.config.set_keyword_highlight(*checked);
                                    this.config.save_logged("set_keyword_highlight");
                                    cx.notify();
                                }))
                                .into_any_element()
                        }
                    }),
                )),
        )
        .group(
            SettingGroup::new()
                .title(t!("local_shell_profiles").to_string())
                .item(
                    SettingItem::new(
                        t!("default_local_shell_profile").to_string(),
                        SettingField::render({
                            let view = view.clone();
                            let profiles = local_shell_profiles.clone();
                            let selected_id = default_local_shell_profile.id.clone();
                            let selected_name = default_local_shell_profile.name.clone();
                            move |_, _window, _cx| {
                                let items = profiles
                                    .iter()
                                    .map(|profile| {
                                        let id = profile.id.clone();
                                        super::fast_menu::FastMenuItem::new(
                                            profile.name.clone(),
                                            profile.id == selected_id,
                                            move |this, window, cx| {
                                                this.select_default_local_shell_profile(
                                                    &id, window, cx,
                                                );
                                            },
                                        )
                                    })
                                    .collect();
                                super::fast_menu::fast_settings_menu(
                                    "default-local-shell-profile",
                                    selected_name.clone(),
                                    Some(IconName::ChevronsUpDown),
                                    px(220.),
                                    Some(px(320.)),
                                    items,
                                    view.clone(),
                                )
                                .into_any_element()
                            }
                        }),
                    )
                    .description(t!("default_local_shell_profile_hint").to_string()),
                )
                .item(SettingItem::new(
                    t!("local_shell_profile_name").to_string(),
                    SettingField::render({
                        move |_, _window, _cx| {
                            Input::new(&profile_name_input)
                                .w(px(260.))
                                .into_any_element()
                        }
                    }),
                ))
                .item(SettingItem::new(
                    t!("local_shell_program").to_string(),
                    SettingField::render({
                        move |_, _window, _cx| {
                            Input::new(&profile_program_input)
                                .w(px(360.))
                                .into_any_element()
                        }
                    }),
                ))
                .item(
                    SettingItem::new(
                        t!("local_shell_arguments").to_string(),
                        SettingField::render({
                            move |_, _window, _cx| {
                                Input::new(&profile_args_input)
                                    .w(px(360.))
                                    .into_any_element()
                            }
                        }),
                    )
                    .description(t!("local_shell_arguments_hint").to_string()),
                )
                .item(SettingItem::render({
                    let view = view.clone();
                    let can_remove = local_shell_profiles.len() > 1;
                    move |_, window, _cx| {
                        h_flex()
                            .gap_2()
                            .child(
                                Button::new("save-local-shell-profile")
                                    .small()
                                    .primary()
                                    .label(t!("save").to_string())
                                    .on_click(window.listener_for(&view, |this, _, window, cx| {
                                        this.save_default_local_shell_profile(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("duplicate-local-shell-profile")
                                    .small()
                                    .label(t!("duplicate_local_shell_profile").to_string())
                                    .on_click(window.listener_for(&view, |this, _, window, cx| {
                                        this.duplicate_local_shell_profile(window, cx);
                                    })),
                            )
                            .child(
                                Button::new("remove-local-shell-profile")
                                    .small()
                                    .disabled(!can_remove)
                                    .label(t!("delete").to_string())
                                    .on_click(window.listener_for(&view, |this, _, window, cx| {
                                        this.remove_default_local_shell_profile(window, cx);
                                    })),
                            )
                            .into_any_element()
                    }
                })),
        )
}
