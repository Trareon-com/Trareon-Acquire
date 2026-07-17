//! Minimal fixed VHD-like footer wrapper (Trareon subset).

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::CoreError;

pub const VHD_MAGIC: &[u8; 8] = b"conectix";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VhdSummary {
    pub path: PathBuf,
    pub raw_size: u64,
    pub sha256: String,
}

pub fn write_vhd(mut reader: impl Read, output: &Path) -> Result<VhdSummary, CoreError> {
    let mut raw = Vec::new();
    reader
        .read_to_end(&mut raw)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    let sha256 = hex::encode(Sha256::digest(&raw));
    let mut out = File::create(output).map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&raw)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(VHD_MAGIC)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&(raw.len() as u64).to_le_bytes())
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.sync_all()
        .map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(VhdSummary {
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
    fn vhd_writes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("disk.vhd");
        let s = write_vhd(Cursor::new(b"vhd"), &path).unwrap();
        assert_eq!(s.raw_size, 3);
    }
}
