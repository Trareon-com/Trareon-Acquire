use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};

use trareon_core::{
    AcquisitionSource, CloudSnapshotSource, MobileLogicalSource, NetworkCaptureSource,
    SyntheticRamSource,
};

/// Snapshot differences based solely on relative name and file size.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiffReport {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub changed: Vec<String>,
    pub host_mismatch: bool,
    pub requires_override: bool,
}

/// Copies a synthetic RAM source to `output`.
pub fn run_ram_synthetic(source: &Path, output: &Path) -> Result<PathBuf, String> {
    copy_source(SyntheticRamSource::new(source), output)
}

/// Copies a local network-capture fixture to `output`.
pub fn run_network_fixture(source: &Path, output: &Path) -> Result<PathBuf, String> {
    copy_source(NetworkCaptureSource::new(source), output)
}

/// Serializes a cloud-directory fixture through the core cloud source.
pub fn run_cloud_dir(source_dir: &Path, output: &Path) -> Result<PathBuf, String> {
    copy_source(CloudSnapshotSource::new(source_dir), output)
}

/// Serializes a mobile logical fixture through the core mobile source.
pub fn run_mobile_logical(source_dir: &Path, output: &Path) -> Result<PathBuf, String> {
    copy_source(MobileLogicalSource::new(source_dir), output)
}

fn copy_source(mut source: impl AcquisitionSource, output: &Path) -> Result<PathBuf, String> {
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let mut input = source.open().map_err(|error| error.to_string())?;
    let mut destination = fs::File::create(output).map_err(|error| error.to_string())?;
    std::io::copy(&mut input, &mut destination).map_err(|error| error.to_string())?;
    Ok(output.to_path_buf())
}

/// Compares two local snapshot directories; this intentionally performs no malware verdicting.
pub fn snapshot_compare(a_dir: &Path, b_dir: &Path) -> Result<DiffReport, String> {
    let a = file_sizes(a_dir)?;
    let b = file_sizes(b_dir)?;
    let mut report = DiffReport {
        host_mismatch: marker(a_dir)? != marker(b_dir)?,
        ..DiffReport::default()
    };
    report.requires_override = report.host_mismatch;

    for (name, size) in &b {
        match a.get(name) {
            None if name != "HOST_ID" => report.added.push(name.clone()),
            Some(other_size) if other_size != size && name != "HOST_ID" => {
                report.changed.push(name.clone());
            }
            _ => {}
        }
    }
    for name in a.keys() {
        if !b.contains_key(name) && name != "HOST_ID" {
            report.removed.push(name.clone());
        }
    }
    Ok(report)
}

fn marker(dir: &Path) -> Result<Option<String>, String> {
    let path = dir.join("HOST_ID");
    if path.is_file() {
        fs::read_to_string(path)
            .map(|value| Some(value.trim().to_owned()))
            .map_err(|error| error.to_string())
    } else {
        Ok(None)
    }
}

fn file_sizes(root: &Path) -> Result<BTreeMap<String, u64>, String> {
    if !root.is_dir() {
        return Err(format!(
            "snapshot directory does not exist: {}",
            root.display()
        ));
    }
    let mut files = BTreeMap::new();
    collect_file_sizes(root, root, &mut files)?;
    Ok(files)
}

fn collect_file_sizes(
    root: &Path,
    current: &Path,
    files: &mut BTreeMap<String, u64>,
) -> Result<(), String> {
    for entry in fs::read_dir(current).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.is_dir() {
            collect_file_sizes(root, &path, files)?;
        } else if path.is_file() {
            let name = path
                .strip_prefix(root)
                .map_err(|error| error.to_string())?
                .to_string_lossy()
                .replace('\\', "/");
            files.insert(
                name,
                entry.metadata().map_err(|error| error.to_string())?.len(),
            );
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn snapshot_compare_reports_changes_and_host_override_requirement() {
        let dir = tempfile::tempdir().unwrap();
        let first = dir.path().join("first");
        let second = dir.path().join("second");
        std::fs::create_dir_all(&first).unwrap();
        std::fs::create_dir_all(&second).unwrap();
        std::fs::write(first.join("HOST_ID"), "host-a").unwrap();
        std::fs::write(second.join("HOST_ID"), "host-b").unwrap();
        std::fs::write(first.join("removed.txt"), "old").unwrap();
        std::fs::write(second.join("added.txt"), "new").unwrap();
        std::fs::write(first.join("same-name.txt"), "one").unwrap();
        std::fs::write(second.join("same-name.txt"), "longer").unwrap();

        let report = snapshot_compare(&first, &second).unwrap();

        assert_eq!(report.added, vec!["added.txt"]);
        assert_eq!(report.removed, vec!["removed.txt"]);
        assert_eq!(report.changed, vec!["same-name.txt"]);
        assert!(report.host_mismatch);
        assert!(report.requires_override);
    }
}
