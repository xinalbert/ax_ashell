use super::*;

impl AxShell {
    pub(crate) fn show_ssh_dialog(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.active_dialog.is_some() {
            return;
        }
        self.active_dialog = Some(crate::app::DialogKind::NewSsh);

        let view = cx.entity();
        let session_name_input = self.session_name_input.clone();
        let session_group_input = self.session_group_input.clone();
        let host_input = self.host_input.clone();
        let focus_host_input = host_input.clone();
        let port_input = self.port_input.clone();
        let user_input = self.user_input.clone();
        let password_input = self.password_input.clone();
        let key_path_input = self.key_path_input.clone();
        let key_inline_input = self.key_inline_input.clone();
        let passphrase_input = self.passphrase_input.clone();
        let proxy_host_input = self.proxy_host_input.clone();
        let proxy_port_input = self.proxy_port_input.clone();
        let proxy_user_input = self.proxy_user_input.clone();
        let proxy_password_input = self.proxy_password_input.clone();
        let session_sftp_path_input = self.session_sftp_path_input.clone();

        window.open_dialog(cx, move |dialog: Dialog, _window, _cx| {
            dialog
                .title(t!("new_ssh_connection"))
                .w(px(520.))
                .overlay_closable(false)
                .on_ok({
                    let view = view.clone();
                    move |_, window, cx| {
                        view.update(cx, |this, cx| {
                            this.connect_ssh(window, cx);
                        });
                        false
                    }
                })
                .on_close({
                    let view = view.clone();
                    move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            cx.notify();
                        });
                    }
                })
                .content({
                    let view = view.clone();
                    let session_name_input = session_name_input.clone();
                    let session_group_input = session_group_input.clone();
                    let host_input = host_input.clone();
                    let port_input = port_input.clone();
                    let user_input = user_input.clone();
                    let password_input = password_input.clone();
                    let key_path_input = key_path_input.clone();
                    let key_inline_input = key_inline_input.clone();
                    let passphrase_input = passphrase_input.clone();
                    let proxy_host_input = proxy_host_input.clone();
                    let proxy_port_input = proxy_port_input.clone();
                    let proxy_user_input = proxy_user_input.clone();
                    let proxy_password_input = proxy_password_input.clone();
                    let session_sftp_path_input = session_sftp_path_input.clone();
                    move |content, window, cx| {
                        let is_password = view.read(cx).ssh_auth_method == AuthMethod::Password;
                        let proxy_type = view.read(cx).ssh_proxy_type.clone();
                        let show_proxy_fields = proxy_type != "none";
                        let session_x11_forwarding = view.read(cx).session_x11_forwarding;
                        let x11_server_missing = session_x11_forwarding
                            && !crate::platform::x_server::local_x_server_available(
                                view.read(cx).config.local_x_server_app_path(),
                            );
                        let saved_group_names = view.read(cx).saved_group_names();
                        let current_group_name =
                            session_group_input.read(cx).value().trim().to_string();
                        content.child(
                            v_flex()
                                .track_focus(&view.read(cx).focus_handle)
                                .on_key_down(window.listener_for(
                                    &view,
                                    |this, event, window, cx| {
                                        this.record_session_shortcut(event, window, cx);
                                    },
                                ))
                                .gap_3()
                                .child(Input::new(&session_name_input).tab_index(0))
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Input::new(&session_group_input)
                                                .flex_1()
                                                .tab_index(1),
                                        )
                                        .child(
                                            settings::fast_menu::fast_settings_menu_lazy_disabled(
                                                "ssh-group-dropdown",
                                                t!("choose_group").to_string(),
                                                Some(IconName::ChevronsUpDown),
                                                px(180.),
                                                Some(px(320.)),
                                                saved_group_names.is_empty(),
                                                {
                                                    let saved_group_names =
                                                        saved_group_names.clone();
                                                    let current_group_name =
                                                        current_group_name.clone();
                                                    move |_, _| {
                                                        let mut items = vec![
                                                            settings::fast_menu::FastMenuItem::new(
                                                                t!("ungrouped_group").to_string(),
                                                                current_group_name.is_empty(),
                                                                |this, window, cx| {
                                                                    Self::set_input_value(
                                                                        &this.session_group_input,
                                                                        "",
                                                                        window,
                                                                        cx,
                                                                    );
                                                                },
                                                            ),
                                                        ];
                                                        for group_name in &saved_group_names {
                                                            let checked =
                                                                current_group_name == *group_name;
                                                            let group_name = group_name.clone();
                                                            items.push(
                                                                settings::fast_menu::FastMenuItem::new(
                                                                    group_name.clone(),
                                                                    checked,
                                                                    move |this, window, cx| {
                                                                        Self::set_input_value(
                                                                            &this.session_group_input,
                                                                            group_name.clone(),
                                                                            window,
                                                                            cx,
                                                                        );
                                                                    },
                                                                ),
                                                            );
                                                        }
                                                        items
                                                    }
                                                },
                                                view.clone(),
                                            ),
                                        ),
                                )
                                .child(Input::new(&host_input).tab_index(2))
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(Input::new(&port_input).w(px(96.)).tab_index(3))
                                        .child(Input::new(&user_input).flex_1().tab_index(4)),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Button::new("ssh-auth-password")
                                                .label(t!("password").to_string())
                                                .when(is_password, |button| button.primary())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_auth_method(
                                                            AuthMethod::Password,
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("ssh-auth-key")
                                                .label(t!("key").to_string())
                                                .when(!is_password, |button| button.primary())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_auth_method(
                                                            AuthMethod::Key,
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        ),
                                )
                                .when(is_password, |this| {
                                    this.child(
                                        Input::new(&password_input).mask_toggle().tab_index(5),
                                    )
                                })
                                .when(!is_password, |this| {
                                    this.child(
                                        h_flex()
                                            .gap_2()
                                            .child(
                                                div()
                                                    .flex_1()
                                                    .cursor_pointer()
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        window.listener_for(
                                                            &view,
                                                            |this, _, window, cx| {
                                                                this.pick_ssh_key_path(window, cx);
                                                            },
                                                        ),
                                                    )
                                                    .child(
                                                        Input::new(&key_path_input).tab_index(5),
                                                    ),
                                            )
                                            .child(
                                                Button::new("clear-key-path")
                                                    .ghost()
                                                    .icon(IconName::Close)
                                                    .on_click(window.listener_for(
                                                        &view,
                                                        |this, _, window, cx| {
                                                            Self::set_input_value(
                                                                &this.key_path_input,
                                                                "",
                                                                window,
                                                                cx,
                                                            );
                                                        },
                                                    )),
                                            ),
                                    )
                                    .child(Input::new(&key_inline_input).h(px(128.)).tab_index(6))
                                    .child(Input::new(&passphrase_input).mask_toggle().tab_index(7))
                                })
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .child(t!("proxy").to_string()),
                                )
                                .child(
                                    h_flex()
                                        .gap_2()
                                        .child(
                                            Button::new("proxy-none")
                                                .label(t!("proxy_none").to_string())
                                                .when(proxy_type == "none", |button| {
                                                    button.primary()
                                                })
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_proxy_type(
                                                            "none".to_string(),
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("proxy-socks5")
                                                .label("SOCKS5")
                                                .when(proxy_type == "socks5", |button| {
                                                    button.primary()
                                                })
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_proxy_type(
                                                            "socks5".to_string(),
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("proxy-http")
                                                .label("HTTP")
                                                .when(proxy_type == "http", |button| {
                                                    button.primary()
                                                })
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, _, cx| {
                                                        this.set_ssh_proxy_type(
                                                            "http".to_string(),
                                                            cx,
                                                        )
                                                    },
                                                )),
                                        ),
                                )
                                .when(show_proxy_fields, |this| {
                                    this.child(
                                        h_flex()
                                            .gap_2()
                                            .child(Input::new(&proxy_host_input).flex_1())
                                            .child(Input::new(&proxy_port_input).w(px(96.))),
                                    )
                                    .child(
                                        h_flex()
                                            .gap_2()
                                            .child(Input::new(&proxy_user_input).flex_1())
                                            .child(Input::new(&proxy_password_input).flex_1()),
                                    )
                                })
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .child(t!("settings_sftp").to_string()),
                                )
                                .child(Input::new(&session_sftp_path_input).tab_index(8))
                                .child(
                                    h_flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_sm()
                                                .font_weight(FontWeight::BOLD)
                                                .child(t!("session_shortcut").to_string()),
                                        )
                                        .child(
                                            h_flex()
                                                .gap_2()
                                                .child(
                                                    Button::new("record-session-shortcut")
                                                        .label(if view
                                                            .read(cx)
                                                            .recording_session_shortcut
                                                        {
                                                            t!("press_new_key").to_string()
                                                        } else if view
                                                            .read(cx)
                                                            .session_shortcut
                                                            .is_empty()
                                                        {
                                                            t!("none").to_string()
                                                        } else {
                                                            crate::app::keybinding_recorder::format_keystroke(
                                                                &view.read(cx).session_shortcut,
                                                            )
                                                        })
                                                        .small()
                                                        .when(
                                                            view.read(cx).recording_session_shortcut,
                                                            |button| button.primary(),
                                                        )
                                                        .when(
                                                            view.read(cx)
                                                                .session_shortcut_error
                                                                .is_some(),
                                                            |button| button.danger(),
                                                        )
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, window, cx| {
                                                                this.recording_session_shortcut = true;
                                                                this.session_shortcut_error = None;
                                                                window.focus(&this.focus_handle, cx);
                                                                cx.notify();
                                                            },
                                                        )),
                                                )
                                                .child(
                                                    Button::new("clear-session-shortcut")
                                                        .ghost()
                                                        .icon(IconName::Close)
                                                        .when(
                                                            view.read(cx).session_shortcut.is_empty(),
                                                            |button| button.disabled(true),
                                                        )
                                                        .on_click(window.listener_for(
                                                            &view,
                                                            |this, _, _, cx| {
                                                                this.recording_session_shortcut = false;
                                                                this.session_shortcut_error = None;
                                                                this.session_shortcut.clear();
                                                                cx.notify();
                                                            },
                                                        )),
                                                ),
                                        ),
                                )
                                .when_some(
                                    view.read(cx).session_shortcut_error.clone(),
                                    |this, error| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(cx.theme().danger)
                                                .child(error),
                                        )
                                    },
                                )
                                .child(
                                    Checkbox::new("ssh-session-x11-forwarding")
                                        .checked(session_x11_forwarding)
                                        .label(t!("x11_forwarding").to_string())
                                        .on_click(window.listener_for(
                                            &view,
                                            |this, checked, _, cx| {
                                                this.session_x11_forwarding = *checked;
                                                cx.notify();
                                            },
                                        )),
                                )
                                .when(x11_server_missing, |this| {
                                    this.child(
                                        div()
                                            .text_xs()
                                            .text_color(cx.theme().muted_foreground)
                                            .child(t!("x11_server_install_hint").to_string()),
                                    )
                                })
                                .when_some(
                                    view.read(cx).session_import_error.clone(),
                                    |this, error| {
                                        this.child(
                                            div()
                                                .text_xs()
                                                .text_color(cx.theme().danger)
                                                .child(error),
                                        )
                                    },
                                )
                                .child(
                                    h_flex()
                                        .justify_end()
                                        .gap_2()
                                        .child(
                                            Button::new("import-ssh-session-clipboard")
                                                .ghost()
                                                .label(t!("import_from_clipboard").to_string())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, window, cx| {
                                                        this.import_ssh_session_from_clipboard(
                                                            window, cx,
                                                        );
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("connect-ssh-cancel")
                                                .label(t!("cancel").to_string())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, window, cx| {
                                                        this.active_dialog = None;
                                                        window.close_dialog(cx);
                                                        cx.notify();
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("save-ssh-session")
                                                .label(t!("save").to_string())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, window, cx| {
                                                        this.save_ssh(window, cx)
                                                    },
                                                )),
                                        )
                                        .child(
                                            Button::new("save-and-connect-ssh-session")
                                                .primary()
                                                .label(t!("save_and_connect").to_string())
                                                .on_click(window.listener_for(
                                                    &view,
                                                    |this, _, window, cx| {
                                                        this.connect_ssh(window, cx)
                                                    },
                                                )),
                                        ),
                                ),
                        )
                    }
                })
        });
        window.defer(cx, move |window, cx| {
            window.focus(&focus_host_input.read(cx).focus_handle(cx), cx);
        });
    }
}
