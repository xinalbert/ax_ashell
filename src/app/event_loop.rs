use std::time::Duration;

use gpui::{Context, Window, px};
use gpui_component::input::{InputEvent, InputState};

use crate::{AxShell, terminal::BackendEvent};

impl AxShell {
    pub(crate) fn on_input_event(
        &mut self,
        input: &gpui::Entity<InputState>,
        event: &InputEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if input == &self.sftp_path_input {
            if let InputEvent::PressEnter { .. } = event {
                let path = self
                    .sftp_path_input
                    .read(cx)
                    .text()
                    .to_string()
                    .trim()
                    .to_string();
                self.navigate_sftp(if path.is_empty() { "/".into() } else { path }, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
        } else if input == &self.local_sftp_path_input {
            if let InputEvent::PressEnter { .. } = event {
                let path = self
                    .local_sftp_path_input
                    .read(cx)
                    .text()
                    .to_string()
                    .trim()
                    .to_string();
                self.navigate_local_file_browser(path, cx);
                window.prevent_default();
                cx.stop_propagation();
            }
        } else if input == &self.sftp_new_folder_input {
            match event {
                InputEvent::PressEnter { .. } => {
                    let name = self.sftp_new_folder_input.read(cx).text().to_string();
                    if !name.is_empty() {
                        let base_path = self.sftp_path_input.read(cx).text().to_string();
                        let path = crate::sftp::join_remote(&base_path, &name);
                        if let Some(handle) = self.active_sftp_handle() {
                            let _ = handle
                                .commands
                                .send(crate::sftp::SftpCommand::CreateDir(path));
                        }
                    }
                    self.sftp_creating_folder = false;
                    window.prevent_default();
                    cx.stop_propagation();
                }
                InputEvent::Blur => {
                    self.sftp_creating_folder = false;
                }
                _ => {}
            }
        } else if input == &self.search_input {
            if let InputEvent::PressEnter { .. } = event {
                if self.search_query.is_empty()
                    || self.search_input.read(cx).text().to_string() != self.search_query
                {
                    self.perform_search(window, cx);
                } else {
                    self.search_goto_next(cx);
                }
                window.prevent_default();
                cx.stop_propagation();
            }
        } else if input == &self.saved_group_name_input {
            match event {
                InputEvent::PressEnter { .. } => {
                    self.commit_saved_group_rename(cx);
                    window.prevent_default();
                    cx.stop_propagation();
                }
                InputEvent::Blur => {
                    self.commit_saved_group_rename(cx);
                }
                _ => {}
            }
        } else if self
            .custom_theme_inputs
            .values()
            .any(|custom_input| input == custom_input)
            && matches!(event, InputEvent::PressEnter { .. })
        {
            self.save_custom_appearance(window, cx);
            window.prevent_default();
            cx.stop_propagation();
        }
        cx.notify();
    }

    pub(crate) fn start_event_pump(&self, cx: &mut Context<Self>) {
        cx.spawn(async move |this, cx| {
            let mut idle_frames = 0u32;
            let mut last_blink_time = std::time::Instant::now();
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                if this
                    .update(cx, |this, cx| {
                        let changed = this.drain_backend_events();
                        let system_sampled = this.sample_system_if_due();
                        this.sync_theme_if_due(cx);
                        let is_blinking = matches!(
                            this.cursor_style,
                            crate::session::config::CursorStyle::Blink
                                | crate::session::config::CursorStyle::BeamBlink
                        );
                        let now = std::time::Instant::now();
                        let blink_due = is_blinking
                            && now.duration_since(last_blink_time)
                                >= std::time::Duration::from_millis(600);
                        if changed || system_sampled || blink_due {
                            cx.notify();
                            idle_frames = 0;
                            if blink_due {
                                last_blink_time = now;
                            }
                        } else {
                            idle_frames += 1;
                            if idle_frames >= 60 {
                                cx.notify();
                                idle_frames = 0;
                            }
                        }
                    })
                    .is_err()
                {
                    break;
                }
            }
        })
        .detach();
    }

    pub(crate) fn drain_backend_events(&mut self) -> bool {
        let mut changed = false;
        let mut transfers_changed = false;
        while let Ok(event) = self.events_rx.try_recv() {
            changed = true;
            match event {
                BackendEvent::Output { tab_id, bytes } => {
                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                        tab.backend_initialized = true;
                        tab.feed(&bytes);
                    }
                    if self.terminal_marked_text.take().is_some() {
                        changed = true;
                    }
                }
                BackendEvent::Status { tab_id, text } => {
                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                        tab.backend_initialized = true;
                        tab.status = text.clone();
                    }
                    if let Some(progress) = self.connection_progress.as_mut()
                        && progress.tab_id == tab_id
                    {
                        progress.lines.push(text.clone().into());
                        self.connection_scroll_handle
                            .set_offset(gpui::point(px(0.), px(-99999.0)));
                    }
                    self.status = text.into();
                }
                BackendEvent::Connected { tab_id } => {
                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                        tab.backend_initialized = true;
                        tab.connected = true;
                        tab.disconnected_reason = None;
                    }
                    self.sync_system_tab_to_active_group();
                    self.request_active_system_snapshot();
                    if self
                        .connection_progress
                        .as_ref()
                        .is_some_and(|progress| progress.tab_id == tab_id && !progress.failed)
                    {
                        self.connection_progress = None;
                    }
                }
                BackendEvent::SshConnectionModeResolved {
                    tab_id,
                    session_id,
                    mode,
                } => {
                    for tab in self.tabs.iter_mut() {
                        if tab.id == tab_id
                            || tab
                                .session
                                .as_ref()
                                .is_some_and(|session| session.id == session_id)
                        {
                            if let Some(session) = tab.session.as_mut() {
                                session.last_successful_ssh_mode = Some(mode);
                            }
                        }
                    }
                    if self
                        .config
                        .set_session_last_successful_ssh_mode(&session_id, mode)
                    {
                        let _ = self.config.save();
                    }
                }
                BackendEvent::SftpEntries {
                    tab_id,
                    path,
                    entries,
                } => {
                    if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == tab_id)
                        && let Some(sftp) = group.sftp.as_mut()
                    {
                        sftp.current_path = path;
                        sftp.entries = entries;
                        self.pending_sftp_path_sync = Some(sftp.current_path.clone());
                    }
                }
                BackendEvent::SftpPreview { tab_id, preview } => {
                    if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == tab_id)
                        && let Some(sftp) = group.sftp.as_mut()
                    {
                        sftp.selected_path = Some(preview.path.clone());
                        sftp.preview = Some(preview);
                    }
                }
                BackendEvent::SftpStatus { tab_id, text } => {
                    if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == tab_id)
                        && let Some(sftp) = group.sftp.as_mut()
                    {
                        sftp.status = text.clone();
                    }
                    if self.active_group.as_ref() == Some(&tab_id) {
                        self.status = text.into();
                    }
                }
                BackendEvent::RemoteSystem { tab_id, snapshot } => {
                    self.remote_sample_in_flight = false;
                    if self.system_tab_id.as_deref() == Some(tab_id.as_str()) {
                        self.system_status = None;
                        self.system = snapshot.clone();
                        self.cpu_history.push(snapshot.cpu_percent);
                        if self.cpu_history.len() > 20 {
                            self.cpu_history.remove(0);
                        }
                        self.net_rx_history.push(snapshot.net_rx_rate as f32);
                        if self.net_rx_history.len() > 20 {
                            self.net_rx_history.remove(0);
                        }
                        self.net_tx_history.push(snapshot.net_tx_rate as f32);
                        if self.net_tx_history.len() > 20 {
                            self.net_tx_history.remove(0);
                        }
                    }
                }
                BackendEvent::RemoteSystemUnavailable { tab_id, reason } => {
                    self.remote_sample_in_flight = false;
                    if self.system_tab_id.as_deref() == Some(tab_id.as_str()) {
                        self.system_status = Some(reason.clone().into());
                        self.status = reason.into();
                    }
                }
                BackendEvent::Closed { tab_id, reason } => {
                    self.remote_sample_in_flight = false;
                    let is_stale = self
                        .tabs
                        .iter()
                        .find(|t| t.id == tab_id)
                        .is_some_and(|tab| tab.backend_generation > 0 && !tab.backend_initialized);
                    if is_stale {
                        continue;
                    }
                    let is_graceful_exit =
                        reason == "local shell closed" || reason == "ssh session closed";
                    if is_graceful_exit {
                        self.handle_tab_close(tab_id.clone());
                        self.status = reason.into();
                        continue;
                    }
                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                        tab.connected = false;
                        tab.status = reason.clone();
                        tab.disconnected_reason = Some(reason.clone());
                    }
                    if self.system_tab_id.as_deref() == Some(tab_id.as_str()) {
                        self.system_status = Some(reason.clone().into());
                    }
                    if let Some(progress) = self.connection_progress.as_mut()
                        && progress.tab_id == tab_id
                    {
                        progress.lines.push(reason.clone().into());
                        self.connection_scroll_handle
                            .set_offset(gpui::point(px(0.), px(-99999.0)));
                        progress.title = rust_i18n::t!("connection_failed").into();
                        progress.failed = true;
                    }
                    self.status = reason.into();
                }
                BackendEvent::TransferProgress {
                    tab_id: _,
                    id,
                    transferred,
                    total,
                    state,
                } => {
                    if let Some(t) = self.transfers.iter_mut().find(|t| t.info.id == id) {
                        t.transferred = transferred;
                        if let Some(total) = total {
                            t.total = Some(total);
                        }
                        t.state = state;
                        transfers_changed = true;
                    }
                }
                BackendEvent::TransferStarted { tab_id, info } => {
                    let tab_title = self.transfer_source_title(&tab_id);
                    self.transfers.insert(
                        0,
                        crate::terminal::Transfer {
                            tab_id,
                            tab_title,
                            info,
                            transferred: 0,
                            total: None,
                            state: crate::terminal::TransferState::Running,
                        },
                    );
                    if self.transfers.len() > 100 {
                        self.transfers.truncate(100);
                    }
                    transfers_changed = true;
                }
                BackendEvent::SftpHome { tab_id, home } => {
                    if let Some(group) = self.tab_groups.iter_mut().find(|g| g.id == tab_id)
                        && let Some(sftp) = group.sftp.as_mut()
                    {
                        sftp.home_dir = home;
                    }
                }
                BackendEvent::TerminalTitleChanged { tab_id, title } => {
                    if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
                        tab.title = title.clone();
                    }
                }
                BackendEvent::SyncFinished(result) => {
                    self.sync_in_progress = false;
                    match result {
                        crate::sync::SyncResult::Uploaded { etag } => {
                            if etag.is_some() {
                                self.config.set_sync_etag(etag);
                            }
                            self.sync_status = rust_i18n::t!("sync_upload_complete").into();
                            let _ = self.config.save();
                        }
                        crate::sync::SyncResult::Downloaded { payload, etag } => {
                            self.config.replace_sessions(payload.sessions);
                            self.config.set_sync_etag(etag);
                            match self.config.save() {
                                Ok(()) => {
                                    self.sync_status =
                                        rust_i18n::t!("sync_download_complete").into()
                                }
                                Err(err) => {
                                    self.sync_status =
                                        format!("{}: {err:#}", rust_i18n::t!("sync_failed")).into()
                                }
                            }
                        }
                        crate::sync::SyncResult::Failed(error) => {
                            self.sync_status =
                                format!("{}: {error}", rust_i18n::t!("sync_failed")).into();
                        }
                    }
                }
            }
        }
        if transfers_changed {
            self.config.set_transfers(self.transfers.clone());
        }
        changed
    }

    pub(crate) fn sample_system_if_due(&mut self) -> bool {
        if !self.is_monitoring_visible() {
            return false;
        }
        if self.last_system_sample.elapsed() >= crate::system::SystemSampler::interval() {
            self.last_system_sample = std::time::Instant::now();
            if let Some(ref tab_id) = self.system_tab_id.clone()
                && self.tabs.iter().any(|t| {
                    t.id == *tab_id && t.kind == crate::terminal::TabKind::Ssh && t.connected
                })
                && self.system_status.is_none()
            {
                self.request_active_system_snapshot();
                return false;
            }
            let snapshot = self.system_sampler.sample();
            let cpu_usage = snapshot.cpu_percent;
            self.cpu_history.push(cpu_usage);
            if self.cpu_history.len() > 20 {
                self.cpu_history.remove(0);
            }
            self.net_rx_history.push(snapshot.net_rx_rate as f32);
            if self.net_rx_history.len() > 20 {
                self.net_rx_history.remove(0);
            }
            self.net_tx_history.push(snapshot.net_tx_rate as f32);
            if self.net_tx_history.len() > 20 {
                self.net_tx_history.remove(0);
            }
            self.system = snapshot;
            return true;
        }
        false
    }

    pub(crate) fn sync_theme_if_due(&mut self, cx: &mut Context<Self>) {
        if self.follow_system_theme && self.last_theme_sync.elapsed() >= Duration::from_secs(1) {
            self.last_theme_sync = std::time::Instant::now();
            gpui_component::Theme::sync_system_appearance(None, cx);
            cx.refresh_windows();
        }
    }
}
