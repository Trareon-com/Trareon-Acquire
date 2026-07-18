use std::{
    fs::File,
    io::{Cursor, Read},
    path::{Path, PathBuf},
};

use super::{AcquisitionSource, AcquisitionSourceKind};
use crate::CoreError;

/// Synthetic network capture: reads a local PCAP/PCAPNG file.
pub struct NetworkCaptureSource {
    path: PathBuf,
}

impl NetworkCaptureSource {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

impl AcquisitionSource for NetworkCaptureSource {
    fn kind(&self) -> AcquisitionSourceKind {
        AcquisitionSourceKind::Network
    }

    fn open(&mut self) -> Result<Box<dyn Read + Send>, CoreError> {
        let f = File::open(&self.path).map_err(|e| CoreError::Io(e.to_string()))?;
        Ok(Box::new(f))
    }

    fn byte_length(&self) -> Result<Option<u64>, CoreError> {
        Ok(Some(
            std::fs::metadata(&self.path)
                .map_err(|e| CoreError::Io(e.to_string()))?
                .len(),
        ))
    }

    fn order_of_volatility(&self) -> u8 {
        1
    }
}

/// Cloud snapshot import: concatenates files under a local directory (lab stand-in).
pub struct CloudSnapshotSource {
    dir: PathBuf,
}

impl CloudSnapshotSource {
    pub fn new(dir: impl Into<PathBuf>) -> Self {
        Self { dir: dir.into() }
    }

    fn bundle_bytes(&self) -> Result<Vec<u8>, CoreError> {
        let mut entries: Vec<_> = std::fs::read_dir(&self.dir)
            .map_err(|e| CoreError::Io(e.to_string()))?
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_file())
            .collect();
        entries.sort();
        let mut out = Vec::new();
        for path in entries {
            let name = path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("file")
                .as_bytes();
            out.extend_from_slice(&(name.len() as u32).to_le_bytes());
            out.extend_from_slice(name);
            let data = std::fs::read(&path).map_err(|e| CoreError::Io(e.to_string()))?;
            out.extend_from_slice(&(data.len() as u64).to_le_bytes());
            out.extend_from_slice(&data);
        }
        Ok(out)
    }
}

impl AcquisitionSource for CloudSnapshotSource {
    fn kind(&self) -> AcquisitionSourceKind {
        AcquisitionSourceKind::Cloud
    }

    fn open(&mut self) -> Result<Box<dyn Read + Send>, CoreError> {
        let bytes = self.bundle_bytes()?;
        Ok(Box::new(Cursor::new(bytes)))
    }

    fn byte_length(&self) -> Result<Option<u64>, CoreError> {
        Ok(Some(self.bundle_bytes()?.len() as u64))
    }

    fn order_of_volatility(&self) -> u8 {
        3
    }
}

/// Mobile logical collection: selected paths listed in a manifest text file (one path per line),
/// or all files under a directory.
pub struct MobileLogicalSource {
    root: PathBuf,
}

impl MobileLogicalSource {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    fn collect(&self) -> Result<Vec<u8>, CoreError> {
        let mut out = Vec::new();
        let manifest = self.root.join("mobile-manifest.txt");
        let paths: Vec<PathBuf> = if manifest.is_file() {
            std::fs::read_to_string(&manifest)
                .map_err(|e| CoreError::Io(e.to_string()))?
                .lines()
                .map(|l| l.trim())
                .filter(|l| !l.is_empty())
                .map(|l| self.root.join(l))
                .collect()
        } else {
            walk_files(&self.root)?
        };
        for path in paths {
            if !path.is_file() {
                continue;
            }
            let rel = path
                .strip_prefix(&self.root)
                .unwrap_or(&path)
                .to_string_lossy();
            let data = std::fs::read(&path).map_err(|e| CoreError::Io(e.to_string()))?;
            out.extend_from_slice(&(rel.len() as u32).to_le_bytes());
            out.extend_from_slice(rel.as_bytes());
            out.extend_from_slice(&(data.len() as u64).to_le_bytes());
            out.extend_from_slice(&data);
        }
        Ok(out)
    }
}

fn walk_files(root: &Path) -> Result<Vec<PathBuf>, CoreError> {
    let mut out = Vec::new();
    for entry in std::fs::read_dir(root).map_err(|e| CoreError::Io(e.to_string()))? {
        let entry = entry.map_err(|e| CoreError::Io(e.to_string()))?;
        let path = entry.path();
        if path.is_file() {
            out.push(path);
        } else if path.is_dir() {
            out.extend(walk_files(&path)?);
        }
    }
    out.sort();
    Ok(out)
}

impl AcquisitionSource for MobileLogicalSource {
    fn kind(&self) -> AcquisitionSourceKind {
        AcquisitionSourceKind::Mobile
    }

    fn open(&mut self) -> Result<Box<dyn Read + Send>, CoreError> {
        Ok(Box::new(Cursor::new(self.collect()?)))
    }

    fn byte_length(&self) -> Result<Option<u64>, CoreError> {
        Ok(Some(self.collect()?.len() as u64))
    }

    fn order_of_volatility(&self) -> u8 {
        3
    }
}
