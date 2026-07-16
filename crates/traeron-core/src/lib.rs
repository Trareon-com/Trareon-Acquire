#![forbid(unsafe_code)]

mod audit;
mod domain;

pub use audit::{AuditEvent, AuditJournal};
pub use domain::{AcquisitionId, AcquisitionState, CaseId, CoreError};

pub const fn build_identity() -> &'static str {
    "traeron-acquire-foundation-v1"
}
