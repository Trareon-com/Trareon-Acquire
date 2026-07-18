use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use zip::{CompressionMethod, ZipArchive, ZipWriter, write::SimpleFileOptions};

use crate::{CoreError, verify_fsnap};

pub fn pack_fsnap(package: &Path, archive_path: &Path) -> Result<(), CoreError> {
    verify_fsnap(package)?;
    let archive = File::create(archive_path).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut writer = ZipWriter::new(archive);
    let options = SimpleFileOptions::default().compression_method(CompressionMethod::Deflated);
    for path in regular_files(package)? {
        let relative = path
            .strip_prefix(package)
            .map_err(|error| CoreError::Io(error.to_string()))?
            .to_string_lossy()
            .replace('\\', "/");
        writer
            .start_file(relative, options)
            .map_err(|error| CoreError::Io(error.to_string()))?;
        let mut source = File::open(&path).map_err(|error| CoreError::Io(error.to_string()))?;
        std::io::copy(&mut source, &mut writer)
            .map_err(|error| CoreError::Io(error.to_string()))?;
    }
    writer
        .finish()
        .map_err(|error| CoreError::Io(error.to_string()))?
        .sync_all()
        .map_err(|error| CoreError::Io(error.to_string()))
}

pub fn unpack_fsnap(archive_path: &Path, destination: &Path) -> Result<(), CoreError> {
    if destination.exists() {
        return Err(CoreError::Verification(
            "archive destination must not already exist".to_string(),
        ));
    }
    let archive = File::open(archive_path).map_err(|error| CoreError::Io(error.to_string()))?;
    let mut zip =
        ZipArchive::new(archive).map_err(|error| CoreError::Verification(error.to_string()))?;
    fs::create_dir_all(destination).map_err(|error| CoreError::Io(error.to_string()))?;
    for index in 0..zip.len() {
        let mut entry = zip
            .by_index(index)
            .map_err(|error| CoreError::Verification(error.to_string()))?;
        let enclosed = entry
            .enclosed_name()
            .ok_or_else(|| {
                CoreError::Verification("archive entry escapes destination".to_string())
            })?
            .to_path_buf();
        if entry.is_dir() {
            return Err(CoreError::Verification(
                "fsnap archive must contain files only".to_string(),
            ));
        }
        let output = destination.join(enclosed);
        let parent = output
            .parent()
            .ok_or_else(|| CoreError::Verification("archive entry has no parent".to_string()))?;
        fs::create_dir_all(parent).map_err(|error| CoreError::Io(error.to_string()))?;
        let mut file = File::create(output).map_err(|error| CoreError::Io(error.to_string()))?;
        std::io::copy(&mut entry, &mut file).map_err(|error| CoreError::Io(error.to_string()))?;
        file.sync_all()
            .map_err(|error| CoreError::Io(error.to_string()))?;
    }
    verify_fsnap(destination).map(|_| ())
}

fn regular_files(root: &Path) -> Result<Vec<PathBuf>, CoreError> {
    let mut files = Vec::new();
    collect_regular_files(root, &mut files)?;
    files.sort();
    Ok(files)
}

fn collect_regular_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), CoreError> {
    for entry in fs::read_dir(root).map_err(|error| CoreError::Io(error.to_string()))? {
        let entry = entry.map_err(|error| CoreError::Io(error.to_string()))?;
        let metadata =
            fs::symlink_metadata(entry.path()).map_err(|error| CoreError::Io(error.to_string()))?;
        if metadata.file_type().is_symlink() {
            return Err(CoreError::Verification(
                "fsnap archive cannot contain symlinks".to_string(),
            ));
        }
        if metadata.is_dir() {
            collect_regular_files(&entry.path(), files)?;
        } else if metadata.is_file() {
            files.push(entry.path());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{AcquisitionId, AcquisitionState, AuditJournal, create_fsnap};
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn package_round_trips_through_zip() {
        let dir = tempfile::tempdir().unwrap();
        let raw = dir.path().join("raw");
        let audit = dir.path().join("audit.jsonl");
        let package = dir.path().join("evidence.fsnap");
        fs::write(&raw, b"evidence").unwrap();
        let mut journal = AuditJournal::default();
        journal
            .append(
                AcquisitionId(Uuid::nil()),
                Utc::now(),
                AcquisitionState::Acquiring,
                "archive test",
            )
            .unwrap();
        journal.write_jsonl(&audit).unwrap();
        create_fsnap(&raw, &audit, &package).unwrap();
        let archive = dir.path().join("evidence.fsnap.zip");
        let unpacked = dir.path().join("unpacked.fsnap");
        pack_fsnap(&package, &archive).unwrap();
        unpack_fsnap(&archive, &unpacked).unwrap();
    }
}
