//! Minimal monolithic flat VMDK-like wrapper (descriptor + raw payload file pair simplified into one file).

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::CoreError;

pub const VMDK_MAGIC: &[u8; 4] = b"KDMV";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VmdkSummary {
    pub path: PathBuf,
    pub raw_size: u64,
    pub sha256: String,
}

pub fn write_vmdk(mut reader: impl Read, output: &Path) -> Result<VmdkSummary, CoreError> {
    let mut raw = Vec::new();
    reader
        .read_to_end(&mut raw)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    let sha256 = hex::encode(Sha256::digest(&raw));
    let mut out = File::create(output).map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(VMDK_MAGIC)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&(raw.len() as u64).to_le_bytes())
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&raw)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.sync_all()
        .map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(VmdkSummary {
        path: output.to_path_buf(),
        raw_size: raw.len() as u64,
        sha256,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn vmdk_roundtrip_header() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("disk.vmdk");
        let s = write_vmdk(Cursor::new(b"abcd"), &path).unwrap();
        assert_eq!(s.raw_size, 4);
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[..4], VMDK_MAGIC);
    }
}
