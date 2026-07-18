use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
};

use serde::{Deserialize, Serialize};

use crate::CoreError;

pub const CUSTODY_SCHEMA: &str = "trareon.custody/1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CustodyEntry {
    pub schema: String,
    pub timestamp: String,
    pub action: String,
    pub actor: String,
    pub evidence_id: String,
    #[serde(default)]
    pub details: String,
}

impl CustodyEntry {
    pub fn new(
        timestamp: impl Into<String>,
        action: impl Into<String>,
        actor: impl Into<String>,
        evidence_id: impl Into<String>,
    ) -> Self {
        Self {
            schema: CUSTODY_SCHEMA.to_string(),
            timestamp: timestamp.into(),
            action: action.into(),
            actor: actor.into(),
            evidence_id: evidence_id.into(),
            details: String::new(),
        }
    }
}

pub fn append_custody_jsonl(path: &Path, entry: &CustodyEntry) -> Result<(), CoreError> {
    if entry.schema != CUSTODY_SCHEMA {
        return Err(CoreError::Serialization(
            "unsupported custody schema".to_string(),
        ));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| CoreError::Io(error.to_string()))?;
    }
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .map_err(|error| CoreError::Io(error.to_string()))?;
    serde_json::to_writer(&mut file, entry)
        .map_err(|error| CoreError::Serialization(error.to_string()))?;
    file.write_all(b"\n")
        .and_then(|_| file.sync_all())
        .map_err(|error| CoreError::Io(error.to_string()))
}

pub fn read_custody_jsonl(path: &Path) -> Result<Vec<CustodyEntry>, CoreError> {
    let contents = fs::read_to_string(path).map_err(|error| CoreError::Io(error.to_string()))?;
    contents
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let entry: CustodyEntry = serde_json::from_str(line)
                .map_err(|error| CoreError::Serialization(error.to_string()))?;
            if entry.schema != CUSTODY_SCHEMA {
                return Err(CoreError::Serialization(
                    "unsupported custody schema".to_string(),
                ));
            }
            Ok(entry)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_round_trip_as_jsonl() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("custody.jsonl");
        let entry = CustodyEntry::new("2026-07-18T00:00:00Z", "received", "examiner", "TR-1");
        append_custody_jsonl(&path, &entry).unwrap();
        assert_eq!(read_custody_jsonl(&path).unwrap(), vec![entry]);
    }
}
