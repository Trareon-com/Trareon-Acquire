#![forbid(unsafe_code)]

mod acquisition;
mod audit;
mod domain;
mod package;

pub use acquisition::{AcquireRequest, AcquisitionSummary, acquire_file};
pub use audit::{AuditEvent, AuditJournal};
pub use domain::{AcquisitionId, AcquisitionState, CaseId, CoreError};
pub use package::{FsnapManifestV1, create_fsnap, verify_fsnap};

pub const fn build_identity() -> &'static str {
    "traeron-acquire-foundation-v1"
}
