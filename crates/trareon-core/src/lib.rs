#![forbid(unsafe_code)]

mod acquisition;
mod audit;
mod broker;
mod checkpoint;
mod domain;
mod lab_policy;
mod package;
pub mod platform;

pub use acquisition::{AcquireRequest, AcquisitionSummary, SegmentInfo, acquire_file};
pub use audit::{AuditEvent, AuditJournal};
pub use broker::{
    BrokerOperation, BrokerRequest, BrokerResponse, ElevationHelper, StubElevationHelper,
    evaluate_broker_request, plan_digest,
};
pub use checkpoint::{
    AcquisitionCheckpoint, clear_checkpoint, default_checkpoint_path, load_checkpoint,
    write_checkpoint,
};
pub use domain::{AcquisitionId, AcquisitionState, CaseId, CoreError};
pub use lab_policy::{
    LabAllowlist, LabAllowlistEntry, SourceIdentity, SourceKind, assert_source_permitted,
    classify_source_path, load_lab_allowlist, source_identity_for, write_allowlist_template,
};
pub use package::{
    EvidenceSegmentV1, FsnapManifestV1, create_fsnap, create_fsnap_from_segments, verify_fsnap,
};
pub use platform::RawDeviceAccessCapability;

pub const fn build_identity() -> &'static str {
    "trareon-acquire-foundation-v1"
}
