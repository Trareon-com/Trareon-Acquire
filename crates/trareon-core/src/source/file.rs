use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use super::{AcquisitionSource, AcquisitionSourceKind};
use crate::CoreError;

pub struct FileSource {
    path: PathBuf,
}

impl FileSource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl AcquisitionSource for FileSource {
    fn kind(&self) -> AcquisitionSourceKind {
        AcquisitionSourceKind::File
    }

    fn open(&mut self) -> Result<Box<dyn Read + Send>, CoreError> {
        let f = File::open(&self.path).map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(Box::new(f))
    }

    fn byte_length(&self) -> Result<Option<u64>, CoreError> {
        let meta = std::fs::metadata(&self.path).map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(Some(meta.len()))
    }

    fn order_of_volatility(&self) -> u8 {
        5
    }
}

/// Block/raw device source — opens the path like a file; lab allowlist is caller's duty.
pub struct BlockDeviceSource {
    path: PathBuf,
}

impl BlockDeviceSource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl AcquisitionSource for BlockDeviceSource {
    fn kind(&self) -> AcquisitionSourceKind {
        AcquisitionSourceKind::BlockDevice
    }

    fn open(&mut self) -> Result<Box<dyn Read + Send>, CoreError> {
        let f = File::open(&self.path).map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(Box::new(f))
    }

    fn byte_length(&self) -> Result<Option<u64>, CoreError> {
        match std::fs::metadata(&self.path) {
            Ok(meta) => Ok(Some(meta.len())),
            Err(_) => Ok(None),
        }
    }

    fn order_of_volatility(&self) -> u8 {
        4
    }
}
