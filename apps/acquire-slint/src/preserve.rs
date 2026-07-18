//! M6 preservation helpers kept independent from the Slint UI.

use std::{
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use chrono::Utc;
use ed25519_dalek::SigningKey;
use serde::Serialize;
use sha2::{Digest, Sha256};
use trareon_core::{
    CustodyEntry, EvidenceCoC, EvidenceId, append_custody_jsonl, generate_signing_key, pack_fsnap,
    sign_package_seal, verify_fsnap, write_detached_signature,
};

const PRIVATE_KEY_FILE: &str = "signing-key.ed25519";
const PUBLIC_KEY_FILE: &str = "signing-key.ed25519.pub";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreservationCopies {
    pub working: PathBuf,
    pub archive: PathBuf,
    pub manifest: PathBuf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopReason {
    Paused,
    Cancelled,
}

impl StopReason {
    fn as_str(self) -> &'static str {
        match self {
            Self::Paused => "paused",
            Self::Cancelled => "cancelled",
        }
    }
}

#[derive(Serialize)]
struct CopiesManifest {
    schema: &'static str,
    algorithm: &'static str,
    source_sha256: String,
    working_sha256: String,
    archive_sha256: String,
}

/// Verify and seal a package, writing operator-facing CoC artifacts beside it.
pub fn seal_package(
    package: &Path,
    case_id: &str,
    examiner: &str,
    device_id: &str,
) -> Result<EvidenceCoC, String> {
    let manifest = verify_fsnap(package).map_err(core_error)?;
    let parent = package_parent(package)?;
    let coc = EvidenceCoC::build(
        &EvidenceId::new(case_id, 1, 1),
        &manifest.evidence_sha256,
        &Utc::now().to_rfc3339(),
        examiner,
        device_id,
    )
    .with_rfc3161_attempt();

    write_json(&parent.join("coc.json"), &coc)?;
    coc.write_qr_png(&parent.join("qr.png"))
        .map_err(core_error)?;
    write_file(&parent.join("sticker.html"), coc.sticker_html().as_bytes())?;

    let mut entry = CustodyEntry::new(
        coc.acquired_at.clone(),
        "Created",
        examiner,
        coc.evidence_id.clone(),
    );
    entry.details = format!("sealed package {}", package.display());
    append_custody_jsonl(&parent.join("custody.jsonl"), &entry).map_err(core_error)?;
    Ok(coc)
}

/// Sign a package manifest with a persistent Ed25519 key from `keys_dir`.
///
/// Pass `~/.trareon/keys` for the application default; a missing keypair is generated there.
pub fn sign_package(package: &Path, keys_dir: &Path) -> Result<PathBuf, String> {
    verify_fsnap(package).map_err(core_error)?;
    fs::create_dir_all(keys_dir).map_err(io_error)?;
    let signing_key = load_or_generate_signing_key(keys_dir)?;
    let signature = sign_package_seal(package, &signing_key).map_err(core_error)?;
    let signature_path = package.with_extension("fsnap.sig");
    write_detached_signature(&signature_path, &signature).map_err(core_error)?;
    Ok(signature_path)
}

/// Copy a verified package into separate working and archival directories.
pub fn make_working_and_archive_copies(
    package: &Path,
    dest_parent: &Path,
) -> Result<PreservationCopies, String> {
    verify_fsnap(package).map_err(core_error)?;
    fs::create_dir_all(dest_parent).map_err(io_error)?;
    let working = dest_parent.join("working");
    let archive = dest_parent.join("archive");
    if working.exists() || archive.exists() {
        return Err("working and archive destinations must not already exist".to_string());
    }
    copy_tree(package, &working)?;
    copy_tree(package, &archive)?;
    verify_fsnap(&working).map_err(core_error)?;
    verify_fsnap(&archive).map_err(core_error)?;

    let manifest = dest_parent.join("copies-manifest.json");
    write_json(
        &manifest,
        &CopiesManifest {
            schema: "trareon.preservation-copies/1",
            algorithm: "sha256",
            source_sha256: hash_tree(package)?,
            working_sha256: hash_tree(&working)?,
            archive_sha256: hash_tree(&archive)?,
        },
    )?;
    Ok(PreservationCopies {
        working,
        archive,
        manifest,
    })
}

/// Write a report beside a verified package and return its path.
pub fn write_acquisition_report(
    package: &Path,
    case_id: &str,
    source: &str,
    limitations: &[String],
) -> Result<PathBuf, String> {
    let manifest = verify_fsnap(package).map_err(core_error)?;
    let report = package_parent(package)?.join("acquisition-report.html");
    let html = trareon_core::acquisition_report_html(
        case_id,
        source,
        &manifest.evidence_sha256,
        limitations,
    );
    write_file(&report, html.as_bytes())?;
    Ok(report)
}

/// Create a verified ZIP archive adjacent to the package.
pub fn archive_fsnap_zip(package: &Path) -> Result<PathBuf, String> {
    let archive = package.with_extension("fsnap.zip");
    pack_fsnap(package, &archive).map_err(core_error)?;
    Ok(archive)
}

/// Record the semantic meaning of an `acquire_file` cancellation beside its checkpoint.
///
/// `AcquireRequest::with_cancel_flag` only signals that acquisition stopped. The UI calls this
/// helper to distinguish an operator pause from a permanent cancellation without changing core
/// acquisition semantics.
pub fn write_stop_reason(checkpoint: &Path, reason: StopReason) -> Result<PathBuf, String> {
    let parent = checkpoint
        .parent()
        .ok_or_else(|| "checkpoint path has no parent directory".to_string())?;
    let note = parent.join(".acquire-stop-reason");
    write_file(&note, format!("{}\n", reason.as_str()).as_bytes())?;
    Ok(note)
}

fn load_or_generate_signing_key(keys_dir: &Path) -> Result<SigningKey, String> {
    let private_path = keys_dir.join(PRIVATE_KEY_FILE);
    if private_path.exists() {
        let bytes = fs::read(&private_path).map_err(io_error)?;
        let seed: [u8; 32] = bytes
            .try_into()
            .map_err(|_| "invalid Ed25519 private key length".to_string())?;
        return Ok(SigningKey::from_bytes(&seed));
    }

    let key = generate_signing_key();
    write_file(&private_path, &key.to_bytes())?;
    write_file(
        &keys_dir.join(PUBLIC_KEY_FILE),
        key.verifying_key().as_bytes(),
    )?;
    Ok(key)
}

fn copy_tree(source: &Path, destination: &Path) -> Result<(), String> {
    let metadata = fs::symlink_metadata(source).map_err(io_error)?;
    if metadata.file_type().is_symlink() {
        return Err(format!("refusing to copy symlink: {}", source.display()));
    }
    if metadata.is_file() {
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(io_error)?;
        }
        fs::copy(source, destination).map_err(io_error)?;
        return Ok(());
    }
    fs::create_dir(destination).map_err(io_error)?;
    for entry in fs::read_dir(source).map_err(io_error)? {
        let entry = entry.map_err(io_error)?;
        copy_tree(&entry.path(), &destination.join(entry.file_name()))?;
    }
    Ok(())
}

fn hash_tree(root: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    files.sort();
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 64 * 1024];
    for file in files {
        let relative = file
            .strip_prefix(root)
            .map_err(|error| error.to_string())?
            .to_string_lossy();
        hasher.update(relative.as_bytes());
        hasher.update(b"\0");
        let mut input = fs::File::open(&file).map_err(io_error)?;
        loop {
            let read = input.read(&mut buffer).map_err(io_error)?;
            if read == 0 {
                break;
            }
            hasher.update(&buffer[..read]);
        }
    }
    Ok(hex::encode(hasher.finalize()))
}

fn collect_files(path: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(path).map_err(io_error)? {
        let entry = entry.map_err(io_error)?;
        let entry_path = entry.path();
        let metadata = fs::symlink_metadata(&entry_path).map_err(io_error)?;
        if metadata.file_type().is_symlink() {
            return Err(format!(
                "refusing to hash symlink: {}",
                entry_path.display()
            ));
        }
        if metadata.is_dir() {
            collect_files(&entry_path, files)?;
        } else if metadata.is_file() {
            files.push(entry_path);
        }
    }
    Ok(())
}

fn package_parent(package: &Path) -> Result<&Path, String> {
    package
        .parent()
        .ok_or_else(|| "package path has no parent directory".to_string())
}

fn write_json(path: &Path, value: &impl Serialize) -> Result<(), String> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| error.to_string())?;
    write_file(path, &bytes)
}

fn write_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(io_error)?;
    }
    let mut file = fs::File::create(path).map_err(io_error)?;
    file.write_all(bytes).map_err(io_error)?;
    file.sync_all().map_err(io_error)
}

fn io_error(error: std::io::Error) -> String {
    error.to_string()
}

fn core_error(error: trareon_core::CoreError) -> String {
    error.to_string()
}
