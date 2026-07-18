use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;
use trareon_analysis::{
    analysis_report_html, browse_listing, import_fsnap_readonly, timeline_from_audit,
};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures")
        .join("fsnap-v0.1")
}

fn package_fingerprint(package: &Path) -> Vec<(String, Vec<u8>)> {
    let mut entries = Vec::new();
    collect_files(package, package, &mut entries);
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    entries
}

fn collect_files(root: &Path, current: &Path, entries: &mut Vec<(String, Vec<u8>)>) {
    for entry in fs::read_dir(current).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            collect_files(root, &path, entries);
        } else if path.is_file() {
            entries.push((
                path.strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
                fs::read(path).unwrap(),
            ));
        }
    }
}

#[test]
fn imports_valid_golden_outside_package() {
    let dir = tempdir().unwrap();
    let index_dir = dir.path().join("index");
    let package = fixtures_root().join("valid");
    let before = package_fingerprint(&package);
    let (manifest, index_path) = import_fsnap_readonly(&package, &index_dir).unwrap();
    assert!(manifest.evidence_size > 0);
    assert!(index_path.starts_with(&index_dir));
    assert!(!index_path.starts_with(&package));
    let body = fs::read_to_string(&index_path).unwrap();
    assert!(body.contains("\"verify_status\": \"valid\""));
    assert_eq!(package_fingerprint(&package), before);
}

#[test]
fn rejects_all_invalid_goldens_without_writing_index() {
    let cases = [
        "mutated",
        "truncated",
        "removed-file",
        "audit-discontinuous",
        "unsupported-version",
    ];
    for name in cases {
        let dir = tempdir().unwrap();
        let index_dir = dir.path().join("idx");
        let package = fixtures_root().join(name);
        let before = package_fingerprint(&package);
        let err = import_fsnap_readonly(&package, &index_dir).unwrap_err();
        assert!(!err.to_string().is_empty(), "{name} should reject");
        assert!(!index_dir.join("import-index.json").exists());
        assert_eq!(
            package_fingerprint(&package),
            before,
            "{name} package must remain unmodified"
        );
    }
}

#[test]
fn rejects_index_dir_inside_package() {
    let package = fixtures_root().join("valid");
    let inside = package.join("nested-index");
    let err = import_fsnap_readonly(&package, &inside).unwrap_err();
    assert!(err.to_string().contains("must not be inside"));
}

#[test]
fn browsing_timeline_and_report_leave_package_unchanged() {
    let package = fixtures_root().join("valid");
    let before = package_fingerprint(&package);
    let manifest = trareon_core::verify_fsnap(&package).unwrap();
    let timeline = timeline_from_audit(&package).unwrap();
    let listing = browse_listing(&package).unwrap();
    let report = analysis_report_html(&manifest, &timeline, &listing);

    assert!(!timeline.is_empty());
    assert!(
        listing
            .iter()
            .any(|entry| entry.relative_path.ends_with("manifest.json"))
    );
    assert!(report.contains("Read-only report"));
    assert_eq!(package_fingerprint(&package), before);
}
