//! Output format writers/readers (RAW companions: E01, AFF4 subset, VM images).

mod aff4;
mod dmg;
mod e01;
#[cfg(feature = "ewf")]
mod ewf;
#[cfg(feature = "libewf-oracle")]
mod libewf_oracle;
mod qcow2;
mod vhd;
mod vmdk;
#[cfg(feature = "zff")]
mod zff_out;

pub use aff4::{Aff4Summary, write_aff4};
pub use dmg::{DmgSummary, write_dmg};
pub use e01::{
    CaseMetadata, E01Summary, OutputFormat, read_e01_to_raw, verify_e01, write_e01, write_e01_lite,
};
#[cfg(feature = "ewf")]
pub use ewf::{summarize_ewf, write_ewf_physical};
#[cfg(feature = "libewf-oracle")]
pub use libewf_oracle::{OracleStatus, ewfverify, resolve_ewfverify};
pub use qcow2::{Qcow2Summary, write_qcow2};
pub use vhd::{VhdSummary, write_vhd};
pub use vmdk::{VmdkSummary, write_vmdk};
#[cfg(feature = "zff")]
pub use zff_out::{ZffWriteResult, resolve_zffacquire, write_zff_physical};
