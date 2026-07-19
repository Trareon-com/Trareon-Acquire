use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use sha2::{Digest, Sha512};
use trareon_core::{
    AcquirePhase, AcquireProgress, AcquireRequest, CaseMetadata, ProgressCallback, acquire_file,
    create_fsnap_from_segments, verify_fsnap, write_e01,
};

/// Formats exposed by the desktop acquisition workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiOutputFormat {
    RawFsnap,
    E01,
    /// Optional ZFF via external `zffacquire` (feature `zff` on trareon-core).
    Zff,
}

/// Read adapter: cancel + throttled progress while feeding a format writer.
struct ProgressCancelReader<R> {
    inner: R,
    cancel: Option<Arc<AtomicBool>>,
    progress: Option<ProgressCallback>,
    bytes_done: u64,
    bytes_total: Option<u64>,
    last_report: u64,
}

impl<R: Read> ProgressCancelReader<R> {
    fn new(
        inner: R,
        cancel: Option<Arc<AtomicBool>>,
        progress: Option<ProgressCallback>,
        bytes_total: Option<u64>,
    ) -> Self {
        Self {
            inner,
            cancel,
            progress,
            bytes_done: 0,
            bytes_total,
            last_report: 0,
        }
    }

    fn report(&self) {
        if let Some(cb) = &self.progress {
            cb(AcquireProgress::new(
                AcquirePhase::Acquiring,
                self.bytes_done,
                self.bytes_total,
                "streaming to container",
            ));
        }
    }
}

impl<R: Read> Read for ProgressCancelReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        if self
            .cancel
            .as_ref()
            .is_some_and(|f| f.load(Ordering::SeqCst))
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Interrupted,
                "acquisition cancelled",
            ));
        }
        let n = self.inner.read(buf)?;
        if n > 0 {
            self.bytes_done += n as u64;
            // Throttle: every 256 KiB or when finished.
            if self.bytes_done.saturating_sub(self.last_report) >= 256 * 1024
                || self.bytes_total == Some(self.bytes_done)
            {
                self.last_report = self.bytes_done;
                self.report();
            }
        }
        Ok(n)
    }
}

/// Acquires a file and writes it in the selected output representation.
pub fn acquire_to_format_with_sha512(
    source: &Path,
    output_dir: &Path,
    format: UiOutputFormat,
    write_sha512_sidecar: bool,
) -> Result<PathBuf, String> {
    acquire_to_format_with_controls(source, output_dir, format, write_sha512_sidecar, None, None)
}

/// Stream source → E01 in one pass; ZFF still stages once (external tool needs a path).
pub fn acquire_to_format_with_controls(
    source: &Path,
    output_dir: &Path,
    format: UiOutputFormat,
    write_sha512_sidecar: bool,
    cancel_flag: Option<Arc<AtomicBool>>,
    progress: Option<ProgressCallback>,
) -> Result<PathBuf, String> {
    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;
    let output = match format {
        UiOutputFormat::RawFsnap => PathBuf::from(
            crate::run_foundation_demo_with_progress(source, output_dir, cancel_flag, progress)?
                .package_path,
        ),
        UiOutputFormat::E01 => {
            let output = output_dir.join("evidence.e01");
            let total = source.metadata().ok().map(|m| m.len());
            let file = File::open(source).map_err(|error| error.to_string())?;
            let cancel = cancel_flag.clone();
            let reader =
                ProgressCancelReader::new(BufReader::new(file), cancel_flag, progress, total);
            write_e01(reader, &output, &CaseMetadata::default()).map_err(|error| {
                let msg = error.to_string();
                if msg.to_lowercase().contains("cancel") || msg.contains("Interrupted") {
                    "acquisition cancelled".into()
                } else {
                    msg
                }
            })?;
            // Ensure cancel surfaces even if writer swallowed Interrupted oddly.
            if cancel.as_ref().is_some_and(|f| f.load(Ordering::SeqCst)) {
                let _ = std::fs::remove_file(&output);
                return Err("acquisition cancelled".into());
            }
            output
        }
        UiOutputFormat::Zff => {
            // External zffacquire requires a path; one staging file, then delete.
            let raw = output_dir.join("format-input.raw");
            let mut request = AcquireRequest::new(source, &raw).with_buffer_size(64 * 1024);
            if let Some(flag) = cancel_flag {
                request = request.with_cancel_flag(flag);
            }
            if let Some(cb) = progress {
                request = request.with_progress(cb);
            }
            acquire_file(&request).map_err(|error| error.to_string())?;
            #[cfg(feature = "zff")]
            {
                let prefix = output_dir.join("evidence-zff");
                let result = trareon_core::write_zff_physical(&raw, &prefix)
                    .map_err(|error| error.to_string());
                let _ = std::fs::remove_file(&raw);
                result?;
                prefix
            }
            #[cfg(not(feature = "zff"))]
            {
                let _ = std::fs::remove_file(&raw);
                return Err(
                    "ZFF output requires trareon-core feature `zff` + zffacquire on PATH".into(),
                );
            }
        }
    };

    if write_sha512_sidecar && output.is_file() {
        write_sha512_sidecar_for(&output)?;
    }
    Ok(output)
}

/// File-backed split-raw acquisition into a multi-segment `.fsnap` package.
pub fn acquire_split_raw(
    source: &Path,
    output_dir: &Path,
    segment_bytes: u64,
) -> Result<PathBuf, String> {
    acquire_split_raw_with_controls(source, output_dir, segment_bytes, None, None)
}

pub fn acquire_split_raw_with_controls(
    source: &Path,
    output_dir: &Path,
    segment_bytes: u64,
    cancel_flag: Option<Arc<AtomicBool>>,
    progress: Option<ProgressCallback>,
) -> Result<PathBuf, String> {
    if segment_bytes == 0 {
        return Err("segment_bytes must be > 0".into());
    }
    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;
    let evidence = output_dir.join("evidence.raw");
    let package = output_dir.join("split.fsnap");
    let mut request = AcquireRequest::new(source, &evidence)
        .with_buffer_size(64 * 1024)
        .with_split_segment_bytes(segment_bytes);
    if let Some(flag) = cancel_flag {
        request = request.with_cancel_flag(flag);
    }
    if let Some(cb) = progress {
        request = request.with_progress(cb);
    }
    let summary = acquire_file(&request).map_err(|error| error.to_string())?;
    let segments: Vec<PathBuf> = summary
        .segments
        .iter()
        .map(|segment| segment.path.clone())
        .collect();
    if segments.is_empty() {
        return Err("split acquire produced no segments".into());
    }
    create_fsnap_from_segments(&segments, &summary.audit_path, &package)
        .map_err(|error| error.to_string())?;
    verify_fsnap(&package).map_err(|error| error.to_string())?;
    Ok(package)
}

/// Writes the hexadecimal SHA-512 digest of `path` to a `.sha512` sidecar.
pub fn write_sha512_sidecar_for(path: &Path) -> Result<PathBuf, String> {
    let mut reader = BufReader::new(File::open(path).map_err(|error| error.to_string())?);
    let mut hasher = Sha512::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|error| error.to_string())?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    let sidecar = path.with_extension(format!(
        "{}.sha512",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("")
    ));
    std::fs::write(&sidecar, format!("{}\n", hex::encode(hasher.finalize())))
        .map_err(|error| error.to_string())?;
    Ok(sidecar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_raw_produces_verifiable_package() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.bin");
        std::fs::write(&source, vec![0xAB; 200 * 1024]).unwrap();
        let out = dir.path().join("out");
        let package = acquire_split_raw(&source, &out, 64 * 1024).expect("split");
        assert!(package.is_dir());
        verify_fsnap(&package).expect("verify split");
    }

    #[test]
    fn e01_acquisition_writes_requested_container_and_sha512_sidecar() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.bin");
        std::fs::write(&source, b"format fixture").unwrap();

        let output = acquire_to_format_with_sha512(
            &source,
            &dir.path().join("out"),
            UiOutputFormat::E01,
            true,
        )
        .unwrap();

        assert_eq!(output.extension().and_then(|ext| ext.to_str()), Some("e01"));
        assert!(output.is_file());
        assert!(
            !dir.path().join("out").join("format-input.raw").exists(),
            "E01 must stream without staging raw"
        );
        assert_eq!(
            std::fs::read_to_string(output.with_extension("e01.sha512"))
                .unwrap()
                .trim()
                .len(),
            128
        );
    }

    #[test]
    fn e01_cancel_during_raw_stage_surfaces_cancel() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.bin");
        std::fs::write(&source, vec![0xCD; 512 * 1024]).unwrap();
        let flag = Arc::new(AtomicBool::new(true));
        let err = acquire_to_format_with_controls(
            &source,
            &dir.path().join("out"),
            UiOutputFormat::E01,
            false,
            Some(flag),
            None,
        )
        .expect_err("cancel");
        assert!(
            err.to_lowercase().contains("cancel"),
            "unexpected error: {err}"
        );
    }
}
