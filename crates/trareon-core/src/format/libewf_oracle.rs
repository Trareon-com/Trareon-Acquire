//! Optional libewf oracle: run `ewfverify` when present on PATH.
//!
//! Feature: `libewf-oracle`. Never shells through `sh -c`. Missing binary is not an error
//! for CI — callers treat [`OracleStatus::ToolMissing`] as skip.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OracleStatus {
    ToolMissing,
    Passed { stdout: String },
    Failed { code: Option<i32>, stderr: String },
}

/// Resolve `ewfverify` from `PATH` or `TRAREON_EWFVERIFY`.
pub fn resolve_ewfverify() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("TRAREON_EWFVERIFY") {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(p);
        }
    }
    which("ewfverify")
}

/// Run libewf `ewfverify` against an EWF segment path.
pub fn ewfverify(path: &Path) -> Result<OracleStatus, CoreError> {
    let Some(bin) = resolve_ewfverify() else {
        return Ok(OracleStatus::ToolMissing);
    };
    let output = Command::new(&bin)
        .arg(path)
        .output()
        .map_err(|e| CoreError::Io(format!("ewfverify spawn failed: {e}")))?;
    if output.status.success() {
        Ok(OracleStatus::Passed {
            stdout: String::from_utf8_lossy(&output.stdout).into_owned(),
        })
    } else {
        Ok(OracleStatus::Failed {
            code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).into_owned(),
        })
    }
}

fn which(name: &str) -> Option<PathBuf> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_tool_is_skip_not_panic() {
        // If the host has ewfverify, ToolMissing won't apply — either status is fine.
        let status = ewfverify(Path::new("/nonexistent.E01")).unwrap();
        assert!(matches!(
            status,
            OracleStatus::ToolMissing | OracleStatus::Failed { .. }
        ));
    }
}
