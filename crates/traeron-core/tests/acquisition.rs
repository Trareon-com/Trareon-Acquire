use std::fs;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
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

#[test]
fn cancellation_stops_acquisition_without_false_complete() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    fs::write(&source, vec![7u8; 4096]).unwrap();

    let cancel_flag = Arc::new(AtomicBool::new(true));
    let request = AcquireRequest::new(&source, &output).with_cancel_flag(cancel_flag);

    let error = acquire_file(&request).unwrap_err();
    assert!(matches!(error, CoreError::Cancelled));

    // A cancelled acquisition must never be mistaken for a completed one:
    // there is no summary to check, and the audit trail (if written) must
    // not claim AcquiredUnverified/VerifiedComplete anywhere.
    if output.with_extension("audit.jsonl").exists() {
        let contents = fs::read_to_string(output.with_extension("audit.jsonl")).unwrap();
        assert!(!contents.contains("acquired_unverified"));
        assert!(contents.contains("cancelled"));
    }
}

#[test]
fn split_raw_boundary_size_produces_one_full_segment() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    let bytes = vec![9u8; 4096];
    fs::write(&source, &bytes).unwrap();

    let request = AcquireRequest::new(&source, &output).with_split_segment_bytes(4096);
    let summary = acquire_file(&request).unwrap();

    assert_eq!(summary.segments.len(), 1);
    assert_eq!(summary.segments[0].size, 4096);
    assert_eq!(fs::read(&summary.segments[0].path).unwrap(), bytes);
}

#[test]
fn split_raw_zero_length_final_segment_is_not_created() {
    // Total bytes are an exact multiple of the segment size: there must be
    // no trailing empty segment file after the last full one.
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    let bytes = vec![3u8; 8192];
    fs::write(&source, &bytes).unwrap();

    let request = AcquireRequest::new(&source, &output).with_split_segment_bytes(4096);
    let summary = acquire_file(&request).unwrap();

    assert_eq!(summary.segments.len(), 2);
    assert_eq!(summary.segments[0].size, 4096);
    assert_eq!(summary.segments[1].size, 4096);
    assert!(!dir.path().join("evidence.003.raw").exists());
}

#[test]
fn split_raw_final_short_segment_and_reassembly() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    let bytes: Vec<u8> = (0..=255).cycle().take(4096 * 2 + 777).collect();
    fs::write(&source, &bytes).unwrap();

    let request = AcquireRequest::new(&source, &output).with_split_segment_bytes(4096);
    let summary = acquire_file(&request).unwrap();

    // segment-order: three segments, in the order they were written.
    assert_eq!(summary.segments.len(), 3);
    assert!(summary.segments[0].path.ends_with("evidence.001.raw"));
    assert!(summary.segments[1].path.ends_with("evidence.002.raw"));
    assert!(summary.segments[2].path.ends_with("evidence.003.raw"));

    // final-short-segment: the last segment is shorter than the limit.
    assert_eq!(summary.segments[0].size, 4096);
    assert_eq!(summary.segments[1].size, 4096);
    assert_eq!(summary.segments[2].size, 777);

    // reassembly: concatenating segments in order reproduces the source
    // byte-for-byte.
    let mut reassembled = Vec::new();
    for segment in &summary.segments {
        reassembled.extend(fs::read(&segment.path).unwrap());
    }
    assert_eq!(reassembled, bytes);
    assert_eq!(summary.bytes_written, bytes.len() as u64);
}

#[test]
fn destination_write_failure_produces_no_false_complete() {
    // Portable, deterministic substitute for a destination-full simulation:
    // point the output at a path whose parent component is itself a file
    // (not a directory), which every OS rejects deterministically at file
    // creation time without needing to fill a real disk.
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    fs::write(&source, b"evidence").unwrap();

    let blocking_file = dir.path().join("not_a_directory");
    fs::write(&blocking_file, b"x").unwrap();
    let output = blocking_file.join("evidence.raw");

    let error = acquire_file(&AcquireRequest::new(&source, &output));
    assert!(error.is_err());
    assert!(!output.exists());
}
