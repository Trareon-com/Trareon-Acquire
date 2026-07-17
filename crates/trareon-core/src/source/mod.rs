//! Acquisition source trait and concrete sources (file, block, ram, net, cloud, mobile).

mod extended;
mod file;
mod ram;

pub use extended::{CloudSnapshotSource, MobileLogicalSource, NetworkCaptureSource};
pub use file::{BlockDeviceSource, FileSource};
pub use ram::SyntheticRamSource;

use std::io::Read;

use crate::CoreError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AcquisitionSourceKind {
    File,
    BlockDevice,
    Ram,
    Hibernation,
    PageFile,
    Network,
    Cloud,
    Mobile,
    LiveTriage,
}

pub trait AcquisitionSource: Send {
    fn kind(&self) -> AcquisitionSourceKind;
    fn open(&mut self) -> Result<Box<dyn Read + Send>, CoreError>;
    fn byte_length(&self) -> Result<Option<u64>, CoreError>;
    /// NIST SP 800-86 style: 1 = most volatile (network/RAM), 5 = archival disk.
    fn order_of_volatility(&self) -> u8;
}
