//! Read-only source and destination checks performed before acquisition.

use std::{
    fs::File,
    io::{Read, Result as IoResult},
    path::Path,
};

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreflightResult {
    pub source_size: u64,
    pub destination_free_bytes: u64,
    pub free_space_ok: bool,
    pub pre_hash_hex: String,
}

/// Capacity check before acquisition. Optional full-source SHA-256 via `with_pre_hash`.
pub fn preflight(source: &Path, output_dir: &Path) -> Result<PreflightResult, String> {
    preflight_opts(source, output_dir, false)
}

pub fn preflight_opts(
    source: &Path,
    output_dir: &Path,
    with_pre_hash: bool,
) -> Result<PreflightResult, String> {
    let source_size = source.metadata().map_err(|error| error.to_string())?.len();
    let destination_free_bytes = trareon_core::freespace::destination_free_bytes(output_dir)
        .map_err(|error| error.to_string())?;
    Ok(PreflightResult {
        source_size,
        destination_free_bytes,
        free_space_ok: trareon_core::freespace::freespace_ok(destination_free_bytes, source_size),
        pre_hash_hex: if with_pre_hash {
            sha256_file(source).map_err(|error| error.to_string())?
        } else {
            String::new()
        },
    })
}

pub fn sha256_file(path: &Path) -> IoResult<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hex::encode(hasher.finalize()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preflight_checks_destination_without_forced_hash() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("source.bin");
        std::fs::write(&source, b"test").unwrap();
        let result = preflight(&source, temp.path()).unwrap();
        assert!(result.pre_hash_hex.is_empty());
        assert!(result.destination_free_bytes > 0);
        let hashed = preflight_opts(&source, temp.path(), true).unwrap();
        assert_eq!(hashed.pre_hash_hex.len(), 64);
    }
}
