#![forbid(unsafe_code)]

mod acquisition;
mod audit;
mod domain;
mod package;
pub mod platform;

pub use acquisition::{AcquireRequest, AcquisitionSummary, SegmentInfo, acquire_file};
pub use audit::{AuditEvent, AuditJournal};
pub use domain::{AcquisitionId, AcquisitionState, CaseId, CoreError};
pub use package::{FsnapManifestV1, create_fsnap, verify_fsnap};
pub use platform::RawDeviceAccessCapability;

pub const fn build_identity() -> &'static str {
    "trareon-acquire-foundation-v1"
}
