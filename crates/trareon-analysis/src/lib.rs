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
use trareon_core::{AuditJournal, CoreError, FsnapManifestV1, verify_fsnap};

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

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TimelineEvent {
    pub sequence: u64,
    pub timestamp_utc: String,
    pub state: String,
    pub action: String,
    pub event_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct Entry {
    pub relative_path: String,
    pub size: u64,
}

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

/// Return the verified audit trail in display order without modifying the package.
pub fn timeline_from_audit(package: &Path) -> Result<Vec<TimelineEvent>, CoreError> {
    let manifest = verify_fsnap(package)?;
    let journal = AuditJournal::read_jsonl(&package.join(manifest.audit_relative_path))?;
    let events = serde_json::to_value(journal)
        .map_err(|error| CoreError::Serialization(error.to_string()))?
        .get("events")
        .cloned()
        .ok_or_else(|| CoreError::Serialization("audit journal missing events".to_string()))?;
    serde_json::from_value(events).map_err(|error| CoreError::Serialization(error.to_string()))
}

/// List files that belong to a verified package. Symlinks and invalid packages reject first.
pub fn browse_listing(package: &Path) -> Result<Vec<Entry>, CoreError> {
    verify_fsnap(package)?;
    let package_canon =
        fs::canonicalize(package).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut paths = Vec::new();
    collect_entries(&package_canon, &package_canon, &mut paths)?;
    paths.sort_by(|left, right| left.relative_path.cmp(&right.relative_path));
    Ok(paths)
}

fn collect_entries(root: &Path, current: &Path, out: &mut Vec<Entry>) -> Result<(), CoreError> {
    for item in fs::read_dir(current).map_err(|error| CoreError::Io(error.to_string()))? {
        let item = item.map_err(|error| CoreError::Io(error.to_string()))?;
        let path = item.path();
        let metadata =
            fs::symlink_metadata(&path).map_err(|error| CoreError::Io(error.to_string()))?;
        if metadata.file_type().is_symlink() {
            return Err(CoreError::Verification(
                "package content must not be a symlink".to_string(),
            ));
        }
        if metadata.is_dir() {
            collect_entries(root, &path, out)?;
        } else if metadata.is_file() {
            let relative_path = path
                .strip_prefix(root)
                .map_err(|error| CoreError::Io(error.to_string()))?
                .to_string_lossy()
                .to_string();
            out.push(Entry {
                relative_path,
                size: metadata.len(),
            });
        }
    }
    Ok(())
}

/// Produce a standalone inspection report. Callers must write it outside the package.
pub fn analysis_report_html(
    manifest: &FsnapManifestV1,
    timeline: &[TimelineEvent],
    entries: &[Entry],
) -> String {
    fn escape(value: &str) -> String {
        value
            .replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    }

    let timeline_rows = timeline
        .iter()
        .map(|event| {
            format!(
                "<tr><td>{}</td><td>{}</td><td>{}</td><td>{}</td></tr>",
                event.sequence,
                escape(&event.timestamp_utc),
                escape(&event.state),
                escape(&event.action)
            )
        })
        .collect::<String>();
    let entry_rows = entries
        .iter()
        .map(|entry| {
            format!(
                "<tr><td>{}</td><td>{}</td></tr>",
                escape(&entry.relative_path),
                entry.size
            )
        })
        .collect::<String>();

    format!(
        "<!doctype html><html><head><meta charset=\"utf-8\"><title>Trareon Analysis Report</title>\
         <style>body{{font-family:system-ui;margin:2rem;color:#17212b}}table{{border-collapse:collapse;width:100%}}\
         th,td{{border:1px solid #cbd5e1;padding:.45rem;text-align:left}}</style></head><body>\
         <h1>Trareon Analysis Report</h1><p>Read-only report generated from a verified package.</p>\
         <dl><dt>Evidence SHA-256</dt><dd>{}</dd><dt>Evidence size</dt><dd>{}</dd><dt>Audit root</dt><dd>{}</dd></dl>\
         <h2>Timeline</h2><table><tr><th>Sequence</th><th>UTC</th><th>State</th><th>Action</th></tr>{}</table>\
         <h2>Package listing</h2><table><tr><th>Path</th><th>Bytes</th></tr>{}</table></body></html>",
        escape(&manifest.evidence_sha256),
        manifest.evidence_size,
        escape(&manifest.audit_root),
        timeline_rows,
        entry_rows
    )
}
