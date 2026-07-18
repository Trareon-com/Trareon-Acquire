//! Resumable acquire-wizard state.

use crate::{UiMode, UiSnapshot};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default)]
pub struct WizardDraft {
    pub mode: String,
    pub case_id: String,
    pub source_path: String,
    pub output_dir: String,
    pub confirmed_synthetic: bool,
}

impl Default for WizardDraft {
    fn default() -> Self {
        Self {
            mode: UiMode::Guided.as_str().into(),
            case_id: String::new(),
            source_path: String::new(),
            output_dir: String::new(),
            confirmed_synthetic: false,
        }
    }
}

impl WizardDraft {
    pub fn from_snapshot(snapshot: &UiSnapshot) -> Self {
        Self {
            mode: snapshot.mode.as_str().into(),
            case_id: snapshot.case_id.clone(),
            source_path: snapshot.source_path.clone(),
            output_dir: snapshot.output_dir.clone(),
            confirmed_synthetic: snapshot.confirmed_synthetic,
        }
    }

    pub fn apply_to(&self, snapshot: &mut UiSnapshot) {
        snapshot.mode = match self.mode.as_str() {
            "expert" => UiMode::Expert,
            "standard" => UiMode::Standard,
            _ => UiMode::Guided,
        };
        snapshot.case_id = self.case_id.clone();
        snapshot.source_path = self.source_path.clone();
        snapshot.output_dir = self.output_dir.clone();
        snapshot.confirmed_synthetic = self.confirmed_synthetic;
    }

    pub fn load() -> Option<Self> {
        Self::default_path().and_then(|path| Self::load_at(path).ok().flatten())
    }

    pub fn save(&self) {
        if let Some(path) = Self::default_path() {
            let _ = self.save_at(path);
        }
    }

    pub fn clear() {
        if let Some(path) = Self::default_path() {
            let _ = Self::clear_at(path);
        }
    }

    pub fn load_at(path: impl AsRef<Path>) -> Result<Option<Self>, String> {
        let path = path.as_ref();
        if !path.is_file() {
            return Ok(None);
        }
        let body = std::fs::read_to_string(path).map_err(io_error)?;
        serde_json::from_str(&body).map(Some).map_err(json_error)
    }

    pub fn save_at(&self, path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(io_error)?;
        }
        let body = serde_json::to_vec_pretty(self).map_err(json_error)?;
        std::fs::write(path, body).map_err(io_error)
    }

    pub fn clear_at(path: impl AsRef<Path>) -> Result<(), String> {
        let path = path.as_ref();
        if path.is_file() {
            std::fs::remove_file(path).map_err(io_error)?;
        }
        Ok(())
    }

    fn default_path() -> Option<PathBuf> {
        std::env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(".trareon").join("acquire-wizard-draft.json"))
    }
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
    fn draft_round_trips_and_clears() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("draft.json");
        let draft = WizardDraft {
            mode: "guided".into(),
            case_id: "CASE-1".into(),
            source_path: "/tmp/source.bin".into(),
            output_dir: "/tmp/out".into(),
            confirmed_synthetic: true,
        };

        draft.save_at(&path).unwrap();
        assert_eq!(WizardDraft::load_at(&path).unwrap(), Some(draft));
        WizardDraft::clear_at(&path).unwrap();
        assert_eq!(WizardDraft::load_at(&path).unwrap(), None);
    }
}
