use chrono::Utc;
use traeron_core::{AcquisitionId, AcquisitionState, AuditJournal};
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
