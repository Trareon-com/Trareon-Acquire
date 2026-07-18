//! Local case records for the Acquire shell.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaseState {
    Identify,
    Acquire,
    Preserve,
    Archived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CaseRecord {
    pub id: String,
    pub title: String,
    pub examiner: String,
    pub state: CaseState,
    pub created_at: String,
    pub updated_at: String,
    pub notes: String,
}

#[derive(Debug, Clone)]
pub struct CaseStore {
    root: PathBuf,
}

impl CaseStore {
    pub fn local() -> Option<Self> {
        local_root().map(Self::at)
    }

    pub fn at(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn list(&self) -> Result<Vec<CaseRecord>, String> {
        if !self.root.exists() {
            return Ok(Vec::new());
        }
        let mut cases: Vec<CaseRecord> = Vec::new();
        for entry in std::fs::read_dir(&self.root).map_err(io_error)? {
            let path = entry.map_err(io_error)?.path();
            if path.extension().and_then(|value| value.to_str()) != Some("json") {
                continue;
            }
            let body = std::fs::read_to_string(path).map_err(io_error)?;
            cases.push(serde_json::from_str(&body).map_err(json_error)?);
        }
        cases.sort_by(|left, right| right.updated_at.cmp(&left.updated_at));
        Ok(cases)
    }

    pub fn create(
        &self,
        title: impl Into<String>,
        examiner: impl Into<String>,
        notes: impl Into<String>,
    ) -> Result<CaseRecord, String> {
        std::fs::create_dir_all(&self.root).map_err(io_error)?;
        let timestamp = now();
        let record = CaseRecord {
            id: self.next_id(&timestamp),
            title: title.into(),
            examiner: examiner.into(),
            state: CaseState::Identify,
            created_at: timestamp.clone(),
            updated_at: timestamp,
            notes: notes.into(),
        };
        self.write(&record)?;
        Ok(record)
    }

    pub fn open(&self, id: &str) -> Result<Option<CaseRecord>, String> {
        let path = self.path_for(id)?;
        if !path.is_file() {
            return Ok(None);
        }
        let body = std::fs::read_to_string(path).map_err(io_error)?;
        serde_json::from_str(&body).map(Some).map_err(json_error)
    }

    pub fn archive(&self, id: &str) -> Result<CaseRecord, String> {
        let mut record = self
            .open(id)?
            .ok_or_else(|| format!("case not found: {id}"))?;
        record.state = CaseState::Archived;
        record.updated_at = now();
        self.write(&record)?;
        Ok(record)
    }

    fn next_id(&self, timestamp: &str) -> String {
        let base = format!("CASE-{}", timestamp.replace(['-', ':', 'T', 'Z'], ""));
        let mut id = base.clone();
        let mut suffix = 1;
        while self.root.join(format!("{id}.json")).exists() {
            suffix += 1;
            id = format!("{base}-{suffix}");
        }
        id
    }

    pub fn path_for(&self, id: &str) -> Result<PathBuf, String> {
        let id = require_case_id(id)?;
        if id.contains('/') || id.contains('\\') {
            return Err("case id must not contain path separators".into());
        }
        Ok(self.root.join(format!("{id}.json")))
    }

    fn write(&self, record: &CaseRecord) -> Result<(), String> {
        std::fs::create_dir_all(&self.root).map_err(io_error)?;
        let path = self.path_for(&record.id)?;
        let body = serde_json::to_vec_pretty(record).map_err(json_error)?;
        std::fs::write(path, body).map_err(io_error)
    }
}

pub fn require_case_id(case_id: &str) -> Result<&str, String> {
    let case_id = case_id.trim();
    (!case_id.is_empty())
        .then_some(case_id)
        .ok_or_else(|| "case ID is required".into())
}

fn local_root() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .or_else(|| std::env::var_os("USERPROFILE"))
        .map(PathBuf::from)
        .map(|home| home.join(".trareon").join("cases"))
}

fn now() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
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
    fn case_store_creates_opens_and_archives_case() {
        let temp = tempfile::tempdir().unwrap();
        let store = CaseStore::at(temp.path());
        let record = store
            .create("Training acquisition", "Examiner One", "Initial note")
            .unwrap();
        assert_eq!(record.state, CaseState::Identify);
        assert_eq!(store.list().unwrap().len(), 1);
        assert_eq!(
            store.open(&record.id).unwrap().unwrap().title,
            "Training acquisition"
        );

        let archived = store.archive(&record.id).unwrap();
        assert_eq!(archived.state, CaseState::Archived);
        assert!(archived.updated_at >= archived.created_at);
    }

    #[test]
    fn require_case_id_rejects_an_orphan_run() {
        assert!(require_case_id("  ").is_err());
        assert_eq!(require_case_id("CASE-1").unwrap(), "CASE-1");
    }
}
