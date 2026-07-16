use super::*;

impl AxShell {
    pub(crate) fn show_selector_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_dialog.is_some() {
            return;
        }
        self.active_dialog = Some(crate::app::DialogKind::SessionSelector);

        let view = cx.entity();
        let selector_focus_handle = self.selector_focus_handle.clone();
        let deferred_selector_focus_handle = selector_focus_handle.clone();
        let sessions = self.config.sessions().to_vec();
        let active_session_id = self.active_session_id().map(ToOwned::to_owned);
        self.selector_selection = self.default_selector_index();
        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .title(t!("open_session").to_string())
                .w(px(520.))
                .overlay_closable(false)
                .on_close({
                    let view = view.clone();
                    move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            cx.notify();
                        });
                    }
                })
                .on_ok({
                    let view = view.clone();
                    move |_, window, cx| {
                        view.update(cx, |this, cx| {
                            this.activate_selector_selection(window, cx);
                        });
                        false
                    }
                })
                .content({
                    let view = view.clone();
                    let sessions = sessions.clone();
                    let _active_session_id = active_session_id.clone();
                    let selector_focus_handle = selector_focus_handle.clone();
                    move |content, window, _cx| {
                        let selected_index = view.read(_cx).selector_selection;
                        let scroll_handle = view.read(_cx).selector_scroll_handle.clone();
                        content.child(
                            v_flex()
                                .track_focus(&selector_focus_handle)
                                .on_key_down(window.listener_for(
                                    &view,
                                    |this, event, window, cx| {
                                        this.on_selector_key_down(event, window, cx)
                                    },
                                ))
                                .gap_2()
                                .child(
                                    div()
                                        .w_full()
                                        .p_2()
                                        .rounded_md()
                                        .border_1()
                                        .border_color(if selected_index == 0 {
                                            _cx.theme().primary
                                        } else {
                                            _cx.theme().border
                                        })
                                        .bg(if selected_index == 0 {
                                            _cx.theme().tab_active
                                        } else {
                                            _cx.theme().muted
                                        })
                                        .cursor_pointer()
                                        .fast_hover(_cx)
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            window.listener_for(&view, |this, _, window, cx| {
                                                this.active_dialog = None;
                                                this.open_local_and_focus(window, cx);
                                                window.close_dialog(cx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .child(
                                                    selectable_plain_text(
                                                        "selector-local-title",
                                                        t!("local_terminal"),
                                                    )
                                                    .text_size(rems(1.0))
                                                    .font_weight(FontWeight::SEMIBOLD),
                                                )
                                                .child(
                                                    selectable_plain_text(
                                                        "selector-local-description",
                                                        t!("open_local_shell_tab"),
                                                    )
                                                    .text_size(rems(0.917))
                                                    .text_color(_cx.theme().muted_foreground),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .w_full()
                                        .p_2()
                                        .rounded_md()
                                        .border_1()
                                        .border_color(if selected_index == 1 {
                                            _cx.theme().primary
                                        } else {
                                            _cx.theme().border
                                        })
                                        .bg(if selected_index == 1 {
                                            _cx.theme().tab_active
                                        } else {
                                            _cx.theme().muted
                                        })
                                        .cursor_pointer()
                                        .fast_hover(_cx)
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            window.listener_for(&view, |this, _, window, cx| {
                                                this.active_dialog = None;
                                                window.close_dialog(cx);
                                                this.open_new_ssh_dialog(window, cx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            v_flex()
                                                .gap_1()
                                                .child(
                                                    selectable_plain_text(
                                                        "selector-new-ssh-title",
                                                        t!("new_ssh_connection"),
                                                    )
                                                    .text_size(rems(1.0))
                                                    .font_weight(FontWeight::SEMIBOLD),
                                                )
                                                .child(
                                                    selectable_plain_text(
                                                        "selector-new-ssh-description",
                                                        t!("create_or_edit_ssh_session"),
                                                    )
                                                    .text_size(rems(0.917))
                                                    .text_color(_cx.theme().muted_foreground),
                                                ),
                                        ),
                                )
                                .child(
                                    div()
                                        .relative()
                                        .max_h(px(320.))
                                        .w_full()
                                        .when(!sessions.is_empty(), |this| {
                                            let sessions = sessions.clone();
                                            let view = view.clone();
                                            this.child(
                                                uniform_list(
                                                    "selector-saved-sessions-list",
                                                    sessions.len(),
                                                    move |range, list_window, _cx| {
                                                        range
                                                            .into_iter()
                                                            .filter_map(|ix| {
                                                                let session =
                                                                    sessions.get(ix)?.clone();
                                                                let connect_id =
                                                                    session.id.clone();
                                                                let is_selected =
                                                                    selected_index == ix + 2;
                                                                let name = session.name.clone();
                                                                let detail = match session.kind {
                                                                    crate::session::SessionKind::Ssh => format!(
                                                                        "{}@{}:{}",
                                                                        session.user,
                                                                        session.host,
                                                                        session.port
                                                                    ),
                                                                    crate::session::SessionKind::Telnet => format!(
                                                                        "Telnet {}:{}",
                                                                        session.host,
                                                                        session.port
                                                                    ),
                                                                    crate::session::SessionKind::Serial => format!(
                                                                        "{} @ {}",
                                                                        session.serial_port,
                                                                        session.baud_rate
                                                                    ),
                                                                };
                                                                Some(
                                                                    div()
                                                                        .id(("selector-open", ix))
                                                                        .w_full()
                                                                        .h(px(72.))
                                                                        .p_1()
                                                                        .child(
                                                                            div()
                                                                                .size_full()
                                                                                .p_2()
                                                                                .rounded_md()
                                                                                .border_1()
                                                                                .border_color(
                                                                                    if is_selected {
                                                                                        _cx.theme().primary
                                                                                    } else {
                                                                                        _cx.theme().border
                                                                                    },
                                                                                )
                                                                                .bg(if is_selected {
                                                                                    _cx.theme().tab_active
                                                                                } else {
                                                                                    _cx.theme().muted
                                                                                })
                                                                                .cursor_pointer()
                                                                                .fast_hover(_cx)
                                                                                .on_mouse_down(
                                                                                    MouseButton::Left,
                                                                                    list_window.listener_for(
                                                                                        &view,
                                                                                        move |this, _, window, cx| {
                                                                                            this.active_dialog = None;
                                                                                            this.connect_saved_session_and_focus(
                                                                                                connect_id.clone(),
                                                                                                window,
                                                                                                cx,
                                                                                            );
                                                                                            window.close_dialog(cx);
                                                                                            cx.notify();
                                                                                        },
                                                                                    ),
                                                                                )
                                                                                .child(
                                                                                    v_flex()
                                                                                        .gap_1()
                                                                                        .child(
                                                                                            selectable_plain_text(
                                                                                                ElementId::Name(
                                                                                                    format!(
                                                                                                        "selector-session-name-{ix}"
                                                                                                    )
                                                                                                    .into(),
                                                                                                ),
                                                                                                name,
                                                                                            )
                                                                                            .text_size(rems(1.0))
                                                                                            .font_weight(
                                                                                                FontWeight::SEMIBOLD,
                                                                                            ),
                                                                                        )
                                                                                        .child(
                                                                                            selectable_plain_text(
                                                                                                ElementId::Name(
                                                                                                    format!(
                                                                                                        "selector-session-detail-{ix}"
                                                                                                    )
                                                                                                    .into(),
                                                                                                ),
                                                                                                detail,
                                                                                            )
                                                                                            .text_size(rems(0.917))
                                                                                            .text_color(
                                                                                                _cx.theme()
                                                                                                    .muted_foreground,
                                                                                            ),
                                                                                        ),
                                                                                ),
                                                                        ),
                                                                )
                                                            })
                                                            .collect::<Vec<_>>()
                                                    },
                                                )
                                                .track_scroll(&scroll_handle)
                                                .w_full()
                                                .h(px(320.)),
                                            )
                                        }),
                                ),
                        )
                    }
                })
        });
        window.defer(cx, move |window, cx| {
            window.focus(&deferred_selector_focus_handle, cx);
        });
    }
}
