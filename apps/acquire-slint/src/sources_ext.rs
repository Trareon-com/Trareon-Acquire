use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use trareon_core::{
    AcquisitionSource, CloudSnapshotSource, MobileLogicalSource, NetworkCaptureSource,
    SyntheticRamSource,
};

/// Live RAM / macOS adapter availability (Wave 3c).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiveAdapterStatus {
    Available { binary: PathBuf },
    Unavailable { reason: String },
}

/// Probe for Microsoft AVML (Linux RAM) on PATH or `TRAREON_AVML`.
pub fn probe_avml() -> LiveAdapterStatus {
    match resolve_tool("avml", "TRAREON_AVML") {
        Some(binary) => LiveAdapterStatus::Available { binary },
        None => LiveAdapterStatus::Unavailable {
            reason: "avml not on PATH (install microsoft/avml or set TRAREON_AVML); live RAM Unavailable"
                .into(),
        },
    }
}

/// Probe for Fuji-style macOS collection helper on PATH or `TRAREON_FUJI`.
pub fn probe_fuji() -> LiveAdapterStatus {
    match resolve_tool("fuji", "TRAREON_FUJI") {
        Some(binary) => LiveAdapterStatus::Available { binary },
        None => LiveAdapterStatus::Unavailable {
            reason: "fuji not on PATH (Lazza/Fuji GPL — audit before use; set TRAREON_FUJI); macOS live Unavailable"
                .into(),
        },
    }
}

/// Run AVML into `output` when the binary is present. Never invents a RAM image.
pub fn run_avml_capture(output: &Path) -> Result<PathBuf, String> {
    let binary = match probe_avml() {
        LiveAdapterStatus::Available { binary } => binary,
        LiveAdapterStatus::Unavailable { reason } => return Err(reason),
    };
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    let status = Command::new(&binary)
        .arg("--output")
        .arg(output)
        .status()
        .map_err(|e| format!("avml spawn failed: {e}"))?;
    if !status.success() {
        return Err(format!("avml exited with {status}"));
    }
    if !output.is_file() {
        return Err("avml reported success but output file is missing".into());
    }
    Ok(output.to_path_buf())
}

/// Documented Fuji dry-run: prints plan only unless binary exists; never claims capture without tool.
pub fn run_fuji_plan(output_dir: &Path) -> Result<String, String> {
    match probe_fuji() {
        LiveAdapterStatus::Available { binary } => {
            let status = Command::new(&binary)
                .arg("--help")
                .output()
                .map_err(|e| format!("fuji spawn failed: {e}"))?;
            Ok(format!(
                "Fuji helper present at {}\noutput_dir={}\nhelp_exit={:?}\n{}",
                binary.display(),
                output_dir.display(),
                status.status.code(),
                String::from_utf8_lossy(&status.stdout)
                    .lines()
                    .take(8)
                    .collect::<Vec<_>>()
                    .join("\n")
            ))
        }
        LiveAdapterStatus::Unavailable { reason } => Err(reason),
    }
}

fn resolve_tool(name: &str, env_key: &str) -> Option<PathBuf> {
    if let Ok(path) = std::env::var(env_key) {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(p);
        }
    }
    let path = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path) {
        let candidate = dir.join(name);
        if candidate.is_file() {
            return Some(candidate);
        }
        #[cfg(windows)]
        {
            let exe = dir.join(format!("{name}.exe"));
            if exe.is_file() {
                return Some(exe);
            }
        }
    }
    None
}

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
    fn avml_and_fuji_probe_are_honest_when_missing() {
        // Hosts without the tools must report Unavailable — never invent capture.
        if !matches!(probe_avml(), LiveAdapterStatus::Available { .. }) {
            assert!(matches!(
                probe_avml(),
                LiveAdapterStatus::Unavailable { .. }
            ));
            assert!(run_avml_capture(Path::new("/tmp/trareon-avml-test.lime")).is_err());
        }
        if !matches!(probe_fuji(), LiveAdapterStatus::Available { .. }) {
            assert!(run_fuji_plan(Path::new("/tmp")).is_err());
        }
    }

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
