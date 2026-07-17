//! ATA HPA/DCO detection and write-blocker USB classification.
//!
//! Platform IOCTLs may require `unsafe`. Synthetic helpers are safe and CI-friendly.

use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AtaError {
    #[error("io: {0}")]
    Io(String),
    #[error("unsupported: {0}")]
    Unsupported(String),
    #[error("destructive lab feature disabled")]
    DestructiveDisabled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DetectionStatus {
    Ok,
    Unavailable { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HpaDcoResult {
    pub reported_size: u64,
    pub hpa_detected: bool,
    pub hpa_native_max: Option<u64>,
    pub dco_detected: bool,
    pub dco_max: Option<u64>,
    pub accessible_size: u64,
    pub status: DetectionStatus,
}

impl HpaDcoResult {
    pub fn is_hidden_area(&self) -> bool {
        self.hpa_detected || self.dco_detected
    }
}

/// Pure helper for synthetic / unit tests (no IOCTL).
pub fn detect_hpa_dco_with_probe(
    reported_size: u64,
    native_max: Option<u64>,
    dco_max: Option<u64>,
) -> HpaDcoResult {
    let hpa_detected = native_max.is_some_and(|n| n > reported_size);
    let dco_detected = dco_max.is_some_and(|n| n > reported_size);
    let accessible_size = reported_size;
    HpaDcoResult {
        reported_size,
        hpa_detected,
        hpa_native_max: native_max,
        dco_detected,
        dco_max,
        accessible_size,
        status: DetectionStatus::Ok,
    }
}

/// Best-effort live detect. Falls back to Unavailable when ATA passthrough is not available.
pub fn detect_hpa_dco(device: &Path) -> Result<HpaDcoResult, AtaError> {
    let reported = reported_size_bytes(device)?;
    #[cfg(target_os = "linux")]
    {
        // Live ATA IDENTIFY via SG_IO is lab-gated; report sysfs size with Unavailable ATA detail.
        let _ = device;
        return Ok(HpaDcoResult {
            reported_size: reported,
            hpa_detected: false,
            hpa_native_max: None,
            dco_detected: false,
            dco_max: None,
            accessible_size: reported,
            status: DetectionStatus::Unavailable {
                reason: "linux ATA passthrough not enabled in this build; reported size from sysfs/stat only"
                    .into(),
            },
        });
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = device;
        Ok(HpaDcoResult {
            reported_size: reported,
            hpa_detected: false,
            hpa_native_max: None,
            dco_detected: false,
            dco_max: None,
            accessible_size: reported,
            status: DetectionStatus::Unavailable {
                reason: format!(
                    "ATA HPA/DCO passthrough not implemented on {}",
                    std::env::consts::OS
                ),
            },
        })
    }
}

fn reported_size_bytes(device: &Path) -> Result<u64, AtaError> {
    let meta = std::fs::metadata(device).map_err(|e| AtaError::Io(e.to_string()))?;
    Ok(meta.len())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WriteBlockerState {
    Detected {
        vendor: String,
        model: String,
        vid_pid: Option<(u16, u16)>,
    },
    NotDetected,
    Uncertain {
        reason: String,
    },
    ManualConfirmed {
        note: String,
    },
    NotApplicable,
}

/// Known hardware write-blocker USB IDs (initial matrix).
pub fn classify_usb_vid_pid(vid: u16, pid: u16) -> WriteBlockerState {
    match (vid, pid) {
        (0x0B21, _) => WriteBlockerState::Detected {
            vendor: "Tableau".into(),
            model: format!("USB pid={pid:04x}"),
            vid_pid: Some((vid, pid)),
        },
        (0x0A70, _) => WriteBlockerState::Detected {
            vendor: "WiebeTech".into(),
            model: format!("USB pid={pid:04x}"),
            vid_pid: Some((vid, pid)),
        },
        (0x1A4A, _) => WriteBlockerState::Detected {
            vendor: "CRU".into(),
            model: format!("USB pid={pid:04x}"),
            vid_pid: Some((vid, pid)),
        },
        _ => WriteBlockerState::NotDetected,
    }
}

pub fn probe_write_blocker(device: &Path) -> WriteBlockerState {
    let path = device.to_string_lossy().to_lowercase();
    if path.contains("tableau") {
        return WriteBlockerState::Detected {
            vendor: "Tableau".into(),
            model: "path-heuristic".into(),
            vid_pid: Some((0x0B21, 0)),
        };
    }
    if device.is_file() {
        return WriteBlockerState::NotApplicable;
    }
    WriteBlockerState::Uncertain {
        reason: "USB VID/PID enumeration not available; confirm write-blocker manually".into(),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HpaUnlockRecord {
    pub requested: bool,
    pub applied: bool,
    pub restored: bool,
    pub note: String,
}

/// Temporary HPA unlock — disabled unless `destructive-lab` feature is compiled in.
pub fn temporary_hpa_unlock(_device: &Path, reason: &str) -> Result<HpaUnlockRecord, AtaError> {
    if reason.trim().is_empty() {
        return Err(AtaError::Unsupported(
            "destructive HPA unlock requires a non-empty reason".into(),
        ));
    }
    #[cfg(feature = "destructive-lab")]
    {
        // Real SET MAX sequence would go here under lab allowlist.
        Ok(HpaUnlockRecord {
            requested: true,
            applied: false,
            restored: false,
            note: "destructive-lab feature present but ATA SET MAX not wired".into(),
        })
    }
    #[cfg(not(feature = "destructive-lab"))]
    {
        Err(AtaError::DestructiveDisabled)
    }
}

pub fn restore_hpa(_device: &Path) -> Result<HpaUnlockRecord, AtaError> {
    #[cfg(feature = "destructive-lab")]
    {
        Ok(HpaUnlockRecord {
            requested: false,
            applied: false,
            restored: false,
            note: "destructive-lab feature present but ATA restore not wired".into(),
        })
    }
    #[cfg(not(feature = "destructive-lab"))]
    {
        Err(AtaError::DestructiveDisabled)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_hpa_detection() {
        let r = detect_hpa_dco_with_probe(1000, Some(2000), None);
        assert!(r.hpa_detected);
        assert!(!r.dco_detected);
        assert!(r.is_hidden_area());
    }

    #[test]
    fn synthetic_no_hpa() {
        let r = detect_hpa_dco_with_probe(1000, Some(1000), Some(1000));
        assert!(!r.hpa_detected);
        assert!(!r.dco_detected);
    }

    #[test]
    fn tableau_vid_classified() {
        match classify_usb_vid_pid(0x0B21, 0x0001) {
            WriteBlockerState::Detected { vendor, .. } => assert_eq!(vendor, "Tableau"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn unknown_vid_not_detected() {
        assert_eq!(
            classify_usb_vid_pid(0x1234, 0x5678),
            WriteBlockerState::NotDetected
        );
    }

    #[test]
    fn unlock_requires_feature() {
        let err = temporary_hpa_unlock(Path::new("/dev/null"), "lab").unwrap_err();
        assert!(matches!(err, AtaError::DestructiveDisabled));
    }

    #[test]
    fn unlock_requires_reason() {
        let err = temporary_hpa_unlock(Path::new("/dev/null"), "  ").unwrap_err();
        assert!(matches!(err, AtaError::Unsupported(_)));
    }
}
