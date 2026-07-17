//! Read-only Trareon Analysis importer for `.fsnap` v0.1.
//!
//! Verifies before import, writes indexes **outside** the evidence package,
//! and never repairs or upgrades the package.

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use chrono::Utc;
use trareon_core::{CoreError, FsnapManifestV1, verify_fsnap};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AnalysisImportIndex {
    pub schema: String,
    pub package_path: String,
    pub evidence_sha256: String,
    pub evidence_size: u64,
    pub audit_root: String,
    pub imported_at: String,
    pub verify_status: String,
}

const INDEX_SCHEMA: &str = "trareon.analysis.import-index/1";

fn ensure_index_outside_package(package: &Path, index_dir: &Path) -> Result<(), CoreError> {
    let package_canon =
        fs::canonicalize(package).map_err(|error| CoreError::Io(error.to_string()))?;
    fs::create_dir_all(index_dir).map_err(|error| CoreError::Io(error.to_string()))?;
    let index_canon =
        fs::canonicalize(index_dir).map_err(|error| CoreError::Io(error.to_string()))?;
    if index_canon.starts_with(&package_canon) {
        return Err(CoreError::Verification(
            "analysis index directory must not be inside the evidence package".to_string(),
        ));
    }
    Ok(())
}

/// Verify a package then write a sidecar index JSON under `index_dir`.
/// The package tree is never modified.
pub fn import_fsnap_readonly(
    package: &Path,
    index_dir: &Path,
) -> Result<(FsnapManifestV1, PathBuf), CoreError> {
    ensure_index_outside_package(package, index_dir)?;
    let manifest = verify_fsnap(package)?;

    let index = AnalysisImportIndex {
        schema: INDEX_SCHEMA.to_string(),
        package_path: package.to_string_lossy().to_string(),
        evidence_sha256: manifest.evidence_sha256.clone(),
        evidence_size: manifest.evidence_size,
        audit_root: manifest.audit_root.clone(),
        imported_at: Utc::now().to_rfc3339(),
        verify_status: "valid".to_string(),
    };

    let index_path = index_dir.join("import-index.json");
    let bytes = serde_json::to_vec_pretty(&index)
        .map_err(|error| CoreError::Serialization(error.to_string()))?;
    let mut file = File::create(&index_path).map_err(|error| CoreError::Io(error.to_string()))?;
    file.write_all(&bytes)
        .map_err(|error| CoreError::Io(error.to_string()))?;
    file.sync_all()
        .map_err(|error| CoreError::Io(error.to_string()))?;

    Ok((manifest, index_path))
}
