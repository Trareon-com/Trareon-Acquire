//! Minimal AFF4-like container (Trareon subset — not full AFF4 RDF/ZIP interop).
//!
//! Framing: magic `AF4\0` | u32 meta_len | JSON meta | raw bytes.

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::CoreError;

pub const AFF4_MAGIC: &[u8; 4] = b"AF4\0";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Aff4Summary {
    pub path: PathBuf,
    pub raw_size: u64,
    pub sha256: String,
}

pub fn write_aff4(mut reader: impl Read, output: &Path) -> Result<Aff4Summary, CoreError> {
    let mut raw = Vec::new();
    reader
        .read_to_end(&mut raw)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    let sha256 = hex::encode(Sha256::digest(&raw));
    let meta = serde_json::json!({
        "schema": "trareon.aff4-lite/1",
        "raw_size": raw.len(),
        "sha256": sha256,
        "note": "AFF4-like subset; not full AFF4 RDF/ZIP"
    });
    let meta_bytes = meta.to_string().into_bytes();
    let mut out = File::create(output).map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(AFF4_MAGIC)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&(meta_bytes.len() as u32).to_le_bytes())
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&meta_bytes)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&raw)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.sync_all()
        .map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(Aff4Summary {
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
    fn aff4_writes() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ev.aff4");
        let s = write_aff4(Cursor::new(b"hello-aff4"), &path).unwrap();
        assert_eq!(s.raw_size, 10);
        assert!(path.is_file());
        let bytes = std::fs::read(&path).unwrap();
        assert_eq!(&bytes[..4], AFF4_MAGIC);
    }
}
