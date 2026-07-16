use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::{AcquisitionId, AcquisitionState, AuditJournal, CoreError};

#[derive(Debug, Clone)]
pub struct AcquireRequest {
    pub source: PathBuf,
    pub output: PathBuf,
    pub audit_output: PathBuf,
    pub buffer_size: usize,
}

impl AcquireRequest {
    pub fn new(source: impl Into<PathBuf>, output: impl Into<PathBuf>) -> Self {
        let output = output.into();
        let audit_output = output.with_extension("audit.jsonl");
        Self {
            source: source.into(),
            output,
            audit_output,
            buffer_size: 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AcquisitionSummary {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub sha256: String,
    pub audit_root: String,
    pub audit_path: PathBuf,
    pub state: AcquisitionState,
}

fn canonical_parent(path: &Path) -> Result<PathBuf, CoreError> {
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let parent = if parent.as_os_str().is_empty() {
        Path::new(".")
    } else {
        parent
    };
    fs::canonicalize(parent).map_err(|error| CoreError::Io(error.to_string()))
}

pub fn acquire_file(request: &AcquireRequest) -> Result<AcquisitionSummary, CoreError> {
    let source_metadata = fs::metadata(&request.source)
        .map_err(|error| CoreError::Io(format!("source unavailable: {error}")))?;
    if !source_metadata.is_file() {
        return Err(CoreError::Verification(
            "source must be a regular file".to_string(),
        ));
    }
    if source_metadata.len() == 0 {
        return Err(CoreError::Verification(
            "source must not be empty".to_string(),
        ));
    }

    let source_canonical =
        fs::canonicalize(&request.source).map_err(|error| CoreError::Io(error.to_string()))?;
    let output_parent_canonical = canonical_parent(&request.output)?;
    let output_file_name = request
        .output
        .file_name()
        .ok_or_else(|| CoreError::Verification("output must name a file".to_string()))?;
    let destination_canonical = output_parent_canonical.join(output_file_name);

    if source_canonical == destination_canonical {
        return Err(CoreError::Verification(
            "source and destination must not be identical".to_string(),
        ));
    }

    let acquisition_id = AcquisitionId(uuid::Uuid::new_v4());
    let mut journal = AuditJournal::default();
    journal.append(
        acquisition_id,
        Utc::now(),
        AcquisitionState::Planned,
        "planned",
    )?;
    journal.append(
        acquisition_id,
        Utc::now(),
        AcquisitionState::PreflightPassed,
        "preflight_passed",
    )?;

    let mut run = || -> Result<AcquisitionSummary, CoreError> {
        journal.append(
            acquisition_id,
            Utc::now(),
            AcquisitionState::Acquiring,
            "acquiring",
        )?;

        let mut source_file =
            File::open(&request.source).map_err(|error| CoreError::Io(error.to_string()))?;
        let mut output_file = File::options()
            .write(true)
            .create_new(true)
            .open(&request.output)
            .map_err(|error| CoreError::Io(error.to_string()))?;

        let mut buffer = vec![0u8; request.buffer_size];
        let mut hasher = Sha256::new();
        let mut bytes_read: u64 = 0;
        let mut bytes_written: u64 = 0;

        loop {
            let read = source_file
                .read(&mut buffer)
                .map_err(|error| CoreError::Io(error.to_string()))?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
            output_file
                .write_all(&buffer[..read])
                .map_err(|error| CoreError::Io(error.to_string()))?;
            bytes_read += read as u64;
            bytes_written += read as u64;
        }

        output_file
            .flush()
            .map_err(|error| CoreError::Io(error.to_string()))?;
        output_file
            .sync_all()
            .map_err(|error| CoreError::Io(error.to_string()))?;

        let sha256 = hex::encode(hasher.finalize());

        journal.append(
            acquisition_id,
            Utc::now(),
            AcquisitionState::AcquiredUnverified,
            "acquired_unverified",
        )?;

        Ok(AcquisitionSummary {
            bytes_read,
            bytes_written,
            sha256,
            audit_root: String::new(),
            audit_path: request.audit_output.clone(),
            state: AcquisitionState::AcquiredUnverified,
        })
    };

    match run() {
        Ok(mut summary) => {
            let audit_root = journal.verify()?;
            journal.write_jsonl(&request.audit_output)?;
            summary.audit_root = audit_root;
            Ok(summary)
        }
        Err(error) => {
            if journal
                .append(
                    acquisition_id,
                    Utc::now(),
                    AcquisitionState::Failed,
                    "failed",
                )
                .is_ok()
            {
                journal
                    .write_jsonl(&request.audit_output)
                    .map_err(|write_error| {
                        CoreError::Io(format!(
                            "acquisition failed ({error}) and audit journal could not be persisted: {write_error}"
                        ))
                    })?;
            }
            Err(error)
        }
    }
}
