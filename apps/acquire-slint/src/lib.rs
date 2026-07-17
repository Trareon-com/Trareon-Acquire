//! Slint desktop shell for Trareon Acquire.

pub mod ui_model;

pub use ui_model::{UiMode, UiSnapshot};

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use trareon_core::{AcquireRequest, acquire_file, create_fsnap, verify_fsnap};

#[cfg(feature = "gui")]
slint::include_modules!();

/// Result of a foundation acquire → package → verify run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationDemoResult {
    pub status: String,
    pub package_path: String,
    pub evidence_sha256: String,
    pub evidence_size: u64,
}

/// File-backed foundation demo (same contract as the former Tauri demo).
///
/// When `cancel_flag` is set during streaming reads, returns an error containing
/// `cancel` (does not report `verified_complete`).
pub fn run_foundation_demo(
    source: &Path,
    output_dir: &Path,
    cancel_flag: Option<Arc<AtomicBool>>,
) -> Result<FoundationDemoResult, String> {
    if !source.is_file() {
        return Err(format!("source path is not a file: {}", source.display()));
    }

    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;

    let evidence_path = output_dir.join("evidence.raw");
    let package_path = output_dir.join("foundation.fsnap");
    let _ = std::fs::remove_file(&evidence_path);

    let mut request = AcquireRequest::new(source, &evidence_path).with_buffer_size(64 * 1024);
    if let Some(flag) = cancel_flag {
        request = request.with_cancel_flag(flag);
    }

    let summary = acquire_file(&request).map_err(|error| error.to_string())?;
    create_fsnap(&evidence_path, &summary.audit_path, &package_path)
        .map_err(|error| error.to_string())?;
    let manifest = verify_fsnap(&package_path).map_err(|error| error.to_string())?;

    Ok(FoundationDemoResult {
        status: "verified_complete".to_string(),
        package_path: package_path.to_string_lossy().to_string(),
        evidence_sha256: manifest.evidence_sha256,
        evidence_size: manifest.evidence_size,
    })
}

/// Write a minimal operator CoC/report JSON next to a package (Hari 58).
pub fn write_coc_summary(path: &Path, body: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    std::fs::write(path, body).map_err(|e| e.to_string())
}

/// Convenience: write a small synthetic source, then run the foundation demo.
pub fn run_synthetic_demo(out_dir: &Path) -> Result<PathBuf, String> {
    std::fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let source = out_dir.join("synthetic-source.bin");
    std::fs::write(&source, vec![0u8; 256 * 1024]).map_err(|e| e.to_string())?;
    let result = run_foundation_demo(&source, out_dir, None)?;
    Ok(PathBuf::from(result.package_path))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synthetic_demo_produces_verifiable_package() {
        let dir = tempfile::tempdir().unwrap();
        let package = run_synthetic_demo(dir.path()).expect("demo");
        assert!(package.is_dir());
        assert!(package.join("manifest/manifest.json").is_file());
        assert!(package.join("acquisitions/0001/evidence.raw").is_file());
    }

    #[test]
    fn foundation_demo_reports_sha_and_size() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.bin");
        std::fs::write(&source, vec![0u8; 32 * 1024]).unwrap();
        let out = dir.path().join("out");
        let result = run_foundation_demo(&source, &out, None).expect("demo");
        assert_eq!(result.status, "verified_complete");
        assert_eq!(result.evidence_size, 32 * 1024);
        assert_eq!(result.evidence_sha256.len(), 64);
    }

    #[test]
    fn prearmed_cancel_never_reports_verified_complete() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.bin");
        std::fs::write(&source, vec![0u8; 1024 * 64]).unwrap();
        let out = dir.path().join("out");
        let flag = Arc::new(AtomicBool::new(true));
        let err = run_foundation_demo(&source, &out, Some(flag)).unwrap_err();
        assert!(err.to_lowercase().contains("cancel"));
    }

    #[test]
    fn write_coc_summary_creates_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("coc.json");
        write_coc_summary(&path, "{\"ok\":true}\n").unwrap();
        assert_eq!(std::fs::read_to_string(path).unwrap(), "{\"ok\":true}\n");
    }
}
