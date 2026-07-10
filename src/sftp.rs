mod auth;
mod path;

use std::{
    collections::VecDeque,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, Mutex},
    time::Duration,
};

use anyhow::{Context, Result, anyhow};
use flate2::read::GzDecoder;
use russh::Disconnect;
use russh_sftp::{
    client::{RawSftpSession, SftpSession, error::Error as SftpClientError},
    protocol::StatusCode,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::{JoinHandle, JoinSet},
};
use uuid::Uuid;
use walkdir::WalkDir;
use zip::read::ZipArchive;

use rust_i18n::t;

use crate::{
    session::config::Session,
    terminal::{BackendEvent, BackendEventSender, TransferState},
};

use self::{
    auth::{SftpClientHandler, connect_and_authenticate},
    path::{base_name, format_bytes, remote_parent, shell_quote, strip_archive_suffix},
};

pub use self::path::format_mtime;
pub(crate) use self::path::{join_remote, parent_dir, resolve_remote_path};

const SFTP_BROWSE_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(30);
const SFTP_SHUTDOWN_TIMEOUT: Duration = Duration::from_secs(2);
const SFTP_BROWSE_ENTRY_LIMIT: usize = 2_000;
const SFTP_BROWSE_NAME_BYTES_LIMIT: usize = 2 * 1024 * 1024;
const SFTP_BROWSE_PAGE_ENTRY_LIMIT: usize = 250;
const SFTP_PREVIEW_BYTE_LIMIT: usize = 128 * 1024;
const SFTP_DIRECTORY_PREVIEW_ENTRY_LIMIT: usize = 200;
const SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT: usize = SFTP_PREVIEW_BYTE_LIMIT - 512;

#[derive(Debug, Clone)]
pub struct RemoteEntry {
    pub name: String,
    pub full_path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u32,
}

struct DirectoryListing {
    entries: Vec<RemoteEntry>,
    truncated: bool,
}

struct DirectoryPage {
    entries: Vec<RemoteEntry>,
    has_more: bool,
    reached_limit: bool,
}

struct BrowseCursor {
    sftp: Option<RawSftpSession>,
    directory_handle: Option<String>,
    path: String,
    pending_entries: VecDeque<RemoteEntry>,
    retained_entries: usize,
    retained_name_bytes: usize,
    reached_limit: bool,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PreviewData {
    pub path: String,
    pub title: String,
    pub body: String,
    pub is_binary: bool,
}

enum SftpCommand {
    ListDir {
        path: String,
        pin: SftpWorkPin,
    },
    LoadMoreEntries {
        pin: SftpWorkPin,
    },
    RevealPath {
        path: String,
        pin: SftpWorkPin,
    },
    #[allow(dead_code)]
    Preview {
        path: String,
        pin: SftpWorkPin,
    },
    Download {
        remote: String,
        local_dir: String,
        pin: SftpWorkPin,
    },
    EditFile {
        remote_path: String,
        pin: SftpWorkPin,
    },
    CreateDir {
        path: String,
        pin: SftpWorkPin,
    },
    DeletePaths {
        paths: Vec<String>,
        pin: SftpWorkPin,
    },
    UploadEditedFile {
        local_path: String,
        remote_path: String,
        pin: SftpWorkPin,
    },
    UploadPaths {
        locals: Vec<String>,
        remote_dir: String,
        pin: SftpWorkPin,
    },
    PauseTransfer(String),
    ResumeTransfer(String),
    CancelTransfer(String),
    TransferFinished(String),
    Close,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RevealPathKind {
    Directory,
    File,
    Missing,
}

use std::sync::atomic::{AtomicU8, AtomicU64, AtomicUsize, Ordering};

pub struct TransferStateFlag(pub Arc<AtomicU8>);

impl TransferStateFlag {
    pub fn new() -> Self {
        Self(Arc::new(AtomicU8::new(0)))
    }

    pub fn pause(&self) {
        self.0.store(1, Ordering::SeqCst);
    }
    pub fn resume(&self) {
        self.0.store(0, Ordering::SeqCst);
    }
    pub fn cancel(&self) {
        self.0.store(2, Ordering::SeqCst);
    }

    pub async fn yield_if_paused(
        &self,
        events: &BackendEventSender,
        tab_id: &str,
        id: &str,
        transferred: u64,
        total: Option<u64>,
    ) -> anyhow::Result<()> {
        let mut was_paused = false;
        loop {
            let state = self.0.load(Ordering::SeqCst);
            if state == 2 {
                return Err(anyhow::anyhow!("transfer cancelled"));
            }
            if state == 1 {
                if !was_paused {
                    send_transfer_progress(
                        events,
                        tab_id,
                        id,
                        transferred,
                        total,
                        TransferState::Paused,
                    )
                    .await;
                    was_paused = true;
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            } else {
                if was_paused {
                    send_transfer_progress(
                        events,
                        tab_id,
                        id,
                        transferred,
                        total,
                        TransferState::Running,
                    )
                    .await;
                }
                return Ok(());
            }
        }
    }
}

async fn send_sftp_status(events: &BackendEventSender, tab_id: &str, text: impl Into<String>) {
    let _ = events
        .send(BackendEvent::SftpStatus {
            tab_id: tab_id.to_string(),
            text: text.into(),
        })
        .await;
}

async fn send_transfer_progress(
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    transferred: u64,
    total: Option<u64>,
    state: TransferState,
) {
    let _ = events
        .send(BackendEvent::TransferProgress {
            tab_id: tab_id.to_string(),
            id: id.to_string(),
            transferred,
            total,
            state,
        })
        .await;
}

async fn send_transfer_error(
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    err_msg: String,
    failed_status: String,
) {
    let is_cancelled = err_msg.contains("transfer cancelled");
    let state = if is_cancelled {
        TransferState::Interrupted("User cancelled".to_string())
    } else {
        TransferState::Failed(err_msg)
    };
    send_sftp_status(
        events,
        tab_id,
        if is_cancelled {
            "Transmission cancelled".to_string()
        } else {
            failed_status
        },
    )
    .await;
    send_transfer_progress(events, tab_id, id, 0, None, state).await;
}

async fn fail_transfer_start(
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    action: &str,
    err: anyhow::Error,
) {
    let err_msg = format!("{err:#}");
    send_transfer_error(
        events,
        tab_id,
        id,
        err_msg.clone(),
        format!("{action} failed: {err_msg}"),
    )
    .await;
}

struct SftpWorkTracker {
    pins: AtomicUsize,
}

impl SftpWorkTracker {
    fn pin(self: &Arc<Self>) -> SftpWorkPin {
        self.pins.fetch_add(1, Ordering::SeqCst);
        SftpWorkPin {
            tracker: self.clone(),
        }
    }

    fn active_pins(&self) -> usize {
        self.pins.load(Ordering::SeqCst)
    }
}

struct SftpWorkPin {
    tracker: Arc<SftpWorkTracker>,
}

impl Drop for SftpWorkPin {
    fn drop(&mut self) {
        let previous = self.tracker.pins.fetch_sub(1, Ordering::SeqCst);
        debug_assert!(previous > 0, "SFTP work pin underflow");
    }
}

pub(crate) struct SftpHandle {
    commands: UnboundedSender<SftpCommand>,
    worker: Arc<SftpWorker>,
}

struct SftpWorker {
    runtime: tokio::runtime::Handle,
    join: Mutex<Option<JoinHandle<()>>>,
    closing: std::sync::atomic::AtomicBool,
    work_tracker: Arc<SftpWorkTracker>,
}

impl Clone for SftpHandle {
    fn clone(&self) -> Self {
        Self {
            commands: self.commands.clone(),
            worker: self.worker.clone(),
        }
    }
}

impl SftpHandle {
    fn send_work_command(&self, build: impl FnOnce(SftpWorkPin) -> SftpCommand) -> bool {
        let pin = self.worker.work_tracker.pin();
        self.commands.send(build(pin)).is_ok()
    }

    pub(crate) fn commands_closed(&self) -> bool {
        self.commands.is_closed()
    }

    pub(crate) fn active_work_pins(&self) -> usize {
        self.worker.work_tracker.active_pins()
    }

    pub(crate) fn list_dir(&self, path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::ListDir { path, pin })
    }

    pub(crate) fn load_more_entries(&self) -> bool {
        self.send_work_command(|pin| SftpCommand::LoadMoreEntries { pin })
    }

    pub(crate) fn reveal_path(&self, path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::RevealPath { path, pin })
    }

    #[allow(dead_code)]
    pub(crate) fn preview(&self, path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::Preview { path, pin })
    }

    pub(crate) fn download(&self, remote: String, local_dir: String) -> bool {
        self.send_work_command(|pin| SftpCommand::Download {
            remote,
            local_dir,
            pin,
        })
    }

    pub(crate) fn upload_paths(&self, locals: Vec<String>, remote_dir: String) -> bool {
        self.send_work_command(|pin| SftpCommand::UploadPaths {
            locals,
            remote_dir,
            pin,
        })
    }

    pub(crate) fn edit_file(&self, remote_path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::EditFile { remote_path, pin })
    }

    pub(crate) fn create_dir(&self, path: String) -> bool {
        self.send_work_command(|pin| SftpCommand::CreateDir { path, pin })
    }

    pub(crate) fn delete_paths(&self, paths: Vec<String>) -> bool {
        self.send_work_command(|pin| SftpCommand::DeletePaths { paths, pin })
    }

    pub(crate) fn close(&self) {
        let _ = self.commands.send(SftpCommand::Close);
        if self.worker.closing.swap(true, Ordering::SeqCst) {
            return;
        }

        let join = self
            .worker
            .join
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .take();
        let Some(mut join) = join else {
            return;
        };

        self.worker.runtime.spawn(async move {
            if tokio::time::timeout(SFTP_SHUTDOWN_TIMEOUT, &mut join)
                .await
                .is_ok()
            {
                return;
            }

            tracing::warn!("[sftp] graceful shutdown timed out; aborting worker");
            join.abort();
            let _ = join.await;
        });
    }

    pub(crate) fn pause_transfer(&self, id: String) {
        let _ = self.commands.send(SftpCommand::PauseTransfer(id));
    }

    pub(crate) fn resume_transfer(&self, id: String) {
        let _ = self.commands.send(SftpCommand::ResumeTransfer(id));
    }

    pub(crate) fn cancel_transfer(&self, id: String) {
        let _ = self.commands.send(SftpCommand::CancelTransfer(id));
    }
}

pub fn spawn_sftp(
    runtime: &tokio::runtime::Handle,
    tab_id: String,
    session: Session,
    events: BackendEventSender,
) -> SftpHandle {
    let (cmd_tx, cmd_rx) = mpsc::unbounded_channel();
    let cmd_tx_clone = cmd_tx.clone();
    let work_tracker = Arc::new(SftpWorkTracker {
        pins: AtomicUsize::new(0),
    });
    let initial_pin = work_tracker.pin();
    let worker_tracker = work_tracker.clone();
    let join = runtime.spawn(async move {
        if let Err(err) = run_sftp(
            tab_id.clone(),
            session,
            cmd_rx,
            cmd_tx_clone,
            events.clone(),
            worker_tracker,
            initial_pin,
        )
        .await
        {
            let _ = events
                .send(BackendEvent::SftpStatus {
                    tab_id: tab_id.clone(),
                    text: format!("sftp error: {err:#}"),
                })
                .await;
            let _ = events
                .send(BackendEvent::Closed {
                    tab_id,
                    reason: format!("sftp error: {err:#}"),
                })
                .await;
        }
    });
    SftpHandle {
        commands: cmd_tx,
        worker: Arc::new(SftpWorker {
            runtime: runtime.clone(),
            join: Mutex::new(Some(join)),
            closing: std::sync::atomic::AtomicBool::new(false),
            work_tracker,
        }),
    }
}

async fn run_sftp(
    tab_id: String,
    session: Session,
    mut commands: UnboundedReceiver<SftpCommand>,
    commands_tx: UnboundedSender<SftpCommand>,
    events: BackendEventSender,
    work_tracker: Arc<SftpWorkTracker>,
    initial_pin: SftpWorkPin,
) -> Result<()> {
    let _ = events
        .send(BackendEvent::SftpStatus {
            tab_id: tab_id.clone(),
            text: t!("sftp_connecting").to_string(),
        })
        .await;

    let session_id = session.id.clone();
    let (handle, connected_mode) = connect_and_authenticate(&session).await?;
    let _ = events
        .send(BackendEvent::SshConnectionModeResolved {
            tab_id: tab_id.clone(),
            session_id,
            mode: connected_mode,
        })
        .await;
    let sftp = open_sftp_session(&handle).await?;

    let home = sftp
        .canonicalize(".")
        .await
        .unwrap_or_else(|_| "/".to_string());

    let _ = events
        .send(BackendEvent::SftpHome {
            tab_id: tab_id.clone(),
            home: home.clone(),
        })
        .await;

    let mut browse_cursor = None;
    open_and_emit_browser_page(&events, &tab_id, &handle, &home, &mut browse_cursor).await?;
    let mut browse_path = home.clone();
    drop(initial_pin);

    let mut active_transfers: std::collections::HashMap<String, TransferStateFlag> =
        std::collections::HashMap::new();
    let mut child_tasks = JoinSet::new();

    loop {
        let command = tokio::select! {
            command = commands.recv() => command,
            result = child_tasks.join_next(), if !child_tasks.is_empty() => {
                if let Some(Err(err)) = result
                    && !err.is_cancelled()
                {
                    tracing::warn!("[sftp] child task failed: {err}");
                }
                continue;
            }
        };
        let Some(command) = command else {
            cancel_sftp_child_tasks(&mut active_transfers, &mut child_tasks).await;
            close_browse_cursor(&mut browse_cursor).await;
            break;
        };
        match command {
            SftpCommand::Close => {
                cancel_sftp_child_tasks(&mut active_transfers, &mut child_tasks).await;
                close_browse_cursor(&mut browse_cursor).await;
                break;
            }
            SftpCommand::PauseTransfer(id) => {
                if let Some(flag) = active_transfers.get(&id) {
                    flag.pause();
                }
            }
            SftpCommand::ResumeTransfer(id) => {
                if let Some(flag) = active_transfers.get(&id) {
                    flag.resume();
                }
            }
            SftpCommand::CancelTransfer(id) => {
                if let Some(flag) = active_transfers.remove(&id) {
                    flag.cancel();
                }
            }
            SftpCommand::TransferFinished(id) => {
                active_transfers.remove(&id);
            }
            SftpCommand::ListDir { path, pin: _pin } => {
                let actual_path = if path == "~" {
                    home.clone()
                } else if let Some(rest) = path.strip_prefix("~/") {
                    crate::sftp::join_remote(&home, rest)
                } else {
                    path
                };

                match open_and_emit_browser_page(
                    &events,
                    &tab_id,
                    &handle,
                    &actual_path,
                    &mut browse_cursor,
                )
                .await
                {
                    Ok(()) => browse_path = actual_path,
                    Err(err) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: format!("list failed: {err:#}"),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::LoadMoreEntries { pin: _pin } => {
                if let Err(err) =
                    emit_next_browser_page(&events, &tab_id, &mut browse_cursor, true).await
                {
                    close_browse_cursor(&mut browse_cursor).await;
                    let _ = emit_browser_page(
                        &events,
                        &tab_id,
                        &browse_path,
                        DirectoryPage {
                            entries: Vec::new(),
                            has_more: false,
                            reached_limit: false,
                        },
                        true,
                    )
                    .await;
                    let _ = events
                        .send(BackendEvent::SftpStatus {
                            tab_id: tab_id.clone(),
                            text: format!("list failed: {err:#}"),
                        })
                        .await;
                }
            }
            SftpCommand::RevealPath { path, pin: _pin } => {
                let actual_path = if path == "~" {
                    home.clone()
                } else if let Some(rest) = path.strip_prefix("~/") {
                    crate::sftp::join_remote(&home, rest)
                } else {
                    path
                };

                match reveal_path_target(&handle, &actual_path).await {
                    Ok(directory) => {
                        match open_and_emit_browser_page(
                            &events,
                            &tab_id,
                            &handle,
                            &directory,
                            &mut browse_cursor,
                        )
                        .await
                        {
                            Ok(()) => browse_path = directory,
                            Err(err) => {
                                let _ = events
                                    .send(BackendEvent::SftpStatus {
                                        tab_id: tab_id.clone(),
                                        text: format!("list failed: {err:#}"),
                                    })
                                    .await;
                            }
                        }
                    }
                    Err(err) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: format!("list failed: {err:#}"),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::Preview { path, pin: _pin } => {
                match preview_impl(&sftp, &handle, &path).await {
                    Ok(preview) => {
                        let _ = events
                            .send(BackendEvent::SftpPreview {
                                tab_id: tab_id.clone(),
                                preview,
                            })
                            .await;
                    }
                    Err(err) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: t!("preview_failed", err = format!("{err:#}")).into(),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::Download {
                remote,
                local_dir,
                pin,
            } => {
                let id = uuid::Uuid::new_v4().to_string();
                let flag = TransferStateFlag::new();
                active_transfers.insert(id.clone(), TransferStateFlag(flag.0.clone()));

                let info = crate::terminal::TransferInfo {
                    id: id.clone(),
                    name: base_name(&remote).to_string(),
                    source: remote.clone(),
                    target: local_dir.clone(),
                    kind: crate::terminal::TransferType::Download,
                    total_bytes: None,
                };
                let _ = events
                    .send(BackendEvent::TransferStarted {
                        tab_id: tab_id.clone(),
                        info,
                    })
                    .await;

                let handle_clone = handle.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();
                let commands_tx_clone = commands_tx.clone();

                child_tasks.spawn(async move {
                    let _transfer_pin = pin;
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            fail_transfer_start(&events_clone, &tab_id_clone, &id, "download", err)
                                .await;
                            let _ = commands_tx_clone.send(SftpCommand::TransferFinished(id));
                            return;
                        }
                    };

                    send_sftp_status(
                        &events_clone,
                        &tab_id_clone,
                        t!("downloading_file", base = base_name(&remote)).to_string(),
                    )
                    .await;

                    match download_path_impl(
                        &handle_clone,
                        &sftp_session,
                        &remote,
                        Path::new(&local_dir),
                        flag,
                        &events_clone,
                        &tab_id_clone,
                        &id,
                    )
                    .await
                    {
                        Ok(summary) => {
                            send_sftp_status(&events_clone, &tab_id_clone, summary).await;
                        }
                        Err(err) => {
                            let err_msg = format!("{err:#}");
                            send_transfer_error(
                                &events_clone,
                                &tab_id_clone,
                                &id,
                                err_msg.clone(),
                                t!("download_failed", err = err_msg).to_string(),
                            )
                            .await;
                        }
                    }
                    let _ = commands_tx_clone.send(SftpCommand::TransferFinished(id));
                });
            }
            SftpCommand::UploadPaths {
                locals,
                remote_dir,
                pin,
            } => {
                let id = uuid::Uuid::new_v4().to_string();
                let flag = TransferStateFlag::new();
                active_transfers.insert(id.clone(), TransferStateFlag(flag.0.clone()));

                let name = if locals.len() == 1 {
                    base_name(&locals[0]).to_string()
                } else {
                    let mut file_count = 0;
                    let mut folder_count = 0;
                    for local in &locals {
                        if std::path::Path::new(local).is_dir() {
                            folder_count += 1;
                        } else {
                            file_count += 1;
                        }
                    }
                    if file_count > 0 && folder_count == 0 {
                        t!("n_files", files = file_count).to_string()
                    } else if file_count == 0 && folder_count > 0 {
                        t!("n_folders", folders = folder_count).to_string()
                    } else {
                        t!(
                            "n_files_and_folders",
                            files = file_count,
                            folders = folder_count
                        )
                        .to_string()
                    }
                };

                let info = crate::terminal::TransferInfo {
                    id: id.clone(),
                    name,
                    source: "local".to_string(),
                    target: remote_dir.clone(),
                    kind: crate::terminal::TransferType::Upload,
                    total_bytes: None,
                };
                let _ = events
                    .send(BackendEvent::TransferStarted {
                        tab_id: tab_id.clone(),
                        info,
                    })
                    .await;

                let handle_clone = handle.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();
                let commands_tx_clone = commands_tx.clone();
                let work_tracker_clone = work_tracker.clone();

                child_tasks.spawn(async move {
                    let _transfer_pin = pin;
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            fail_transfer_start(&events_clone, &tab_id_clone, &id, "upload", err)
                                .await;
                            let _ = commands_tx_clone.send(SftpCommand::TransferFinished(id));
                            return;
                        }
                    };

                    send_sftp_status(&events_clone, &tab_id_clone, t!("uploading").to_string())
                        .await;

                    match upload_paths_impl(
                        &sftp_session,
                        &locals,
                        &remote_dir,
                        flag,
                        &events_clone,
                        &tab_id_clone,
                        &id,
                    )
                    .await
                    {
                        Ok(summary) => {
                            send_sftp_status(&events_clone, &tab_id_clone, summary).await;
                            queue_list_dir(&commands_tx_clone, &work_tracker_clone, remote_dir);
                        }
                        Err(err) => {
                            let err_msg = format!("{err:#}");
                            send_transfer_error(
                                &events_clone,
                                &tab_id_clone,
                                &id,
                                err_msg.clone(),
                                t!("upload_failed", err = err_msg).to_string(),
                            )
                            .await;
                        }
                    }
                    let _ = commands_tx_clone.send(SftpCommand::TransferFinished(id));
                });
            }
            SftpCommand::EditFile { remote_path, pin } => {
                let id = uuid::Uuid::new_v4().to_string();
                let config = crate::session::config::ConfigStore::load()
                    .unwrap_or_else(|_| crate::session::config::ConfigStore::in_memory());
                let tmp_dir = config.tmp_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
                let base = base_name(&remote_path);
                let local_path = tmp_dir.join(format!("{}-{}", id, base));

                let handle_clone = handle.clone();
                let commands_tx_clone = commands_tx.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();
                let work_tracker_clone = work_tracker.clone();

                child_tasks.spawn(async move {
                    let _edit_watcher_pin = pin;
                    let flag = TransferStateFlag::new();
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Edit download failed: {err:#}"),
                                })
                                .await;
                            return;
                        }
                    };

                    let _ = events_clone
                        .send(BackendEvent::SftpStatus {
                            tab_id: tab_id_clone.clone(),
                            text: t!("downloading_file", base = base).to_string(),
                        })
                        .await;

                    if let Err(err) = download_file_impl(
                        &sftp_session,
                        &remote_path,
                        &local_path,
                        &flag,
                        &events_clone,
                        &tab_id_clone,
                        "edit-download",
                    )
                    .await
                    {
                        let _ = events_clone
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id_clone.clone(),
                                text: format!("Edit download failed: {err:#}"),
                            })
                            .await;
                        return;
                    }

                    if let Err(err) = open::that(&local_path) {
                        let _ = events_clone
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id_clone.clone(),
                                text: format!("Failed to open editor: {err:#}"),
                            })
                            .await;
                        return;
                    }

                    use notify::Watcher;
                    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
                    let mut watcher = match notify::recommended_watcher(
                        move |res: notify::Result<notify::Event>| {
                            if let Ok(event) = res {
                                if event.kind.is_modify() {
                                    let _ = tx.send(());
                                }
                            }
                        },
                    ) {
                        Ok(w) => w,
                        Err(err) => {
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Failed to watch local edit file: {err:#}"),
                                })
                                .await;
                            return;
                        }
                    };

                    if let Err(err) =
                        watcher.watch(&local_path, notify::RecursiveMode::NonRecursive)
                    {
                        let _ = events_clone
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id_clone.clone(),
                                text: format!("Failed to watch local edit file: {err:#}"),
                            })
                            .await;
                        return;
                    }

                    while let Some(_) = rx.recv().await {
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        while let Ok(_) = rx.try_recv() {} // drain pending

                        let upload_pin = work_tracker_clone.pin();
                        if commands_tx_clone
                            .send(SftpCommand::UploadEditedFile {
                                local_path: local_path.to_string_lossy().to_string(),
                                remote_path: remote_path.clone(),
                                pin: upload_pin,
                            })
                            .is_err()
                        {
                            break;
                        }
                    }
                });
            }
            SftpCommand::UploadEditedFile {
                local_path,
                remote_path,
                pin,
            } => {
                let handle_clone = handle.clone();
                let events_clone = events.clone();
                let tab_id_clone = tab_id.clone();

                child_tasks.spawn(async move {
                    let _auto_upload_pin = pin;
                    let flag = TransferStateFlag::new();
                    let sftp_session = match open_transfer_sftp_session(&handle_clone).await {
                        Ok(session) => session,
                        Err(err) => {
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Auto-upload failed: {err:#}"),
                                })
                                .await;
                            return;
                        }
                    };

                    let transferred = Arc::new(AtomicU64::new(0));
                    match upload_file_impl(
                        &sftp_session,
                        Path::new(&local_path),
                        &remote_path,
                        &flag,
                        &events_clone,
                        &tab_id_clone,
                        "edit-upload",
                        transferred,
                        None,
                    )
                    .await
                    {
                        Ok(_) => {
                            let now = chrono::Local::now().format("%H:%M:%S");
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!(
                                        "{} ({})",
                                        t!(
                                            "auto_saved_and_uploaded",
                                            base = base_name(&remote_path)
                                        ),
                                        now
                                    ),
                                })
                                .await;
                        }
                        Err(err) => {
                            let _ = events_clone
                                .send(BackendEvent::SftpStatus {
                                    tab_id: tab_id_clone.clone(),
                                    text: format!("Auto-upload failed: {err:#}"),
                                })
                                .await;
                        }
                    }
                });
            }
            SftpCommand::CreateDir { path, pin: _pin } => {
                let actual_path = if path == "~" {
                    home.clone()
                } else if let Some(rest) = path.strip_prefix("~/") {
                    crate::sftp::join_remote(&home, rest)
                } else {
                    path.clone()
                };

                tracing::info!("[sftp] creating directory: '{}'", actual_path);

                match sftp.create_dir(&actual_path).await {
                    Ok(_) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: t!("create_folder_success", name = base_name(&actual_path))
                                    .to_string(),
                            })
                            .await;

                        // Re-fetch the parent directory to show the newly created folder
                        if let Some(parent) = parent_dir(&actual_path) {
                            queue_list_dir(&commands_tx, &work_tracker, parent);
                        } else {
                            queue_list_dir(&commands_tx, &work_tracker, "/".to_string());
                        }
                    }
                    Err(err) => {
                        let _ = events
                            .send(BackendEvent::SftpStatus {
                                tab_id: tab_id.clone(),
                                text: t!("create_folder_failed", err = format!("{err:#}"))
                                    .to_string(),
                            })
                            .await;
                    }
                }
            }
            SftpCommand::DeletePaths { paths, pin: _pin } => {
                tracing::info!("[sftp] batch deleting {} paths", paths.len());
                let _ = events
                    .send(BackendEvent::SftpStatus {
                        tab_id: tab_id.clone(),
                        text: t!("deleting_paths", count = paths.len()).to_string(),
                    })
                    .await;

                let mut errors = Vec::new();
                for path in paths.clone() {
                    let actual_path = if path == "~" {
                        home.clone()
                    } else if let Some(rest) = path.strip_prefix("~/") {
                        crate::sftp::join_remote(&home, rest)
                    } else {
                        path.clone()
                    };

                    if let Err(e) = recursive_delete(&sftp, actual_path).await {
                        errors.push(format!("{path}: {e:#}"));
                    }
                }

                if errors.is_empty() {
                    let _ = events
                        .send(BackendEvent::SftpStatus {
                            tab_id: tab_id.clone(),
                            text: t!("delete_success", count = paths.len()).to_string(),
                        })
                        .await;
                } else {
                    let _ = events
                        .send(BackendEvent::SftpStatus {
                            tab_id: tab_id.clone(),
                            text: t!("delete_failed", err = errors.join(", ")).to_string(),
                        })
                        .await;
                }

                if let Some(first) = paths.first() {
                    let actual_path = if first == "~" {
                        home.clone()
                    } else if let Some(rest) = first.strip_prefix("~/") {
                        crate::sftp::join_remote(&home, rest)
                    } else {
                        first.clone()
                    };
                    if let Some(parent) = parent_dir(&actual_path) {
                        queue_list_dir(&commands_tx, &work_tracker, parent);
                    } else {
                        queue_list_dir(&commands_tx, &work_tracker, "/".to_string());
                    }
                }
            }
        }
    }

    let _ = handle
        .disconnect(Disconnect::ByApplication, "bye", "")
        .await;
    Ok(())
}

fn queue_list_dir(
    commands: &UnboundedSender<SftpCommand>,
    work_tracker: &Arc<SftpWorkTracker>,
    path: String,
) {
    let pin = work_tracker.pin();
    let _ = commands.send(SftpCommand::ListDir { path, pin });
}

async fn cancel_sftp_child_tasks(
    active_transfers: &mut std::collections::HashMap<String, TransferStateFlag>,
    child_tasks: &mut JoinSet<()>,
) {
    for transfer in active_transfers.values() {
        transfer.cancel();
    }
    active_transfers.clear();
    child_tasks.abort_all();
    while child_tasks.join_next().await.is_some() {}
}

#[cfg(test)]
mod lifecycle_tests {
    use std::{collections::HashMap, sync::atomic::Ordering};

    use tokio::task::JoinSet;

    use super::{
        DirectoryListing, RemoteEntry, SFTP_BROWSE_ENTRY_LIMIT, SFTP_BROWSE_NAME_BYTES_LIMIT,
        SFTP_PREVIEW_BYTE_LIMIT, SftpWorkTracker, TransferStateFlag, browser_entry_fits_budget,
        browser_page_state, cancel_sftp_child_tasks, directory_preview_body,
    };

    #[tokio::test]
    async fn closing_worker_cancels_transfers_and_aborts_child_tasks() {
        let transfer = TransferStateFlag::new();
        let transfer_state = transfer.0.clone();
        let mut active_transfers = HashMap::from([("transfer-1".to_string(), transfer)]);
        let mut child_tasks = JoinSet::new();
        child_tasks.spawn(async {
            std::future::pending::<()>().await;
        });

        cancel_sftp_child_tasks(&mut active_transfers, &mut child_tasks).await;

        assert_eq!(transfer_state.load(Ordering::SeqCst), 2);
        assert!(active_transfers.is_empty());
        assert!(child_tasks.is_empty());
    }

    #[test]
    fn work_pins_remain_active_until_the_last_guard_drops() {
        let tracker = std::sync::Arc::new(SftpWorkTracker {
            pins: std::sync::atomic::AtomicUsize::new(0),
        });
        let first = tracker.pin();
        let second = tracker.pin();

        assert_eq!(tracker.active_pins(), 2);
        drop(first);
        assert_eq!(tracker.active_pins(), 1);
        drop(second);
        assert_eq!(tracker.active_pins(), 0);
    }

    #[test]
    fn browser_listing_stops_at_the_entry_limit() {
        assert!(browser_entry_fits_budget(
            SFTP_BROWSE_ENTRY_LIMIT - 1,
            0,
            "/remote",
            "entry",
            SFTP_BROWSE_ENTRY_LIMIT,
            SFTP_BROWSE_NAME_BYTES_LIMIT,
        ));
        assert!(!browser_entry_fits_budget(
            SFTP_BROWSE_ENTRY_LIMIT,
            0,
            "/remote",
            "entry",
            SFTP_BROWSE_ENTRY_LIMIT,
            SFTP_BROWSE_NAME_BYTES_LIMIT,
        ));
    }

    #[test]
    fn browser_listing_stops_at_the_name_byte_limit() {
        let path = "/";
        let name = "entry";
        let exact_bytes = SFTP_BROWSE_NAME_BYTES_LIMIT - path.len() - name.len() - 1;

        assert!(browser_entry_fits_budget(
            0,
            exact_bytes,
            path,
            name,
            SFTP_BROWSE_ENTRY_LIMIT,
            SFTP_BROWSE_NAME_BYTES_LIMIT,
        ));
        assert!(!browser_entry_fits_budget(
            0,
            exact_bytes + 1,
            path,
            name,
            SFTP_BROWSE_ENTRY_LIMIT,
            SFTP_BROWSE_NAME_BYTES_LIMIT,
        ));
    }

    #[test]
    fn browser_page_state_preserves_pending_entries_after_the_cursor_closes() {
        assert_eq!(browser_page_state(3, false, true), (true, true));
        assert_eq!(browser_page_state(0, false, true), (false, true));
        assert_eq!(browser_page_state(0, true, false), (true, false));
        assert_eq!(browser_page_state(0, false, false), (false, false));
    }

    #[test]
    fn directory_preview_has_a_fixed_byte_budget_and_marks_truncation() {
        let listing = DirectoryListing {
            entries: vec![RemoteEntry {
                name: "x".repeat(SFTP_PREVIEW_BYTE_LIMIT),
                full_path: "/remote/x".to_string(),
                is_dir: false,
                size: 0,
                modified: 0,
            }],
            truncated: true,
        };

        let body = directory_preview_body("/remote", listing);

        assert!(body.len() <= SFTP_PREVIEW_BYTE_LIMIT);
        assert!(body.starts_with("Directory: /remote"));
    }

    #[test]
    fn directory_preview_truncates_a_long_utf8_path_without_exceeding_the_limit() {
        let path = "路径".repeat(SFTP_PREVIEW_BYTE_LIMIT);
        let body = directory_preview_body(
            &path,
            DirectoryListing {
                entries: Vec::new(),
                truncated: false,
            },
        );

        assert!(body.len() <= SFTP_PREVIEW_BYTE_LIMIT);
        assert!(body.starts_with("Directory: "));
        assert!(body.contains("..."));
    }
}

async fn open_sftp_session(
    handle: &russh::client::Handle<SftpClientHandler>,
) -> Result<SftpSession> {
    let channel = handle
        .channel_open_session()
        .await
        .context("open sftp channel")?;
    channel
        .request_subsystem(true, "sftp")
        .await
        .context("request sftp subsystem")?;
    SftpSession::new(channel.into_stream())
        .await
        .context("sftp handshake")
}

async fn open_browse_sftp_session(
    handle: &russh::client::Handle<SftpClientHandler>,
) -> Result<RawSftpSession> {
    let channel = handle
        .channel_open_session()
        .await
        .context("open browse sftp channel")?;
    channel
        .request_subsystem(true, "sftp")
        .await
        .context("request browse sftp subsystem")?;
    let session = RawSftpSession::new(channel.into_stream());
    session.set_timeout(SFTP_BROWSE_TIMEOUT.as_secs());
    session.init().await.context("browse sftp handshake")?;
    Ok(session)
}

async fn open_transfer_sftp_session(
    handle: &russh::client::Handle<SftpClientHandler>,
) -> Result<SftpSession> {
    open_sftp_session(handle)
        .await
        .context("open transfer sftp session")
}

use std::future::Future;
use std::pin::Pin;

fn recursive_delete<'a>(
    sftp: &'a SftpSession,
    path: String,
) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>> {
    Box::pin(async move {
        match sftp.read_dir(&path).await {
            Ok(entries) => {
                for entry in entries {
                    let name = entry.file_name();
                    if name == "." || name == ".." {
                        continue;
                    }
                    let child_path = crate::sftp::join_remote(&path, &name);

                    let meta = entry.metadata();
                    let permissions = meta.permissions.unwrap_or(0);
                    let is_dir = (permissions & 0o170_000) == 0o040_000;

                    if is_dir {
                        recursive_delete(sftp, child_path).await?;
                    } else {
                        sftp.remove_file(&child_path)
                            .await
                            .with_context(|| format!("Failed to delete file {child_path}"))?;
                    }
                }
                sftp.remove_dir(&path)
                    .await
                    .with_context(|| format!("Failed to delete dir {path}"))?;
            }
            Err(_) => {
                sftp.remove_file(&path)
                    .await
                    .with_context(|| format!("Failed to delete {path}"))?;
            }
        }
        Ok(())
    })
}

async fn emit_browser_page(
    events: &BackendEventSender,
    tab_id: &str,
    path: &str,
    page: DirectoryPage,
    append: bool,
) -> Result<()> {
    let _ = events
        .send(BackendEvent::SftpEntries {
            tab_id: tab_id.to_string(),
            path: path.to_string(),
            entries: page.entries,
            append,
            has_more: page.has_more,
            reached_limit: page.reached_limit,
        })
        .await;
    let status = if page.reached_limit {
        t!(
            "sftp_directory_truncated",
            entries = SFTP_BROWSE_ENTRY_LIMIT,
            bytes = format_bytes(SFTP_BROWSE_NAME_BYTES_LIMIT as u64)
        )
        .to_string()
    } else {
        path.to_string()
    };
    let _ = events
        .send(BackendEvent::SftpStatus {
            tab_id: tab_id.to_string(),
            text: status,
        })
        .await;
    Ok(())
}

async fn open_and_emit_browser_page(
    events: &BackendEventSender,
    tab_id: &str,
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
    cursor: &mut Option<BrowseCursor>,
) -> Result<()> {
    close_browse_cursor(cursor).await;
    let new_cursor = tokio::time::timeout(SFTP_BROWSE_TIMEOUT, open_browse_cursor(handle, path))
        .await
        .map_err(|_| {
            anyhow!(
                "list directory timed out after {}s: {path}",
                SFTP_BROWSE_TIMEOUT.as_secs()
            )
        })??;
    *cursor = Some(new_cursor);

    if let Err(err) = emit_next_browser_page(events, tab_id, cursor, false).await {
        close_browse_cursor(cursor).await;
        return Err(err);
    }
    Ok(())
}

async fn emit_next_browser_page(
    events: &BackendEventSender,
    tab_id: &str,
    cursor: &mut Option<BrowseCursor>,
    append: bool,
) -> Result<()> {
    let Some(cursor) = cursor.as_mut() else {
        return Err(anyhow!("directory cursor is closed"));
    };
    let path = cursor.path.clone();
    let page = tokio::time::timeout(SFTP_BROWSE_TIMEOUT, read_next_browser_page(cursor))
        .await
        .map_err(|_| {
            anyhow!(
                "list directory timed out after {}s: {path}",
                SFTP_BROWSE_TIMEOUT.as_secs()
            )
        })??;
    emit_browser_page(events, tab_id, &path, page, append).await
}

async fn read_browser_listing_with_timeout(
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
    entry_limit: usize,
    name_bytes_limit: usize,
) -> Result<DirectoryListing> {
    tokio::time::timeout(SFTP_BROWSE_TIMEOUT, async {
        let sftp = open_browse_sftp_session(handle).await?;
        list_dir_for_browser(&sftp, path, entry_limit, name_bytes_limit).await
    })
    .await
    .map_err(|_| {
        anyhow!(
            "list directory timed out after {}s: {path}",
            SFTP_BROWSE_TIMEOUT.as_secs()
        )
    })?
}

async fn open_browse_cursor(
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
) -> Result<BrowseCursor> {
    let sftp = open_browse_sftp_session(handle).await?;
    let directory_handle = sftp
        .opendir(path)
        .await
        .with_context(|| format!("opendir {path}"))?;
    Ok(BrowseCursor {
        sftp: Some(sftp),
        directory_handle: Some(directory_handle.handle),
        path: path.to_string(),
        pending_entries: VecDeque::new(),
        retained_entries: 0,
        retained_name_bytes: 0,
        reached_limit: false,
    })
}

async fn read_next_browser_page(cursor: &mut BrowseCursor) -> Result<DirectoryPage> {
    let mut entries = Vec::with_capacity(SFTP_BROWSE_PAGE_ENTRY_LIMIT);

    while entries.len() < SFTP_BROWSE_PAGE_ENTRY_LIMIT {
        while entries.len() < SFTP_BROWSE_PAGE_ENTRY_LIMIT {
            let Some(entry) = cursor.pending_entries.pop_front() else {
                break;
            };
            entries.push(entry);
        }
        if entries.len() == SFTP_BROWSE_PAGE_ENTRY_LIMIT
            || cursor.reached_limit
            || cursor.directory_handle.is_none()
        {
            break;
        }

        let Some(sftp) = cursor.sftp.as_ref() else {
            break;
        };
        let Some(directory_handle) = cursor.directory_handle.clone() else {
            break;
        };
        let batch = match sftp.readdir(directory_handle).await {
            Ok(batch) => batch,
            Err(SftpClientError::Status(status)) if status.status_code == StatusCode::Eof => {
                close_open_browse_cursor(cursor).await;
                break;
            }
            Err(err) => return Err(err).with_context(|| format!("readdir {} failed", cursor.path)),
        };

        for entry in batch.files {
            if entry.filename == "." || entry.filename == ".." {
                continue;
            }
            if !browser_entry_fits_budget(
                cursor.retained_entries,
                cursor.retained_name_bytes,
                &cursor.path,
                &entry.filename,
                SFTP_BROWSE_ENTRY_LIMIT,
                SFTP_BROWSE_NAME_BYTES_LIMIT,
            ) {
                cursor.reached_limit = true;
                close_open_browse_cursor(cursor).await;
                break;
            }

            cursor.retained_entries += 1;
            cursor.retained_name_bytes += cursor.path.len() + entry.filename.len() + 1;
            let permissions = entry.attrs.permissions.unwrap_or(0);
            cursor.pending_entries.push_back(RemoteEntry {
                full_path: join_remote(&cursor.path, &entry.filename),
                is_dir: (permissions & 0o170_000) == 0o040_000,
                size: entry.attrs.size.unwrap_or(0),
                modified: entry.attrs.mtime.unwrap_or(0),
                name: entry.filename,
            });
        }
    }

    let (has_more, reached_limit) = browser_page_state(
        cursor.pending_entries.len(),
        cursor.directory_handle.is_some(),
        cursor.reached_limit,
    );
    Ok(DirectoryPage {
        entries,
        has_more,
        reached_limit,
    })
}

fn browser_page_state(
    pending_entries: usize,
    directory_open: bool,
    reached_limit: bool,
) -> (bool, bool) {
    (pending_entries > 0 || directory_open, reached_limit)
}

async fn close_browse_cursor(cursor: &mut Option<BrowseCursor>) {
    if let Some(mut cursor) = cursor.take() {
        close_open_browse_cursor(&mut cursor).await;
    }
}

async fn close_open_browse_cursor(cursor: &mut BrowseCursor) {
    if let Some(sftp) = cursor.sftp.as_ref() {
        if let Some(directory_handle) = cursor.directory_handle.take() {
            let _ = tokio::time::timeout(SFTP_SHUTDOWN_TIMEOUT, sftp.close(directory_handle)).await;
        }
        let _ = sftp.close_session();
    }
    cursor.sftp = None;
}

async fn reveal_path_target(
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
) -> Result<String> {
    let sftp = open_sftp_session(handle).await?;
    match sftp.metadata(path).await {
        Ok(metadata) => {
            let is_dir = metadata
                .permissions
                .map(|mode| (mode & 0o170_000) == 0o040_000)
                .unwrap_or(false);
            Ok(reveal_target_directory(
                path,
                if is_dir {
                    RevealPathKind::Directory
                } else {
                    RevealPathKind::File
                },
            ))
        }
        Err(_) => Ok(reveal_target_directory(path, RevealPathKind::Missing)),
    }
}

fn reveal_target_directory(path: &str, kind: RevealPathKind) -> String {
    match kind {
        RevealPathKind::Directory => path.to_string(),
        RevealPathKind::File | RevealPathKind::Missing => {
            parent_dir(path).unwrap_or_else(|| "/".to_string())
        }
    }
}

async fn list_dir_impl(sftp: &SftpSession, path: &str) -> Result<Vec<RemoteEntry>> {
    let raw = sftp
        .read_dir(path)
        .await
        .with_context(|| format!("read_dir {path} failed"))?;

    let mut entries = raw
        .into_iter()
        .filter(|entry| {
            let name = entry.file_name();
            name != "." && name != ".."
        })
        .map(|entry| {
            let name = entry.file_name().to_string();
            let full_path = join_remote(path, &name);
            let meta = entry.metadata();
            let permissions = meta.permissions.unwrap_or(0);
            let is_dir = (permissions & 0o170_000) == 0o040_000;
            let size = meta.size.unwrap_or(0);
            let modified = meta.mtime.unwrap_or(0);
            RemoteEntry {
                name,
                full_path,
                is_dir,
                size,
                modified,
            }
        })
        .collect::<Vec<_>>();

    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(entries)
}

async fn list_dir_for_browser(
    sftp: &RawSftpSession,
    path: &str,
    entry_limit: usize,
    name_bytes_limit: usize,
) -> Result<DirectoryListing> {
    let handle = sftp
        .opendir(path)
        .await
        .with_context(|| format!("opendir {path}"))?;
    let mut entries = Vec::new();
    let mut name_bytes = 0usize;
    let mut truncated = false;

    loop {
        let batch = match sftp.readdir(handle.handle.clone()).await {
            Ok(batch) => batch,
            Err(SftpClientError::Status(status)) if status.status_code == StatusCode::Eof => {
                break;
            }
            Err(err) => {
                let _ = sftp.close(handle.handle.clone()).await;
                let _ = sftp.close_session();
                return Err(err).with_context(|| format!("readdir {path} failed"));
            }
        };

        for entry in batch.files {
            if entry.filename == "." || entry.filename == ".." {
                continue;
            }
            if !browser_entry_fits_budget(
                entries.len(),
                name_bytes,
                path,
                &entry.filename,
                entry_limit,
                name_bytes_limit,
            ) {
                truncated = true;
                break;
            }

            name_bytes += path.len() + entry.filename.len() + 1;
            let permissions = entry.attrs.permissions.unwrap_or(0);
            entries.push(RemoteEntry {
                full_path: join_remote(path, &entry.filename),
                is_dir: (permissions & 0o170_000) == 0o040_000,
                size: entry.attrs.size.unwrap_or(0),
                modified: entry.attrs.mtime.unwrap_or(0),
                name: entry.filename,
            });
        }
        if truncated {
            break;
        }
    }

    let _ = sftp.close(handle.handle).await;
    let _ = sftp.close_session();
    entries.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    Ok(DirectoryListing { entries, truncated })
}

fn browser_entry_fits_budget(
    entry_count: usize,
    name_bytes: usize,
    path: &str,
    name: &str,
    entry_limit: usize,
    name_bytes_limit: usize,
) -> bool {
    entry_count < entry_limit
        && name_bytes.saturating_add(path.len() + name.len() + 1) <= name_bytes_limit
}

async fn preview_impl(
    sftp: &SftpSession,
    handle: &russh::client::Handle<SftpClientHandler>,
    path: &str,
) -> Result<PreviewData> {
    let metadata = sftp
        .metadata(path)
        .await
        .with_context(|| format!("metadata {path}"))?;
    let is_dir = metadata
        .permissions
        .map(|mode| (mode & 0o170_000) == 0o040_000)
        .unwrap_or(false);

    if is_dir {
        let listing = read_browser_listing_with_timeout(
            handle,
            path,
            SFTP_DIRECTORY_PREVIEW_ENTRY_LIMIT,
            SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT,
        )
        .await?;
        let body = directory_preview_body(path, listing);
        return Ok(PreviewData {
            path: path.to_string(),
            title: base_name(path),
            body,
            is_binary: false,
        });
    }

    let mut remote_file = sftp
        .open(path)
        .await
        .with_context(|| format!("open remote {path}"))?;
    let mut buffer = vec![0u8; SFTP_PREVIEW_BYTE_LIMIT];
    let read = remote_file
        .read(&mut buffer)
        .await
        .context("read preview bytes")?;
    buffer.truncate(read);

    let nul_ratio = if buffer.is_empty() {
        0.0
    } else {
        buffer.iter().filter(|byte| **byte == 0).count() as f32 / buffer.len() as f32
    };
    let is_binary = nul_ratio > 0.01;
    let body = if is_binary {
        format!(
            "Binary file\npath: {path}\nsize: {}\npreview: unavailable in-app",
            format_bytes(metadata.size.unwrap_or(0)),
        )
    } else {
        String::from_utf8_lossy(&buffer).into_owned()
    };

    Ok(PreviewData {
        path: path.to_string(),
        title: base_name(path),
        body,
        is_binary,
    })
}

fn directory_preview_body(path: &str, listing: DirectoryListing) -> String {
    let mut body = String::with_capacity(SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT);
    let mut content_truncated = append_preview_text(
        &mut body,
        "Directory: ",
        SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT,
    );
    content_truncated |=
        append_preview_text(&mut body, path, SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT);
    content_truncated |=
        append_preview_text(&mut body, "\n\n", SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT);

    for entry in listing.entries {
        let kind = if entry.is_dir { "dir " } else { "file" };
        let line_len = kind.len() + 2 + entry.name.len() + 1;
        if body.len().saturating_add(line_len) > SFTP_DIRECTORY_PREVIEW_CONTENT_BYTE_LIMIT {
            content_truncated = true;
            break;
        }
        body.push_str(kind);
        body.push_str("  ");
        body.push_str(&entry.name);
        body.push('\n');
    }

    if listing.truncated || content_truncated {
        let notice = t!("sftp_directory_preview_truncated").to_string();
        append_preview_text(&mut body, "\n", SFTP_PREVIEW_BYTE_LIMIT);
        append_preview_text(&mut body, &notice, SFTP_PREVIEW_BYTE_LIMIT);
    }
    body
}

fn append_preview_text(body: &mut String, text: &str, byte_limit: usize) -> bool {
    let remaining = byte_limit.saturating_sub(body.len());
    if text.len() <= remaining {
        body.push_str(text);
        return false;
    }

    let suffix = if remaining >= 3 { "..." } else { "" };
    let prefix_len = remaining.saturating_sub(suffix.len());
    let prefix_end = text
        .char_indices()
        .map(|(index, character)| index + character.len_utf8())
        .take_while(|end| *end <= prefix_len)
        .last()
        .unwrap_or(0);
    body.push_str(&text[..prefix_end]);
    body.push_str(suffix);
    true
}

async fn download_path_impl(
    handle: &russh::client::Handle<SftpClientHandler>,
    sftp: &SftpSession,
    remote: &str,
    local_dir: &Path,
    flag: TransferStateFlag,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
) -> Result<String> {
    tokio::fs::create_dir_all(local_dir)
        .await
        .with_context(|| format!("create {}", local_dir.display()))?;

    // Check for cancellation after initial setup
    let state = flag.0.load(Ordering::SeqCst);
    if state == 2 {
        return Err(anyhow::anyhow!("transfer cancelled"));
    }

    let metadata = sftp
        .metadata(remote)
        .await
        .with_context(|| format!("metadata {remote}"))?;
    let is_dir = metadata
        .permissions
        .map(|mode| (mode & 0o170_000) == 0o040_000)
        .unwrap_or(false);

    if is_dir {
        let local_archive = local_dir.join(format!(
            ".ax_shell-{}-{}.tar.gz",
            base_name(remote),
            Uuid::new_v4()
        ));
        let extracted_to = download_remote_directory_archive(
            handle,
            sftp,
            remote,
            &local_archive,
            &flag,
            events,
            tab_id,
            id,
        )
        .await?;
        return Ok(t!("downloaded_folder", path = extracted_to.display()).to_string());
    }

    let local_path = local_dir.join(base_name(remote));
    download_file_impl(sftp, remote, &local_path, &flag, events, tab_id, id).await?;
    Ok(t!("downloaded_file", path = local_path.display()).to_string())
}

#[allow(dead_code)]
async fn download_dir_recursive(
    sftp: &SftpSession,
    remote_dir: &str,
    local_dir: &Path,
    flag: &TransferStateFlag,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
) -> Result<()> {
    tokio::fs::create_dir_all(local_dir)
        .await
        .with_context(|| format!("create {}", local_dir.display()))?;
    let entries = list_dir_impl(sftp, remote_dir).await?;
    for entry in entries {
        let local_path = local_dir.join(&entry.name);
        if entry.is_dir {
            Box::pin(download_dir_recursive(
                sftp,
                &entry.full_path,
                &local_path,
                flag,
                events,
                tab_id,
                id,
            ))
            .await?;
        } else {
            download_file_impl(
                sftp,
                &entry.full_path,
                &local_path,
                flag,
                events,
                tab_id,
                id,
            )
            .await?;
            let _ = maybe_extract_archive(&local_path).await;
        }
    }
    Ok(())
}

async fn download_remote_directory_archive(
    handle: &russh::client::Handle<SftpClientHandler>,
    sftp: &SftpSession,
    remote_dir: &str,
    local_archive: &Path,
    flag: &TransferStateFlag,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
) -> Result<PathBuf> {
    let remote_archive = format!(
        "/tmp/ax_shell-{}-{}.tar.gz",
        base_name(remote_dir),
        Uuid::new_v4()
    );

    // Check for cancellation before creating remote archive
    let state = flag.0.load(Ordering::SeqCst);
    if state == 2 {
        return Err(anyhow::anyhow!("transfer cancelled"));
    }

    create_remote_archive(handle, remote_dir, &remote_archive).await?;

    let local_extract_root = local_archive
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(base_name(remote_dir));

    let archive_download = async {
        download_file_impl(
            sftp,
            &remote_archive,
            local_archive,
            flag,
            events,
            tab_id,
            id,
        )
        .await?;
        extract_archive_to(
            local_archive,
            local_archive.parent().unwrap_or_else(|| Path::new(".")),
        )
        .await?;
        tokio::fs::remove_file(local_archive)
            .await
            .with_context(|| format!("remove {}", local_archive.display()))?;
        Ok::<PathBuf, anyhow::Error>(local_extract_root)
    }
    .await;

    let cleanup_result = remove_remote_path(handle, &remote_archive).await;

    let extracted_to = archive_download?;
    if let Err(err) = cleanup_result {
        tracing::warn!("failed to clean remote archive {remote_archive}: {err:#}");
    }

    Ok(extracted_to)
}

async fn download_file_impl(
    sftp: &SftpSession,
    remote: &str,
    local: &Path,
    flag: &TransferStateFlag,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
) -> Result<()> {
    let mut remote_file = sftp
        .open(remote)
        .await
        .with_context(|| format!("open remote {remote}"))?;
    let mut local_file = tokio::fs::File::create(local)
        .await
        .with_context(|| format!("create local {}", local.display()))?;

    let total = sftp.metadata(remote).await.ok().and_then(|m| m.size);
    let mut transferred = 0u64;

    let mut buffer = vec![0u8; 128 * 1024];
    loop {
        flag.yield_if_paused(events, tab_id, id, transferred, total)
            .await?;
        let read = remote_file
            .read(&mut buffer)
            .await
            .context("read remote file")?;
        if read == 0 {
            break;
        }
        local_file
            .write_all(&buffer[..read])
            .await
            .with_context(|| format!("write {}", local.display()))?;

        transferred += read as u64;
        send_transfer_progress(
            events,
            tab_id,
            id,
            transferred,
            total,
            TransferState::Running,
        )
        .await;
    }
    local_file.flush().await.context("flush local file")?;

    send_transfer_progress(
        events,
        tab_id,
        id,
        transferred,
        total,
        TransferState::Completed,
    )
    .await;

    Ok(())
}

async fn upload_paths_impl(
    sftp: &SftpSession,
    locals: &[String],
    remote_dir: &str,
    flag: TransferStateFlag,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
) -> Result<String> {
    // Check for cancellation before starting
    let state = flag.0.load(Ordering::SeqCst);
    if state == 2 {
        return Err(anyhow::anyhow!("transfer cancelled"));
    }

    create_remote_dir_all(sftp, remote_dir).await?;
    let mut file_count = 0usize;
    let mut folder_count = 0usize;

    let mut total_bytes = 0u64;
    let mut files_to_upload = Vec::new();
    let mut dirs_to_create = Vec::new();

    for local in locals {
        let p = PathBuf::from(local);
        if p.is_dir() {
            folder_count += 1;
            let root_name = p.file_name().and_then(|n| n.to_str()).unwrap_or("folder");
            let remote_root = join_remote(remote_dir, root_name);
            dirs_to_create.push(remote_root.clone());

            for entry in WalkDir::new(&p) {
                let entry = entry?;
                let path = entry.path();
                if path == p {
                    continue;
                }

                if let Ok(meta) = tokio::fs::metadata(&path).await {
                    let relative = path.strip_prefix(&p)?;
                    let remote_path = if relative.as_os_str().is_empty() {
                        remote_root.clone()
                    } else {
                        let rel = relative
                            .components()
                            .map(|c| c.as_os_str().to_string_lossy().to_string())
                            .collect::<Vec<_>>()
                            .join("/");
                        join_remote(&remote_root, &rel)
                    };

                    if path.is_dir() {
                        dirs_to_create.push(remote_path);
                    } else {
                        total_bytes += meta.len();
                        files_to_upload.push((path.to_path_buf(), remote_path));
                    }
                }
            }
        } else if let Ok(meta) = tokio::fs::metadata(&p).await {
            total_bytes += meta.len();
            let file_name = p.file_name().and_then(|n| n.to_str()).unwrap_or("file");
            files_to_upload.push((p.clone(), join_remote(remote_dir, file_name)));
            file_count += 1;
        }
    }

    // Check for cancellation before creating directories
    let state = flag.0.load(Ordering::SeqCst);
    if state == 2 {
        return Err(anyhow::anyhow!("transfer cancelled"));
    }

    // Create directories sequentially first
    for dir in dirs_to_create {
        // Check for cancellation between each directory creation
        let state = flag.0.load(Ordering::SeqCst);
        if state == 2 {
            return Err(anyhow::anyhow!("transfer cancelled"));
        }
        create_remote_dir_all(sftp, &dir).await?;
    }

    let transferred = Arc::new(AtomicU64::new(0));
    let mut futures = Vec::new();

    for (local_path, remote_path) in files_to_upload {
        let flag_clone = TransferStateFlag(Arc::clone(&flag.0));
        let events_clone = events.clone();
        let tab_id_clone = tab_id.to_string();
        let id_clone = id.to_string();
        let transferred_clone = Arc::clone(&transferred);

        futures.push(async move {
            upload_file_impl(
                sftp,
                &local_path,
                &remote_path,
                &flag_clone,
                &events_clone,
                &tab_id_clone,
                &id_clone,
                transferred_clone,
                Some(total_bytes),
            )
            .await
        });
    }

    use futures::StreamExt as _;
    let mut stream = futures::stream::iter(futures).buffer_unordered(4);
    while let Some(res) = stream.next().await {
        res?;
    }

    send_transfer_progress(
        events,
        tab_id,
        id,
        total_bytes,
        Some(total_bytes),
        TransferState::Completed,
    )
    .await;

    let summary = if file_count == 1 && folder_count == 0 {
        t!("uploaded_file").to_string()
    } else if file_count == 0 && folder_count == 1 {
        t!("uploaded_folder").to_string()
    } else if file_count > 0 && folder_count == 0 {
        t!("uploaded_n_files", files = file_count).to_string()
    } else if file_count == 0 && folder_count > 0 {
        t!("uploaded_n_folders", folders = folder_count).to_string()
    } else {
        t!(
            "uploaded_files_and_folders",
            files = file_count,
            folders = folder_count
        )
        .to_string()
    };
    Ok(summary)
}

async fn upload_file_impl(
    sftp: &SftpSession,
    local_file: &Path,
    remote_path: &str,
    flag: &TransferStateFlag,
    events: &BackendEventSender,
    tab_id: &str,
    id: &str,
    transferred: Arc<AtomicU64>,
    total: Option<u64>,
) -> Result<()> {
    let mut local = tokio::fs::File::open(local_file)
        .await
        .with_context(|| format!("open local {}", local_file.display()))?;
    let mut remote = sftp
        .create(remote_path)
        .await
        .with_context(|| format!("create remote {remote_path}"))?;

    let mut buffer = vec![0u8; 128 * 1024];
    loop {
        let cur = transferred.load(Ordering::Relaxed);
        flag.yield_if_paused(events, tab_id, id, cur, total).await?;
        let read = local.read(&mut buffer).await.context("read local file")?;
        if read == 0 {
            break;
        }
        remote
            .write_all(&buffer[..read])
            .await
            .with_context(|| format!("write remote {remote_path}"))?;

        let new_cur = transferred.fetch_add(read as u64, Ordering::Relaxed) + read as u64;
        send_transfer_progress(events, tab_id, id, new_cur, total, TransferState::Running).await;
    }
    remote.flush().await.context("flush remote file")?;
    Ok(())
}

async fn create_remote_dir_all(sftp: &SftpSession, remote_dir: &str) -> Result<()> {
    if remote_dir.is_empty() || remote_dir == "/" {
        return Ok(());
    }

    let mut current = String::from("/");
    for segment in remote_dir.split('/').filter(|segment| !segment.is_empty()) {
        current = join_remote(&current, segment);
        let _ = sftp.create_dir(&current).await;
    }
    Ok(())
}

async fn create_remote_archive(
    handle: &russh::client::Handle<SftpClientHandler>,
    remote_dir: &str,
    remote_archive: &str,
) -> Result<()> {
    let remote_dir = remote_dir.trim_end_matches('/');
    let parent = remote_parent(remote_dir);
    let name = base_name(remote_dir);
    let command = format!(
        "tar -C {} -czf {} {}",
        shell_quote(&parent),
        shell_quote(remote_archive),
        shell_quote(&name),
    );
    exec_remote_command(handle, &command)
        .await
        .with_context(|| format!("archive remote directory {remote_dir}"))?;
    Ok(())
}

async fn remove_remote_path(
    handle: &russh::client::Handle<SftpClientHandler>,
    remote_path: &str,
) -> Result<()> {
    let command = format!("rm -f {}", shell_quote(remote_path));
    exec_remote_command(handle, &command)
        .await
        .with_context(|| format!("remove remote temporary file {remote_path}"))?;
    Ok(())
}

async fn exec_remote_command(
    handle: &russh::client::Handle<SftpClientHandler>,
    command: &str,
) -> Result<()> {
    let mut channel = handle
        .channel_open_session()
        .await
        .context("open remote exec session")?;
    channel
        .exec(true, command)
        .await
        .with_context(|| format!("exec remote command: {command}"))?;

    let mut stderr = Vec::new();
    let mut stdout = Vec::new();
    let mut exit_status = None;

    // Add timeout to prevent indefinite blocking (300 seconds = 5 minutes)
    let timeout = tokio::time::Duration::from_secs(300);
    let result = tokio::time::timeout(timeout, async {
        loop {
            // Yield to allow cancellation
            tokio::task::yield_now().await;

            if let Some(msg) = channel.wait().await {
                match msg {
                    russh::ChannelMsg::Data { data } => stdout.extend_from_slice(&data),
                    russh::ChannelMsg::ExtendedData { data, .. } => stderr.extend_from_slice(&data),
                    russh::ChannelMsg::ExitStatus { exit_status: code } => exit_status = Some(code),
                    russh::ChannelMsg::Close => break,
                    _ => {}
                }
            } else {
                break;
            }
        }
    })
    .await;

    if result.is_err() {
        return Err(anyhow!("remote command timeout: {command}"));
    }

    match exit_status.unwrap_or(0) {
        0 => Ok(()),
        code => {
            let stderr = String::from_utf8_lossy(&stderr).trim().to_string();
            let stdout = String::from_utf8_lossy(&stdout).trim().to_string();
            Err(anyhow!(
                "remote command exited with {code}: {}",
                if !stderr.is_empty() { stderr } else { stdout }
            ))
        }
    }
}

#[allow(dead_code)]
async fn maybe_extract_archive(path: &Path) -> Result<Option<PathBuf>> {
    let Some(file_name) = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
    else {
        return Ok(None);
    };
    let is_archive = [".zip", ".tar", ".tar.gz", ".tgz"]
        .iter()
        .any(|suffix| file_name.ends_with(suffix));
    if !is_archive {
        return Ok(None);
    }

    let extract_root = path
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join(strip_archive_suffix(&file_name));
    let archive_path = path.to_path_buf();
    let target_dir = extract_root.clone();

    tokio::task::spawn_blocking(move || -> Result<()> {
        fs::create_dir_all(&target_dir)
            .with_context(|| format!("create {}", target_dir.display()))?;

        if file_name.ends_with(".zip") {
            let file = fs::File::open(&archive_path)
                .with_context(|| format!("open {}", archive_path.display()))?;
            let mut zip = ZipArchive::new(file).context("read zip archive")?;
            for index in 0..zip.len() {
                let mut entry = zip.by_index(index).context("read zip entry")?;
                let Some(name) = entry.enclosed_name().map(|name| name.to_path_buf()) else {
                    continue;
                };
                let output = target_dir.join(name);
                if entry.is_dir() {
                    fs::create_dir_all(&output)?;
                } else {
                    if let Some(parent) = output.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let mut output_file = fs::File::create(&output)?;
                    std::io::copy(&mut entry, &mut output_file)?;
                }
            }
        } else if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
            let file = fs::File::open(&archive_path)
                .with_context(|| format!("open {}", archive_path.display()))?;
            let decoder = GzDecoder::new(file);
            let mut archive = tar::Archive::new(decoder);
            archive
                .unpack(&target_dir)
                .context("unpack tar.gz archive")?;
        } else if file_name.ends_with(".tar") {
            let file = fs::File::open(&archive_path)
                .with_context(|| format!("open {}", archive_path.display()))?;
            let mut archive = tar::Archive::new(file);
            archive.unpack(&target_dir).context("unpack tar archive")?;
        }

        Ok(())
    })
    .await
    .context("extract archive task join failure")??;

    Ok(Some(extract_root))
}

async fn extract_archive_to(path: &Path, target_dir: &Path) -> Result<()> {
    let Some(file_name) = path
        .file_name()
        .and_then(|name| name.to_str())
        .map(|name| name.to_string())
    else {
        return Ok(());
    };
    let archive_path = path.to_path_buf();
    let target_dir = target_dir.to_path_buf();

    tokio::task::spawn_blocking(move || -> Result<()> {
        fs::create_dir_all(&target_dir)
            .with_context(|| format!("create {}", target_dir.display()))?;

        if file_name.ends_with(".zip") {
            let file = fs::File::open(&archive_path)
                .with_context(|| format!("open {}", archive_path.display()))?;
            let mut zip = ZipArchive::new(file).context("read zip archive")?;
            for index in 0..zip.len() {
                let mut entry = zip.by_index(index).context("read zip entry")?;
                let Some(name) = entry.enclosed_name().map(|name| name.to_path_buf()) else {
                    continue;
                };
                let output = target_dir.join(name);
                if entry.is_dir() {
                    fs::create_dir_all(&output)?;
                } else {
                    if let Some(parent) = output.parent() {
                        fs::create_dir_all(parent)?;
                    }
                    let mut output_file = fs::File::create(&output)?;
                    std::io::copy(&mut entry, &mut output_file)?;
                }
            }
        } else if file_name.ends_with(".tar.gz") || file_name.ends_with(".tgz") {
            let file = fs::File::open(&archive_path)
                .with_context(|| format!("open {}", archive_path.display()))?;
            let decoder = GzDecoder::new(file);
            let mut archive = tar::Archive::new(decoder);
            archive
                .unpack(&target_dir)
                .context("unpack tar.gz archive")?;
        } else if file_name.ends_with(".tar") {
            let file = fs::File::open(&archive_path)
                .with_context(|| format!("open {}", archive_path.display()))?;
            let mut archive = tar::Archive::new(file);
            archive.unpack(&target_dir).context("unpack tar archive")?;
        }

        Ok(())
    })
    .await
    .context("extract archive task join failure")??;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{RevealPathKind, reveal_target_directory};

    #[test]
    fn reveal_target_directory_enters_existing_directory() {
        assert_eq!(
            reveal_target_directory("/srv/app/logs", RevealPathKind::Directory),
            "/srv/app/logs"
        );
    }

    #[test]
    fn reveal_target_directory_falls_back_for_file_or_missing_target() {
        assert_eq!(
            reveal_target_directory("/srv/app/logs/output.log", RevealPathKind::File),
            "/srv/app/logs"
        );
        assert_eq!(
            reveal_target_directory("/srv/app/missing/output.log", RevealPathKind::Missing),
            "/srv/app/missing"
        );
    }
}
