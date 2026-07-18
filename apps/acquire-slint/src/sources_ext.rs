//! Live adapter probes (AVML / Fuji). No synthetic fixture-copy acquisition.

use std::{env, path::PathBuf, process::Command};

/// Live RAM / macOS adapter availability.
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

fn resolve_tool(name: &str, env_key: &str) -> Option<PathBuf> {
    if let Ok(path) = env::var(env_key) {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(p);
        }
    }
    let output = Command::new("which").arg(name).output().ok()?;
    if !output.status.success() {
        return None;
    }
    let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if path.is_empty() {
        None
    } else {
        Some(PathBuf::from(path))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn probes_return_structured_status() {
        let _ = probe_avml();
        let _ = probe_fuji();
    }
}
