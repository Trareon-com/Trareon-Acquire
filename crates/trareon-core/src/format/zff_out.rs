//! Optional ZFF output via external `zffacquire` (Apache-2.0 / MIT tooling).
//!
//! The `zff` crate API is large; Wave 3c ships an honest external-tool adapter.
//! Library-level `zff-rs` embedding can follow once a minimal physical-write
//! helper is reviewed. Feature: `zff`.

use std::{
    path::{Path, PathBuf},
    process::Command,
};

use crate::CoreError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ZffWriteResult {
    pub output_prefix: PathBuf,
    pub note: String,
}

/// Resolve `zffacquire` from PATH or `TRAREON_ZFFACQUIRE`.
pub fn resolve_zffacquire() -> Option<PathBuf> {
    if let Ok(path) = std::env::var("TRAREON_ZFFACQUIRE") {
        let p = PathBuf::from(path);
        if p.is_file() {
            return Some(p);
        }
    }
    which("zffacquire")
}

/// Write a physical ZFF container from a raw file using `zffacquire` when installed.
///
/// `output_prefix` is the `-o` argument (tool appends segment extensions).
pub fn write_zff_physical(
    raw_input: &Path,
    output_prefix: &Path,
) -> Result<ZffWriteResult, CoreError> {
    let Some(bin) = resolve_zffacquire() else {
        return Err(CoreError::Verification(
            "zffacquire not on PATH (install zffacquire or set TRAREON_ZFFACQUIRE); ZFF optional"
                .into(),
        ));
    };
    if !raw_input.is_file() {
        return Err(CoreError::Io(format!(
            "ZFF source is not a file: {}",
            raw_input.display()
        )));
    }
    if let Some(parent) = output_prefix.parent() {
        std::fs::create_dir_all(parent).map_err(|e| CoreError::Io(e.to_string()))?;
    }
    let output = Command::new(&bin)
        .args([
            "physical",
            "-i",
            &raw_input.display().to_string(),
            "-o",
            &output_prefix.display().to_string(),
        ])
        .output()
        .map_err(|e| CoreError::Io(format!("zffacquire spawn failed: {e}")))?;
    if !output.status.success() {
        return Err(CoreError::Io(format!(
            "zffacquire failed (code {:?}): {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stderr)
        )));
    }
    Ok(ZffWriteResult {
        output_prefix: output_prefix.to_path_buf(),
        note: format!("wrote via {}", bin.display()),
    })
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
    fn missing_zffacquire_is_unsupported() {
        // Only assert when tool is truly absent.
        if resolve_zffacquire().is_none() {
            let err =
                write_zff_physical(Path::new("/tmp/nope.raw"), Path::new("/tmp/out")).unwrap_err();
            assert!(matches!(err, CoreError::Verification(_)));
        }
    }
}
