//! Real EWF writer via `ewf-image` (Apache-2.0). Feature-gated: `ewf`.
//!
//! UI must keep the `E01-lite` product label until libewf/Autopsy evidence is green
//! (see `docs/format-interop/EWF-SPIKE.md`). This module is the Path B writer.

use std::{
    io::Read,
    path::{Path, PathBuf},
};

use ewf_image::{EwfWriter, WriteCompression, WriteFormat, WriteOptions};
use md5::{Digest as Md5Digest, Md5};
use sha1::{Digest as Sha1Digest, Sha1};
use sha2::{Digest as Sha2Digest, Sha256};

use super::e01::{CaseMetadata, E01Summary};
use crate::CoreError;

/// Write a physical EWF1 `.E01` using `ewf-image` (not the Trareon e01-lite framing).
pub fn write_ewf_physical(
    mut reader: impl Read,
    output: &Path,
    case: &CaseMetadata,
) -> Result<E01Summary, CoreError> {
    let mut options = WriteOptions {
        format: WriteFormat::Ewf1Physical,
        compression: WriteCompression::Zlib,
        ..WriteOptions::default()
    };
    if !case.case_number.is_empty() {
        options
            .metadata
            .set_header_value("case_number", &case.case_number);
    }
    if !case.evidence_number.is_empty() {
        options
            .metadata
            .set_header_value("evidence_number", &case.evidence_number);
    }
    if !case.examiner.is_empty() {
        options
            .metadata
            .set_header_value("examiner_name", &case.examiner);
    }
    if !case.description.is_empty() {
        options
            .metadata
            .set_header_value("description", &case.description);
    }
    if !case.notes.is_empty() {
        options.metadata.set_header_value("notes", &case.notes);
    }
    options
        .metadata
        .set_header_value("acquiry_software", "Trareon Acquire");
    options
        .metadata
        .set_header_value("acquiry_software_version", env!("CARGO_PKG_VERSION"));

    let mut writer = EwfWriter::create(output, options).map_err(ewf_err)?;
    let mut buf = vec![0u8; 64 * 1024];
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
        std::io::Write::write_all(&mut writer, chunk).map_err(|e| CoreError::Io(e.to_string()))?;
        Md5Digest::update(&mut md5, chunk);
        Sha1Digest::update(&mut sha1, chunk);
        Sha2Digest::update(&mut sha256, chunk);
        raw_size += n as u64;
        chunk_count += 1;
    }

    let finished = writer.finish().map_err(ewf_err)?;
    let path = finished
        .segment_paths
        .first()
        .cloned()
        .unwrap_or_else(|| output.to_path_buf());

    Ok(E01Summary {
        path,
        raw_size,
        md5: hex::encode(Md5Digest::finalize(md5)),
        sha1: hex::encode(Sha1Digest::finalize(sha1)),
        sha256: hex::encode(Sha2Digest::finalize(sha256)),
        chunk_count,
    })
}

/// Open an EWF image with `ewf-image` and report logical size + path.
pub fn summarize_ewf(path: &Path) -> Result<(PathBuf, u64), CoreError> {
    let image = ewf_image::Image::open(path).map_err(ewf_err)?;
    let info = image.info();
    Ok((path.to_path_buf(), info.logical_size))
}

fn ewf_err(err: impl std::fmt::Display) -> CoreError {
    CoreError::Io(format!("ewf-image: {err}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn ewf_physical_writes_openable_e01() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("spike.E01");
        let data: Vec<u8> = (0..=255).cycle().take(64 * 1024).collect();
        let case = CaseMetadata {
            case_number: "SPIKE-1".into(),
            evidence_number: "E1".into(),
            examiner: "lab".into(),
            description: "ewf feature".into(),
            notes: String::new(),
        };
        let summary = write_ewf_physical(Cursor::new(&data), &path, &case).unwrap();
        assert_eq!(summary.raw_size, data.len() as u64);
        let (opened, logical) = summarize_ewf(&summary.path).unwrap();
        assert_eq!(opened, summary.path);
        assert_eq!(logical, data.len() as u64);
    }
}
