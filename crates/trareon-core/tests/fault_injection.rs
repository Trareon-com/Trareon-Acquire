//! Portable synthetic fault-injection cases for M2 prep (no raw devices).
//!
//! Maps to `docs/M1-FAILURE-MATRIX.md` semantics: never invent VerifiedComplete.

use std::fs;
use tempfile::tempdir;
use trareon_core::{
    AcquireRequest, CoreError, acquire_file, assert_source_permitted, write_allowlist_template,
};

#[test]
fn system_disk_path_never_acquires() {
    let dir = tempdir().unwrap();
    let output = dir.path().join("out.raw");
    let err = acquire_file(&AcquireRequest::new(r"\\.\PhysicalDrive0", &output)).unwrap_err();
    assert!(
        err.to_string().contains("hard-denied") || err.to_string().contains("unavailable"),
        "{err}"
    );
    assert!(!output.exists());
}

#[test]
fn source_destination_parent_collision_is_not_complete() {
    // Destination under a file path component — portable destination-full substitute.
    let dir = tempdir().unwrap();
    let source = dir.path().join("src.img");
    fs::write(&source, b"evidence-bytes").unwrap();
    let blocker = dir.path().join("blocker");
    fs::write(&blocker, b"x").unwrap();
    let output = blocker.join("evidence.raw");
    assert!(acquire_file(&AcquireRequest::new(&source, &output)).is_err());
    assert!(!output.exists());
}

#[test]
fn allowlist_template_is_not_human_approved() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("allowlist.json");
    write_allowlist_template(&path).unwrap();
    let body = fs::read_to_string(&path).unwrap();
    assert!(body.contains("\"human_approved\": false"));
    assert!(body.contains("trareon.lab-allowlist/1"));
}

#[test]
fn empty_source_never_reports_success() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("empty.img");
    fs::write(&source, b"").unwrap();
    let err = acquire_file(&AcquireRequest::new(&source, dir.path().join("out.raw"))).unwrap_err();
    assert!(matches!(err, CoreError::Verification(_)));
}

#[test]
fn assert_source_permitted_rejects_nvme_system_disk() {
    let err = assert_source_permitted(std::path::Path::new("/dev/nvme0n1p1"), None).unwrap_err();
    assert!(err.to_string().contains("hard-denied"));
}
