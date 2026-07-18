//! Output format writers/readers (E01/EWF, optional ZFF).

mod e01;
#[cfg(feature = "ewf")]
mod ewf;
#[cfg(feature = "libewf-oracle")]
mod libewf_oracle;
#[cfg(feature = "zff")]
mod zff_out;

pub use e01::{
    CaseMetadata, E01Summary, read_e01_to_raw, verify_e01, write_e01, write_e01_lite,
};
#[cfg(feature = "ewf")]
pub use ewf::{summarize_ewf, write_ewf_physical};
#[cfg(feature = "libewf-oracle")]
pub use libewf_oracle::{OracleStatus, ewfverify, resolve_ewfverify};
#[cfg(feature = "zff")]
pub use zff_out::{ZffWriteResult, resolve_zffacquire, write_zff_physical};
