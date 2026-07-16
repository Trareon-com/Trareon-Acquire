use traeron_core::{AcquisitionState, CoreError};

#[test]
fn verified_complete_requires_verifying_state() {
    let state = AcquisitionState::Verifying
        .transition(AcquisitionState::VerifiedComplete)
        .expect("verifying may complete");
    assert_eq!(state, AcquisitionState::VerifiedComplete);
}

#[test]
fn planned_cannot_skip_to_verified_complete() {
    let error = AcquisitionState::Planned
        .transition(AcquisitionState::VerifiedComplete)
        .expect_err("skip must fail");
    assert!(matches!(error, CoreError::InvalidTransition { .. }));
}
