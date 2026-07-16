use std::fs;
use tempfile::tempdir;
use traeron_core::{AcquireRequest, AcquisitionState, CoreError, acquire_file};

#[test]
fn acquisition_copies_every_byte_and_hashes_output() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    let bytes: Vec<u8> = (0..=255).cycle().take(1_048_777).collect();
    fs::write(&source, &bytes).unwrap();

    let summary = acquire_file(&AcquireRequest::new(&source, &output)).unwrap();
    assert_eq!(fs::read(&output).unwrap(), bytes);
    assert_eq!(summary.bytes_read, 1_048_777);
    assert_eq!(summary.bytes_written, 1_048_777);
    assert_eq!(summary.state, AcquisitionState::AcquiredUnverified);
    assert_eq!(summary.sha256.len(), 64);
}

#[test]
fn acquisition_rejects_same_source_and_destination() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("same.img");
    fs::write(&source, b"evidence").unwrap();
    let error = acquire_file(&AcquireRequest::new(&source, &source)).unwrap_err();
    assert!(matches!(error, CoreError::Verification(_)));
}
