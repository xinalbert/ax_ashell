use std::{collections::HashMap, ops::Range};

use gpui::{Bounds, Context, Pixels, Window, point, px, size};
use gpui_component::WindowExt as _;
use rust_i18n::t;

use crate::{
    AxShell,
    app::{
        ConnectionProgress, PaneLayout, SftpUiState, TerminalPasswordPrompt,
        session_ui::should_use_terminal_password_prompt,
    },
    config::ConfigStore,
    session::Session,
    terminal::{BackendCommand, TabKind},
};

#[derive(Clone)]
pub(crate) struct TabGroup {
    pub(crate) id: String,
    pub(crate) title: String,
    pub(crate) instance_number: usize,
    pub(crate) pane_root: PaneLayout,
    pub(crate) sftp: Option<SftpUiState>,
    pub(crate) sftp_page_open: bool,
    pub(crate) sftp_session: Option<Session>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum WorkspacePage {
    #[default]
    Terminal,
    Sftp,
    Settings,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct WorkspaceTabDescriptor {
    pub(crate) group_id: Option<String>,
    pub(crate) group_index: Option<usize>,
    pub(crate) page: WorkspacePage,
}

fn workspace_tab_selected_for(
    entry: &WorkspaceTabDescriptor,
    workspace_page: WorkspacePage,
    active_group: Option<&str>,
    settings_page_open: bool,
) -> bool {
    match entry.page {
        WorkspacePage::Settings => settings_page_open && workspace_page == WorkspacePage::Settings,
        page => workspace_page == page && entry.group_id.as_deref() == active_group,
    }
}

fn active_workspace_tab_index_for(
    tabs: &[WorkspaceTabDescriptor],
    workspace_page: WorkspacePage,
    active_group: Option<&str>,
    settings_page_open: bool,
) -> usize {
    tabs.iter()
        .position(|entry| {
            workspace_tab_selected_for(entry, workspace_page, active_group, settings_page_open)
        })
        .or_else(|| {
            active_group.and_then(|group_id| {
                tabs.iter().position(|entry| {
                    entry.page == WorkspacePage::Terminal
                        && entry.group_id.as_deref() == Some(group_id)
                })
            })
        })
        .unwrap_or(0)
}

/// Move an item before a target item, or to the end when no target is supplied.
fn move_item_before<T>(
    items: &mut Vec<T>,
    source_index: usize,
    target_index: Option<usize>,
) -> bool {
    if source_index >= items.len() {
        return false;
    }

    let target_index = target_index.unwrap_or(items.len());
    if target_index > items.len() || target_index == source_index {
        return false;
    }

    let insertion_index = target_index - usize::from(source_index < target_index);
    if insertion_index == source_index {
        return false;
    }

    let item = items.remove(source_index);
    items.insert(insertion_index, item);
    true
}

fn next_workspace_group_instance(
    instance_counts: &mut HashMap<String, usize>,
    title: &str,
) -> usize {
    let instance_number = instance_counts.entry(title.to_string()).or_default();
    *instance_number += 1;
    *instance_number
}

pub(crate) fn workspace_group_tab_label(
    group_index: usize,
    title: &str,
    instance_number: usize,
    page: WorkspacePage,
    pane_count: usize,
) -> String {
    let tab_number = group_index + 1;
    match page {
        WorkspacePage::Terminal => {
            let instance_label = format!("{title} #{instance_number}");
            if pane_count > 1 {
                format!("{tab_number} {instance_label} ({pane_count})")
            } else {
                format!("{tab_number} {instance_label}")
            }
        }
        WorkspacePage::Sftp => format!("{tab_number} {title} #{instance_number} SFTP"),
        WorkspacePage::Settings => unreachable!("settings is not a workspace group tab"),
    }
}

impl AxShell {
    /// Allocate a stable, per-window instance number for a workspace title.
    pub(crate) fn next_workspace_group_instance(&mut self, title: &str) -> usize {
        next_workspace_group_instance(&mut self.workspace_group_instance_counts, title)
    }

    pub(crate) fn workspace_tabs(&self) -> Vec<WorkspaceTabDescriptor> {
        let mut tabs = Vec::new();

        for (group_index, group) in self.tab_groups.iter().enumerate() {
            if self.group_has_terminal_tab(&group.id) {
                tabs.push(WorkspaceTabDescriptor {
                    group_id: Some(group.id.clone()),
                    group_index: Some(group_index),
                    page: WorkspacePage::Terminal,
                });
            }

            if group.sftp.is_some() && group.sftp_page_open {
                tabs.push(WorkspaceTabDescriptor {
                    group_id: Some(group.id.clone()),
                    group_index: Some(group_index),
                    page: WorkspacePage::Sftp,
                });
            }
        }

        if self.settings_page_open {
            tabs.push(WorkspaceTabDescriptor {
                group_id: None,
                group_index: None,
                page: WorkspacePage::Settings,
            });
        }

        tabs
    }

    /// Reorder an entire workspace group so its Terminal and SFTP pages stay adjacent.
    pub(crate) fn move_workspace_group_before(
        &mut self,
        source_group_id: &str,
        target_group_id: Option<&str>,
        cx: &mut Context<Self>,
    ) {
        let Some(source_index) = self
            .tab_groups
            .iter()
            .position(|group| group.id == source_group_id)
        else {
            return;
        };
        let target_index = match target_group_id {
            Some(target_group_id) => {
                let Some(target_index) = self
                    .tab_groups
                    .iter()
                    .position(|group| group.id == target_group_id)
                else {
                    return;
                };
                Some(target_index)
            }
            None => None,
        };

        if move_item_before(&mut self.tab_groups, source_index, target_index) {
            self.ensure_active_workspace_tab_visible();
            cx.notify();
        }
    }

    pub(crate) fn workspace_tab_selected(&self, entry: &WorkspaceTabDescriptor) -> bool {
        workspace_tab_selected_for(
            entry,
            self.workspace_page,
            self.active_group.as_deref(),
            self.settings_page_open,
        )
    }

    pub(crate) fn active_workspace_tab_index(&self, tabs: &[WorkspaceTabDescriptor]) -> usize {
        active_workspace_tab_index_for(
            tabs,
            self.workspace_page,
            self.active_group.as_deref(),
            self.settings_page_open,
        )
    }

    /// Scroll the tab bar just enough to reveal the active rendered workspace tab.
    pub(crate) fn ensure_active_workspace_tab_visible(&self) {
        let workspace_tabs = self.workspace_tabs();
        if workspace_tabs.is_empty() {
            return;
        }

        self.tabs_scroll_handle
            .scroll_to_item(self.active_workspace_tab_index(&workspace_tabs));
    }

    pub(crate) fn active_group_sftp_page_open(&self) -> bool {
        self.active_group
            .as_ref()
            .and_then(|group_id| self.tab_groups.iter().find(|group| &group.id == group_id))
            .is_some_and(|group| group.sftp.is_some() && group.sftp_page_open)
    }

    pub(crate) fn group_has_terminal_tab(&self, group_id: &str) -> bool {
        let Some(group) = self.tab_groups.iter().find(|group| group.id == group_id) else {
            return false;
        };
        group
            .pane_root
            .tab_ids()
            .into_iter()
            .any(|tab_id| !tab_id.is_empty() && self.tabs.iter().any(|tab| tab.id == tab_id))
    }

    pub(crate) fn transfer_source_title(&self, tab_id: &str) -> String {
        self.tabs
            .iter()
            .find(|tab| tab.id == tab_id)
            .map(|tab| tab.title.clone())
            .or_else(|| {
                self.tab_groups
                    .iter()
                    .find(|group| group.id == tab_id)
                    .map(|group| group.title.clone())
            })
            .or_else(|| {
                self.tab_groups
                    .iter()
                    .find(|group| group.pane_root.contains(tab_id))
                    .map(|group| group.title.clone())
            })
            .unwrap_or_else(|| "Unknown".to_string())
    }

    pub(crate) fn set_workspace_page(&mut self, page: WorkspacePage, cx: &mut Context<Self>) {
        let page = if page == WorkspacePage::Sftp && !self.active_group_sftp_page_open() {
            WorkspacePage::Terminal
        } else {
            page
        };

        if self.workspace_page == page {
            if page == WorkspacePage::Sftp {
                self.restore_active_local_sftp_path(cx);
            }
            self.ensure_active_workspace_tab_visible();
            return;
        }

        if self.workspace_page == WorkspacePage::Settings {
            self.keybinds_suspended = false;
            self.recording_action = None;
            self.keybind_error = None;
            crate::app::keybinding_recorder::bind_workspace_keys_from_config(cx, &self.config);
            crate::app::app_menu::refresh(cx);
        }

        if self.workspace_page == WorkspacePage::Terminal && page != WorkspacePage::Terminal {
            self.search.active = false;
            self.search.query.clear();
            self.search.matches.clear();
            self.search.current = 0;
            self.search.target_tab = None;
            self.search.bar_bounds = None;
        }

        if page == WorkspacePage::Settings {
            crate::app::keybinding_recorder::unbind_all_workspace_keys(cx, &self.config);
            self.keybinds_suspended = true;
        }

        self.workspace_page = page;
        if page == WorkspacePage::Sftp {
            self.restore_active_local_sftp_path(cx);
        }
        self.ensure_active_workspace_tab_visible();
        cx.notify();
    }

    pub(crate) fn open_settings_page(&mut self, cx: &mut Context<Self>) {
        self.open_settings_page_at(0, cx);
    }

    pub(crate) fn open_about_page(&mut self, cx: &mut Context<Self>) {
        const ABOUT_PAGE_INDEX: usize = 10;
        self.open_settings_page_at(ABOUT_PAGE_INDEX, cx);
    }

    fn open_settings_page_at(&mut self, initial_page: usize, cx: &mut Context<Self>) {
        let page_changed = self.settings_initial_page != initial_page;
        self.settings_initial_page = initial_page;
        if !self.settings_page_open || page_changed {
            self.settings_page_generation = self.settings_page_generation.wrapping_add(1);
        }
        self.settings_page_open = true;
        self.set_workspace_page(WorkspacePage::Settings, cx);
    }

    pub(crate) fn request_close_settings_page(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.settings_page_open {
            return;
        }

        self.show_settings_close_confirm_dialog(window, cx);
    }

    pub(crate) fn confirm_settings_close_with_shortcut(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.active_dialog != Some(crate::app::DialogKind::SettingsCloseConfirm) {
            return false;
        }

        let close_settings = self.config.settings_close_shortcut_confirms();
        self.apply_settings_close_choice(close_settings, false, window, cx);
        true
    }

    pub(crate) fn apply_settings_close_choice(
        &mut self,
        close_settings: bool,
        remember: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if remember {
            self.config
                .set_settings_close_shortcut_confirms(close_settings);
            self.config.save_logged("remember_settings_close_choice");
        }

        self.active_dialog = None;
        self.settings_close_remember_choice = false;
        window.close_dialog(cx);

        if close_settings {
            self.close_settings_page(cx);
        } else {
            cx.notify();
        }
    }

    pub(crate) fn close_settings_page(&mut self, cx: &mut Context<Self>) {
        self.settings_page_open = false;
        if self.workspace_page == WorkspacePage::Settings {
            self.set_workspace_page(WorkspacePage::Terminal, cx);
        } else {
            self.ensure_active_workspace_tab_visible();
            cx.notify();
        }
    }

    pub(crate) fn switch_workspace_tab(
        &mut self,
        step: isize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let workspace_tabs = self.workspace_tabs();
        if workspace_tabs.len() <= 1 {
            return;
        }

        let current_index = self.active_workspace_tab_index(&workspace_tabs);
        let next_index =
            (current_index as isize + step).rem_euclid(workspace_tabs.len() as isize) as usize;
        let Some(target) = workspace_tabs.get(next_index).cloned() else {
            return;
        };

        match target.page {
            WorkspacePage::Settings => self.open_settings_page(cx),
            page => {
                let Some(group_id) = target.group_id else {
                    return;
                };
                self.activate_group_page(group_id, page, window, cx);
            }
        }
    }

    pub(crate) fn toggle_active_sftp_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.workspace_page == WorkspacePage::Sftp {
            if let Some(active_group_id) = self.active_group.clone() {
                if !self.group_has_terminal_tab(&active_group_id) {
                    self.status = t!("sftp_shortcut_requires_ssh").into();
                    cx.notify();
                    return;
                }
                if self.confirm_sftp_close_with_shortcut(&active_group_id, window, cx) {
                    return;
                }
                self.close_sftp_page(active_group_id, window, cx);
            } else {
                self.set_workspace_page(WorkspacePage::Terminal, cx);
                self.focus_handle.focus(window, cx);
            }
            return;
        }

        let Some(active_group_id) = self.active_group.clone() else {
            self.status = t!("open_ssh_tab_sftp").into();
            cx.notify();
            return;
        };

        if let Some(group) = self
            .tab_groups
            .iter_mut()
            .find(|group| group.id == active_group_id)
            && group.sftp.is_some()
        {
            group.sftp_page_open = true;
            self.ensure_sftp_handle_for_group(&active_group_id);
            self.mark_sftp_activity_for_group(&active_group_id);
            self.set_workspace_page(WorkspacePage::Sftp, cx);
            self.focus_handle.focus(window, cx);
        } else {
            self.status = t!("open_ssh_tab_sftp").into();
            cx.notify();
        }
    }

    pub(crate) fn open_sftp_transfers_page(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(active_group_id) = self.active_group.clone() else {
            self.status = t!("open_ssh_tab_sftp").into();
            cx.notify();
            return;
        };

        if let Some(group) = self
            .tab_groups
            .iter_mut()
            .find(|group| group.id == active_group_id)
            && group.sftp.is_some()
        {
            group.sftp_page_open = true;
            self.sftp_transfer_tab = crate::app::SftpTransferTab::Active;
            self.ensure_sftp_handle_for_group(&active_group_id);
            self.mark_sftp_activity_for_group(&active_group_id);
            self.set_workspace_page(WorkspacePage::Sftp, cx);
            self.focus_handle.focus(window, cx);
        } else {
            self.status = t!("open_ssh_tab_sftp").into();
            cx.notify();
        }
    }

    pub(crate) fn activate_first_visible_group_or_home(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let mut next = None;
        for group in &self.tab_groups {
            if self.group_has_terminal_tab(&group.id) {
                next = Some((group.id.clone(), WorkspacePage::Terminal));
                break;
            }
            if group.sftp.is_some() && group.sftp_page_open {
                next = Some((group.id.clone(), WorkspacePage::Sftp));
                break;
            }
        }

        if let Some((group_id, page)) = next {
            self.active_group = None;
            self.activate_group_page(group_id, page, window, cx);
            return;
        }

        self.active_group = None;
        self.active_tab = None;
        self.pane_root = PaneLayout::Single(String::new());
        self.focused_pane_path = vec![];
        self.set_workspace_page(WorkspacePage::Terminal, cx);
        self.focus_handle.focus(window, cx);
        cx.notify();
    }

    pub(crate) fn close_sftp_page(
        &mut self,
        group_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.has_dirty_sftp_edit_sessions(&group_id) {
            self.request_sftp_edit_uploads(&group_id, true, cx);
            return;
        }
        let has_uploading_edit = self
            .tab_groups
            .iter()
            .find(|group| group.id == group_id)
            .and_then(|group| group.sftp.as_ref())
            .is_some_and(|sftp| sftp.edit_sessions.iter().any(|session| session.uploading));
        if has_uploading_edit {
            self.sftp_edit_close_group_id = Some(group_id);
            return;
        }
        if !self.group_has_active_sftp_transfer(&group_id) {
            self.close_sftp_page_now(group_id, window, cx);
            return;
        }

        self.show_sftp_transfer_close_dialog(group_id, window, cx);
    }

    pub(crate) fn finish_pending_sftp_edit_close(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(group_id) = self.sftp_edit_close_group_id.clone() else {
            return;
        };
        let has_uploading = self
            .tab_groups
            .iter()
            .find(|group| group.id == group_id)
            .and_then(|group| group.sftp.as_ref())
            .is_some_and(|sftp| sftp.edit_sessions.iter().any(|session| session.uploading));
        let request_is_pending = self
            .sftp_edit_upload_request
            .as_ref()
            .is_some_and(|request| request.group_id == group_id)
            || self
                .sftp_edit_upload_requests
                .iter()
                .any(|request| request.group_id == group_id);
        if self.has_dirty_sftp_edit_sessions(&group_id) || has_uploading || request_is_pending {
            return;
        }
        self.sftp_edit_close_group_id = None;
        self.close_sftp_page(group_id, window, cx);
    }

    pub(crate) fn confirm_sftp_close_with_shortcut(
        &mut self,
        group_id: &str,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> bool {
        if self.active_dialog != Some(crate::app::DialogKind::SftpCloseConfirm)
            || self.sftp_close_confirm_group_id.as_deref() != Some(group_id)
        {
            return false;
        }

        let choice = self.config.sftp_transfer_close_behavior().to_string();
        if choice == "ask" {
            self.status = rust_i18n::t!("sftp_shortcut_choice_not_set").into();
            cx.notify();
            return true;
        }

        self.apply_sftp_transfer_close_choice(group_id.to_string(), &choice, false, window, cx);
        self.active_dialog = None;
        self.sftp_close_confirm_group_id = None;
        window.close_dialog(cx);
        true
    }

    pub(crate) fn apply_sftp_transfer_close_choice(
        &mut self,
        group_id: String,
        choice: &str,
        remember: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if remember {
            self.config.set_sftp_transfer_close_behavior(choice);
            self.config.save_logged("remember_sftp_close_choice");
        }
        self.sftp_close_remember_choice = false;
        self.sftp_close_confirm_group_id = None;

        match choice {
            "keep_page_open" => {
                self.status = rust_i18n::t!("sftp_close_kept_for_transfer").into();
                cx.notify();
            }
            "background" => self.close_sftp_page_now(group_id, window, cx),
            "cancel_disconnect" => {
                self.release_sftp_handle_for_group(&group_id, true);
                self.close_sftp_page_now(group_id, window, cx);
            }
            _ => unreachable!("invalid SFTP close choice"),
        }
    }

    pub(crate) fn close_sftp_page_now(
        &mut self,
        group_id: String,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let was_active_sftp_page = self.workspace_page == WorkspacePage::Sftp
            && self.active_group.as_deref() == Some(group_id.as_str());
        let has_terminal_tab = self.group_has_terminal_tab(&group_id);
        let keep_for_transfer = self.group_has_active_sftp_transfer(&group_id);

        self.discard_all_sftp_edit_sessions(&group_id);

        if let Some(group) = self
            .tab_groups
            .iter_mut()
            .find(|group| group.id == group_id)
        {
            group.sftp_page_open = false;
        }

        if !has_terminal_tab && !keep_for_transfer {
            self.release_sftp_handle_for_group(&group_id, false);
            if let Some(index) = self
                .tab_groups
                .iter()
                .position(|group| group.id == group_id)
            {
                self.tab_groups.remove(index);
            }
        }

        if was_active_sftp_page {
            if has_terminal_tab {
                self.set_workspace_page(WorkspacePage::Terminal, cx);
                self.focus_handle.focus(window, cx);
            } else {
                self.activate_first_visible_group_or_home(window, cx);
            }
        } else {
            cx.notify();
        }
    }

    pub(crate) fn request_active_system_snapshot(&mut self) {
        if !self.lifecycle.is_foreground() || !self.is_monitoring_visible() {
            return;
        }
        let Some(ref tab_id) = self.monitoring.system_tab_id.clone() else {
            return;
        };
        let Some(backend) = (|| {
            let tab = self.tabs.iter().find(|t| t.id == *tab_id)?;
            if !tab.connected || tab.connection_may_be_stale {
                return None;
            }
            Some(tab.backend.clone())
        })() else {
            return;
        };
        let Some(generation) = self.monitoring.begin_remote_sample(false) else {
            return;
        };
        if let Ok(backend) = backend.lock() {
            backend.send(BackendCommand::SampleMetrics { generation });
        } else {
            self.monitoring.finish_remote_sample(generation);
        }
    }

    pub(crate) fn request_active_ssh_resume_health_check(&mut self) -> bool {
        if !self.lifecycle.is_foreground() || self.workspace_page != WorkspacePage::Terminal {
            return false;
        }
        let Some(tab_id) = self.active_tab.clone() else {
            return false;
        };
        let Some((backend, backend_generation)) = self
            .tabs
            .iter()
            .find(|tab| tab.id == tab_id)
            .and_then(|tab| {
                (tab.kind == TabKind::Ssh && tab.connected && tab.connection_may_be_stale)
                    .then(|| (tab.backend.clone(), tab.backend_generation))
            })
        else {
            return false;
        };
        let Some(generation) = self.monitoring.begin_remote_sample(true) else {
            return false;
        };
        if let Ok(backend) = backend.lock() {
            backend.send(BackendCommand::CheckConnection {
                generation,
                backend_generation,
            });
            return true;
        }

        let _ = self.monitoring.finish_remote_sample(generation);
        false
    }

    pub(crate) fn group_primary_ssh_tab_id(&self, group_id: &str) -> Option<String> {
        let group = self.tab_groups.iter().find(|group| group.id == group_id)?;
        if let Some(active_tab) = self.active_tab.as_ref()
            && group.pane_root.contains(active_tab)
            && self
                .tabs
                .iter()
                .any(|tab| tab.id == *active_tab && tab.kind == TabKind::Ssh && tab.connected)
        {
            return Some(active_tab.clone());
        }
        group.pane_root.tab_ids().into_iter().find_map(|tab_id| {
            self.tabs
                .iter()
                .find(|tab| tab.id == tab_id && tab.kind == TabKind::Ssh && tab.connected)
                .map(|tab| tab.id.clone())
        })
    }

    pub(crate) fn is_monitoring_visible(&self) -> bool {
        if self.is_detached_workspace || !self.config.show_monitoring_dashboard() {
            return false;
        }
        match self.config.monitoring_position() {
            "Bottom" => true,
            "Sidebar" => !self.sidebar_collapsed,
            _ => false,
        }
    }

    pub(crate) fn terminal_ime_bounds_for_range(
        &self,
        range_utf16: Range<usize>,
        element_bounds: Bounds<Pixels>,
        cell_width: f32,
        line_height: f32,
    ) -> Option<Bounds<Pixels>> {
        let active_id = self.active_tab.as_ref()?;
        let snapshot = self.active_snapshot()?;
        let (row, col, range_start) = self
            .terminal_composition
            .as_ref()
            .filter(|composition| composition.tab_id == *active_id)
            .map(|composition| {
                (
                    composition.anchor_row,
                    composition.anchor_col,
                    range_utf16
                        .start
                        .min(composition.text.encode_utf16().count()),
                )
            })
            .or_else(|| {
                snapshot
                    .cursor
                    .map(|cursor| (cursor.row, cursor.col, range_utf16.start))
            })?;

        let row = row.min(snapshot.rows.saturating_sub(1));
        let col = col.min(snapshot.cols.saturating_sub(1));
        let x = element_bounds.origin.x
            + px(cell_width) * col as f32
            + px(cell_width) * range_start as f32;
        let y = element_bounds.origin.y + px(line_height) * row as f32;
        Some(Bounds::new(
            point(x, y),
            size(px(cell_width), px(line_height)),
        ))
    }

    pub(crate) fn remove_transfer(&mut self, transfer_id: &str, cx: &mut Context<Self>) {
        self.transfers.retain(|t| t.info.id != transfer_id);
        self.persist_transfers();
        cx.notify();
    }

    pub(crate) fn persist_transfers(&mut self) {
        let owned_group_ids = self
            .tab_groups
            .iter()
            .map(|group| group.id.as_str())
            .collect::<std::collections::HashSet<_>>();
        let mut persisted = ConfigStore::load()
            .unwrap_or_else(|_| ConfigStore::in_memory())
            .transfers();
        persisted.retain(|transfer| !owned_group_ids.contains(transfer.tab_id.as_str()));
        persisted.extend(self.transfers.iter().cloned());
        self.config.set_transfers(persisted);
    }

    pub(crate) fn should_begin_terminal_password_prompt(&self, tab_id: &str, reason: &str) -> bool {
        let password_from_terminal_prompt = self.terminal_password_retry_tabs.contains(tab_id);
        self.tabs
            .iter()
            .find(|tab| tab.id == tab_id)
            .and_then(|tab| tab.session.as_ref())
            .is_some_and(|session| {
                should_use_terminal_password_prompt(session, reason, password_from_terminal_prompt)
            })
    }

    pub(crate) fn begin_terminal_password_prompt(
        &mut self,
        tab_id: &str,
        reason: &str,
        cx: &mut Context<Self>,
    ) -> bool {
        if !self.should_begin_terminal_password_prompt(tab_id, reason) {
            return false;
        }

        let password_from_terminal_prompt = self.terminal_password_retry_tabs.remove(tab_id);
        if password_from_terminal_prompt
            && let Some(session) = self
                .tabs
                .iter_mut()
                .find(|tab| tab.id == tab_id)
                .and_then(|tab| tab.session.as_mut())
        {
            session.password.clear();
        }

        if self
            .connection_progress
            .as_ref()
            .is_some_and(|progress| progress.tab_id == tab_id)
        {
            self.connection_progress = None;
        }
        self.terminal_password_prompt = Some(TerminalPasswordPrompt::new(tab_id.to_string()));

        if let Some(group) = self
            .tab_groups
            .iter()
            .find(|group| group.pane_root.contains(tab_id))
        {
            self.active_group = Some(group.id.clone());
            self.pane_root = group.pane_root.clone();
        }
        self.focus_pane_with_id(tab_id.to_string());
        self.set_workspace_page(WorkspacePage::Terminal, cx);

        let prompt = if password_from_terminal_prompt {
            "\r\nPermission denied, please try again.\r\nPassword: ".to_string()
        } else {
            format!("\r\n{reason}\r\nPassword: ")
        };
        self.status = "waiting for ssh password".into();
        self.feed_terminal_tab_bytes(tab_id, prompt.as_bytes())
    }

    pub(crate) fn feed_terminal_tab_bytes(&mut self, tab_id: &str, bytes: &[u8]) -> bool {
        let Some(tab) = self.tabs.iter_mut().find(|tab| tab.id == tab_id) else {
            return false;
        };
        let changed = tab.feed(bytes);
        tab.scroll_to_bottom();
        changed
    }

    pub(crate) fn retry_terminal_password_prompt(
        &mut self,
        tab_id: &str,
        password: String,
        cx: &mut Context<Self>,
    ) {
        let Some(ix) = self.tabs.iter().position(|tab| tab.id == tab_id) else {
            return;
        };
        let Some(mut session) = self.tabs[ix].session.clone() else {
            return;
        };
        if session.auth != crate::session::AuthMethod::Password {
            return;
        }

        session.password = password;
        let new_generation = self.tabs[ix].backend_generation + 1;
        let cols = self.tabs[ix].cols;
        let rows = self.tabs[ix].rows;

        self.tabs[ix].send_backend(BackendCommand::Close);
        let (runtime, task_tracker) = self.runtime_state.runtime_handle_and_tracker();
        let backend = crate::backend::ssh::spawn_ssh_terminal(
            &runtime,
            task_tracker,
            tab_id.to_string(),
            session.clone(),
            cols,
            rows,
            self.runtime_state.events_tx.clone(),
        );

        self.tabs[ix].session = Some(session);
        self.tabs[ix].set_backend(backend);
        self.tabs[ix].connected = false;
        self.tabs[ix].status = "connecting".into();
        self.tabs[ix].disconnected_reason = None;
        self.tabs[ix].backend_generation = new_generation;
        self.tabs[ix].backend_initialized = false;
        self.terminal_password_retry_tabs.insert(tab_id.to_string());

        if let Some(group) = self
            .tab_groups
            .iter()
            .find(|group| group.pane_root.contains(tab_id))
        {
            let group_id = group.id.clone();
            let group_session = self
                .tabs
                .iter()
                .find(|tab| group.pane_root.contains(&tab.id) && tab.session.is_some())
                .and_then(|tab| tab.session.clone());

            if group_session.is_some() {
                self.restart_sftp_handle_for_group(&group_id);
            }
        }

        self.connection_progress = Some(ConnectionProgress {
            tab_id: tab_id.to_string(),
            title: t!("connecting").into(),
            lines: vec![t!("starting_connection").into()],
            failed: false,
        });
        self.status = "ssh tab retrying".into();
        cx.notify();
    }

    pub(crate) fn retry_connection_progress(&mut self, cx: &mut Context<Self>) {
        let Some(progress) = self.connection_progress.clone() else {
            return;
        };
        self.connection_progress = None;
        let can_retry = self.tabs.iter().any(|tab| {
            matches!(tab.kind, TabKind::Ssh | TabKind::Serial | TabKind::Telnet)
                && !tab.connected
                && tab.session.is_some()
                && tab.disconnected_reason.is_some()
                && tab.id == progress.tab_id
        });
        if !can_retry {
            cx.notify();
            return;
        }

        // The terminal retry path preserves scrollback and recreates the
        // protocol-specific backend. Only SSH restarts its SFTP handle there.
        self.retry_disconnected_tab(&progress.tab_id, cx);

        self.connection_progress = Some(ConnectionProgress {
            tab_id: progress.tab_id.clone(),
            title: t!("connecting").into(),
            lines: vec![t!("starting_connection").into()],
            failed: false,
        });
        self.status = "connection retrying".into();
        cx.notify();
    }

    pub(crate) fn cancel_connection_progress(&mut self, cx: &mut Context<Self>) {
        if let Some(progress) = &self.connection_progress {
            let tab_id = progress.tab_id.clone();
            self.connection_progress = None;
            self.handle_tab_close(tab_id, cx);
        }
        cx.notify();
    }

    pub(crate) fn save_layout_state(&self, window: &mut gpui::Window, cx: &gpui::App) {
        if self.is_layout_reset || !self.persist_window_layout {
            tracing::info!(
                component = "workspace",
                operation = "save_layout",
                "Layout persistence is disabled; skipping layout save"
            );
            return;
        }
        let current_bounds = window.window_bounds();
        let bounds = match current_bounds {
            gpui::WindowBounds::Fullscreen(b) => b,
            gpui::WindowBounds::Maximized(b) => b,
            gpui::WindowBounds::Windowed(b) => b,
        };
        let size = bounds.size;
        if size.width.as_f32() > 400.0 && size.height.as_f32() > 300.0 {
            tracing::info!(
                component = "workspace",
                operation = "save_layout",
                "Saving workspace layout"
            );
            let mut config = ConfigStore::load().unwrap_or_else(|_| ConfigStore::in_memory());
            let saved_bounds = match current_bounds {
                gpui::WindowBounds::Fullscreen(b) => crate::config::SavedWindowBounds::Fullscreen {
                    x: b.origin.x.into(),
                    y: b.origin.y.into(),
                    width: b.size.width.into(),
                    height: b.size.height.into(),
                },
                gpui::WindowBounds::Maximized(b) => {
                    let mut restore_bounds = (
                        b.origin.x.into(),
                        b.origin.y.into(),
                        b.size.width.into(),
                        b.size.height.into(),
                    );
                    if let Some(existing_bounds) = config.window_bounds() {
                        match existing_bounds {
                            crate::config::SavedWindowBounds::Windowed {
                                x,
                                y,
                                width,
                                height,
                            } => {
                                restore_bounds = (*x, *y, *width, *height);
                            }
                            crate::config::SavedWindowBounds::Maximized {
                                x,
                                y,
                                width,
                                height,
                            } => {
                                restore_bounds = (*x, *y, *width, *height);
                            }
                            _ => {}
                        }
                    }
                    crate::config::SavedWindowBounds::Maximized {
                        x: restore_bounds.0,
                        y: restore_bounds.1,
                        width: restore_bounds.2,
                        height: restore_bounds.3,
                    }
                }
                gpui::WindowBounds::Windowed(b) => crate::config::SavedWindowBounds::Windowed {
                    x: b.origin.x.into(),
                    y: b.origin.y.into(),
                    width: b.size.width.into(),
                    height: b.size.height.into(),
                },
            };
            let workspace_sizes: Vec<f32> = self
                .workspace_panels
                .read(cx)
                .sizes()
                .iter()
                .map(|s| s.into())
                .collect();
            let body_sizes: Vec<f32> = self
                .body_panels
                .read(cx)
                .sizes()
                .iter()
                .map(|s| s.into())
                .collect();

            config.set_layout_state(Some(saved_bounds), Some(workspace_sizes), Some(body_sizes));
            config.set_sidebar_collapsed(self.sidebar_collapsed);
            config.save_logged("save_layout");
        } else {
            tracing::warn!(
                component = "workspace",
                operation = "save_layout",
                width = size.width.as_f32(),
                height = size.height.as_f32(),
                "Window is too small; skipping layout save"
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WorkspacePage, WorkspaceTabDescriptor, active_workspace_tab_index_for, move_item_before,
        next_workspace_group_instance, workspace_group_tab_label,
    };
    use std::collections::HashMap;

    fn workspace_tab(group_id: Option<&str>, page: WorkspacePage) -> WorkspaceTabDescriptor {
        WorkspaceTabDescriptor {
            group_id: group_id.map(str::to_string),
            group_index: None,
            page,
        }
    }

    #[test]
    fn active_workspace_tab_index_uses_rendered_tab_order() {
        let tabs = vec![
            workspace_tab(Some("group-a"), WorkspacePage::Terminal),
            workspace_tab(Some("group-a"), WorkspacePage::Sftp),
            workspace_tab(Some("group-b"), WorkspacePage::Terminal),
            workspace_tab(None, WorkspacePage::Settings),
        ];

        assert_eq!(
            active_workspace_tab_index_for(&tabs, WorkspacePage::Terminal, Some("group-a"), true,),
            0
        );
        assert_eq!(
            active_workspace_tab_index_for(&tabs, WorkspacePage::Sftp, Some("group-a"), true),
            1
        );
        assert_eq!(
            active_workspace_tab_index_for(&tabs, WorkspacePage::Terminal, Some("group-b"), true,),
            2
        );
        assert_eq!(
            active_workspace_tab_index_for(&tabs, WorkspacePage::Settings, None, true),
            3
        );
    }

    #[test]
    fn move_item_before_reorders_forward_backward_and_to_end() {
        let mut items = vec!["a", "b", "c", "d"];

        assert!(move_item_before(&mut items, 0, Some(3)));
        assert_eq!(items, ["b", "c", "a", "d"]);

        assert!(move_item_before(&mut items, 3, Some(1)));
        assert_eq!(items, ["b", "d", "c", "a"]);

        assert!(move_item_before(&mut items, 1, None));
        assert_eq!(items, ["b", "c", "a", "d"]);
    }

    #[test]
    fn move_item_before_ignores_noop_and_invalid_targets() {
        let mut items = vec!["a", "b", "c"];

        assert!(!move_item_before(&mut items, 0, Some(0)));
        assert!(!move_item_before(&mut items, 0, Some(1)));
        assert!(!move_item_before(&mut items, 2, None));
        assert!(!move_item_before(&mut items, 3, None));
        assert!(!move_item_before(&mut items, 1, Some(4)));
        assert_eq!(items, ["a", "b", "c"]);
    }

    #[test]
    fn workspace_group_instances_are_stable_per_title() {
        let mut counts = HashMap::new();

        assert_eq!(next_workspace_group_instance(&mut counts, "Local"), 1);
        assert_eq!(next_workspace_group_instance(&mut counts, "prod-web"), 1);
        assert_eq!(next_workspace_group_instance(&mut counts, "Local"), 2);
        assert_eq!(next_workspace_group_instance(&mut counts, "prod-web"), 2);
    }

    #[test]
    fn workspace_group_tab_labels_include_stable_instance_numbers() {
        assert_eq!(
            workspace_group_tab_label(0, "Local", 1, WorkspacePage::Terminal, 1),
            "1 Local #1"
        );
        assert_eq!(
            workspace_group_tab_label(1, "prod-web", 2, WorkspacePage::Terminal, 3),
            "2 prod-web #2 (3)"
        );
        assert_eq!(
            workspace_group_tab_label(2, "prod-web", 2, WorkspacePage::Sftp, 0),
            "3 prod-web #2 SFTP"
        );
    }
}
