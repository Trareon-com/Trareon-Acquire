use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use super::{AcquisitionSource, AcquisitionSourceKind};
use crate::CoreError;

/// Treat a file as a synthetic RAM image (lab / CI).
pub struct SyntheticRamSource {
    path: PathBuf,
}

impl SyntheticRamSource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl AcquisitionSource for SyntheticRamSource {
    fn kind(&self) -> AcquisitionSourceKind {
        AcquisitionSourceKind::Ram
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
        1
    }
}
