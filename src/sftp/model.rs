#[derive(Debug, Clone)]
pub struct RemoteEntry {
    pub name: String,
    pub full_path: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SftpOverwriteDecision {
    Skip,
    Replace,
    ReplaceAllInTransfer,
}

#[derive(Debug)]
pub(crate) struct SftpOverwriteRequest {
    pub(crate) tab_id: String,
    pub(crate) transfer_id: String,
    pub(crate) remote_path: String,
    pub(crate) local_path: String,
    pub(crate) response: tokio::sync::oneshot::Sender<SftpOverwriteDecision>,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct PreviewData {
    pub path: String,
    pub title: String,
    pub body: String,
    pub is_binary: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransferType {
    Upload,
    Download,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
pub enum TransferState {
    Running,
    Paused,
    Completed,
    Failed(String),
    Interrupted(String),
    Zombie(String),
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
enum TransferStateCompat {
    Running,
    Paused,
    Completed,
    Failed(String),
    Interrupted(String),
    Zombie(String),
    Cancelled,
}

impl From<TransferStateCompat> for TransferState {
    fn from(value: TransferStateCompat) -> Self {
        match value {
            TransferStateCompat::Running => Self::Running,
            TransferStateCompat::Paused => Self::Paused,
            TransferStateCompat::Completed => Self::Completed,
            TransferStateCompat::Failed(reason) => Self::Failed(reason),
            TransferStateCompat::Interrupted(reason) => Self::Interrupted(reason),
            TransferStateCompat::Zombie(reason) => Self::Zombie(reason),
            TransferStateCompat::Cancelled => Self::Interrupted("Cancelled".to_string()),
        }
    }
}

impl<'de> serde::Deserialize<'de> for TransferState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        TransferStateCompat::deserialize(deserializer).map(Into::into)
    }
}

impl TransferState {
    pub(crate) fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Failed(_) | Self::Interrupted(_) | Self::Zombie(_)
        )
    }
}

pub(crate) fn unix_timestamp_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TransferInfo {
    pub id: String,
    pub name: String,
    pub source: String,
    pub target: String,
    pub kind: TransferType,
    pub total_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum TransferFileState {
    Running,
    Paused,
    Completed,
    Skipped,
    Failed(String),
    Interrupted(String),
}

impl TransferFileState {
    pub(crate) fn is_terminal(&self) -> bool {
        matches!(
            self,
            Self::Completed | Self::Skipped | Self::Failed(_) | Self::Interrupted(_)
        )
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct TransferFile {
    pub id: String,
    pub source: String,
    pub target: String,
    pub total_bytes: Option<u64>,
    pub state: TransferFileState,
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Transfer {
    pub tab_id: String,
    pub tab_title: String,
    pub info: TransferInfo,
    pub transferred: u64,
    pub total: Option<u64>,
    pub state: TransferState,
    #[serde(default)]
    pub started_at: u64,
    #[serde(default)]
    pub finished_at: Option<u64>,
    #[serde(default)]
    pub files: Vec<TransferFile>,
}

#[cfg(test)]
mod tests {
    use super::Transfer;

    #[test]
    fn transfer_history_without_timestamps_stays_compatible() {
        let transfer: Transfer = serde_json::from_str(
            r#"{
                "tab_id":"group-a",
                "tab_title":"Session A",
                "info":{
                    "id":"transfer-a",
                    "name":"report.csv",
                    "source":"/remote/report.csv",
                    "target":"/local",
                    "kind":"Download",
                    "total_bytes":128
                },
                "transferred":128,
                "total":128,
                "state":"Completed"
            }"#,
        )
        .expect("legacy transfer history should deserialize");

        assert_eq!(transfer.started_at, 0);
        assert_eq!(transfer.finished_at, None);
        assert!(transfer.files.is_empty());
    }

    #[test]
    fn transfer_file_state_marks_terminal_entries() {
        use super::TransferFileState;

        assert!(!TransferFileState::Running.is_terminal());
        assert!(!TransferFileState::Paused.is_terminal());
        assert!(TransferFileState::Completed.is_terminal());
        assert!(TransferFileState::Skipped.is_terminal());
        assert!(TransferFileState::Failed("failed".to_string()).is_terminal());
        assert!(TransferFileState::Interrupted("cancelled".to_string()).is_terminal());
    }

    #[test]
    fn transfer_history_persists_download_file_details() {
        let transfer: Transfer = serde_json::from_str(
            r#"{
                "tab_id":"group-a",
                "tab_title":"Session A",
                "info":{
                    "id":"transfer-a",
                    "name":"2 files",
                    "source":"remote",
                    "target":"/local",
                    "kind":"Download",
                    "total_bytes":null
                },
                "transferred":0,
                "total":null,
                "state":"Running",
                "files":[{
                    "id":"/remote/report.csv",
                    "source":"/remote/report.csv",
                    "target":"/local/report.csv",
                    "total_bytes":128,
                    "state":"Completed"
                }]
            }"#,
        )
        .expect("transfer history should deserialize file details");

        assert_eq!(transfer.files.len(), 1);
        assert!(transfer.files[0].state.is_terminal());
        assert_eq!(
            serde_json::to_value(&transfer)
                .expect("transfer history should serialize")
                .get("files")
                .and_then(serde_json::Value::as_array)
                .map(Vec::len),
            Some(1)
        );
    }
}
use std::time::{SystemTime, UNIX_EPOCH};
