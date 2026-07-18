use std::{
    fs::{self, File},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use chrono::Utc;
use sha2::{Digest, Sha256};

use crate::{
    AcquisitionId, AcquisitionState, AuditJournal, CoreError,
    checkpoint::{self, AcquisitionCheckpoint},
    lab_policy::{self, LabAllowlist},
};

/// Coarse phase reported to UI / operators during acquire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcquirePhase {
    Preflight,
    Acquiring,
    Hashing,
    Packaging,
    Verifying,
    Done,
    Failed,
}

impl AcquirePhase {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Preflight => "preflight",
            Self::Acquiring => "acquiring",
            Self::Hashing => "hashing",
            Self::Packaging => "packaging",
            Self::Verifying => "verifying",
            Self::Done => "done",
            Self::Failed => "failed",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcquireProgress {
    pub phase: AcquirePhase,
    pub bytes_done: u64,
    pub bytes_total: Option<u64>,
    pub message: String,
}

impl AcquireProgress {
    pub fn new(
        phase: AcquirePhase,
        bytes_done: u64,
        bytes_total: Option<u64>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            phase,
            bytes_done,
            bytes_total,
            message: message.into(),
        }
    }

    /// 0.0–1.0 when total known; otherwise `None`.
    pub fn fraction(&self) -> Option<f64> {
        let total = self.bytes_total.filter(|t| *t > 0)?;
        Some((self.bytes_done as f64 / total as f64).min(1.0))
    }
}

pub type ProgressCallback = Arc<dyn Fn(AcquireProgress) + Send + Sync>;

#[derive(Clone)]
pub struct AcquireRequest {
    pub source: PathBuf,
    pub output: PathBuf,
    pub audit_output: PathBuf,
    pub buffer_size: usize,
    pub cancel_flag: Option<Arc<AtomicBool>>,
    pub split_segment_bytes: Option<u64>,
    /// When true, resume from `checkpoint_path` if present (file-backed, including split-RAW).
    pub resume: bool,
    pub checkpoint_path: Option<PathBuf>,
    /// Optional human-approved lab allowlist (required for block-device suspects).
    pub lab_allowlist_path: Option<PathBuf>,
    /// Lab/safety bound: stop after this many bytes (partial sample, never whole-disk claim).
    pub max_bytes: Option<u64>,
    /// Throttled progress callback (optional).
    pub progress: Option<ProgressCallback>,
}

impl std::fmt::Debug for AcquireRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AcquireRequest")
            .field("source", &self.source)
            .field("output", &self.output)
            .field("audit_output", &self.audit_output)
            .field("buffer_size", &self.buffer_size)
            .field("split_segment_bytes", &self.split_segment_bytes)
            .field("resume", &self.resume)
            .field("max_bytes", &self.max_bytes)
            .field("has_progress", &self.progress.is_some())
            .finish_non_exhaustive()
    }
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
            cancel_flag: None,
            split_segment_bytes: None,
            resume: false,
            checkpoint_path: None,
            lab_allowlist_path: None,
            max_bytes: None,
            progress: None,
        }
    }

    /// Cooperative cancellation: the acquisition loop checks this flag before
    /// each read and stops with `CoreError::Cancelled` when it is set.
    pub fn with_cancel_flag(mut self, flag: Arc<AtomicBool>) -> Self {
        self.cancel_flag = Some(flag);
        self
    }

    /// Split RAW output into fixed-size segments named `<stem>.NNN.<ext>`
    /// (starting at 001) instead of one continuous file. The final segment
    /// may be shorter than `segment_bytes`; no empty trailing segment is
    /// ever created.
    pub fn with_split_segment_bytes(mut self, segment_bytes: u64) -> Self {
        self.split_segment_bytes = Some(segment_bytes);
        self
    }

    pub fn with_buffer_size(mut self, buffer_size: usize) -> Self {
        self.buffer_size = buffer_size.max(1);
        self
    }

    /// Resume a previously cancelled/incomplete file-backed acquisition using its checkpoint.
    pub fn with_resume(mut self, resume: bool) -> Self {
        self.resume = resume;
        self
    }

    pub fn with_checkpoint_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.checkpoint_path = Some(path.into());
        self
    }

    pub fn with_lab_allowlist_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.lab_allowlist_path = Some(path.into());
        self
    }

    /// Bound the acquire to at most `max_bytes` (lab smoke / partial sample).
    pub fn with_max_bytes(mut self, max_bytes: u64) -> Self {
        self.max_bytes = Some(max_bytes.max(1));
        self
    }

    pub fn with_progress(mut self, callback: ProgressCallback) -> Self {
        self.progress = Some(callback);
        self
    }

    pub fn checkpoint_file(&self) -> PathBuf {
        self.checkpoint_path
            .clone()
            .unwrap_or_else(|| checkpoint::default_checkpoint_path(&self.output))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SegmentInfo {
    pub path: PathBuf,
    pub size: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AcquisitionSummary {
    pub bytes_read: u64,
    pub bytes_written: u64,
    pub sha256: String,
    pub audit_root: String,
    pub audit_path: PathBuf,
    pub state: AcquisitionState,
    pub segments: Vec<SegmentInfo>,
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

fn segment_path(output: &Path, index: usize) -> PathBuf {
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    let stem = output
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("segment");
    let file_name = match output.extension().and_then(|s| s.to_str()) {
        Some(ext) => format!("{stem}.{index:03}.{ext}"),
        None => format!("{stem}.{index:03}"),
    };
    parent.join(file_name)
}

pub fn acquire_file(request: &AcquireRequest) -> Result<AcquisitionSummary, CoreError> {
    let allowlist: Option<LabAllowlist> = match &request.lab_allowlist_path {
        Some(path) => Some(lab_policy::load_lab_allowlist(path)?),
        None => None,
    };
    let source_identity = lab_policy::assert_source_permitted(&request.source, allowlist.as_ref())?;

    let allow_block_device = matches!(
        source_identity.kind,
        lab_policy::SourceKind::BlockDeviceSuspect
    ) && allowlist.as_ref().is_some_and(|list| list.human_approved);

    if allow_block_device && request.max_bytes.is_none() {
        return Err(CoreError::Verification(
            "raw/block-device acquire requires max_bytes (bounded lab sample; full-disk not auto)"
                .to_string(),
        ));
    }

    let source_metadata = fs::metadata(&request.source)
        .map_err(|error| CoreError::Io(format!("source unavailable: {error}")))?;

    if !source_metadata.is_file() && !allow_block_device {
        return Err(CoreError::Verification(
            "source must be a regular file, or an allowlisted block/raw device with human approval"
                .to_string(),
        ));
    }
    if source_metadata.is_file() && source_metadata.len() == 0 {
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

    let emit = |phase: AcquirePhase, done: u64, total: Option<u64>, msg: &str| {
        if let Some(cb) = &request.progress {
            cb(AcquireProgress::new(phase, done, total, msg));
        }
    };
    emit(AcquirePhase::Preflight, 0, None, "preflight_passed");

    let mut run = || -> Result<AcquisitionSummary, CoreError> {
        journal.append(
            acquisition_id,
            Utc::now(),
            AcquisitionState::Acquiring,
            "acquiring",
        )?;

        let checkpoint_file = request.checkpoint_file();
        let mut resume_bytes: u64 = 0;
        let mut split_progress: Option<checkpoint::SplitProgress> = None;
        if request.resume && checkpoint_file.exists() {
            let loaded = checkpoint::load_checkpoint(&checkpoint_file)?;
            if loaded.source != request.source.to_string_lossy() {
                return Err(CoreError::Verification(
                    "checkpoint source does not match request".to_string(),
                ));
            }
            if loaded.output != request.output.to_string_lossy() {
                return Err(CoreError::Verification(
                    "checkpoint output does not match request".to_string(),
                ));
            }
            if loaded.split_segment_bytes != request.split_segment_bytes {
                return Err(CoreError::Verification(
                    "checkpoint split settings do not match request".to_string(),
                ));
            }
            resume_bytes = loaded.bytes_completed;
            if let Some(limit) = request.split_segment_bytes {
                let measured =
                    checkpoint::measure_split_progress(&request.output, limit, &segment_path)?;
                if measured.bytes_completed != resume_bytes {
                    return Err(CoreError::Verification(
                        "checkpoint bytes do not match on-disk split segments".to_string(),
                    ));
                }
                split_progress = Some(measured);
            }
        }

        let mut source_file =
            File::open(&request.source).map_err(|error| CoreError::Io(error.to_string()))?;
        if resume_bytes > 0 {
            source_file
                .seek(SeekFrom::Start(resume_bytes))
                .map_err(|error| CoreError::Io(error.to_string()))?;
        }

        let mut segment_index: usize = 1;
        let mut current_segment_path = match request.split_segment_bytes {
            Some(_) => segment_path(&request.output, segment_index),
            None => request.output.clone(),
        };
        let mut current_segment_bytes: u64 = 0;
        let mut segments: Vec<SegmentInfo> = Vec::new();

        if let Some(progress) = &split_progress {
            segment_index = progress.segment_index;
            current_segment_path = progress.current_segment_path.clone();
            current_segment_bytes = progress.bytes_in_current_segment;
            for (path, size) in &progress.completed_full_segments {
                segments.push(SegmentInfo {
                    path: path.clone(),
                    size: *size,
                });
            }
        }

        // Resume may leave a zero-length or partial output from a prior cancel.
        // Never invent completion: only append when checkpoint bytes > 0.
        let mut current_file = if let Some(progress) = &split_progress {
            if progress.bytes_in_current_segment > 0 {
                File::options()
                    .append(true)
                    .open(&current_segment_path)
                    .map_err(|error| CoreError::Io(error.to_string()))?
            } else if current_segment_path.exists() {
                File::options()
                    .write(true)
                    .truncate(true)
                    .open(&current_segment_path)
                    .map_err(|error| CoreError::Io(error.to_string()))?
            } else {
                File::options()
                    .write(true)
                    .create_new(true)
                    .open(&current_segment_path)
                    .map_err(|error| CoreError::Io(error.to_string()))?
            }
        } else if resume_bytes > 0 {
            File::options()
                .append(true)
                .open(&current_segment_path)
                .map_err(|error| CoreError::Io(error.to_string()))?
        } else if request.resume && current_segment_path.exists() {
            File::options()
                .write(true)
                .truncate(true)
                .open(&current_segment_path)
                .map_err(|error| CoreError::Io(error.to_string()))?
        } else {
            File::options()
                .write(true)
                .create_new(true)
                .open(&current_segment_path)
                .map_err(|error| CoreError::Io(error.to_string()))?
        };

        let mut buffer = vec![0u8; request.buffer_size];
        let mut hasher = if let Some(progress) = &split_progress {
            checkpoint::hash_split_progress(progress)?
        } else if resume_bytes > 0 {
            checkpoint::hash_prefix(&current_segment_path, resume_bytes)?
        } else {
            Sha256::new()
        };
        let mut bytes_read: u64 = resume_bytes;
        let mut bytes_written: u64 = resume_bytes;

        // Prefer known source size for progress; clamp by max_bytes when set.
        let bytes_total = match (source_metadata.is_file(), request.max_bytes) {
            (true, Some(limit)) => Some(source_metadata.len().min(limit)),
            (true, None) => Some(source_metadata.len()),
            (false, Some(limit)) => Some(limit),
            (false, None) => None,
        };

        let last_emit = Mutex::new(Instant::now() - Duration::from_secs(1));
        let report = |done: u64| {
            let should = {
                let mut last = last_emit.lock().unwrap_or_else(|e| e.into_inner());
                if last.elapsed() >= Duration::from_millis(100) || bytes_total == Some(done) {
                    *last = Instant::now();
                    true
                } else {
                    false
                }
            };
            if should {
                emit(AcquirePhase::Acquiring, done, bytes_total, "acquiring");
            }
        };
        emit(
            AcquirePhase::Acquiring,
            bytes_written,
            bytes_total,
            "acquiring",
        );

        loop {
            if let Some(flag) = &request.cancel_flag
                && flag.load(Ordering::SeqCst)
            {
                return Err(CoreError::Cancelled);
            }

            if let Some(limit) = request.max_bytes
                && bytes_written >= limit
            {
                break;
            }

            let mut read_cap = buffer.len();
            if let Some(limit) = request.max_bytes {
                let remaining = limit.saturating_sub(bytes_written);
                if remaining == 0 {
                    break;
                }
                read_cap = read_cap.min(remaining as usize);
            }

            let read = source_file
                .read(&mut buffer[..read_cap])
                .map_err(|error| CoreError::Io(error.to_string()))?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
            bytes_read += read as u64;

            let mut offset = 0usize;
            while offset < read {
                let take = match request.split_segment_bytes {
                    Some(limit) => {
                        let remaining = limit.saturating_sub(current_segment_bytes);
                        (remaining as usize).min(read - offset)
                    }
                    None => read - offset,
                };

                current_file
                    .write_all(&buffer[offset..offset + take])
                    .map_err(|error| CoreError::Io(error.to_string()))?;
                current_segment_bytes += take as u64;
                bytes_written += take as u64;
                offset += take;

                if let Some(limit) = request.split_segment_bytes
                    && current_segment_bytes == limit
                    && offset < read
                {
                    current_file
                        .flush()
                        .map_err(|error| CoreError::Io(error.to_string()))?;
                    current_file
                        .sync_all()
                        .map_err(|error| CoreError::Io(error.to_string()))?;
                    segments.push(SegmentInfo {
                        path: current_segment_path.clone(),
                        size: current_segment_bytes,
                    });

                    segment_index += 1;
                    current_segment_path = segment_path(&request.output, segment_index);
                    current_file = File::options()
                        .write(true)
                        .create_new(true)
                        .open(&current_segment_path)
                        .map_err(|error| CoreError::Io(error.to_string()))?;
                    current_segment_bytes = 0;
                }
            }
            report(bytes_written);
        }

        current_file
            .flush()
            .map_err(|error| CoreError::Io(error.to_string()))?;
        current_file
            .sync_all()
            .map_err(|error| CoreError::Io(error.to_string()))?;
        segments.push(SegmentInfo {
            path: current_segment_path.clone(),
            size: current_segment_bytes,
        });

        emit(
            AcquirePhase::Hashing,
            bytes_written,
            bytes_total,
            "finalizing_hash",
        );
        let sha256 = hex::encode(hasher.finalize());

        journal.append(
            acquisition_id,
            Utc::now(),
            AcquisitionState::AcquiredUnverified,
            "acquired_unverified",
        )?;

        emit(
            AcquirePhase::Done,
            bytes_written,
            bytes_total,
            "acquired_unverified",
        );

        Ok(AcquisitionSummary {
            bytes_read,
            bytes_written,
            sha256,
            audit_root: String::new(),
            audit_path: request.audit_output.clone(),
            state: AcquisitionState::AcquiredUnverified,
            segments,
        })
    };

    match run() {
        Ok(mut summary) => {
            let audit_root = journal.verify()?;
            journal.write_jsonl(&request.audit_output)?;
            summary.audit_root = audit_root;
            checkpoint::clear_checkpoint(&request.checkpoint_file())?;
            Ok(summary)
        }
        Err(CoreError::Cancelled) => {
            emit(AcquirePhase::Failed, 0, None, "cancelled");
            if journal
                .append(
                    acquisition_id,
                    Utc::now(),
                    AcquisitionState::Cancelled,
                    "cancelled",
                )
                .is_ok()
            {
                journal.write_jsonl(&request.audit_output).map_err(|write_error| {
                    CoreError::Io(format!(
                        "acquisition cancelled and audit journal could not be persisted: {write_error}"
                    ))
                })?;
            }
            // Persist resumable checkpoint for file-backed acquires (including split-RAW).
            let cp = if let Some(limit) = request.split_segment_bytes {
                let measured =
                    checkpoint::measure_split_progress(&request.output, limit, &segment_path)
                        .unwrap_or(checkpoint::SplitProgress {
                            bytes_completed: 0,
                            segment_index: 1,
                            bytes_in_current_segment: 0,
                            completed_full_segments: Vec::new(),
                            current_segment_path: segment_path(&request.output, 1),
                        });
                AcquisitionCheckpoint::new(
                    &request.source,
                    &request.output,
                    measured.bytes_completed,
                )
                .with_split(limit)
            } else {
                let completed = fs::metadata(&request.output)
                    .map(|meta| meta.len())
                    .unwrap_or(0);
                AcquisitionCheckpoint::new(&request.source, &request.output, completed)
            };
            let _ = checkpoint::write_checkpoint(&request.checkpoint_file(), &cp);
            Err(CoreError::Cancelled)
        }
        Err(error) => {
            emit(AcquirePhase::Failed, 0, None, "failed");
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
