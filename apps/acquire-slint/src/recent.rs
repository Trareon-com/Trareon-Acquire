//! Recently completed acquisitions for the local shell.

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const MAX_RECENT: usize = 10;

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct RecentEntry {
    pub case_id: String,
    pub package_path: String,
    pub completed_at: String,
}

impl RecentEntry {
    pub fn completed(case_id: impl Into<String>, package_path: impl Into<String>) -> Self {
        Self {
            case_id: case_id.into(),
            package_path: package_path.into(),
            completed_at: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
                .to_string(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.package_path.trim().is_empty()
    }
}

pub fn append(entry: RecentEntry) {
    if let Some(path) = default_path() {
        let _ = append_at(path, entry);
    }
}

pub fn list() -> Vec<RecentEntry> {
    default_path()
        .and_then(|path| list_at(path).ok())
        .unwrap_or_default()
}

pub fn append_at(path: impl AsRef<Path>, entry: RecentEntry) -> Result<(), String> {
    if entry.is_empty() {
        return Ok(());
    }
    let path = path.as_ref();
    let mut entries = list_at(path)?;
    entries.push(entry);
    if entries.len() > MAX_RECENT {
        entries.drain(..entries.len() - MAX_RECENT);
    }
    write_entries(path, &entries)
}

pub fn list_at(path: impl AsRef<Path>) -> Result<Vec<RecentEntry>, String> {
    let path = path.as_ref();
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let body = std::fs::read_to_string(path).map_err(io_error)?;
    let entries: Vec<RecentEntry> = serde_json::from_str(&body).map_err(json_error)?;
    Ok(entries
        .into_iter()
        .filter(|entry| !entry.is_empty())
        .collect())
}

fn write_entries(path: &Path, entries: &[RecentEntry]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(io_error)?;
    }
    let body = serde_json::to_vec_pretty(entries).map_err(json_error)?;
    std::fs::write(path, body).map_err(io_error)
}

fn default_path() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".trareon").join("acquire-recent.json"))
}

fn io_error(error: std::io::Error) -> String {
    error.to_string()
}

fn json_error(error: serde_json::Error) -> String {
    error.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recent_entries_keep_only_latest_ten_and_hide_empty() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("recent.json");
        for i in 0..11 {
            append_at(
                &path,
                RecentEntry {
                    case_id: format!("CASE-{i}"),
                    package_path: format!("/tmp/{i}.fsnap"),
                    completed_at: i.to_string(),
                },
            )
            .unwrap();
        }
        append_at(&path, RecentEntry::default()).unwrap();

        let entries = list_at(&path).unwrap();
        assert_eq!(entries.len(), 10);
        assert_eq!(entries[0].case_id, "CASE-1");
        assert_eq!(entries.last().unwrap().case_id, "CASE-10");
    }
}
