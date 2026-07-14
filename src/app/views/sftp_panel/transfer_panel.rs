use std::time::{SystemTime, UNIX_EPOCH};

use super::super::*;

const TRANSFER_ICON_COLUMN_WIDTH: f32 = 24.;
const TRANSFER_STATUS_COLUMN_WIDTH: f32 = 180.;
const TRANSFER_TIME_COLUMN_WIDTH: f32 = 72.;
const TRANSFER_SPEED_COLUMN_WIDTH: f32 = 84.;
const TRANSFER_ACTIONS_COLUMN_WIDTH: f32 = 72.;

impl AxShell {
    pub(super) fn render_sftp_transfer_panel(&self, cx: &mut Context<Self>) -> impl IntoElement {
        let selected_tab = self.sftp_transfer_tab;
        let active_group_id = self.active_group.as_deref();
        let active_count = self
            .transfers
            .iter()
            .filter(|transfer| transfer_belongs_to_group(transfer, active_group_id))
            .filter(|transfer| transfer_belongs_to_tab(transfer, SftpTransferTab::Active))
            .count();
        let failed_count = self
            .transfers
            .iter()
            .filter(|transfer| transfer_belongs_to_group(transfer, active_group_id))
            .filter(|transfer| transfer_belongs_to_tab(transfer, SftpTransferTab::Failed))
            .count();
        let completed_count = self
            .transfers
            .iter()
            .filter(|transfer| transfer_belongs_to_group(transfer, active_group_id))
            .filter(|transfer| transfer_belongs_to_tab(transfer, SftpTransferTab::Completed))
            .count();
        let transfers = self
            .transfers
            .iter()
            .filter(|transfer| transfer_belongs_to_group(transfer, active_group_id))
            .filter(|transfer| transfer_belongs_to_tab(transfer, selected_tab))
            .cloned()
            .collect::<Vec<_>>();
        let visible_count = transfers.len();
        let visible_running_count = transfers
            .iter()
            .filter(|transfer| matches!(transfer.state, crate::sftp::TransferState::Running))
            .count();
        let visible_paused_count = transfers
            .iter()
            .filter(|transfer| matches!(transfer.state, crate::sftp::TransferState::Paused))
            .count();
        let transfer_count = transfers.len();
        let scroll_handle = self.sftp_transfer_scroll_handle.clone();
        let view = cx.entity();

        v_flex()
            .size_full()
            .min_h(px(0.))
            .border_t_1()
            .border_color(cx.theme().border)
            .bg(cx.theme().background)
            .child(
                h_flex()
                    .flex_none()
                    .h(px(36.))
                    .px_3()
                    .items_center()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().border)
                    .bg(cx.theme().tab_bar)
                    .child(Self::sftp_transfer_tab_button(
                        SftpTransferTab::Active,
                        t!("sftp_transfer_active").to_string(),
                        active_count,
                        selected_tab,
                        cx,
                    ))
                    .child(Self::sftp_transfer_tab_button(
                        SftpTransferTab::Failed,
                        t!("failed").to_string(),
                        failed_count,
                        selected_tab,
                        cx,
                    ))
                    .child(Self::sftp_transfer_tab_button(
                        SftpTransferTab::Completed,
                        t!("completed").to_string(),
                        completed_count,
                        selected_tab,
                        cx,
                    ))
                    .child(div().flex_1())
                    .when(selected_tab == SftpTransferTab::Active, |this| {
                        this.child(
                            Button::new("sftp-transfer-resume-all")
                                .ghost()
                                .small()
                                .icon(IconName::Play)
                                .label(t!("resume_all").to_string())
                                .disabled(visible_paused_count == 0)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.resume_sftp_transfers_in_tab(selected_tab, cx);
                                })),
                        )
                        .child(
                            Button::new("sftp-transfer-pause-all")
                                .ghost()
                                .small()
                                .icon(IconName::Pause)
                                .label(t!("pause_all").to_string())
                                .disabled(visible_running_count == 0)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.pause_sftp_transfers_in_tab(selected_tab, cx);
                                })),
                        )
                        .child(
                            Button::new("sftp-transfer-cancel-remove-all")
                                .ghost()
                                .small()
                                .icon(IconName::Close)
                                .label(t!("cancel_remove_all").to_string())
                                .disabled(visible_count == 0)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.cancel_remove_sftp_transfers_in_tab(
                                        selected_tab,
                                        true,
                                        cx,
                                    );
                                })),
                        )
                    })
                    .when(selected_tab != SftpTransferTab::Active, |this| {
                        this.child(
                            Button::new("sftp-transfer-remove-visible")
                                .ghost()
                                .small()
                                .icon(IconName::Close)
                                .label(t!("remove_all").to_string())
                                .disabled(visible_count == 0)
                                .on_click(cx.listener(move |this, _, _, cx| {
                                    this.cancel_remove_sftp_transfers_in_tab(
                                        selected_tab,
                                        false,
                                        cx,
                                    );
                                })),
                        )
                    }),
            )
            .child(Self::render_sftp_transfer_header(cx))
            .child(
                div()
                    .flex_1()
                    .relative()
                    .min_h(px(0.))
                    .child(if transfers.is_empty() {
                        selectable_plain_text(
                            "sftp-transfers-empty",
                            t!("no_transfers_yet").to_string(),
                        )
                        .size_full()
                        .p_4()
                        .text_center()
                        .text_color(cx.theme().muted_foreground)
                        .into_any_element()
                    } else {
                        let transfers = transfers.clone();
                        uniform_list(
                            "sftp-transfer-history-list",
                            transfer_count,
                            move |range, list_window, cx| {
                                range
                                    .into_iter()
                                    .filter_map(|ix| {
                                        let transfer = transfers.get(ix)?.clone();
                                        Some(Self::render_sftp_transfer_row(
                                            transfer,
                                            view.clone(),
                                            list_window,
                                            cx,
                                        ))
                                    })
                                    .collect::<Vec<_>>()
                            },
                        )
                        .track_scroll(&scroll_handle)
                        .w_full()
                        .h_full()
                        .into_any_element()
                    }),
            )
    }

    fn sftp_transfer_tab_button(
        tab: SftpTransferTab,
        label: String,
        count: usize,
        selected_tab: SftpTransferTab,
        cx: &mut Context<Self>,
    ) -> Button {
        Button::new(ElementId::Name(format!("sftp-transfer-tab-{tab:?}").into()))
            .ghost()
            .small()
            .selected(tab == selected_tab)
            .label(format!("{label} ({count})"))
            .on_click(cx.listener(move |this, _, _, cx| {
                this.set_sftp_transfer_tab(tab, cx);
            }))
    }

    fn render_sftp_transfer_header(cx: &mut Context<Self>) -> impl IntoElement {
        let column_label = |label: String| {
            div()
                .min_w(px(0.))
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .text_size(rems(0.75))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(cx.theme().muted_foreground)
                .child(label)
        };

        h_flex()
            .w_full()
            .h(px(26.))
            .flex_none()
            .items_center()
            .gap_2()
            .px_3()
            .border_b_1()
            .border_color(cx.theme().border.opacity(0.35))
            .bg(cx.theme().tab_bar)
            .child(div().w(px(TRANSFER_ICON_COLUMN_WIDTH)).flex_none())
            .child(column_label(t!("name").to_string()).flex_1())
            .child(
                column_label(t!("sftp_transfer_status").to_string())
                    .w(px(TRANSFER_STATUS_COLUMN_WIDTH))
                    .flex_none(),
            )
            .child(
                column_label(t!("sftp_transfer_time").to_string())
                    .w(px(TRANSFER_TIME_COLUMN_WIDTH))
                    .flex_none(),
            )
            .child(
                column_label(t!("sftp_transfer_speed").to_string())
                    .w(px(TRANSFER_SPEED_COLUMN_WIDTH))
                    .flex_none(),
            )
            .child(
                column_label(t!("sftp_transfer_actions").to_string())
                    .w(px(TRANSFER_ACTIONS_COLUMN_WIDTH))
                    .flex_none(),
            )
    }

    fn render_sftp_transfer_row(
        transfer: crate::sftp::Transfer,
        view: gpui::Entity<AxShell>,
        window: &mut Window,
        cx: &mut gpui::App,
    ) -> AnyElement {
        let icon = match transfer.info.kind {
            crate::sftp::TransferType::Upload => IconName::ArrowUp,
            crate::sftp::TransferType::Download => IconName::ArrowDown,
        };
        let status_text = sftp_transfer_status_text(&transfer);
        let transfer_id = transfer.info.id.clone();
        let transfer_group_id = transfer.tab_id.clone();
        let show_file_list = matches!(transfer.info.kind, crate::sftp::TransferType::Download);
        let file_list_transfer_id = transfer.info.id.clone();
        let file_list_group_id = transfer.tab_id.clone();
        let action_transfer_id = transfer.info.id.clone();
        let action_group_id = transfer.tab_id.clone();
        let right_click_transfer_id = transfer.info.id.clone();
        let right_click_group_id = transfer.tab_id.clone();

        h_flex()
            .id(ElementId::Name(
                format!("sftp-transfer-row-{transfer_group_id}-{transfer_id}").into(),
            ))
            .w_full()
            .h(px(32.))
            .items_center()
            .gap_2()
            .px_3()
            .border_b_1()
            .border_color(cx.theme().border.opacity(0.35))
            .fast_hover_options(cx, list_fast_hover_options(cx))
            .on_mouse_down(
                MouseButton::Right,
                window.listener_for(&view, move |this, event: &MouseDownEvent, _, cx| {
                    this.open_sftp_transfer_context_menu(
                        right_click_group_id.clone(),
                        right_click_transfer_id.clone(),
                        event.position,
                        cx,
                    );
                    cx.stop_propagation();
                }),
            )
            .child(
                div()
                    .w(px(TRANSFER_ICON_COLUMN_WIDTH))
                    .flex_none()
                    .items_center()
                    .justify_center()
                    .child(
                        Icon::new(icon)
                            .with_size(Size::Small)
                            .text_color(cx.theme().primary),
                    ),
            )
            .child(
                selectable_plain_text(
                    ElementId::Name(format!("sftp-transfer-name-{transfer_id}").into()),
                    transfer.info.name.clone(),
                )
                .flex_1()
                .min_w(px(0.))
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .text_size(rems(0.917))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(cx.theme().foreground),
            )
            .child(
                selectable_plain_text(
                    ElementId::Name(format!("sftp-transfer-status-{transfer_id}").into()),
                    status_text,
                )
                .w(px(TRANSFER_STATUS_COLUMN_WIDTH))
                .flex_none()
                .min_w(px(0.))
                .overflow_hidden()
                .text_ellipsis()
                .whitespace_nowrap()
                .text_size(rems(0.833))
                .text_color(cx.theme().muted_foreground),
            )
            .child(
                selectable_plain_text(
                    ElementId::Name(format!("sftp-transfer-time-{transfer_id}").into()),
                    sftp_transfer_time_text(&transfer),
                )
                .w(px(TRANSFER_TIME_COLUMN_WIDTH))
                .flex_none()
                .whitespace_nowrap()
                .text_size(rems(0.833))
                .text_color(cx.theme().muted_foreground),
            )
            .child(
                selectable_plain_text(
                    ElementId::Name(format!("sftp-transfer-speed-{transfer_id}").into()),
                    sftp_transfer_speed_text(&transfer),
                )
                .w(px(TRANSFER_SPEED_COLUMN_WIDTH))
                .flex_none()
                .whitespace_nowrap()
                .text_size(rems(0.833))
                .text_color(cx.theme().muted_foreground),
            )
            .child(
                h_flex()
                    .w(px(TRANSFER_ACTIONS_COLUMN_WIDTH))
                    .flex_none()
                    .items_center()
                    .justify_end()
                    .gap_1()
                    .when(show_file_list, |this| {
                        this.child(
                            Button::new(ElementId::Name(
                                format!("sftp-transfer-files-{file_list_transfer_id}").into(),
                            ))
                            .ghost()
                            .small()
                            .icon(IconName::File)
                            .tooltip(t!("sftp_transfer_files").to_string())
                            .on_click(window.listener_for(
                                &view,
                                move |this, _, window, cx| {
                                    this.show_sftp_transfer_files_dialog(
                                        file_list_group_id.clone(),
                                        file_list_transfer_id.clone(),
                                        window,
                                        cx,
                                    );
                                },
                            )),
                        )
                    })
                    .child(
                        Button::new(ElementId::Name(
                            format!("sftp-transfer-actions-{transfer_id}").into(),
                        ))
                        .ghost()
                        .small()
                        .icon(IconName::Ellipsis)
                        .on_click(window.listener_for(
                            &view,
                            move |this, _, window, cx| {
                                this.open_sftp_transfer_context_menu(
                                    action_group_id.clone(),
                                    action_transfer_id.clone(),
                                    window.mouse_position(),
                                    cx,
                                );
                            },
                        )),
                    ),
            )
            .into_any_element()
    }
}

fn transfer_belongs_to_group(transfer: &crate::sftp::Transfer, group_id: Option<&str>) -> bool {
    group_id.is_some_and(|group_id| transfer.tab_id == group_id)
}

fn transfer_belongs_to_tab(transfer: &crate::sftp::Transfer, tab: SftpTransferTab) -> bool {
    match tab {
        SftpTransferTab::Active => matches!(
            transfer.state,
            crate::sftp::TransferState::Running | crate::sftp::TransferState::Paused
        ),
        SftpTransferTab::Failed => matches!(
            transfer.state,
            crate::sftp::TransferState::Failed(_)
                | crate::sftp::TransferState::Interrupted(_)
                | crate::sftp::TransferState::Zombie(_)
        ),
        SftpTransferTab::Completed => {
            matches!(transfer.state, crate::sftp::TransferState::Completed)
        }
    }
}

fn sftp_transfer_percent(transfer: &crate::sftp::Transfer) -> f32 {
    match transfer.state {
        crate::sftp::TransferState::Completed => 100.0,
        _ => transfer
            .total
            .filter(|total| *total > 0)
            .map(|total| (transfer.transferred as f64 / total as f64 * 100.0) as f32)
            .unwrap_or(0.0)
            .clamp(0.0, 100.0),
    }
}

fn sftp_transfer_status_text(transfer: &crate::sftp::Transfer) -> String {
    if matches!(transfer.info.kind, crate::sftp::TransferType::Download)
        && !transfer.files.is_empty()
        && matches!(
            transfer.state,
            crate::sftp::TransferState::Running
                | crate::sftp::TransferState::Paused
                | crate::sftp::TransferState::Completed
        )
    {
        let total = transfer.files.len();
        let completed = transfer
            .files
            .iter()
            .filter(|file| file.state.is_terminal())
            .count();
        let summary = format!("{completed}/{}", t!("n_files", files = total));
        if let Some(file) = transfer.files.iter().rev().find(|file| {
            matches!(
                file.state,
                crate::sftp::TransferFileState::Running | crate::sftp::TransferFileState::Paused
            )
        }) {
            return format!("{summary} - {}", transfer_file_name(&file.source));
        }
        return summary;
    }

    match &transfer.state {
        crate::sftp::TransferState::Running => {
            if let Some(total) = transfer.total.filter(|total| *total > 0) {
                format!(
                    "{:.1}% ({}/{})",
                    sftp_transfer_percent(transfer),
                    format_bytes(transfer.transferred),
                    format_bytes(total)
                )
            } else {
                match transfer.info.kind {
                    crate::sftp::TransferType::Upload => format!("{}...", t!("uploading")),
                    crate::sftp::TransferType::Download => {
                        format!("{}...", t!("downloading"))
                    }
                }
            }
        }
        crate::sftp::TransferState::Paused => t!("paused").to_string(),
        crate::sftp::TransferState::Completed => t!("completed").to_string(),
        crate::sftp::TransferState::Failed(err) => format!("{}: {err}", t!("failed")),
        crate::sftp::TransferState::Interrupted(reason) => {
            format!("{}: {reason}", t!("interrupted"))
        }
        crate::sftp::TransferState::Zombie(reason) => format!("{}: {reason}", t!("zombie")),
    }
}

fn transfer_file_name(path: &str) -> &str {
    path.rsplit('/')
        .find(|segment| !segment.is_empty())
        .unwrap_or(path)
}

fn sftp_transfer_time_text(transfer: &crate::sftp::Transfer) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let Some(elapsed) = sftp_transfer_elapsed_secs(transfer, now) else {
        return "--".to_string();
    };

    if elapsed < 60 {
        format!("{elapsed}s")
    } else if elapsed < 3600 {
        format!("{}m {:02}s", elapsed / 60, elapsed % 60)
    } else {
        format!("{}h {:02}m", elapsed / 3600, elapsed % 3600 / 60)
    }
}

fn sftp_transfer_speed_text(transfer: &crate::sftp::Transfer) -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let Some(elapsed) = sftp_transfer_elapsed_secs(transfer, now) else {
        return "--".to_string();
    };
    if transfer.transferred == 0 {
        return "--".to_string();
    }

    format!("{}/s", format_bytes(transfer.transferred / elapsed.max(1)))
}

fn sftp_transfer_elapsed_secs(transfer: &crate::sftp::Transfer, now: u64) -> Option<u64> {
    (transfer.started_at > 0).then(|| {
        transfer
            .finished_at
            .unwrap_or(now)
            .saturating_sub(transfer.started_at)
    })
}

#[cfg(test)]
mod tests {
    use crate::sftp::{Transfer, TransferInfo, TransferState, TransferType};

    use super::{
        SftpTransferTab, sftp_transfer_elapsed_secs, transfer_belongs_to_group,
        transfer_belongs_to_tab,
    };

    fn transfer(group_id: &str, state: TransferState) -> Transfer {
        Transfer {
            tab_id: group_id.to_string(),
            tab_title: group_id.to_string(),
            info: TransferInfo {
                id: "transfer".to_string(),
                name: "file.txt".to_string(),
                source: "/remote/file.txt".to_string(),
                target: "/local".to_string(),
                kind: TransferType::Download,
                total_bytes: Some(1024),
            },
            transferred: 512,
            total: Some(1024),
            state,
            started_at: 100,
            finished_at: None,
            files: Vec::new(),
        }
    }

    #[test]
    fn transfer_filter_keeps_records_in_the_active_sftp_group() {
        let active = transfer("active", TransferState::Running);
        let other = transfer("other", TransferState::Running);

        assert!(transfer_belongs_to_group(&active, Some("active")));
        assert!(!transfer_belongs_to_group(&other, Some("active")));
        assert!(!transfer_belongs_to_group(&active, None));
    }

    #[test]
    fn transfer_filter_uses_the_expected_status_tab() {
        assert!(transfer_belongs_to_tab(
            &transfer("group", TransferState::Paused),
            SftpTransferTab::Active
        ));
        assert!(transfer_belongs_to_tab(
            &transfer("group", TransferState::Failed("failed".to_string())),
            SftpTransferTab::Failed
        ));
        assert!(transfer_belongs_to_tab(
            &transfer("group", TransferState::Completed),
            SftpTransferTab::Completed
        ));
    }

    #[test]
    fn transfer_elapsed_time_uses_the_terminal_timestamp() {
        let mut transfer = transfer("group", TransferState::Completed);
        transfer.finished_at = Some(145);

        assert_eq!(sftp_transfer_elapsed_secs(&transfer, 999), Some(45));
    }
}
