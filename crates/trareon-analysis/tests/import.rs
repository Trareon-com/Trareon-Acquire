use std::path::{Path, PathBuf};
use tempfile::tempdir;
use trareon_analysis::import_fsnap_readonly;

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join("fixtures")
        .join("fsnap-v0.1")
}

#[test]
fn imports_valid_golden_outside_package() {
    let dir = tempdir().unwrap();
    let index_dir = dir.path().join("index");
    let package = fixtures_root().join("valid");
    let (manifest, index_path) = import_fsnap_readonly(&package, &index_dir).unwrap();
    assert_eq!(manifest.evidence_size > 0, true);
    assert!(index_path.starts_with(&index_dir));
    assert!(!index_path.starts_with(&package));
    let body = std::fs::read_to_string(&index_path).unwrap();
    assert!(body.contains("\"verify_status\": \"valid\""));
}

#[test]
fn rejects_mutated_golden() {
    let dir = tempdir().unwrap();
    let err = import_fsnap_readonly(&fixtures_root().join("mutated"), &dir.path().join("idx"))
        .unwrap_err();
    let msg = err.to_string();
    assert!(!msg.is_empty());
}

#[test]
fn rejects_index_dir_inside_package() {
    let package = fixtures_root().join("valid");
    let inside = package.join("nested-index");
    let err = import_fsnap_readonly(&package, &inside).unwrap_err();
    assert!(err.to_string().contains("must not be inside"));
}
