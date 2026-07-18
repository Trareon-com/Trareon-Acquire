//! M3 identification record and non-invasive encryption checks.

use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

/// Encryption state observed during identification. Probes never alter the source.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EncryptionState {
    Detected,
    Unavailable,
    Unknown,
}

/// Encryption probe result together with its operator-readable limitation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EncryptionStatus {
    pub state: EncryptionState,
    pub note: String,
}

impl Default for EncryptionStatus {
    fn default() -> Self {
        Self {
            state: EncryptionState::Unknown,
            note: "encryption has not been probed".into(),
        }
    }
}

/// Identification checklist retained beside a local case record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifyRecord {
    pub power_state: String,
    pub network_isolated: bool,
    pub encryption_status: EncryptionStatus,
    pub oov_ack: bool,
    pub checklist_complete: IdentifyChecklist,
    pub attachments: Vec<String>,
    pub clock_offset_note: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct IdentifyChecklist {
    pub power: bool,
    pub network: bool,
    pub encryption: bool,
    pub anti_forensics: bool,
}

impl IdentifyChecklist {
    pub fn complete(&self) -> bool {
        self.power && self.network && self.encryption && self.anti_forensics
    }
}

impl Default for IdentifyRecord {
    fn default() -> Self {
        Self {
            power_state: String::new(),
            network_isolated: false,
            encryption_status: EncryptionStatus::default(),
            oov_ack: false,
            checklist_complete: IdentifyChecklist::default(),
            attachments: Vec::new(),
            clock_offset_note: String::new(),
        }
    }
}

/// Live acquisition is blocked until all checklist items and out-of-view acknowledgement exist.
pub fn identify_blocks_live(record: &IdentifyRecord) -> bool {
    !record.oov_ack || !record.checklist_complete.complete()
}

/// Store an identification record beside a case JSON file.
pub fn save_beside_case(case_path: &Path, record: &IdentifyRecord) -> Result<PathBuf, String> {
    let path = identify_path(case_path)?;
    let body = serde_json::to_vec_pretty(record).map_err(|error| error.to_string())?;
    fs::write(&path, body).map_err(|error| error.to_string())?;
    Ok(path)
}

/// Load an identification record stored with a case, if present.
pub fn load_beside_case(case_path: &Path) -> Result<Option<IdentifyRecord>, String> {
    let path = identify_path(case_path)?;
    if !path.is_file() {
        return Ok(None);
    }
    let body = fs::read_to_string(path).map_err(|error| error.to_string())?;
    serde_json::from_str(&body)
        .map(Some)
        .map_err(|error| error.to_string())
}

fn identify_path(case_path: &Path) -> Result<PathBuf, String> {
    let stem = case_path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| "case path has no valid filename".to_string())?;
    Ok(case_path.with_file_name(format!("{stem}.identify.json")))
}

/// Best-effort, read-only encryption probe. It deliberately does not unlock or bypass anything.
pub fn probe_encryption(path: &Path) -> EncryptionStatus {
    // Inspect file/header magic before host-wide OS status tools so synthetic
    // LUKS fixtures are not masked by FileVault/BitLocker host state.
    match luks_header(path) {
        Ok(true) => {
            return EncryptionStatus {
                state: EncryptionState::Detected,
                note: "LUKS header detected".into(),
            };
        }
        Ok(false) => {}
        Err(error) => {
            return EncryptionStatus {
                state: EncryptionState::Unknown,
                note: format!("could not inspect source header: {error}"),
            };
        }
    }

    if path_looks_windows_volume(path) {
        #[cfg(target_os = "windows")]
        {
            let output = std::process::Command::new("manage-bde")
                .args(["-status", &path.to_string_lossy()])
                .output();
            if let Ok(output) = output {
                let text = String::from_utf8_lossy(&output.stdout);
                if text.contains("Percentage Encrypted") || text.contains("Protection Status") {
                    return EncryptionStatus {
                        state: EncryptionState::Detected,
                        note: "BitLocker status was reported by manage-bde".into(),
                    };
                }
            }
        }
        return EncryptionStatus {
            state: EncryptionState::Unknown,
            note: "Windows volume path may require a BitLocker status check".into(),
        };
    }

    #[cfg(target_os = "macos")]
    {
        // Host FileVault status is only meaningful for whole-disk sources.
        let looks_block = path
            .to_string_lossy()
            .contains("/dev/");
        if looks_block {
            if let Ok(output) = std::process::Command::new("fdesetup").arg("status").output() {
                let text = String::from_utf8_lossy(&output.stdout);
                if text.contains("FileVault is On") {
                    return EncryptionStatus {
                        state: EncryptionState::Detected,
                        note: "FileVault reported enabled by fdesetup".into(),
                    };
                }
                if text.contains("FileVault is Off") {
                    return EncryptionStatus {
                        state: EncryptionState::Unavailable,
                        note: "FileVault reported disabled by fdesetup".into(),
                    };
                }
            }
        }
    }

    EncryptionStatus {
        state: EncryptionState::Unavailable,
        note: "no LUKS header detected".into(),
    }
}

fn path_looks_windows_volume(path: &Path) -> bool {
    let text = path.to_string_lossy();
    text.starts_with(r"\\.\PhysicalDrive")
        || text.starts_with(r"\\?\Volume")
        || text.as_bytes().get(1) == Some(&b':')
}

fn luks_header(path: &Path) -> Result<bool, std::io::Error> {
    let mut header = [0_u8; 6];
    let mut file = fs::File::open(path)?;
    let read = file.read(&mut header)?;
    Ok(read == header.len() && header == *b"LUKS\xba\xbe")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn checklist_and_oov_gate_live_acquisition() {
        let mut record = IdentifyRecord::default();
        assert!(identify_blocks_live(&record));
        record.oov_ack = true;
        record.checklist_complete = IdentifyChecklist {
            power: true,
            network: true,
            encryption: true,
            anti_forensics: true,
        };
        assert!(!identify_blocks_live(&record));
    }

    #[test]
    fn records_round_trip_beside_case() {
        let temp = tempfile::tempdir().unwrap();
        let case = temp.path().join("CASE-1.json");
        fs::write(&case, "{}").unwrap();
        let record = IdentifyRecord {
            power_state: "on".into(),
            ..IdentifyRecord::default()
        };
        let stored = save_beside_case(&case, &record).unwrap();
        assert_eq!(stored.file_name().unwrap(), "CASE-1.identify.json");
        assert_eq!(load_beside_case(&case).unwrap(), Some(record));
    }

    #[test]
    fn luks_magic_is_detected_without_modifying_source() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("luks.img");
        fs::write(&source, b"LUKS\xba\xbe").unwrap();
        assert_eq!(probe_encryption(&source).state, EncryptionState::Detected);
    }
}
