use std::{fs, path::Path};

use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand_core::OsRng;

use crate::CoreError;

const MANIFEST_RELATIVE_PATH: &str = "manifest/manifest.json";

pub fn generate_signing_key() -> SigningKey {
    SigningKey::generate(&mut OsRng)
}

pub fn sign_bytes(signing_key: &SigningKey, bytes: &[u8]) -> Vec<u8> {
    signing_key.sign(bytes).to_bytes().to_vec()
}

pub fn verify_bytes(
    verifying_key: &VerifyingKey,
    bytes: &[u8],
    signature: &[u8],
) -> Result<(), CoreError> {
    let signature = Signature::from_slice(signature)
        .map_err(|error| CoreError::Verification(format!("invalid Ed25519 signature: {error}")))?;
    verifying_key.verify(bytes, &signature).map_err(|error| {
        CoreError::Verification(format!("package seal verification failed: {error}"))
    })
}

pub fn sign_package_seal(package: &Path, signing_key: &SigningKey) -> Result<Vec<u8>, CoreError> {
    let manifest = fs::read(package.join(MANIFEST_RELATIVE_PATH))
        .map_err(|error| CoreError::Io(error.to_string()))?;
    Ok(sign_bytes(signing_key, &manifest))
}

pub fn verify_package_seal(
    package: &Path,
    verifying_key: &VerifyingKey,
    signature: &[u8],
) -> Result<(), CoreError> {
    let manifest = fs::read(package.join(MANIFEST_RELATIVE_PATH))
        .map_err(|error| CoreError::Io(error.to_string()))?;
    verify_bytes(verifying_key, &manifest, signature)
}

pub fn write_detached_signature(path: &Path, signature: &[u8]) -> Result<(), CoreError> {
    fs::write(path, signature).map_err(|error| CoreError::Io(error.to_string()))
}

pub fn read_detached_signature(path: &Path) -> Result<Vec<u8>, CoreError> {
    fs::read(path).map_err(|error| CoreError::Io(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signature_verifies_only_original_data() {
        let key = generate_signing_key();
        let signature = sign_bytes(&key, b"sealed package");
        verify_bytes(&key.verifying_key(), b"sealed package", &signature).unwrap();
        assert!(verify_bytes(&key.verifying_key(), b"changed package", &signature).is_err());
    }
}
