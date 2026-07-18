use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha512};
use trareon_core::{
    AcquireRequest, CaseMetadata, acquire_file, create_fsnap_from_segments, verify_fsnap,
    write_aff4, write_dmg, write_e01, write_qcow2, write_vhd, write_vmdk,
};

/// Formats exposed by the desktop acquisition workflow.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiOutputFormat {
    RawFsnap,
    E01,
    Aff4,
    Vmdk,
    Vhd,
    Qcow2,
    Dmg,
    /// Optional ZFF via external `zffacquire` (feature `zff` on trareon-core).
    Zff,
}

impl UiOutputFormat {
    fn extension(self) -> &'static str {
        match self {
            Self::RawFsnap => "fsnap",
            Self::E01 => "e01",
            Self::Aff4 => "aff4",
            Self::Vmdk => "vmdk",
            Self::Vhd => "vhd",
            Self::Qcow2 => "qcow2",
            Self::Dmg => "dmg",
            Self::Zff => "z01",
        }
    }
}

/// Acquires a file and writes it in the selected output representation.
///
/// Non-RAW output writers are deliberately limited compatibility subsets; they
/// are not a claim of full EWF, AFF4, VMDK, VHD, QCOW2, or UDIF interoperability.
pub fn acquire_to_format(
    source: &Path,
    output_dir: &Path,
    format: UiOutputFormat,
) -> Result<PathBuf, String> {
    acquire_to_format_with_sha512(source, output_dir, format, false)
}

/// As [`acquire_to_format`], optionally emitting a `<output>.<ext>.sha512` sidecar.
pub fn acquire_to_format_with_sha512(
    source: &Path,
    output_dir: &Path,
    format: UiOutputFormat,
    write_sha512_sidecar: bool,
) -> Result<PathBuf, String> {
    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;
    let output = if format == UiOutputFormat::RawFsnap {
        PathBuf::from(crate::run_foundation_demo(source, output_dir, None)?.package_path)
    } else {
        let raw = output_dir.join("format-input.raw");
        let request = AcquireRequest::new(source, &raw).with_buffer_size(64 * 1024);
        acquire_file(&request).map_err(|error| error.to_string())?;

        let output = output_dir.join(format!("evidence.{}", format.extension()));
        match format {
            UiOutputFormat::Zff => {
                #[cfg(feature = "zff")]
                {
                    let prefix = output_dir.join("evidence-zff");
                    trareon_core::write_zff_physical(&raw, &prefix)
                        .map_err(|error| error.to_string())?;
                    prefix
                }
                #[cfg(not(feature = "zff"))]
                {
                    return Err(
                        "ZFF output requires trareon-core feature `zff` + zffacquire on PATH".into(),
                    );
                }
            }
            other => {
                let reader = BufReader::new(File::open(&raw).map_err(|error| error.to_string())?);
                match other {
                    UiOutputFormat::E01 => {
                        write_e01(reader, &output, &CaseMetadata::default()).map(|_| ())
                    }
                    UiOutputFormat::Aff4 => write_aff4(reader, &output).map(|_| ()),
                    UiOutputFormat::Vmdk => write_vmdk(reader, &output).map(|_| ()),
                    UiOutputFormat::Vhd => write_vhd(reader, &output).map(|_| ()),
                    UiOutputFormat::Qcow2 => write_qcow2(reader, &output).map(|_| ()),
                    UiOutputFormat::Dmg => write_dmg(reader, &output).map(|_| ()),
                    UiOutputFormat::RawFsnap | UiOutputFormat::Zff => {
                        unreachable!("handled above")
                    }
                }
                .map_err(|error| error.to_string())?;
                output
            }
        }
    };

    if write_sha512_sidecar && output.is_file() {
        write_sha512_sidecar_for(&output)?;
    }
    Ok(output)
}

/// File-backed split-raw acquisition into a multi-segment `.fsnap` package.
pub fn acquire_split_raw(
    source: &Path,
    output_dir: &Path,
    segment_bytes: u64,
) -> Result<PathBuf, String> {
    if segment_bytes == 0 {
        return Err("segment_bytes must be > 0".into());
    }
    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;
    let evidence = output_dir.join("evidence.raw");
    let package = output_dir.join("split.fsnap");
    let request = AcquireRequest::new(source, &evidence)
        .with_buffer_size(64 * 1024)
        .with_split_segment_bytes(segment_bytes);
    let summary = acquire_file(&request).map_err(|error| error.to_string())?;
    let segments: Vec<PathBuf> = summary
        .segments
        .iter()
        .map(|segment| segment.path.clone())
        .collect();
    if segments.is_empty() {
        return Err("split acquire produced no segments".into());
    }
    create_fsnap_from_segments(&segments, &summary.audit_path, &package)
        .map_err(|error| error.to_string())?;
    verify_fsnap(&package).map_err(|error| error.to_string())?;
    Ok(package)
}

/// Writes the hexadecimal SHA-512 digest of `path` to a `.sha512` sidecar.
pub fn write_sha512_sidecar_for(path: &Path) -> Result<PathBuf, String> {
    let mut reader = BufReader::new(File::open(path).map_err(|error| error.to_string())?);
    let mut hasher = Sha512::new();
    let mut buffer = [0_u8; 64 * 1024];
    loop {
        let read = reader
            .read(&mut buffer)
            .map_err(|error| error.to_string())?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }

    let sidecar = path.with_extension(format!(
        "{}.sha512",
        path.extension()
            .and_then(|extension| extension.to_str())
            .unwrap_or("")
    ));
    std::fs::write(&sidecar, format!("{}\n", hex::encode(hasher.finalize())))
        .map_err(|error| error.to_string())?;
    Ok(sidecar)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn split_raw_produces_verifiable_package() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.bin");
        std::fs::write(&source, vec![0xAB; 200 * 1024]).unwrap();
        let out = dir.path().join("out");
        let package = acquire_split_raw(&source, &out, 64 * 1024).expect("split");
        assert!(package.is_dir());
        verify_fsnap(&package).expect("verify split");
    }

    #[test]
    fn e01_acquisition_writes_requested_container_and_sha512_sidecar() {
        let dir = tempfile::tempdir().unwrap();
        let source = dir.path().join("source.bin");
        std::fs::write(&source, b"format fixture").unwrap();

        let output = acquire_to_format_with_sha512(
            &source,
            &dir.path().join("out"),
            UiOutputFormat::E01,
            true,
        )
        .unwrap();

        assert_eq!(output.extension().and_then(|ext| ext.to_str()), Some("e01"));
        assert!(output.is_file());
        assert_eq!(
            std::fs::read_to_string(output.with_extension("e01.sha512"))
                .unwrap()
                .trim()
                .len(),
            128
        );
    }
}
