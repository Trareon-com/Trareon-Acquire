//! Lab source policy for M2 Storage Lab Beta prep.
//!
//! This module does **not** open raw devices or elevate privileges. It
//! classifies paths, denies system-disk-like identities by default, and
//! optionally enforces an explicit allowlist JSON that a human must approve.

use std::{fs, path::Path};

use sha2::{Digest, Sha256};

use crate::CoreError;

/// How a candidate source path is classified before any raw I/O.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceKind {
    /// Regular file (synthetic / training image).
    RegularFile,
    /// Path string matches a known raw/block device pattern.
    BlockDeviceSuspect,
    /// Could not classify (missing path, unusual metadata).
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct SourceIdentity {
    pub kind: SourceKind,
    pub path: String,
    /// Hex SHA-256 over a stable identity payload (path + size + mtime when available).
    pub identity_digest: String,
    pub size_bytes: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LabAllowlistEntry {
    /// Exact path or identity digest that is permitted for lab use.
    pub source_identity: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LabAllowlist {
    pub schema: String,
    /// Must be true and signed off by a human before block-device suspects may proceed.
    pub human_approved: bool,
    pub approved_by: String,
    pub entries: Vec<LabAllowlistEntry>,
}

pub const LAB_ALLOWLIST_SCHEMA: &str = "trareon.lab-allowlist/1";

fn path_looks_like_block_device(path: &Path) -> bool {
    let raw = path.to_string_lossy().to_ascii_lowercase();
    raw.contains(r"\\.\physicaldrive")
        || raw.contains("/dev/rdisk")
        || raw.contains("/dev/disk")
        || raw.starts_with("/dev/nvme")
        || raw.starts_with("/dev/sd")
        || raw.starts_with("/dev/hd")
        || raw.starts_with("/dev/mmcblk")
        || raw.starts_with("/dev/loop")
        || raw.contains("physicaldrive")
}

/// Hard-denied system-disk identity substrings. Removable lab media must not match these.
pub(crate) fn is_hard_denied_system_disk(path: &Path) -> bool {
    let raw = path.to_string_lossy().to_ascii_lowercase();
    // Whole-disk 0 / system primary candidates — never allowlisted by default.
    raw.contains(r"\\.\physicaldrive0")
        || raw == "/dev/rdisk0"
        || raw == "/dev/disk0"
        || raw.starts_with("/dev/nvme0n1")
        || raw == "/dev/sda"
        || raw.starts_with("/dev/sda")
}

/// Broker-facing identity gate: string match only (no open / no metadata).
///
/// Block-device-like identities require a human-approved allowlist entry.
/// System-disk-like identities are always denied. Non-device lab tokens
/// (e.g. `lab:…`) remain structurally acceptable for the protocol spike.
pub fn assert_broker_source_identity(
    source_identity: &str,
    allowlist: Option<&LabAllowlist>,
) -> Result<(), CoreError> {
    let path = Path::new(source_identity);
    if is_hard_denied_system_disk(path) {
        return Err(CoreError::Verification(
            "system-disk-like source is hard-denied (not lab media)".to_string(),
        ));
    }
    if !path_looks_like_block_device(path) {
        return Ok(());
    }
    let Some(list) = allowlist else {
        return Err(CoreError::Verification(
            "block-device broker source requires a human-approved lab allowlist".to_string(),
        ));
    };
    if !list.human_approved || list.approved_by.trim().is_empty() {
        return Err(CoreError::Verification(
            "lab allowlist is not human-approved".to_string(),
        ));
    }
    let permitted = list
        .entries
        .iter()
        .any(|entry| entry.source_identity == source_identity);
    if !permitted {
        return Err(CoreError::Verification(
            "block-device broker source is not on the lab allowlist".to_string(),
        ));
    }
    Ok(())
}

pub fn classify_source_path(path: &Path) -> SourceKind {
    if path_looks_like_block_device(path) {
        return SourceKind::BlockDeviceSuspect;
    }
    match fs::metadata(path) {
        Ok(meta) if meta.is_file() => SourceKind::RegularFile,
        Ok(_) => SourceKind::Unknown,
        Err(_) => SourceKind::Unknown,
    }
}

pub fn source_identity_for(path: &Path) -> Result<SourceIdentity, CoreError> {
    let kind = classify_source_path(path);
    let meta = fs::metadata(path).ok();
    let size_bytes = meta.as_ref().map(|m| m.len());
    let modified = meta
        .as_ref()
        .and_then(|m| m.modified().ok())
        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
        .map(|d| d.as_secs());
    let payload = format!(
        "{}|{:?}|{:?}|{:?}",
        path.to_string_lossy(),
        kind,
        size_bytes,
        modified
    );
    let identity_digest = hex::encode(Sha256::digest(payload.as_bytes()));
    Ok(SourceIdentity {
        kind,
        path: path.to_string_lossy().to_string(),
        identity_digest,
        size_bytes,
    })
}

pub fn load_lab_allowlist(path: &Path) -> Result<LabAllowlist, CoreError> {
    let bytes = fs::read(path).map_err(|error| CoreError::Io(error.to_string()))?;
    let allowlist: LabAllowlist = serde_json::from_slice(&bytes)
        .map_err(|error| CoreError::Serialization(error.to_string()))?;
    if allowlist.schema != LAB_ALLOWLIST_SCHEMA {
        return Err(CoreError::Verification(
            "unsupported lab allowlist schema".to_string(),
        ));
    }
    Ok(allowlist)
}

/// Fail-closed gate for M2-bound sources.
///
/// - System-disk-like paths are always denied.
/// - Block-device suspects require a human-approved allowlist that lists the
///   exact path or identity digest.
/// - Regular files are allowed without an allowlist (Engineering Alpha path).
pub fn assert_source_permitted(
    path: &Path,
    allowlist: Option<&LabAllowlist>,
) -> Result<SourceIdentity, CoreError> {
    if is_hard_denied_system_disk(path) {
        return Err(CoreError::Verification(
            "system-disk-like source is hard-denied (not lab media)".to_string(),
        ));
    }

    let identity = source_identity_for(path)?;
    match identity.kind {
        SourceKind::RegularFile => Ok(identity),
        SourceKind::Unknown => Err(CoreError::Verification(
            "source kind is unknown; refusing acquisition".to_string(),
        )),
        SourceKind::BlockDeviceSuspect => {
            let Some(list) = allowlist else {
                return Err(CoreError::Verification(
                    "block-device source requires a human-approved lab allowlist".to_string(),
                ));
            };
            if !list.human_approved || list.approved_by.trim().is_empty() {
                return Err(CoreError::Verification(
                    "lab allowlist is not human-approved".to_string(),
                ));
            }
            let permitted = list.entries.iter().any(|entry| {
                entry.source_identity == identity.path
                    || entry.source_identity == identity.identity_digest
            });
            if !permitted {
                return Err(CoreError::Verification(
                    "block-device source is not on the lab allowlist".to_string(),
                ));
            }
            Ok(identity)
        }
    }
}

/// Write a template allowlist (always `human_approved: false`) for operators.
pub fn write_allowlist_template(path: &Path) -> Result<(), CoreError> {
    let template = LabAllowlist {
        schema: LAB_ALLOWLIST_SCHEMA.to_string(),
        human_approved: false,
        approved_by: String::new(),
        entries: vec![LabAllowlistEntry {
            source_identity: "REPLACE_WITH_LAB_MEDIA_PATH_OR_DIGEST".to_string(),
            notes: "Do not list system disks. Human must set human_approved=true.".to_string(),
        }],
    };
    let bytes = serde_json::to_vec_pretty(&template)
        .map_err(|error| CoreError::Serialization(error.to_string()))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| CoreError::Io(error.to_string()))?;
    }
    fs::write(path, bytes).map_err(|error| CoreError::Io(error.to_string()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::tempdir;

    #[test]
    fn regular_file_is_permitted_without_allowlist() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("synthetic.img");
        fs::write(&path, b"lab").unwrap();
        let identity = assert_source_permitted(&path, None).unwrap();
        assert_eq!(identity.kind, SourceKind::RegularFile);
        assert_eq!(identity.identity_digest.len(), 64);
    }

    #[test]
    fn physical_drive_zero_is_hard_denied() {
        let err = assert_source_permitted(Path::new(r"\\.\PhysicalDrive0"), None).unwrap_err();
        assert!(err.to_string().contains("hard-denied"));
    }

    #[test]
    fn rdisk0_is_hard_denied() {
        let err = assert_source_permitted(Path::new("/dev/rdisk0"), None).unwrap_err();
        assert!(err.to_string().contains("hard-denied"));
    }

    #[test]
    fn loop_device_requires_human_approved_allowlist() {
        let path = PathBuf::from("/dev/loop11");
        let err = assert_source_permitted(&path, None).unwrap_err();
        assert!(err.to_string().contains("allowlist"));

        let unsigned = LabAllowlist {
            schema: LAB_ALLOWLIST_SCHEMA.to_string(),
            human_approved: false,
            approved_by: String::new(),
            entries: vec![LabAllowlistEntry {
                source_identity: path.to_string_lossy().to_string(),
                notes: "test".to_string(),
            }],
        };
        let err = assert_source_permitted(&path, Some(&unsigned)).unwrap_err();
        assert!(err.to_string().contains("not human-approved"));

        let signed = LabAllowlist {
            human_approved: true,
            approved_by: "operator-test".to_string(),
            ..unsigned
        };
        // Path may not exist → identity still builds with Unknown/BlockDeviceSuspect.
        // classify sees block pattern first.
        let identity = assert_source_permitted(&path, Some(&signed)).unwrap();
        assert_eq!(identity.kind, SourceKind::BlockDeviceSuspect);
    }
}
