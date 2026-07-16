use chrono::{DateTime, Utc};
use std::fs;
use tempfile::tempdir;
use trareon_core::{AcquisitionId, AcquisitionState, AuditJournal};
use uuid::Uuid;

#[test]
fn audit_chain_detects_modified_event() {
    let id = AcquisitionId(Uuid::nil());
    let mut journal = AuditJournal::default();
    journal
        .append(id, Utc::now(), AcquisitionState::Planned, "created")
        .unwrap();
    journal
        .append(
            id,
            Utc::now(),
            AcquisitionState::PreflightPassed,
            "preflight_passed",
        )
        .unwrap();
    assert!(journal.verify().is_ok());

    journal.events_mut_for_test()[0].action = "modified".into();
    assert!(journal.verify().is_err());
}

fn fixed_timestamp() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339("2026-07-17T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc)
}

#[test]
fn canonical_hash_is_deterministic_across_repeated_runs() {
    let id = AcquisitionId(Uuid::nil());
    let timestamp = fixed_timestamp();

    let mut first = AuditJournal::default();
    first
        .append(id, timestamp, AcquisitionState::Planned, "created")
        .unwrap();

    let mut second = AuditJournal::default();
    second
        .append(id, timestamp, AcquisitionState::Planned, "created")
        .unwrap();

    assert_eq!(
        first.events_mut_for_test()[0].event_hash,
        second.events_mut_for_test()[0].event_hash
    );
}

#[test]
fn changed_field_changes_hash() {
    let id = AcquisitionId(Uuid::nil());
    let timestamp = fixed_timestamp();

    let mut created = AuditJournal::default();
    created
        .append(id, timestamp, AcquisitionState::Planned, "created")
        .unwrap();

    let mut renamed = AuditJournal::default();
    renamed
        .append(id, timestamp, AcquisitionState::Planned, "renamed")
        .unwrap();

    assert_ne!(
        created.events_mut_for_test()[0].event_hash,
        renamed.events_mut_for_test()[0].event_hash
    );
}

#[test]
fn unsupported_state_value_is_rejected() {
    let dir = tempdir().unwrap();
    let path = dir.path().join("unsupported.jsonl");
    let line = r#"{"sequence":0,"acquisition_id":"00000000-0000-0000-0000-000000000000","timestamp_utc":"2026-07-17T00:00:00Z","state":"some_future_state","action":"created","previous_hash":"0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000","event_hash":"deadbeef"}"#;
    fs::write(&path, format!("{line}\n")).unwrap();

    assert!(AuditJournal::read_jsonl(&path).is_err());
}
