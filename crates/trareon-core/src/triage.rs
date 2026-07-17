//! Live triage collectors (process / network / users) → hashed JSON bundle.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::CoreError;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TriageBundle {
    pub schema: String,
    pub collected_at: String,
    pub hostname: String,
    pub processes: String,
    pub network: String,
    pub users: String,
    pub footprint_note: String,
    pub sha256: String,
}

pub fn collect_live_triage() -> Result<TriageBundle, CoreError> {
    let hostname = std::env::var("HOSTNAME")
        .or_else(|_| std::env::var("COMPUTERNAME"))
        .unwrap_or_else(|_| "unknown".into());
    let processes = run_cmd_best_effort(&["ps", "aux"]).unwrap_or_else(|| "unavailable".into());
    let network = run_cmd_best_effort(&["netstat", "-an"])
        .or_else(|| run_cmd_best_effort(&["ss", "-an"]))
        .unwrap_or_else(|| "unavailable".into());
    let users = run_cmd_best_effort(&["who"]).unwrap_or_else(|| "unavailable".into());
    let collected_at = chrono::Utc::now().to_rfc3339();
    let mut hasher = Sha256::new();
    hasher.update(hostname.as_bytes());
    hasher.update(processes.as_bytes());
    hasher.update(network.as_bytes());
    hasher.update(users.as_bytes());
    let sha256 = hex::encode(hasher.finalize());
    Ok(TriageBundle {
        schema: "trareon.triage/1".into(),
        collected_at,
        hostname,
        processes,
        network,
        users,
        footprint_note: "live triage may subtly modify source (process table, logs)".into(),
        sha256,
    })
}

/// Synthetic triage from a fixture directory containing processes.txt, network.txt, users.txt.
pub fn collect_triage_from_fixture(dir: &Path) -> Result<TriageBundle, CoreError> {
    let read = |name: &str| -> Result<String, CoreError> {
        std::fs::read_to_string(dir.join(name)).map_err(|e| CoreError::Io(e.to_string()))
    };
    let hostname = read("hostname.txt").unwrap_or_else(|_| "fixture-host".into());
    let processes = read("processes.txt")?;
    let network = read("network.txt")?;
    let users = read("users.txt")?;
    let collected_at = "1970-01-01T00:00:00Z".into();
    let mut hasher = Sha256::new();
    hasher.update(hostname.as_bytes());
    hasher.update(processes.as_bytes());
    hasher.update(network.as_bytes());
    hasher.update(users.as_bytes());
    Ok(TriageBundle {
        schema: "trareon.triage/1".into(),
        collected_at,
        hostname,
        processes,
        network,
        users,
        footprint_note: "synthetic fixture".into(),
        sha256: hex::encode(hasher.finalize()),
    })
}

pub fn write_triage_bundle(bundle: &TriageBundle, path: &Path) -> Result<PathBuf, CoreError> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| CoreError::Io(e.to_string()))?;
    }
    let json = serde_json::to_string_pretty(bundle)
        .map_err(|e| CoreError::Verification(e.to_string()))?;
    std::fs::write(path, json).map_err(|e| CoreError::Io(e.to_string()))?;
    Ok(path.to_path_buf())
}

fn run_cmd_best_effort(argv: &[&str]) -> Option<String> {
    let (prog, args) = argv.split_first()?;
    let output = Command::new(prog).args(args).output().ok()?;
    if !output.status.success() && output.stdout.is_empty() {
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixture_triage_hashes() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("processes.txt"), "pid 1 init\n").unwrap();
        std::fs::write(dir.path().join("network.txt"), "tcp 0.0.0.0:22\n").unwrap();
        std::fs::write(dir.path().join("users.txt"), "lab\n").unwrap();
        let b = collect_triage_from_fixture(dir.path()).unwrap();
        assert_eq!(b.sha256.len(), 64);
        let out = dir.path().join("triage.json");
        write_triage_bundle(&b, &out).unwrap();
        assert!(out.is_file());
    }
}
