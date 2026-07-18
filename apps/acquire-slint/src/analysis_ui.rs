//! Read-only bridge from Acquire to the Analysis sidecar.

use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

use trareon_analysis::{
    Entry, TimelineEvent, analysis_report_html, browse_listing, import_fsnap_readonly,
    timeline_from_audit,
};
use trareon_core::FsnapManifestV1;

#[derive(Debug, Clone)]
pub struct AnalysisUiResult {
    pub manifest: FsnapManifestV1,
    pub index_path: PathBuf,
    pub report_path: PathBuf,
    pub timeline: Vec<TimelineEvent>,
    pub listing: Vec<Entry>,
}

/// Import and inspect a package. All generated artifacts are sidecars outside it.
pub fn import_for_analysis(
    package: &Path,
    index_dir: &Path,
    report_dir: &Path,
) -> Result<AnalysisUiResult, String> {
    let (manifest, index_path) =
        import_fsnap_readonly(package, index_dir).map_err(|error| error.to_string())?;
    let timeline = timeline_from_audit(package).map_err(|error| error.to_string())?;
    let listing = browse_listing(package).map_err(|error| error.to_string())?;
    let report_path = write_report_outside_package(
        package,
        report_dir,
        &analysis_report_html(&manifest, &timeline, &listing),
    )?;

    Ok(AnalysisUiResult {
        manifest,
        index_path,
        report_path,
        timeline,
        listing,
    })
}

fn write_report_outside_package(
    package: &Path,
    report_dir: &Path,
    body: &str,
) -> Result<PathBuf, String> {
    let package_canon = fs::canonicalize(package).map_err(|error| error.to_string())?;
    fs::create_dir_all(report_dir).map_err(|error| error.to_string())?;
    let report_dir_canon = fs::canonicalize(report_dir).map_err(|error| error.to_string())?;
    if report_dir_canon.starts_with(&package_canon) {
        return Err(
            "analysis report directory must not be inside the evidence package".to_string(),
        );
    }
    let report_path = report_dir.join("analysis-report.html");
    let mut file = File::create(&report_path).map_err(|error| error.to_string())?;
    file.write_all(body.as_bytes())
        .map_err(|error| error.to_string())?;
    file.sync_all().map_err(|error| error.to_string())?;
    Ok(report_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn analysis_writes_only_sidecars() {
        let temp = tempfile::tempdir().unwrap();
        let package = crate::run_synthetic_demo(temp.path()).unwrap();
        let before = trareon_core::verify_fsnap(&package)
            .unwrap()
            .evidence_sha256;
        let result = import_for_analysis(
            &package,
            &temp.path().join("index"),
            &temp.path().join("report"),
        )
        .unwrap();
        assert!(result.index_path.is_file());
        assert!(result.report_path.is_file());
        assert!(!result.report_path.starts_with(&package));
        assert_eq!(
            trareon_core::verify_fsnap(&package)
                .unwrap()
                .evidence_sha256,
            before
        );
    }
}
