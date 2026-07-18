//! M13 boot-media planning. This module never writes a device.

use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use sha2::{Digest, Sha256};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BootWritePlan {
    pub image: PathBuf,
    pub destination_device: PathBuf,
    pub dry_run_only: bool,
    pub warnings: Vec<String>,
}

pub fn verify_boot_image_hash(path: &Path, expected_sha256: &str) -> Result<bool, String> {
    let mut reader = BufReader::new(File::open(path).map_err(|error| error.to_string())?);
    let mut hasher = Sha256::new();
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
    Ok(hex::encode(hasher.finalize()).eq_ignore_ascii_case(expected_sha256.trim()))
}

pub fn prepare_boot_write_plan(image: &Path, dest_device: &Path) -> Result<BootWritePlan, String> {
    if !image.is_file() {
        return Err(format!("boot image is not a file: {}", image.display()));
    }
    Ok(BootWritePlan {
        image: image.to_path_buf(),
        destination_device: dest_device.to_path_buf(),
        dry_run_only: true,
        warnings: vec![
            "Dry-run only: this plan does not write or wipe any device.".into(),
            "Writing boot media destroys data on the selected destination device.".into(),
            "Verify the image SHA-256 and destination identity before enabling a writer.".into(),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_verification_and_write_plan_are_non_destructive() {
        let dir = tempfile::tempdir().unwrap();
        let image = dir.path().join("boot.img");
        std::fs::write(&image, b"boot image").unwrap();
        let expected = hex::encode(Sha256::digest(b"boot image"));
        assert!(verify_boot_image_hash(&image, &expected).unwrap());
        let plan = prepare_boot_write_plan(&image, Path::new("/dev/never-write")).unwrap();
        assert!(plan.dry_run_only);
    }
}
