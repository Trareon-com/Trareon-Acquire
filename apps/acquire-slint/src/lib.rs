//! Slint desktop shell for Trareon Acquire.

pub mod acquire_formats;
pub mod analysis_ui;
pub mod boot_media;
pub mod cases;
pub mod draft;
pub mod identify;
pub mod multisource;
pub mod preflight;
pub mod prefs;
pub mod preserve;
pub mod profiles;
pub mod qms;
pub mod recent;
pub mod shell_ops;
pub mod sources_ext;
pub mod tools_hub;
pub mod triage_ui;
pub mod ui_model;

pub use ui_model::{NONE_SENTINEL, UiLocale, UiMode, UiSnapshot, dest_equals_source_volume};

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use trareon_core::{
    AcquirePhase, AcquireProgress, AcquireRequest, ProgressCallback, acquire_file, create_fsnap,
    verify_fsnap,
};

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
    run_foundation_demo_with_progress(source, output_dir, cancel_flag, None)
}

pub fn run_foundation_demo_with_progress(
    source: &Path,
    output_dir: &Path,
    cancel_flag: Option<Arc<AtomicBool>>,
    progress: Option<ProgressCallback>,
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
    let progress_cb = progress.clone();
    if let Some(cb) = progress {
        request = request.with_progress(cb);
    }

    let emit = |phase: AcquirePhase, msg: &str| {
        if let Some(cb) = &progress_cb {
            cb(AcquireProgress::new(phase, 0, None, msg));
        }
    };

    let summary = acquire_file(&request).map_err(|error| error.to_string())?;
    emit(AcquirePhase::Packaging, "packaging");
    create_fsnap(&evidence_path, &summary.audit_path, &package_path)
        .map_err(|error| error.to_string())?;
    emit(AcquirePhase::Verifying, "verifying");
    let manifest = verify_fsnap(&package_path).map_err(|error| error.to_string())?;
    emit(AcquirePhase::Done, "verified_complete");

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

/// Fixed-folder synthetic packages for manual GUI smoke (Tools / Recent / Compare).
///
/// Layout under `root`:
/// - `pkg-a/foundation.fsnap` — sealed package (zeros)
/// - `pkg-b/foundation.fsnap` — different bytes (compare DIFF)
/// - `PATHS.txt` — operator copy paths
/// - seeds `~/.trareon/acquire-recent.json` + prefs `last_output_dir`
pub fn seed_synthetic_gui_smoke(root: &Path) -> Result<PathBuf, String> {
    let _ = std::fs::remove_dir_all(root);
    std::fs::create_dir_all(root).map_err(|e| e.to_string())?;
    let a_dir = root.join("pkg-a");
    let b_dir = root.join("pkg-b");
    let out_dir = root.join("out");
    std::fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;

    let package_a =
        std::fs::canonicalize(run_synthetic_demo(&a_dir)?).map_err(|e| e.to_string())?;
    let source_b = b_dir.join("synthetic-source.bin");
    std::fs::create_dir_all(&b_dir).map_err(|e| e.to_string())?;
    std::fs::write(&source_b, vec![0xA5u8; 256 * 1024]).map_err(|e| e.to_string())?;
    let package_b = std::fs::canonicalize(PathBuf::from(
        run_foundation_demo(&source_b, &b_dir, None)
            .map_err(|e| e.to_string())?
            .package_path,
    ))
    .map_err(|e| e.to_string())?;
    let source_a =
        std::fs::canonicalize(a_dir.join("synthetic-source.bin")).map_err(|e| e.to_string())?;
    let source_b = std::fs::canonicalize(&source_b).map_err(|e| e.to_string())?;
    let out_dir = std::fs::canonicalize(&out_dir).map_err(|e| e.to_string())?;
    let root = std::fs::canonicalize(root).unwrap_or_else(|_| root.to_path_buf());

    let _ = preserve::seal_package(
        &package_a,
        "SYNTH-GUI-SMOKE",
        "Synthetic Examiner",
        "synth-disk-a",
    )?;
    let _ = preserve::sign_package(&package_a, &a_dir.join("keys"))?;

    recent::append(recent::RecentEntry::completed(
        "SYNTH-GUI-SMOKE",
        package_a.to_string_lossy().as_ref(),
    ));
    recent::append(recent::RecentEntry::completed(
        "SYNTH-GUI-SMOKE-B",
        package_b.to_string_lossy().as_ref(),
    ));

    let mut prefs = prefs::AcquirePrefs::load();
    prefs.last_output_dir = out_dir.display().to_string();
    prefs.save();

    let paths = format!(
        "Synthetic GUI smoke packages\n\
         \n\
         Package A (sealed, in Recent):\n  {}\n\
         Package B (compare DIFF):\n  {}\n\
         Source A (Identify / Acquire file):\n  {}\n\
         Source B:\n  {}\n\
         Output dir (prefs):\n  {}\n\
         CoC / QR (beside A):\n  {}\n  {}\n\
         \n\
         GUI: Tools → click Recent row, or Browse package → pick foundation.fsnap folder.\n\
         Compare: paste Package B path into compare B, then Compare.\n",
        package_a.display(),
        package_b.display(),
        source_a.display(),
        source_b.display(),
        out_dir.display(),
        a_dir.join("coc.json").display(),
        a_dir.join("qr.png").display(),
    );
    std::fs::write(root.join("PATHS.txt"), &paths).map_err(|e| e.to_string())?;
    Ok(package_a)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    #[test]
    fn synthetic_demo_produces_verifiable_package() {
        let dir = tempfile::tempdir().unwrap();
        let package = run_synthetic_demo(dir.path()).expect("demo");
        assert!(package.is_dir());
        assert!(package.join("manifest/manifest.json").is_file());
        assert!(package.join("acquisitions/0001/evidence.raw").is_file());
    }

    #[test]
    #[ignore = "writes tmp/ + seeds ~/.trareon recent; run via scripts/seed-synthetic-gui-smoke.sh"]
    fn seed_fixed_synthetic_packages_for_gui_smoke() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../tmp/synthetic-gui-smoke");
        let package = seed_synthetic_gui_smoke(&root).expect("seed");
        assert!(package.join("manifest/manifest.json").is_file());
        assert!(root.join("PATHS.txt").is_file());
        assert!(root.join("pkg-a/qr.png").is_file());
        eprintln!(
            "{}",
            std::fs::read_to_string(root.join("PATHS.txt")).unwrap()
        );
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

    #[test]
    fn progress_callback_is_monotonic() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.bin");
        std::fs::write(&source, vec![0xAB; 256 * 1024]).unwrap();
        let out = dir.path().join("out");
        let samples = Arc::new(Mutex::new(Vec::<u64>::new()));
        let samples_cb = Arc::clone(&samples);
        let cb: ProgressCallback = Arc::new(move |p: AcquireProgress| {
            if p.phase == AcquirePhase::Acquiring {
                samples_cb.lock().unwrap().push(p.bytes_done);
            }
        });
        run_foundation_demo_with_progress(&source, &out, None, Some(cb)).expect("demo");
        let vals = samples.lock().unwrap();
        assert!(!vals.is_empty());
        for w in vals.windows(2) {
            assert!(w[1] >= w[0], "progress must be monotonic");
        }
    }

    #[test]
    fn preserve_helpers_create_a_complete_preservation_set() {
        let dir = tempfile::tempdir().unwrap();
        let package = run_synthetic_demo(dir.path()).expect("demo");

        let coc = crate::preserve::seal_package(&package, "CASE-42", "Examiner", "disk-serial")
            .expect("seal package");
        assert_eq!(coc.case_ref, "CASE-42");
        assert!(dir.path().join("coc.json").is_file());
        assert!(dir.path().join("qr.png").is_file());
        assert!(dir.path().join("sticker.html").is_file());
        assert!(dir.path().join("custody.jsonl").is_file());

        let signature = crate::preserve::sign_package(&package, &dir.path().join("keys"))
            .expect("sign package");
        assert!(signature.is_file());

        let copies =
            crate::preserve::make_working_and_archive_copies(&package, &dir.path().join("copies"))
                .expect("make copies");
        assert!(copies.working.join("manifest/manifest.json").is_file());
        assert!(copies.archive.join("manifest/manifest.json").is_file());
        assert!(copies.manifest.is_file());

        let report = crate::preserve::write_acquisition_report(
            &package,
            "CASE-42",
            "synthetic source",
            &["Synthetic acquisition".to_string()],
        )
        .expect("write report");
        assert!(report.is_file());
    }

    #[test]
    fn stop_reason_is_written_next_to_checkpoint() {
        let dir = tempfile::tempdir().unwrap();
        let checkpoint = dir.path().join("acquire.checkpoint.json");
        crate::preserve::write_stop_reason(&checkpoint, crate::preserve::StopReason::Paused)
            .expect("write pause reason");
        assert_eq!(
            std::fs::read_to_string(dir.path().join(".acquire-stop-reason")).unwrap(),
            "paused\n"
        );
    }
}
