//! Minimal DMG-like UDIF wrapper (raw payload + magic; Trareon subset).

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::CoreError;

pub const DMG_MAGIC: &[u8; 4] = b"koly";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DmgSummary {
    pub path: PathBuf,
    pub raw_size: u64,
    pub sha256: String,
}

pub fn write_dmg(mut reader: impl Read, output: &Path) -> Result<DmgSummary, CoreError> {
    let mut raw = Vec::new();
    reader
        .read_to_end(&mut raw)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    let sha256 = hex::encode(Sha256::digest(&raw));
    let mut out = File::create(output).map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&raw)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(DMG_MAGIC)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&(raw.len() as u64).to_be_bytes())
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.sync_all().map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(DmgSummary {
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
    fn dmg_writes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("disk.dmg");
        assert_eq!(write_dmg(Cursor::new(b"dmg"), &path).unwrap().raw_size, 3);
    }
}
