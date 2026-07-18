//! Best-effort, local-only preferences for the Acquire shell.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct AcquirePrefs {
    pub dark_mode: bool,
    pub locale: String,
    pub examiner: String,
    pub last_output_dir: String,
    pub last_density: f32,
}

impl Default for AcquirePrefs {
    fn default() -> Self {
        Self {
            dark_mode: false,
            locale: "en".into(),
            examiner: String::new(),
            last_output_dir: String::new(),
            last_density: 1.0,
        }
    }
}

impl AcquirePrefs {
    pub fn load() -> Self {
        let Some(path) = preference_path() else {
            return Self::default();
        };
        std::fs::read_to_string(path)
            .ok()
            .and_then(|raw| serde_json::from_str(&raw).ok())
            .unwrap_or_default()
    }

    /// Preferences are non-essential: a read-only home directory must not block Acquire.
    pub fn save(&self) {
        let Some(path) = preference_path() else {
            return;
        };
        let Some(parent) = path.parent() else {
            return;
        };
        if std::fs::create_dir_all(parent).is_err() {
            return;
        }
        let Ok(body) = serde_json::to_vec_pretty(self) else {
            return;
        };
        let _ = std::fs::write(path, body);
    }
}

fn preference_path() -> Option<PathBuf> {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .map(|home| home.join(".trareon").join("acquire-prefs.json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_light_english() {
        assert_eq!(AcquirePrefs::default().locale, "en");
        assert!(!AcquirePrefs::default().dark_mode);
    }
}
