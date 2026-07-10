use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_terminal_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let right_click_copy_paste = shell.config.right_click_copy_paste();
    let keyword_highlight = shell.config.keyword_highlight();
    let ssh_retry_count_input = shell.ssh_retry_count_input.clone();
    let ssh_retry_delays_input = shell.ssh_retry_delays_input.clone();

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
                                        let _ = this.config.save();
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
                                    let _ = this.config.save();
                                    cx.notify();
                                }))
                                .into_any_element()
                        }
                    }),
                )),
        )
        .group(
            SettingGroup::new()
                .title(t!("settings_ssh_connection").to_string())
                .item(SettingItem::render({
                    let view = view.clone();
                    move |_, window, cx| {
                        let muted_foreground = cx.theme().muted_foreground;
                        v_flex()
                            .w_full()
                            .gap_3()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(div().text_sm().child(t!("ssh_retry_count").to_string()))
                                    .child(Input::new(&ssh_retry_count_input).w_full()),
                            )
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div().text_sm().child(t!("ssh_retry_delays").to_string()),
                                    )
                                    .child(Input::new(&ssh_retry_delays_input).w_full())
                                    .child(
                                        div()
                                            .text_xs()
                                            .text_color(muted_foreground)
                                            .child(t!("ssh_retry_delays_hint").to_string()),
                                    ),
                            )
                            .child(
                                Button::new("save-ssh-retry-settings")
                                    .small()
                                    .primary()
                                    .label(t!("save_ssh_retry_settings").to_string())
                                    .on_click(window.listener_for(&view, |this, _, _, cx| {
                                        let retry_count = this
                                            .ssh_retry_count_input
                                            .read(cx)
                                            .value()
                                            .trim()
                                            .parse::<u32>()
                                            .unwrap_or(2);
                                        let delays = this
                                            .ssh_retry_delays_input
                                            .read(cx)
                                            .value()
                                            .split(',')
                                            .filter_map(|part| {
                                                let trimmed = part.trim();
                                                if trimmed.is_empty() {
                                                    return None;
                                                }
                                                trimmed.parse::<u64>().ok()
                                            })
                                            .collect::<Vec<_>>();
                                        this.config.set_ssh_connect_retry_count(retry_count);
                                        this.config.set_ssh_connect_retry_delays_ms(delays);
                                        let _ = this.config.save();
                                        cx.notify();
                                    })),
                            )
                    }
                }))
                .description(t!("ssh_retry_defaults_hint").to_string()),
        )
}
