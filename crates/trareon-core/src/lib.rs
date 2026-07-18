#![forbid(unsafe_code)]

mod acquisition;
mod audit;
mod broker;
mod checkpoint;
pub mod coc;
pub mod coverage;
pub mod custody;
pub mod disk_enum;
mod domain;
pub mod format;
pub mod freespace;
pub mod fsnap_archive;
pub mod imaging_policy;
mod lab_policy;
mod package;
pub mod platform;
pub mod report;
pub mod sign;
pub mod source;
pub mod triage;

pub use acquisition::{
    AcquirePhase, AcquireProgress, AcquireRequest, AcquisitionSummary, ProgressCallback,
    SegmentInfo, acquire_file,
};
pub use audit::{AuditEvent, AuditJournal};
pub use broker::{
    BrokerOperation, BrokerRequest, BrokerResponse, ElevationHelper, PlatformElevationHelper,
    StubElevationHelper, evaluate_broker_request, evaluate_broker_request_with_allowlist,
    plan_digest,
};
pub use checkpoint::{
    AcquisitionCheckpoint, clear_checkpoint, default_checkpoint_path, load_checkpoint,
    write_checkpoint,
};
pub use coc::{EvidenceCoC, EvidenceId};
pub use coverage::{CoverageState, CoverageSummary, coverage_from_manifest};
pub use custody::{CUSTODY_SCHEMA, CustodyEntry, append_custody_jsonl, read_custody_jsonl};
pub use disk_enum::{DiskRow, EnumError, enumerate_disks};
pub use domain::{AcquisitionId, AcquisitionState, CaseId, CoreError};
pub use format::{
    Aff4Summary, CaseMetadata, DmgSummary, E01Summary, OutputFormat, Qcow2Summary, VhdSummary,
    VmdkSummary, read_e01_to_raw, verify_e01, write_aff4, write_dmg, write_e01, write_e01_lite,
    write_qcow2, write_vhd, write_vmdk,
};
#[cfg(feature = "ewf")]
pub use format::{summarize_ewf, write_ewf_physical};
#[cfg(feature = "libewf-oracle")]
pub use format::{OracleStatus, ewfverify, resolve_ewfverify};
#[cfg(feature = "zff")]
pub use format::{ZffWriteResult, resolve_zffacquire, write_zff_physical};
pub use freespace::{destination_free_bytes, freespace_margin, freespace_ok};
pub use fsnap_archive::{pack_fsnap, unpack_fsnap};
pub use imaging_policy::BadSectorPolicy;
pub use lab_policy::{
    LabAllowlist, LabAllowlistEntry, SourceIdentity, SourceKind, assert_broker_source_identity,
    assert_source_permitted, classify_source_path, load_lab_allowlist, source_identity_for,
    write_allowlist_template,
};
pub use package::{
    EvidenceSegmentV1, FsnapCoverageV1, FsnapManifestV1, create_fsnap, create_fsnap_from_segments,
    verify_fsnap,
};
pub use platform::RawDeviceAccessCapability;
pub use report::acquisition_report_html;
pub use sign::{
    generate_signing_key, read_detached_signature, sign_bytes, sign_package_seal, verify_bytes,
    verify_package_seal, write_detached_signature,
};
pub use source::{
    AcquisitionSource, AcquisitionSourceKind, BlockDeviceSource, CloudSnapshotSource, FileSource,
    MobileLogicalSource, NetworkCaptureSource, SyntheticRamSource,
};
pub use triage::{
    TriageBundle, collect_live_triage, collect_triage_from_fixture, write_triage_bundle,
};

pub const fn build_identity() -> &'static str {
    "trareon-acquire-foundation-v1"
}
