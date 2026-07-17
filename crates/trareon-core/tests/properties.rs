//! Bounded property-style tests for Day 22 (no new fuzz/proptest dependency).
//!
//! Resource bounds (foundation slice):
//! - State matrix: all 8×8 transitions (64 cases).
//! - Audit chains: length 1..=64.
//! - Evidence payloads: sizes in a fixed synthetic set (≤ 8 KiB).
//! - Path corpus: seeds under `fixtures/fuzz-corpus/v0/` (≤ 32 entries).
//! - Verifier hostile inputs: fixed seed set; must return `Err`, never panic.

use std::{
    fs,
    path::{Path, PathBuf},
    sync::{Arc, atomic::AtomicBool},
};

use chrono::Utc;
use trareon_core::{
    AcquireRequest, AcquisitionId, AcquisitionState, AuditJournal, CoreError, acquire_file,
    create_fsnap, verify_fsnap,
};
use uuid::Uuid;

const ALL_STATES: [AcquisitionState; 8] = [
    AcquisitionState::Planned,
    AcquisitionState::PreflightPassed,
    AcquisitionState::Acquiring,
    AcquisitionState::AcquiredUnverified,
    AcquisitionState::Verifying,
    AcquisitionState::VerifiedComplete,
    AcquisitionState::Failed,
    AcquisitionState::Cancelled,
];

fn transition_is_allowed(from: AcquisitionState, to: AcquisitionState) -> bool {
    matches!(
        (from, to),
        (AcquisitionState::Planned, AcquisitionState::PreflightPassed)
            | (
                AcquisitionState::PreflightPassed,
                AcquisitionState::Acquiring
            )
            | (
                AcquisitionState::Acquiring,
                AcquisitionState::AcquiredUnverified
            )
            | (
                AcquisitionState::AcquiredUnverified,
                AcquisitionState::Verifying
            )
            | (
                AcquisitionState::Verifying,
                AcquisitionState::VerifiedComplete
            )
            | (AcquisitionState::Planned, AcquisitionState::Cancelled)
            | (
                AcquisitionState::PreflightPassed,
                AcquisitionState::Cancelled
            )
            | (AcquisitionState::Acquiring, AcquisitionState::Cancelled)
            | (AcquisitionState::Acquiring, AcquisitionState::Failed)
            | (
                AcquisitionState::AcquiredUnverified,
                AcquisitionState::Failed
            )
            | (AcquisitionState::Verifying, AcquisitionState::Failed)
    )
}

#[test]
fn property_state_transition_matrix_is_total_and_matches_allow_list() {
    for from in ALL_STATES {
        for to in ALL_STATES {
            let result = from.transition(to);
            if transition_is_allowed(from, to) {
                assert_eq!(
                    result.expect("allowed transition must succeed"),
                    to,
                    "allowed transition {from:?} -> {to:?}"
                );
            } else {
                let error = result.expect_err("disallowed transition must fail");
                assert!(
                    matches!(
                        error,
                        CoreError::InvalidTransition {
                            from: f,
                            to: t
                        } if f == from && t == to
                    ),
                    "expected InvalidTransition for {from:?} -> {to:?}, got {error:?}"
                );
            }
        }
    }
}

#[test]
fn property_terminal_states_have_no_outbound_transitions() {
    for from in [
        AcquisitionState::VerifiedComplete,
        AcquisitionState::Failed,
        AcquisitionState::Cancelled,
    ] {
        for to in ALL_STATES {
            assert!(
                from.transition(to).is_err(),
                "terminal {from:?} must not transition to {to:?}"
            );
        }
    }
}

fn corpus_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/fuzz-corpus/v0")
}

fn load_path_seeds() -> Vec<String> {
    let path = corpus_dir().join("unsafe-relative-paths.txt");
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("missing path corpus {}: {error}", path.display()))
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .map(str::to_string)
        .collect()
}

fn write_minimal_package(root: &Path) -> PathBuf {
    let package = root.join("pkg.fsnap");
    let source = root.join("source.bin");
    let output = root.join("evidence.raw");
    fs::write(&source, b"property-seed").expect("write source");
    let summary = acquire_file(&AcquireRequest::new(&source, &output)).expect("acquire");
    create_fsnap(&output, &summary.audit_path, &package).expect("create fsnap");
    package
}

#[test]
fn property_unsafe_manifest_paths_are_rejected_without_panic() {
    let seeds = load_path_seeds();
    assert!(
        (1..=32).contains(&seeds.len()),
        "path corpus must stay bounded (1..=32), got {}",
        seeds.len()
    );

    let root = tempfile::tempdir().expect("tempdir");
    let package = write_minimal_package(root.path());
    let manifest_path = package.join("manifest/manifest.json");
    let original = fs::read_to_string(&manifest_path).expect("read manifest");

    for seed in &seeds {
        let mut value: serde_json::Value =
            serde_json::from_str(&original).expect("parse manifest json");
        value["evidence_relative_path"] = serde_json::Value::String(seed.clone());
        fs::write(
            &manifest_path,
            serde_json::to_vec(&value).expect("serialize manifest"),
        )
        .expect("rewrite manifest");

        let result = std::panic::catch_unwind(|| verify_fsnap(&package));
        let outcome = result.expect("verify_fsnap must not panic on hostile path seed");
        assert!(
            outcome.is_err(),
            "unsafe path seed {seed:?} must be rejected"
        );
    }

    // Explicit NUL component (cannot live as a clean text corpus line).
    let mut value: serde_json::Value =
        serde_json::from_str(&original).expect("parse manifest json");
    value["evidence_relative_path"] =
        serde_json::Value::String("acquisitions/0001/evil\0.raw".to_string());
    fs::write(
        &manifest_path,
        serde_json::to_vec(&value).expect("serialize manifest"),
    )
    .expect("rewrite manifest");
    let result = std::panic::catch_unwind(|| verify_fsnap(&package));
    let outcome = result.expect("verify_fsnap must not panic on NUL path");
    assert!(outcome.is_err(), "NUL path must be rejected");
}

#[test]
fn property_audit_chain_verify_is_deterministic_for_bounded_lengths() {
    for length in 1usize..=64 {
        let mut journal = AuditJournal::default();
        let id = AcquisitionId(Uuid::nil());
        for i in 0..length {
            journal
                .append(
                    id,
                    Utc::now(),
                    AcquisitionState::Acquiring,
                    format!("step-{i}"),
                )
                .expect("append");
        }
        let root_a = journal.verify().expect("verify a");
        let root_b = journal.verify().expect("verify b");
        assert_eq!(
            root_a, root_b,
            "audit root must be deterministic at len={length}"
        );
    }
}

#[test]
fn property_audit_tamper_breaks_continuity() {
    let mut journal = AuditJournal::default();
    let id = AcquisitionId(Uuid::nil());
    for i in 0..8 {
        journal
            .append(
                id,
                Utc::now(),
                AcquisitionState::Acquiring,
                format!("step-{i}"),
            )
            .expect("append");
    }
    journal.verify().expect("untampered chain");

    journal.events_mut_for_test()[3].action = "tampered".to_string();
    let error = journal.verify().expect_err("tamper must fail");
    assert!(
        matches!(error, CoreError::Verification(_)),
        "tamper should surface as Verification, got {error:?}"
    );
}

#[test]
fn property_package_roundtrip_for_bounded_evidence_sizes() {
    let sizes = [1usize, 16, 256, 1024, 8 * 1024];
    for size in sizes {
        let root = tempfile::tempdir().expect("tempdir");
        let source = root.path().join("source.bin");
        let output = root.path().join("evidence.raw");
        let package = root.path().join("pkg.fsnap");
        fs::write(&source, vec![0xA5u8; size]).expect("write source");

        let summary = acquire_file(&AcquireRequest::new(&source, &output)).expect("acquire");
        create_fsnap(&output, &summary.audit_path, &package).expect("create");
        let manifest = verify_fsnap(&package).expect("verify");
        assert_eq!(manifest.evidence_size, size as u64);
        assert_eq!(manifest.evidence_sha256, summary.sha256);
    }
}

#[test]
fn property_extra_package_file_breaks_containment() {
    let root = tempfile::tempdir().expect("tempdir");
    let package = write_minimal_package(root.path());
    verify_fsnap(&package).expect("baseline ok");

    fs::write(package.join("unexpected.bin"), b"x").expect("plant extra file");
    let error = verify_fsnap(&package).expect_err("extra file must fail");
    assert!(
        matches!(error, CoreError::Verification(ref message) if message.contains("unexpected file")),
        "got {error:?}"
    );
}

#[test]
fn property_hostile_verifier_inputs_do_not_panic() {
    let root = tempfile::tempdir().expect("tempdir");
    let cases: Vec<(PathBuf, &str)> = vec![
        (root.path().join("missing.fsnap"), "missing package"),
        (root.path().join("empty.fsnap"), "empty dir"),
        (root.path().join("garbage.fsnap"), "garbage tree"),
    ];
    fs::create_dir_all(root.path().join("empty.fsnap")).expect("empty dir");
    let garbage = root.path().join("garbage.fsnap");
    fs::create_dir_all(garbage.join("manifest")).expect("manifest dir");
    fs::write(garbage.join("manifest/manifest.json"), b"{not-json").expect("bad json");

    for (path, label) in cases {
        let result = std::panic::catch_unwind(|| verify_fsnap(&path));
        let outcome = result.unwrap_or_else(|_| panic!("verify_fsnap panicked on {label}"));
        assert!(outcome.is_err(), "{label} must be rejected");
    }
}

#[test]
fn property_cancel_flag_never_reports_verified_complete() {
    let root = tempfile::tempdir().expect("tempdir");
    let source = root.path().join("source.bin");
    let output = root.path().join("evidence.raw");
    fs::write(&source, vec![0u8; 64 * 1024]).expect("write source");

    let cancel = Arc::new(AtomicBool::new(true));
    let request = AcquireRequest::new(&source, &output).with_cancel_flag(cancel);
    let error = acquire_file(&request).expect_err("pre-cancelled acquire must fail");
    assert!(
        matches!(error, CoreError::Cancelled),
        "expected Cancelled, got {error:?}"
    );
    // Cancel must never be reported as a successful verify path. Partial
    // output may exist; the typed error is the contract (see acquisition.rs tests).
}
