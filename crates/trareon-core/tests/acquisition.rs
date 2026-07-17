use std::fs;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tempfile::tempdir;
use trareon_core::{
    AcquireRequest, AcquisitionState, CoreError, acquire_file, default_checkpoint_path,
    load_checkpoint,
};

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

#[test]
fn cancel_writes_incomplete_checkpoint_never_verified_complete() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    fs::write(&source, vec![5u8; 1024 * 64]).unwrap();

    // Pre-arm cancel so the loop exits before any byte is claimed complete
    // (no timing race across CI hosts).
    let cancel_flag = Arc::new(AtomicBool::new(true));
    let request = AcquireRequest::new(&source, &output).with_cancel_flag(cancel_flag);
    let error = acquire_file(&request).unwrap_err();
    assert!(matches!(error, CoreError::Cancelled));

    let checkpoint_path = default_checkpoint_path(&output);
    assert!(checkpoint_path.exists(), "cancel must leave a checkpoint");
    let checkpoint = load_checkpoint(&checkpoint_path).unwrap();
    assert!(checkpoint.incomplete);
    assert_eq!(
        checkpoint.bytes_completed,
        fs::metadata(&output).map(|m| m.len()).unwrap_or(0)
    );

    let audit = fs::read_to_string(output.with_extension("audit.jsonl")).unwrap();
    assert!(audit.contains("cancelled"));
    assert!(!audit.contains("verified_complete"));
    assert!(!audit.contains("\"state\":\"VerifiedComplete\""));
}

#[test]
fn resume_after_cancel_matches_full_hash_and_clears_checkpoint() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    let bytes: Vec<u8> = (0..=255).cycle().take(200_000).collect();
    fs::write(&source, &bytes).unwrap();

    let full = acquire_file(&AcquireRequest::new(&source, dir.path().join("full.raw"))).unwrap();

    // Seed a mid-file cancel by writing a prefix + checkpoint, then resume.
    let prefix = &bytes[..50_000];
    fs::write(&output, prefix).unwrap();
    let checkpoint_path = default_checkpoint_path(&output);
    let cp = trareon_core::AcquisitionCheckpoint::new(&source, &output, prefix.len() as u64);
    trareon_core::write_checkpoint(&checkpoint_path, &cp).unwrap();

    let resumed = acquire_file(
        &AcquireRequest::new(&source, &output)
            .with_resume(true)
            .with_checkpoint_path(&checkpoint_path),
    )
    .unwrap();

    assert_eq!(resumed.sha256, full.sha256);
    assert_eq!(fs::read(&output).unwrap(), bytes);
    assert_eq!(resumed.state, AcquisitionState::AcquiredUnverified);
    assert!(
        !checkpoint_path.exists(),
        "successful resume must clear checkpoint"
    );
}

#[test]
fn checkpoint_claiming_complete_is_rejected() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("bad.checkpoint.json");
    fs::write(
        &path,
        r#"{
  "schema": "trareon.checkpoint/1",
  "source": "/tmp/a",
  "output": "/tmp/b",
  "bytes_completed": 10,
  "incomplete": false
}"#,
    )
    .unwrap();
    let err = load_checkpoint(&path).unwrap_err();
    assert!(matches!(err, CoreError::Verification(_)));
}

#[test]
fn resume_split_after_partial_segments_matches_full_hash() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    let bytes: Vec<u8> = (0..=255).cycle().take(4096 * 2 + 777).collect();
    fs::write(&source, &bytes).unwrap();

    let full = acquire_file(
        &AcquireRequest::new(&source, dir.path().join("full.raw")).with_split_segment_bytes(4096),
    )
    .unwrap();

    // Seed two full segments + a short prefix of the third, then checkpoint.
    fs::write(dir.path().join("evidence.001.raw"), &bytes[..4096]).unwrap();
    fs::write(dir.path().join("evidence.002.raw"), &bytes[4096..8192]).unwrap();
    fs::write(
        dir.path().join("evidence.003.raw"),
        &bytes[8192..8192 + 100],
    )
    .unwrap();
    let checkpoint_path = default_checkpoint_path(&output);
    let cp =
        trareon_core::AcquisitionCheckpoint::new(&source, &output, 8192 + 100).with_split(4096);
    trareon_core::write_checkpoint(&checkpoint_path, &cp).unwrap();

    let resumed = acquire_file(
        &AcquireRequest::new(&source, &output)
            .with_split_segment_bytes(4096)
            .with_resume(true)
            .with_checkpoint_path(&checkpoint_path),
    )
    .unwrap();

    assert_eq!(resumed.sha256, full.sha256);
    assert_eq!(resumed.segments.len(), 3);
    assert!(!checkpoint_path.exists());
}

#[test]
fn max_bytes_bounds_file_acquire() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    fs::write(&source, vec![9u8; 10_000]).unwrap();
    let summary = acquire_file(
        &AcquireRequest::new(&source, &output)
            .with_max_bytes(1500)
            .with_buffer_size(512),
    )
    .unwrap();
    assert_eq!(summary.bytes_written, 1500);
    assert_eq!(fs::metadata(&output).unwrap().len(), 1500);
}

#[test]
fn raw_device_without_max_bytes_is_rejected_even_if_allowlisted() {
    let dir = tempdir().unwrap();
    let allow = dir.path().join("allow.json");
    fs::write(
        &allow,
        r#"{
  "schema": "trareon.lab-allowlist/1",
  "human_approved": true,
  "approved_by": "test",
  "entries": [{"source_identity": "/dev/rdisk10", "notes": "t"}]
}"#,
    )
    .unwrap();
    let err = acquire_file(
        &AcquireRequest::new("/dev/rdisk10", dir.path().join("out.raw"))
            .with_lab_allowlist_path(&allow),
    )
    .unwrap_err();
    assert!(err.to_string().contains("max_bytes"), "{err}");
}

#[test]
fn resume_split_mismatch_settings_is_rejected() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    fs::write(&source, b"abc").unwrap();
    let checkpoint_path = default_checkpoint_path(&output);
    let cp = trareon_core::AcquisitionCheckpoint::new(&source, &output, 0).with_split(64);
    trareon_core::write_checkpoint(&checkpoint_path, &cp).unwrap();
    let err = acquire_file(
        &AcquireRequest::new(&source, &output)
            .with_resume(true)
            .with_split_segment_bytes(128)
            .with_checkpoint_path(&checkpoint_path),
    )
    .unwrap_err();
    assert!(matches!(err, CoreError::Verification(_)));
}
