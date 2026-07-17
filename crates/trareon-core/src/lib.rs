#![forbid(unsafe_code)]

mod acquisition;
mod audit;
mod broker;
mod domain;
mod package;
pub mod platform;

pub use acquisition::{AcquireRequest, AcquisitionSummary, SegmentInfo, acquire_file};
pub use audit::{AuditEvent, AuditJournal};
pub use broker::{
    BrokerOperation, BrokerRequest, BrokerResponse, evaluate_broker_request, plan_digest,
};
pub use domain::{AcquisitionId, AcquisitionState, CaseId, CoreError};
pub use package::{
    EvidenceSegmentV1, FsnapManifestV1, create_fsnap, create_fsnap_from_segments, verify_fsnap,
};
pub use platform::RawDeviceAccessCapability;

pub const fn build_identity() -> &'static str {
    "trareon-acquire-foundation-v1"
}
