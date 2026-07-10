use super::*;

impl AxShell {
    pub(crate) fn show_settings_close_confirm_dialog(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.active_dialog.is_some() {
            return;
        }

        self.active_dialog = Some(crate::app::DialogKind::SettingsCloseConfirm);
        self.settings_close_remember_choice = false;
        let view = cx.entity();
        let shortcut_focus_handle = cx.focus_handle();
        let deferred_shortcut_focus_handle = shortcut_focus_handle.clone();

        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .title(t!("settings_close_confirm_title").to_string())
                .w(px(500.))
                .keyboard(false)
                .close_button(false)
                .overlay_closable(false)
                .on_close({
                    let view = view.clone();
                    move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            this.settings_close_remember_choice = false;
                            cx.notify();
                        });
                    }
                })
                .content({
                    let view = view.clone();
                    let shortcut_focus_handle = shortcut_focus_handle.clone();
                    move |content, window, cx| {
                        let remember = view.read(cx).settings_close_remember_choice;
                        content.child(
                            v_flex()
                                .w_full()
                                .gap_3()
                                .track_focus(&shortcut_focus_handle)
                                .on_key_down({
                                    let view = view.clone();
                                    move |event: &gpui::KeyDownEvent, window, cx| {
                                        let handled = view.update(cx, |this, cx| {
                                            if !crate::app::keybinding_recorder::event_matches_action(
                                                &this.config,
                                                "OpenSettings",
                                                event,
                                            ) {
                                                return false;
                                            }

                                            this.confirm_settings_close_with_shortcut(window, cx)
                                        });
                                        if handled {
                                            window.prevent_default();
                                            cx.stop_propagation();
                                        }
                                    }
                                })
                                .child(
                                    selectable_plain_text(
                                        "settings-close-confirm-description",
                                        t!("settings_close_confirm_description").to_string(),
                                    )
                                    .text_base(),
                                )
                                .child(
                                    Checkbox::new("settings-close-confirm-remember")
                                        .checked(remember)
                                        .label(
                                            t!("settings_close_confirm_remember").to_string(),
                                        )
                                        .on_click(window.listener_for(
                                            &view,
                                            |this, checked, _, cx| {
                                                this.settings_close_remember_choice = *checked;
                                                cx.notify();
                                            },
                                        )),
                                ),
                        )
                    }
                })
                .footer(
                    h_flex()
                        .w_full()
                        .justify_end()
                        .gap_2()
                        .child(
                            Button::new("settings-close-confirm-keep-open")
                                .ghost()
                                .label(t!("settings_close_keep_open").to_string())
                                .on_click({
                                    let view = view.clone();
                                    move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            let remember = this.settings_close_remember_choice;
                                            this.apply_settings_close_choice(
                                                false, remember, window, cx,
                                            );
                                        });
                                    }
                                }),
                        )
                        .child(
                            Button::new("settings-close-confirm-close")
                                .primary()
                                .label(t!("settings_close_confirm_action").to_string())
                                .on_click({
                                    let view = view.clone();
                                    move |_, window, cx| {
                                        view.update(cx, |this, cx| {
                                            let remember = this.settings_close_remember_choice;
                                            this.apply_settings_close_choice(
                                                true, remember, window, cx,
                                            );
                                        });
                                    }
                                }),
                        ),
                )
        });

        window.defer(cx, move |window, cx| {
            window.focus(&deferred_shortcut_focus_handle, cx);
        });
    }
}
