use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

use crate::{AuditJournal, CoreError, build_identity};

const EVIDENCE_RELATIVE_PATH: &str = "acquisitions/0001/evidence.raw";
const AUDIT_RELATIVE_PATH: &str = "audit/audit.jsonl";
const MANIFEST_RELATIVE_PATH: &str = "manifest/manifest.json";
const SUPPORTED_SCHEMA: &str = "trareon.fsnap.manifest/1";

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct EvidenceSegmentV1 {
    pub relative_path: String,
    pub size: u64,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct FsnapCoverageV1 {
    pub imaged_bytes: u64,
    pub total_bytes: u64,
    #[serde(default)]
    pub gaps: Vec<String>,
    #[serde(default)]
    pub errors: Vec<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FsnapManifestV1 {
    pub schema: String,
    pub build_identity: String,
    /// Primary evidence path. For single-file packages this is
    /// `acquisitions/0001/evidence.raw`. For split-RAW packages this is the
    /// first segment path; full segment list is in `evidence_segments`.
    pub evidence_relative_path: String,
    pub evidence_size: u64,
    /// SHA-256 of the logical evidence stream (single file, or concatenation
    /// of `evidence_segments` in listed order).
    pub evidence_sha256: String,
    pub audit_relative_path: String,
    pub audit_sha256: String,
    pub audit_root: String,
    /// When present and non-empty, the package carries split-RAW segments.
    /// Omitted on classic single-file packages (Analysis freeze golden set).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub evidence_segments: Option<Vec<EvidenceSegmentV1>>,
    /// Optional read coverage supplied by an acquisition source.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub coverage: Option<FsnapCoverageV1>,
}

fn hash_file(path: &Path) -> Result<(u64, String), CoreError> {
    let mut file = File::open(path).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    let mut size = 0u64;
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|error| CoreError::Io(error.to_string()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
        size += read as u64;
    }
    Ok((size, hex::encode(hasher.finalize())))
}

fn hash_files_concat(paths: &[&Path]) -> Result<(u64, String), CoreError> {
    let mut hasher = Sha256::new();
    let mut size = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    for path in paths {
        let mut file = File::open(path).map_err(|error| CoreError::Io(error.to_string()))?;
        loop {
            let read = file
                .read(&mut buffer)
                .map_err(|error| CoreError::Io(error.to_string()))?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
            size += read as u64;
        }
    }
    Ok((size, hex::encode(hasher.finalize())))
}

/// One pass: per-segment (size, sha256) plus concatenated digest of all segment bytes.
fn hash_segments_and_concat(
    paths: &[&Path],
) -> Result<(Vec<(u64, String)>, u64, String), CoreError> {
    let mut concat = Sha256::new();
    let mut total = 0u64;
    let mut per = Vec::with_capacity(paths.len());
    let mut buffer = [0u8; 64 * 1024];
    for path in paths {
        let mut file = File::open(path).map_err(|error| CoreError::Io(error.to_string()))?;
        let mut seg = Sha256::new();
        let mut size = 0u64;
        loop {
            let read = file
                .read(&mut buffer)
                .map_err(|error| CoreError::Io(error.to_string()))?;
            if read == 0 {
                break;
            }
            seg.update(&buffer[..read]);
            concat.update(&buffer[..read]);
            size += read as u64;
            total += read as u64;
        }
        per.push((size, hex::encode(seg.finalize())));
    }
    Ok((per, total, hex::encode(concat.finalize())))
}

/// Copy `source` → `destination`, hashing bytes while writing (one pass).
fn copy_hash_and_sync(source: &Path, destination: &Path) -> Result<(u64, String), CoreError> {
    let mut input = File::open(source).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut output = File::create(destination).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    let mut size = 0u64;
    loop {
        let read = input
            .read(&mut buffer)
            .map_err(|error| CoreError::Io(error.to_string()))?;
        if read == 0 {
            break;
        }
        output
            .write_all(&buffer[..read])
            .map_err(|error| CoreError::Io(error.to_string()))?;
        hasher.update(&buffer[..read]);
        size += read as u64;
    }
    output
        .sync_all()
        .map_err(|error| CoreError::Io(error.to_string()))?;
    Ok((size, hex::encode(hasher.finalize())))
}

fn segment_relative_path(index: usize) -> String {
    format!("acquisitions/0001/evidence.{index:03}.raw")
}

fn write_manifest(package: &Path, manifest: &FsnapManifestV1) -> Result<(), CoreError> {
    let manifest_path = package.join(MANIFEST_RELATIVE_PATH);
    let manifest_bytes = serde_json::to_vec(manifest)
        .map_err(|error| CoreError::Serialization(error.to_string()))?;
    let mut manifest_file =
        File::create(&manifest_path).map_err(|error| CoreError::Io(error.to_string()))?;
    manifest_file
        .write_all(&manifest_bytes)
        .map_err(|error| CoreError::Io(error.to_string()))?;
    manifest_file
        .sync_all()
        .map_err(|error| CoreError::Io(error.to_string()))?;
    Ok(())
}

fn prepare_package_dirs(package: &Path) -> Result<(), CoreError> {
    if package.exists() {
        return Err(CoreError::Verification(
            "package path must not already exist".to_string(),
        ));
    }
    let manifest_dir = package.join("manifest");
    let audit_dir = package.join("audit");
    let acquisitions_dir = package.join("acquisitions/0001");
    for dir in [&manifest_dir, &audit_dir, &acquisitions_dir] {
        fs::create_dir_all(dir).map_err(|error| CoreError::Io(error.to_string()))?;
    }
    Ok(())
}

/// Create a classic single-`evidence.raw` `.fsnap` package (Analysis freeze layout).
pub fn create_fsnap(raw: &Path, audit: &Path, package: &Path) -> Result<(), CoreError> {
    prepare_package_dirs(package)?;

    let evidence_path = package.join(EVIDENCE_RELATIVE_PATH);
    let audit_path = package.join(AUDIT_RELATIVE_PATH);
    let (evidence_size, evidence_sha256) = copy_hash_and_sync(raw, &evidence_path)?;
    let (_, audit_sha256) = copy_hash_and_sync(audit, &audit_path)?;
    let audit_root = AuditJournal::read_jsonl(&audit_path)?.verify()?;

    let manifest = FsnapManifestV1 {
        schema: SUPPORTED_SCHEMA.to_string(),
        build_identity: build_identity().to_string(),
        evidence_relative_path: EVIDENCE_RELATIVE_PATH.to_string(),
        evidence_size,
        evidence_sha256,
        audit_relative_path: AUDIT_RELATIVE_PATH.to_string(),
        audit_sha256,
        audit_root,
        evidence_segments: None,
        coverage: None,
    };
    write_manifest(package, &manifest)
}

/// Create a split-RAW `.fsnap` package from ordered segment files.
pub fn create_fsnap_from_segments(
    segments: &[PathBuf],
    audit: &Path,
    package: &Path,
) -> Result<(), CoreError> {
    if segments.is_empty() {
        return Err(CoreError::Verification(
            "split-RAW package requires at least one segment".to_string(),
        ));
    }
    if segments.len() == 1 {
        return create_fsnap(&segments[0], audit, package);
    }

    prepare_package_dirs(package)?;
    let audit_path = package.join(AUDIT_RELATIVE_PATH);
    let (_, audit_sha256) = copy_hash_and_sync(audit, &audit_path)?;

    let mut segment_metas = Vec::with_capacity(segments.len());
    let mut concat = Sha256::new();
    let mut evidence_size = 0u64;
    let mut buffer = [0u8; 64 * 1024];
    for (index, source) in segments.iter().enumerate() {
        let relative = segment_relative_path(index + 1);
        let dest = package.join(&relative);
        let (size, sha256) = {
            let mut input = File::open(source).map_err(|error| CoreError::Io(error.to_string()))?;
            let mut output =
                File::create(&dest).map_err(|error| CoreError::Io(error.to_string()))?;
            let mut seg = Sha256::new();
            let mut size = 0u64;
            loop {
                let read = input
                    .read(&mut buffer)
                    .map_err(|error| CoreError::Io(error.to_string()))?;
                if read == 0 {
                    break;
                }
                output
                    .write_all(&buffer[..read])
                    .map_err(|error| CoreError::Io(error.to_string()))?;
                seg.update(&buffer[..read]);
                concat.update(&buffer[..read]);
                size += read as u64;
                evidence_size += read as u64;
            }
            output
                .sync_all()
                .map_err(|error| CoreError::Io(error.to_string()))?;
            (size, hex::encode(seg.finalize()))
        };
        segment_metas.push(EvidenceSegmentV1 {
            relative_path: relative,
            size,
            sha256,
        });
    }
    let evidence_sha256 = hex::encode(concat.finalize());
    let audit_root = AuditJournal::read_jsonl(&audit_path)?.verify()?;

    let manifest = FsnapManifestV1 {
        schema: SUPPORTED_SCHEMA.to_string(),
        build_identity: build_identity().to_string(),
        evidence_relative_path: segment_metas[0].relative_path.clone(),
        evidence_size,
        evidence_sha256,
        audit_relative_path: AUDIT_RELATIVE_PATH.to_string(),
        audit_sha256,
        audit_root,
        evidence_segments: Some(segment_metas),
        coverage: None,
    };
    write_manifest(package, &manifest)
}

fn reject_unsafe_relative_path(relative: &str) -> Result<(), CoreError> {
    let path = Path::new(relative);
    if path.is_absolute() {
        return Err(CoreError::Verification(
            "manifest path must not be absolute".to_string(),
        ));
    }
    if relative.contains("..") || relative.contains('\\') || relative.contains('\0') {
        return Err(CoreError::Verification(
            "manifest path contains an unsafe component".to_string(),
        ));
    }
    Ok(())
}

fn resolve_within_package(package: &Path, relative: &str) -> Result<PathBuf, CoreError> {
    reject_unsafe_relative_path(relative)?;
    let package_canonical =
        fs::canonicalize(package).map_err(|error| CoreError::Io(error.to_string()))?;
    let candidate = package.join(relative);
    let candidate_canonical =
        fs::canonicalize(&candidate).map_err(|error| CoreError::Io(error.to_string()))?;
    if !candidate_canonical.starts_with(&package_canonical) {
        return Err(CoreError::Verification(
            "manifest path escapes the package boundary".to_string(),
        ));
    }
    if fs::symlink_metadata(&candidate)
        .map_err(|error| CoreError::Io(error.to_string()))?
        .is_symlink()
    {
        return Err(CoreError::Verification(
            "package content must not be a symlink".to_string(),
        ));
    }
    Ok(candidate)
}

fn collect_regular_files(root: &Path, out: &mut Vec<PathBuf>) -> Result<(), CoreError> {
    for entry in fs::read_dir(root).map_err(|error| CoreError::Io(error.to_string()))? {
        let entry = entry.map_err(|error| CoreError::Io(error.to_string()))?;
        let metadata = entry
            .metadata()
            .map_err(|error| CoreError::Io(error.to_string()))?;
        if metadata.is_symlink() {
            return Err(CoreError::Verification(
                "package content must not be a symlink".to_string(),
            ));
        }
        if metadata.is_dir() {
            collect_regular_files(&entry.path(), out)?;
        } else if metadata.is_file() {
            out.push(entry.path());
        }
    }
    Ok(())
}

fn evidence_paths_from_manifest(
    package: &Path,
    manifest: &FsnapManifestV1,
) -> Result<Vec<PathBuf>, CoreError> {
    match &manifest.evidence_segments {
        Some(segments) if !segments.is_empty() => {
            let mut paths = Vec::with_capacity(segments.len());
            for segment in segments {
                paths.push(resolve_within_package(package, &segment.relative_path)?);
            }
            if manifest.evidence_relative_path != segments[0].relative_path {
                return Err(CoreError::Verification(
                    "evidence_relative_path must match the first evidence segment".to_string(),
                ));
            }
            Ok(paths)
        }
        _ => Ok(vec![resolve_within_package(
            package,
            &manifest.evidence_relative_path,
        )?]),
    }
}

pub fn verify_fsnap(package: &Path) -> Result<FsnapManifestV1, CoreError> {
    let manifest_path = resolve_within_package(package, MANIFEST_RELATIVE_PATH)?;
    let manifest_bytes =
        fs::read(&manifest_path).map_err(|error| CoreError::Io(error.to_string()))?;
    let manifest: FsnapManifestV1 = serde_json::from_slice(&manifest_bytes)
        .map_err(|error| CoreError::Serialization(error.to_string()))?;

    if manifest.schema != SUPPORTED_SCHEMA {
        return Err(CoreError::Verification(format!(
            "unsupported fsnap manifest schema: {}",
            manifest.schema
        )));
    }
    if manifest.build_identity != build_identity() {
        return Err(CoreError::Verification(format!(
            "unsupported fsnap build identity: {}",
            manifest.build_identity
        )));
    }

    let evidence_paths = evidence_paths_from_manifest(package, &manifest)?;
    let audit_path = resolve_within_package(package, &manifest.audit_relative_path)?;

    let package_canonical =
        fs::canonicalize(package).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut all_files = Vec::new();
    collect_regular_files(&package_canonical, &mut all_files)?;

    let mut allowed: Vec<PathBuf> = vec![
        fs::canonicalize(&manifest_path).map_err(|error| CoreError::Io(error.to_string()))?,
        fs::canonicalize(&audit_path).map_err(|error| CoreError::Io(error.to_string()))?,
    ];
    for path in &evidence_paths {
        allowed.push(fs::canonicalize(path).map_err(|error| CoreError::Io(error.to_string()))?);
    }
    for file in &all_files {
        let canonical = fs::canonicalize(file).map_err(|error| CoreError::Io(error.to_string()))?;
        if !allowed.contains(&canonical) {
            return Err(CoreError::Verification(format!(
                "unexpected file present in package: {}",
                file.display()
            )));
        }
    }

    let path_refs: Vec<&Path> = evidence_paths.iter().map(PathBuf::as_path).collect();
    let (evidence_size, evidence_sha256) = if let Some(segments) = &manifest.evidence_segments {
        if segments.len() != evidence_paths.len() {
            return Err(CoreError::Verification(
                "evidence segment count mismatch".to_string(),
            ));
        }
        let (per_seg, total, digest) = hash_segments_and_concat(&path_refs)?;
        for (segment, (size, sha256)) in segments.iter().zip(per_seg.iter()) {
            if *size != segment.size {
                return Err(CoreError::Verification(format!(
                    "segment size does not match manifest: {}",
                    segment.relative_path
                )));
            }
            if sha256 != &segment.sha256 {
                return Err(CoreError::Verification(format!(
                    "segment sha256 does not match manifest: {}",
                    segment.relative_path
                )));
            }
        }
        (total, digest)
    } else {
        hash_files_concat(&path_refs)?
    };
    if evidence_size != manifest.evidence_size {
        return Err(CoreError::Verification(
            "evidence size does not match manifest".to_string(),
        ));
    }
    if evidence_sha256 != manifest.evidence_sha256 {
        return Err(CoreError::Verification(
            "evidence sha256 does not match manifest".to_string(),
        ));
    }

    let (_, audit_sha256) = hash_file(&audit_path)?;
    if audit_sha256 != manifest.audit_sha256 {
        return Err(CoreError::Verification(
            "audit sha256 does not match manifest".to_string(),
        ));
    }

    let audit_root = AuditJournal::read_jsonl(&audit_path)?.verify()?;
    if audit_root != manifest.audit_root {
        return Err(CoreError::Verification(
            "audit root does not match manifest".to_string(),
        ));
    }

    Ok(manifest)
}
