//! Minimal EWF/E01-lite writer + reader (pure Rust, no libewf).
//!
//! Framing:
//! - magic `EVF\x09\x0d\x0a\xff\x00`
//! - u32 + JSON header
//! - chunks: u32 zlib_len | zlib | u32 crc32(raw)
//! - footer marker `FTR\0` + u32 + JSON footer (md5/sha1/sha256)

use std::{
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use flate2::{Compression, read::ZlibDecoder, write::ZlibEncoder};
use md5::{Digest as Md5Digest, Md5};
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Digest as Sha2Digest, Sha256};

use crate::CoreError;

pub const E01_MAGIC: &[u8; 8] = b"EVF\x09\x0d\x0a\xff\x00";
pub const FOOTER_MAGIC: &[u8; 4] = b"FTR\0";
pub const DEFAULT_CHUNK: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OutputFormat {
    Raw,
    SplitRaw,
    E01,
    Aff4,
    Vmdk,
    Vhd,
    Qcow2,
    Dmg,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct CaseMetadata {
    pub case_number: String,
    pub evidence_number: String,
    pub examiner: String,
    pub description: String,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct E01Header {
    schema: String,
    chunk_size: u32,
    case: CaseMetadata,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
struct E01Footer {
    raw_size: u64,
    md5: String,
    sha1: String,
    sha256: String,
    chunk_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct E01Summary {
    pub path: PathBuf,
    pub raw_size: u64,
    pub md5: String,
    pub sha1: String,
    pub sha256: String,
    pub chunk_count: u64,
}

/// Product E01 writer: real EWF when feature `ewf` is enabled, else e01-lite.
pub fn write_e01(
    reader: impl Read,
    output: &Path,
    case: &CaseMetadata,
) -> Result<E01Summary, CoreError> {
    #[cfg(feature = "ewf")]
    {
        crate::format::ewf::write_ewf_physical(reader, output, case)
    }
    #[cfg(not(feature = "ewf"))]
    {
        write_e01_lite(reader, output, case)
    }
}

/// Documented `trareon.e01-lite/1` subset (not libewf-compatible).
pub fn write_e01_lite(
    mut reader: impl Read,
    output: &Path,
    case: &CaseMetadata,
) -> Result<E01Summary, CoreError> {
    let mut out = File::create(output).map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(E01_MAGIC)
        .map_err(|e| CoreError::Io(e.to_string()))?;

    let header = E01Header {
        schema: "trareon.e01-lite/1".into(),
        chunk_size: DEFAULT_CHUNK as u32,
        case: case.clone(),
    };
    write_len_prefixed_json(&mut out, &header)?;

    let mut buf = vec![0u8; DEFAULT_CHUNK];
    let mut md5 = <Md5 as Md5Digest>::new();
    let mut sha1 = <Sha1 as Sha1Digest>::new();
    let mut sha256 = <Sha256 as Sha2Digest>::new();
    let mut raw_size = 0u64;
    let mut chunk_count = 0u64;

    loop {
        let n = reader
            .read(&mut buf)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        if n == 0 {
            break;
        }
        let chunk = &buf[..n];
        Md5Digest::update(&mut md5, chunk);
        Sha1Digest::update(&mut sha1, chunk);
        Sha2Digest::update(&mut sha256, chunk);
        raw_size += n as u64;
        chunk_count += 1;

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::fast());
        encoder
            .write_all(chunk)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        let compressed = encoder.finish().map_err(|e| CoreError::Io(e.to_string()))?;
        let crc = crc32fast::hash(chunk);
        out.write_all(&(compressed.len() as u32).to_le_bytes())
            .map_err(|e| CoreError::Io(e.to_string()))?;
        out.write_all(&compressed)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        out.write_all(&crc.to_le_bytes())
            .map_err(|e| CoreError::Io(e.to_string()))?;
    }

    let footer = E01Footer {
        raw_size,
        md5: hex::encode(Md5Digest::finalize(md5)),
        sha1: hex::encode(Sha1Digest::finalize(sha1)),
        sha256: hex::encode(Sha2Digest::finalize(sha256)),
        chunk_count,
    };
    out.write_all(FOOTER_MAGIC)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    write_len_prefixed_json(&mut out, &footer)?;
    out.sync_all().map_err(|e| CoreError::Io(e.to_string()))?;

    Ok(E01Summary {
        path: output.to_path_buf(),
        raw_size: footer.raw_size,
        md5: footer.md5.clone(),
        sha1: footer.sha1.clone(),
        sha256: footer.sha256.clone(),
        chunk_count: footer.chunk_count,
    })
}

pub fn read_e01_to_raw(path: &Path, out: &mut impl Write) -> Result<E01Summary, CoreError> {
    let mut file = File::open(path).map_err(|e| CoreError::Io(e.to_string()))?;
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    if &magic != E01_MAGIC {
        return Err(CoreError::Verification("invalid E01 magic".into()));
    }
    let _header: E01Header = read_len_prefixed_json(&mut file)?;

    let mut md5 = <Md5 as Md5Digest>::new();
    let mut sha1 = <Sha1 as Sha1Digest>::new();
    let mut sha256 = <Sha256 as Sha2Digest>::new();
    let mut raw_size = 0u64;
    let mut chunk_count = 0u64;

    loop {
        let mut tag = [0u8; 4];
        file.read_exact(&mut tag)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        if &tag == FOOTER_MAGIC {
            break;
        }
        // First 4 bytes were compressed length (little-endian), not footer.
        let len = u32::from_le_bytes(tag) as usize;
        let mut compressed = vec![0u8; len];
        file.read_exact(&mut compressed)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        let crc = read_u32(&mut file)?;
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut raw_chunk = Vec::new();
        decoder
            .read_to_end(&mut raw_chunk)
            .map_err(|e| CoreError::Verification(format!("E01 inflate: {e}")))?;
        if crc32fast::hash(&raw_chunk) != crc {
            return Err(CoreError::Verification(format!(
                "E01 chunk CRC mismatch at {chunk_count}"
            )));
        }
        out.write_all(&raw_chunk)
            .map_err(|e| CoreError::Io(e.to_string()))?;
        Md5Digest::update(&mut md5, &raw_chunk);
        Sha1Digest::update(&mut sha1, &raw_chunk);
        Sha2Digest::update(&mut sha256, &raw_chunk);
        raw_size += raw_chunk.len() as u64;
        chunk_count += 1;
    }

    let footer: E01Footer = read_len_prefixed_json(&mut file)?;
    let got_md5 = hex::encode(Md5Digest::finalize(md5));
    let got_sha1 = hex::encode(Sha1Digest::finalize(sha1));
    let got_sha256 = hex::encode(Sha2Digest::finalize(sha256));
    if footer.md5 != got_md5 || footer.sha1 != got_sha1 || footer.sha256 != got_sha256 {
        return Err(CoreError::Verification("E01 hash mismatch".into()));
    }
    if footer.raw_size != raw_size || footer.chunk_count != chunk_count {
        return Err(CoreError::Verification(
            "E01 footer size/chunk mismatch".into(),
        ));
    }
    Ok(E01Summary {
        path: path.to_path_buf(),
        raw_size,
        md5: got_md5,
        sha1: got_sha1,
        sha256: got_sha256,
        chunk_count,
    })
}

pub fn verify_e01(path: &Path) -> Result<E01Summary, CoreError> {
    let mut sink = std::io::sink();
    read_e01_to_raw(path, &mut sink)
}

fn write_len_prefixed_json<T: serde::Serialize>(
    out: &mut impl Write,
    value: &T,
) -> Result<(), CoreError> {
    let bytes = serde_json::to_vec(value).map_err(|e| CoreError::Verification(e.to_string()))?;
    out.write_all(&(bytes.len() as u32).to_le_bytes())
        .map_err(|e| CoreError::Io(e.to_string()))?;
    out.write_all(&bytes)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(())
}

fn read_len_prefixed_json<T: serde::de::DeserializeOwned>(
    r: &mut impl Read,
) -> Result<T, CoreError> {
    let len = read_u32(r)? as usize;
    let mut buf = vec![0u8; len];
    r.read_exact(&mut buf)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    serde_json::from_slice(&buf).map_err(|e| CoreError::Verification(e.to_string()))
}

fn read_u32(r: &mut impl Read) -> Result<u32, CoreError> {
    let mut b = [0u8; 4];
    r.read_exact(&mut b)
        .map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(u32::from_le_bytes(b))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn e01_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("ev.E01");
        let data: Vec<u8> = (0..=255).cycle().take(200_000).collect();
        let case = CaseMetadata {
            case_number: "TR-1".into(),
            evidence_number: "E1".into(),
            examiner: "lab".into(),
            description: "synth".into(),
            notes: String::new(),
        };
        let summary = write_e01_lite(Cursor::new(&data), &path, &case).unwrap();
        assert_eq!(summary.raw_size, data.len() as u64);
        let verified = verify_e01(&path).unwrap();
        assert_eq!(verified.sha256, summary.sha256);
        let mut out = Vec::new();
        read_e01_to_raw(&path, &mut out).unwrap();
        assert_eq!(out, data);
    }
}
