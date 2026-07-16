#![forbid(unsafe_code)]

mod domain;

pub use domain::{AcquisitionId, AcquisitionState, CaseId, CoreError};

pub const fn build_identity() -> &'static str {
    "traeron-acquire-foundation-v1"
}
