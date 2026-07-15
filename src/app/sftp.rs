use std::collections::HashSet;

use gpui::{Pixels, Point};

use crate::sftp::{PreviewData, RemoteEntry};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SftpSortColumn {
    Name,
    Size,
    Modified,
}

impl Default for SftpSortColumn {
    fn default() -> Self {
        Self::Name
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SortDirection {
    Asc,
    Desc,
}

impl Default for SortDirection {
    fn default() -> Self {
        Self::Asc
    }
}

impl SortDirection {
    pub(crate) fn toggled(self) -> Self {
        match self {
            Self::Asc => Self::Desc,
            Self::Desc => Self::Asc,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SftpTransferTab {
    Active,
    Failed,
    Completed,
}

impl Default for SftpTransferTab {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Clone, Default)]
pub(crate) struct SftpUiState {
    pub(crate) current_path: String,
    pub(crate) status: String,
    pub(crate) entries: Vec<RemoteEntry>,
    pub(crate) has_more_entries: bool,
    pub(crate) loading_more_entries: bool,
    pub(crate) reached_entries_limit: bool,
    pub(crate) selected_path: Option<String>,
    pub(crate) preview: Option<PreviewData>,
    pub(crate) selected_entries: HashSet<String>,
    pub(crate) home_dir: String,
    /// A system resume can invalidate the server-side SFTP handle. It is
    /// recreated only when the user next needs this idle connection.
    pub(crate) connection_may_be_stale: bool,
}

#[derive(Clone, Debug)]
pub(crate) struct LocalFileEntry {
    pub(crate) name: String,
    pub(crate) full_path: String,
    pub(crate) is_dir: bool,
    pub(crate) size: u64,
    pub(crate) modified: u32,
}

#[derive(Clone, Default)]
pub(crate) struct LocalFileBrowserState {
    pub(crate) current_path: String,
    pub(crate) status: String,
    pub(crate) entries: Vec<LocalFileEntry>,
    pub(crate) selected_path: Option<String>,
    pub(crate) selected_entries: HashSet<String>,
}

#[derive(Clone)]
pub(crate) enum SftpContextMenuTarget {
    Remote { path: String, is_dir: bool },
    RemoteDirectory,
    Local { path: String, is_dir: bool },
}

#[derive(Clone)]
pub(crate) struct SftpContextMenuState {
    pub(crate) target: SftpContextMenuTarget,
    pub(crate) position: Point<Pixels>,
}

#[derive(Clone)]
pub(crate) struct SftpTransferContextMenuState {
    pub(crate) group_id: String,
    pub(crate) transfer_id: String,
    pub(crate) position: Point<Pixels>,
}
