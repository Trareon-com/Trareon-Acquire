use std::fs;
use tempfile::tempdir;
use traeron_core::{create_fsnap, verify_fsnap};

#[test]
fn package_verifier_rejects_modified_evidence() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let raw = dir.path().join("evidence.raw");
    let package = dir.path().join("case.fsnap");
    fs::write(&source, b"known forensic fixture").unwrap();
    let summary =
        traeron_core::acquire_file(&traeron_core::AcquireRequest::new(&source, &raw)).unwrap();
    create_fsnap(&raw, &summary.audit_path, &package).unwrap();
    assert!(verify_fsnap(&package).is_ok());

    fs::write(package.join("acquisitions/0001/evidence.raw"), b"tampered").unwrap();
    assert!(verify_fsnap(&package).is_err());
}
