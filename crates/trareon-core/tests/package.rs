use std::fs;
use tempfile::tempdir;
use trareon_core::{create_fsnap, verify_fsnap};

#[test]
fn package_verifier_rejects_modified_evidence() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let raw = dir.path().join("evidence.raw");
    let package = dir.path().join("case.fsnap");
    fs::write(&source, b"known forensic fixture").unwrap();
    let summary =
        trareon_core::acquire_file(&trareon_core::AcquireRequest::new(&source, &raw)).unwrap();
    create_fsnap(&raw, &summary.audit_path, &package).unwrap();
    assert!(verify_fsnap(&package).is_ok());

    fs::write(package.join("acquisitions/0001/evidence.raw"), b"tampered").unwrap();
    assert!(verify_fsnap(&package).is_err());
}

#[test]
fn split_raw_segments_roundtrip_in_fsnap() {
    use std::path::PathBuf;
    use trareon_core::{AcquireRequest, acquire_file, create_fsnap_from_segments, verify_fsnap};

    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    let package = dir.path().join("split.fsnap");
    let bytes = vec![0xABu8; 4096 + 4096 + 777];
    fs::write(&source, &bytes).unwrap();
    let summary =
        acquire_file(&AcquireRequest::new(&source, &output).with_split_segment_bytes(4096))
            .unwrap();
    assert_eq!(summary.segments.len(), 3);
    let paths: Vec<PathBuf> = summary.segments.iter().map(|s| s.path.clone()).collect();
    create_fsnap_from_segments(&paths, &summary.audit_path, &package).unwrap();
    let manifest = verify_fsnap(&package).unwrap();
    assert_eq!(manifest.evidence_size, bytes.len() as u64);
    assert_eq!(manifest.evidence_sha256, summary.sha256);
    let segments = manifest.evidence_segments.expect("segments present");
    assert_eq!(segments.len(), 3);
    assert!(package.join("acquisitions/0001/evidence.001.raw").is_file());
    assert!(package.join("acquisitions/0001/evidence.003.raw").is_file());
    assert!(!package.join("acquisitions/0001/evidence.raw").exists());
}

#[test]
fn split_raw_package_rejects_tampered_segment() {
    use std::path::PathBuf;
    use trareon_core::{AcquireRequest, acquire_file, create_fsnap_from_segments, verify_fsnap};

    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    let package = dir.path().join("split.fsnap");
    fs::write(&source, vec![0x11u8; 5000]).unwrap();
    let summary =
        acquire_file(&AcquireRequest::new(&source, &output).with_split_segment_bytes(2048))
            .unwrap();
    let paths: Vec<PathBuf> = summary.segments.iter().map(|s| s.path.clone()).collect();
    create_fsnap_from_segments(&paths, &summary.audit_path, &package).unwrap();
    assert!(verify_fsnap(&package).is_ok());
    fs::write(
        package.join("acquisitions/0001/evidence.002.raw"),
        b"tampered-segment",
    )
    .unwrap();
    assert!(verify_fsnap(&package).is_err());
}
