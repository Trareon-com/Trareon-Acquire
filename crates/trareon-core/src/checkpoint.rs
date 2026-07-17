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
    /// When set, resume continues a split-RAW acquisition with this segment size.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub split_segment_bytes: Option<u64>,
}

impl AcquisitionCheckpoint {
    pub fn new(source: &Path, output: &Path, bytes_completed: u64) -> Self {
        Self {
            schema: CHECKPOINT_SCHEMA.to_string(),
            source: source.to_string_lossy().to_string(),
            output: output.to_string_lossy().to_string(),
            bytes_completed,
            incomplete: true,
            split_segment_bytes: None,
        }
    }

    pub fn with_split(mut self, segment_bytes: u64) -> Self {
        self.split_segment_bytes = Some(segment_bytes);
        self
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

/// Progress reconstructed from on-disk split segments (fail-closed vs checkpoint).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplitProgress {
    pub bytes_completed: u64,
    /// 1-based index of the segment to continue writing.
    pub segment_index: usize,
    /// Bytes already present in that segment (0 means create/open empty next segment).
    pub bytes_in_current_segment: u64,
    pub completed_full_segments: Vec<(PathBuf, u64)>,
    pub current_segment_path: PathBuf,
}

pub fn measure_split_progress(
    output: &Path,
    segment_limit: u64,
    segment_path_fn: &dyn Fn(&Path, usize) -> PathBuf,
) -> Result<SplitProgress, CoreError> {
    if segment_limit == 0 {
        return Err(CoreError::Verification(
            "split segment size must be greater than zero".to_string(),
        ));
    }

    let mut completed_full_segments = Vec::new();
    let mut bytes_completed = 0u64;
    let mut segment_index = 1usize;

    loop {
        let path = segment_path_fn(output, segment_index);
        if !path.exists() {
            break;
        }
        let len = fs::metadata(&path)
            .map_err(|error| CoreError::Io(error.to_string()))?
            .len();
        if len > segment_limit {
            return Err(CoreError::Verification(
                "split segment exceeds configured limit".to_string(),
            ));
        }
        bytes_completed += len;
        if len < segment_limit {
            return Ok(SplitProgress {
                bytes_completed,
                segment_index,
                bytes_in_current_segment: len,
                completed_full_segments,
                current_segment_path: path,
            });
        }
        completed_full_segments.push((path, len));
        segment_index += 1;
    }

    Ok(SplitProgress {
        bytes_completed,
        segment_index,
        bytes_in_current_segment: 0,
        completed_full_segments,
        current_segment_path: segment_path_fn(output, segment_index),
    })
}

/// Hash completed full segments plus any partial prefix on the current segment.
pub fn hash_split_progress(progress: &SplitProgress) -> Result<Sha256, CoreError> {
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    for (path, size) in &progress.completed_full_segments {
        let mut file = File::open(path).map_err(|error| CoreError::Io(error.to_string()))?;
        let mut remaining = *size;
        while remaining > 0 {
            let want = remaining.min(buffer.len() as u64) as usize;
            let read = file
                .read(&mut buffer[..want])
                .map_err(|error| CoreError::Io(error.to_string()))?;
            if read == 0 {
                return Err(CoreError::Verification(
                    "split segment shorter than measured size".to_string(),
                ));
            }
            hasher.update(&buffer[..read]);
            remaining -= read as u64;
        }
    }
    if progress.bytes_in_current_segment > 0 {
        let mut file = File::open(&progress.current_segment_path)
            .map_err(|error| CoreError::Io(error.to_string()))?;
        let mut remaining = progress.bytes_in_current_segment;
        while remaining > 0 {
            let want = remaining.min(buffer.len() as u64) as usize;
            let read = file
                .read(&mut buffer[..want])
                .map_err(|error| CoreError::Io(error.to_string()))?;
            if read == 0 {
                return Err(CoreError::Verification(
                    "current split segment shorter than measured size".to_string(),
                ));
            }
            hasher.update(&buffer[..read]);
            remaining -= read as u64;
        }
    }
    Ok(hasher)
}
