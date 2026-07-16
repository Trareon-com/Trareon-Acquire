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

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FsnapManifestV1 {
    pub schema: String,
    pub build_identity: String,
    pub evidence_relative_path: String,
    pub evidence_size: u64,
    pub evidence_sha256: String,
    pub audit_relative_path: String,
    pub audit_sha256: String,
    pub audit_root: String,
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

fn copy_and_sync(source: &Path, destination: &Path) -> Result<(), CoreError> {
    fs::copy(source, destination).map_err(|error| CoreError::Io(error.to_string()))?;
    // Must be opened with write access: on Windows, FlushFileBuffers (which
    // sync_all calls) fails with ERROR_ACCESS_DENIED on a read-only handle.
    let file = File::options()
        .write(true)
        .open(destination)
        .map_err(|error| CoreError::Io(error.to_string()))?;
    file.sync_all()
        .map_err(|error| CoreError::Io(error.to_string()))?;
    Ok(())
}

pub fn create_fsnap(raw: &Path, audit: &Path, package: &Path) -> Result<(), CoreError> {
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

    let evidence_path = package.join(EVIDENCE_RELATIVE_PATH);
    let audit_path = package.join(AUDIT_RELATIVE_PATH);
    copy_and_sync(raw, &evidence_path)?;
    copy_and_sync(audit, &audit_path)?;

    let (evidence_size, evidence_sha256) = hash_file(&evidence_path)?;
    let (_, audit_sha256) = hash_file(&audit_path)?;
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
    };

    let manifest_path = package.join(MANIFEST_RELATIVE_PATH);
    let manifest_bytes = serde_json::to_vec(&manifest)
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

    let evidence_path = resolve_within_package(package, &manifest.evidence_relative_path)?;
    let audit_path = resolve_within_package(package, &manifest.audit_relative_path)?;

    let package_canonical =
        fs::canonicalize(package).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut all_files = Vec::new();
    collect_regular_files(&package_canonical, &mut all_files)?;

    let allowed: Vec<PathBuf> = vec![
        fs::canonicalize(&manifest_path).map_err(|error| CoreError::Io(error.to_string()))?,
        fs::canonicalize(&evidence_path).map_err(|error| CoreError::Io(error.to_string()))?,
        fs::canonicalize(&audit_path).map_err(|error| CoreError::Io(error.to_string()))?,
    ];
    for file in &all_files {
        let canonical = fs::canonicalize(file).map_err(|error| CoreError::Io(error.to_string()))?;
        if !allowed.contains(&canonical) {
            return Err(CoreError::Verification(format!(
                "unexpected file present in package: {}",
                file.display()
            )));
        }
    }

    let (evidence_size, evidence_sha256) = hash_file(&evidence_path)?;
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
