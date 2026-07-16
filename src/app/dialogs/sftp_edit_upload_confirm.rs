use super::*;

impl AxShell {
    pub(crate) fn show_next_sftp_edit_upload_dialog(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.active_dialog.is_some() {
            return;
        }

        let request = loop {
            let Some(request) = self.sftp_edit_upload_requests.pop_front() else {
                return;
            };
            let is_dirty = self
                .tab_groups
                .iter()
                .find(|group| group.id == request.group_id)
                .and_then(|group| group.sftp.as_ref())
                .and_then(|sftp| {
                    sftp.edit_sessions
                        .iter()
                        .find(|session| session.local_path == request.local_path)
                })
                .is_some_and(|session| session.dirty && !session.uploading);
            if is_dirty {
                break request;
            }
        };

        let remote_path = self
            .tab_groups
            .iter()
            .find(|group| group.id == request.group_id)
            .and_then(|group| group.sftp.as_ref())
            .and_then(|sftp| {
                sftp.edit_sessions
                    .iter()
                    .find(|session| session.local_path == request.local_path)
            })
            .map(|session| session.remote_path.clone())
            .unwrap_or_else(|| request.local_path.clone());
        self.sftp_edit_upload_request = Some(request.clone());
        self.active_dialog = Some(crate::app::DialogKind::SftpEditUploadConfirm);
        let view = cx.entity();

        window.open_dialog(cx, move |dialog: Dialog, _window, _| {
            dialog
                .title(t!("sftp_edit_upload_title").to_string())
                .w(px(560.))
                .keyboard(false)
                .close_button(false)
                .overlay_closable(false)
                .on_close({
                    let view = view.clone();
                    move |_, _, cx| {
                        view.update(cx, |this, cx| {
                            this.active_dialog = None;
                            this.sftp_edit_upload_request = None;
                            cx.notify();
                        });
                    }
                })
                .content({
                    let remote_path = remote_path.clone();
                    let local_path = request.local_path.clone();
                    move |content, _window, cx| {
                        content.child(
                            v_flex()
                                .w_full()
                                .gap_3()
                                .child(
                                    selectable_plain_text(
                                        "sftp-edit-upload-description",
                                        t!("sftp_edit_upload_description").to_string(),
                                    )
                                    .text_base(),
                                )
                                .child(
                                    v_flex()
                                        .w_full()
                                        .gap_1()
                                        .child(
                                            selectable_plain_text(
                                                "sftp-edit-upload-remote",
                                                t!("sftp_edit_upload_remote", path = remote_path)
                                                    .to_string(),
                                            )
                                            .text_sm(),
                                        )
                                        .child(
                                            selectable_plain_text(
                                                "sftp-edit-upload-local",
                                                t!("sftp_edit_upload_local", path = local_path)
                                                    .to_string(),
                                            )
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground),
                                        ),
                                ),
                        )
                    }
                })
                .footer({
                    let continue_view = view.clone();
                    let keep_view = view.clone();
                    let upload_view = view.clone();
                    let request_for_continue = request.clone();
                    let request_for_keep = request.clone();
                    let request_for_upload = request.clone();

                    h_flex()
                        .w_full()
                        .justify_end()
                        .gap_2()
                        .child(
                            Button::new("sftp-edit-upload-continue")
                                .ghost()
                                .label(t!("sftp_edit_continue").to_string())
                                .on_click(move |_, window, cx| {
                                    continue_view.update(cx, |this, cx| {
                                        if this.sftp_edit_close_group_id.as_deref()
                                            == Some(request_for_continue.group_id.as_str())
                                        {
                                            this.sftp_edit_close_group_id = None;
                                        }
                                        this.cancel_sftp_edit_upload_requests(
                                            &request_for_continue.group_id,
                                        );
                                        this.active_dialog = None;
                                        this.sftp_edit_upload_request = None;
                                        cx.notify();
                                    });
                                    window.close_dialog(cx);
                                }),
                        )
                        .child(
                            Button::new("sftp-edit-upload-keep-local")
                                .ghost()
                                .label(t!("sftp_edit_keep_local").to_string())
                                .on_click(move |_, window, cx| {
                                    keep_view.update(cx, |this, cx| {
                                        this.discard_sftp_edit_session(
                                            &request_for_keep.group_id,
                                            &request_for_keep.local_path,
                                        );
                                        this.status = t!(
                                            "sftp_edit_kept_local",
                                            path = request_for_keep.local_path
                                        )
                                        .into();
                                        this.active_dialog = None;
                                        this.sftp_edit_upload_request = None;
                                        cx.notify();
                                    });
                                    window.close_dialog(cx);
                                }),
                        )
                        .child(
                            Button::new("sftp-edit-upload-confirm")
                                .primary()
                                .label(t!("sftp_edit_upload").to_string())
                                .on_click(move |_, window, cx| {
                                    upload_view.update(cx, |this, cx| {
                                        this.upload_sftp_edit_session(
                                            &request_for_upload.group_id,
                                            &request_for_upload.local_path,
                                        );
                                        this.active_dialog = None;
                                        this.sftp_edit_upload_request = None;
                                        cx.notify();
                                    });
                                    window.close_dialog(cx);
                                }),
                        )
                })
        });
    }
}
