use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CaseId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AcquisitionId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionState {
    Planned,
    PreflightPassed,
    Acquiring,
    AcquiredUnverified,
    Verifying,
    VerifiedComplete,
    Failed,
    Cancelled,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum CoreError {
    #[error("invalid acquisition transition from {from:?} to {to:?}")]
    InvalidTransition {
        from: AcquisitionState,
        to: AcquisitionState,
    },
    #[error("I/O error: {0}")]
    Io(String),
    #[error("serialization error: {0}")]
    Serialization(String),
    #[error("verification failed: {0}")]
    Verification(String),
    #[error("acquisition cancelled")]
    Cancelled,
}

impl AcquisitionState {
    pub fn transition(self, next: Self) -> Result<Self, CoreError> {
        let allowed = matches!(
            (self, next),
            (Self::Planned, Self::PreflightPassed)
                | (Self::PreflightPassed, Self::Acquiring)
                | (Self::Acquiring, Self::AcquiredUnverified)
                | (Self::AcquiredUnverified, Self::Verifying)
                | (Self::Verifying, Self::VerifiedComplete)
                | (Self::Planned, Self::Cancelled)
                | (Self::PreflightPassed, Self::Cancelled)
                | (Self::Acquiring, Self::Cancelled)
                | (Self::Acquiring, Self::Failed)
                | (Self::AcquiredUnverified, Self::Failed)
                | (Self::Verifying, Self::Failed)
        );
        allowed.then_some(next).ok_or(CoreError::InvalidTransition {
            from: self,
            to: next,
        })
    }
}
