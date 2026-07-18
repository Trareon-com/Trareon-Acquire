//! M10 verification-tool wrappers for the desktop shell.

use std::{
    fs,
    path::{Path, PathBuf},
};

use ed25519_dalek::VerifyingKey;
use trareon_core::{
    EvidenceSegmentV1, FsnapManifestV1, read_detached_signature, verify_package_seal,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureStatus {
    NotPresent,
    Verified,
    PublicKeyUnavailable,
}

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub manifest: FsnapManifestV1,
    pub signature: SignatureStatus,
}

pub fn verify_package(package: &Path) -> Result<VerificationResult, String> {
    let manifest = trareon_verifier::verify(package)?;
    let signature = verify_signature_if_present(package)?;
    Ok(VerificationResult {
        manifest,
        signature,
    })
}

pub fn hash_only(package: &Path) -> Result<(String, u64), String> {
    trareon_verifier::hash_only(package)
}

pub fn info(package: &Path) -> Result<trareon_verifier::PackageInfo, String> {
    trareon_verifier::info(package)
}

pub fn compare(a: &Path, b: &Path) -> Result<bool, String> {
    trareon_verifier::compare(a, b)
}

pub fn integrity(package: &Path) -> Result<FsnapManifestV1, String> {
    trareon_verifier::integrity(package)
}

pub fn export(package: &Path, output: &Path) -> Result<(), String> {
    trareon_verifier::export(package, output)
}

pub fn export_segments(
    package: &Path,
    segments: &[EvidenceSegmentV1],
    output: &Path,
) -> Result<(), String> {
    trareon_verifier::export_segments(package, segments, output)
}

/// Verify a detached package signature when its public key is available.
pub fn verify_signature(package: &Path, public_key: &Path) -> Result<(), String> {
    let signature_path = signature_path(package);
    let key_bytes = fs::read(public_key).map_err(|error| error.to_string())?;
    let key: [u8; 32] = key_bytes
        .try_into()
        .map_err(|_| "invalid Ed25519 public key length".to_string())?;
    let verifying_key = VerifyingKey::from_bytes(&key).map_err(|error| error.to_string())?;
    let signature = read_detached_signature(&signature_path).map_err(|error| error.to_string())?;
    verify_package_seal(package, &verifying_key, &signature).map_err(|error| error.to_string())
}

/// Checks `<package>.fsnap.sig`; a colocated `signing-key.ed25519.pub` is optional.
pub fn verify_signature_if_present(package: &Path) -> Result<SignatureStatus, String> {
    let signature = signature_path(package);
    if !signature.is_file() {
        return Ok(SignatureStatus::NotPresent);
    }
    let public_key = signature
        .parent()
        .unwrap_or_else(|| Path::new("."))
        .join("signing-key.ed25519.pub");
    if !public_key.is_file() {
        return Ok(SignatureStatus::PublicKeyUnavailable);
    }
    verify_signature(package, &public_key)?;
    Ok(SignatureStatus::Verified)
}

fn signature_path(package: &Path) -> PathBuf {
    package.with_extension("fsnap.sig")
}

#[cfg(test)]
mod tests {
    use super::*;
    use trareon_core::{generate_signing_key, sign_package_seal, write_detached_signature};

    #[test]
    fn wraps_verifier_and_optional_signature() {
        let dir = tempfile::tempdir().unwrap();
        let package = crate::run_synthetic_demo(dir.path()).unwrap();
        let key = generate_signing_key();
        write_detached_signature(
            &signature_path(&package),
            &sign_package_seal(&package, &key).unwrap(),
        )
        .unwrap();
        fs::write(
            dir.path().join("signing-key.ed25519.pub"),
            key.verifying_key().as_bytes(),
        )
        .unwrap();

        assert_eq!(
            verify_package(&package).unwrap().signature,
            SignatureStatus::Verified
        );
        assert_eq!(hash_only(&package).unwrap().1, 256 * 1024);
        let exported = dir.path().join("evidence.raw");
        export(&package, &exported).unwrap();
        assert!(exported.is_file());
    }
}
