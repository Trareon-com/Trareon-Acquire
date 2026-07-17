//! Slint desktop shell for Trareon Acquire.

pub mod ui_model;

pub use ui_model::UiSnapshot;

#[cfg(feature = "gui")]
slint::include_modules!();

/// Run a small synthetic file-backed acquire + `.fsnap` package (no GUI).
pub fn run_synthetic_demo(out_dir: &std::path::Path) -> Result<std::path::PathBuf, String> {
    use std::fs;
    use trareon_core::{AcquireRequest, acquire_file, create_fsnap, verify_fsnap};

    fs::create_dir_all(out_dir).map_err(|e| e.to_string())?;
    let source = out_dir.join("synthetic-source.bin");
    let evidence = out_dir.join("evidence.raw");
    let package = out_dir.join("foundation.fsnap");

    fs::write(&source, vec![0u8; 256 * 1024]).map_err(|e| e.to_string())?;
    let _ = fs::remove_file(&evidence);

    let summary = acquire_file(
        &AcquireRequest::new(&source, &evidence)
            .with_max_bytes(256 * 1024)
            .with_buffer_size(64 * 1024),
    )
    .map_err(|e| e.to_string())?;

    create_fsnap(&evidence, &summary.audit_path, &package).map_err(|e| e.to_string())?;
    let manifest = verify_fsnap(&package).map_err(|e| e.to_string())?;
    if manifest.evidence_size == 0 {
        return Err("empty evidence after acquire".into());
    }
    Ok(package)
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
}
