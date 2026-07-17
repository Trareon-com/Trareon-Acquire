//! File-backed acquisition checkpoint / resume helpers (M1).
//!
//! Incomplete acquisitions never become `VerifiedComplete`. A successful
//! acquire deletes its checkpoint file. Cancel/failure may leave a checkpoint
//! so a later resume can continue without inventing completion.

use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::CoreError;

pub const CHECKPOINT_SCHEMA: &str = "trareon.checkpoint/1";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct AcquisitionCheckpoint {
    pub schema: String,
    pub source: String,
    pub output: String,
    pub bytes_completed: u64,
    /// Always true while the checkpoint file exists — completion deletes it.
    pub incomplete: bool,
}

impl AcquisitionCheckpoint {
    pub fn new(source: &Path, output: &Path, bytes_completed: u64) -> Self {
        Self {
            schema: CHECKPOINT_SCHEMA.to_string(),
            source: source.to_string_lossy().to_string(),
            output: output.to_string_lossy().to_string(),
            bytes_completed,
            incomplete: true,
        }
    }
}

pub fn default_checkpoint_path(output: &Path) -> PathBuf {
    let mut path = output.as_os_str().to_owned();
    path.push(".checkpoint.json");
    PathBuf::from(path)
}

pub fn write_checkpoint(path: &Path, checkpoint: &AcquisitionCheckpoint) -> Result<(), CoreError> {
    if !checkpoint.incomplete {
        return Err(CoreError::Verification(
            "checkpoint must remain incomplete while on disk".to_string(),
        ));
    }
    if checkpoint.schema != CHECKPOINT_SCHEMA {
        return Err(CoreError::Verification(
            "unsupported checkpoint schema".to_string(),
        ));
    }
    let bytes = serde_json::to_vec_pretty(checkpoint)
        .map_err(|error| CoreError::Serialization(error.to_string()))?;
    let mut file = File::create(path).map_err(|error| CoreError::Io(error.to_string()))?;
    file.write_all(&bytes)
        .map_err(|error| CoreError::Io(error.to_string()))?;
    file.sync_all()
        .map_err(|error| CoreError::Io(error.to_string()))?;
    Ok(())
}

pub fn load_checkpoint(path: &Path) -> Result<AcquisitionCheckpoint, CoreError> {
    let bytes = fs::read(path).map_err(|error| CoreError::Io(error.to_string()))?;
    let checkpoint: AcquisitionCheckpoint = serde_json::from_slice(&bytes)
        .map_err(|error| CoreError::Serialization(error.to_string()))?;
    if checkpoint.schema != CHECKPOINT_SCHEMA {
        return Err(CoreError::Verification(
            "unsupported checkpoint schema".to_string(),
        ));
    }
    if !checkpoint.incomplete {
        return Err(CoreError::Verification(
            "checkpoint claims complete; refusing to treat as resumable".to_string(),
        ));
    }
    Ok(checkpoint)
}

pub fn clear_checkpoint(path: &Path) -> Result<(), CoreError> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(CoreError::Io(error.to_string())),
    }
}

/// Hash the first `bytes` of an existing output so resume can continue the digest.
pub fn hash_prefix(path: &Path, bytes: u64) -> Result<Sha256, CoreError> {
    let mut file = File::open(path).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut hasher = Sha256::new();
    let mut remaining = bytes;
    let mut buffer = [0u8; 64 * 1024];
    while remaining > 0 {
        let want = remaining.min(buffer.len() as u64) as usize;
        let read = file
            .read(&mut buffer[..want])
            .map_err(|error| CoreError::Io(error.to_string()))?;
        if read == 0 {
            return Err(CoreError::Verification(
                "checkpoint bytes exceed existing output length".to_string(),
            ));
        }
        hasher.update(&buffer[..read]);
        remaining -= read as u64;
    }
    Ok(hasher)
}
