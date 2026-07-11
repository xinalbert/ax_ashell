use super::*;

use gpui::IntoElement;
use gpui_component::setting::{SettingField, SettingGroup, SettingItem, SettingPage};

pub(super) fn settings_terminal_page(view: &gpui::Entity<AxShell>, shell: &AxShell) -> SettingPage {
    let right_click_copy_paste = shell.config.right_click_copy_paste();
    let keyword_highlight = shell.config.keyword_highlight();

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
}
