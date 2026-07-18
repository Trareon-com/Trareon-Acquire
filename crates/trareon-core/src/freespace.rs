use std::{path::Path, process::Command};

use crate::CoreError;

const MIB: u64 = 1024 * 1024;

pub fn destination_free_bytes(path: &Path) -> Result<u64, CoreError> {
    #[cfg(not(target_os = "windows"))]
    {
        let output = Command::new("df")
            .args(["-kP", &path.to_string_lossy()])
            .output()
            .map_err(|error| CoreError::Io(error.to_string()))?;
        if !output.status.success() {
            return Err(CoreError::Io(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let line = stdout
            .lines()
            .last()
            .ok_or_else(|| CoreError::Io("df returned no filesystem row".to_string()))?;
        let fields: Vec<_> = line.split_whitespace().collect();
        let available_kib = fields
            .get(3)
            .ok_or_else(|| CoreError::Io("df output had no available-space column".to_string()))?
            .parse::<u64>()
            .map_err(|error| CoreError::Io(error.to_string()))?;
        available_kib
            .checked_mul(1024)
            .ok_or_else(|| CoreError::Io("free-space value overflowed".to_string()))
    }
    #[cfg(target_os = "windows")]
    {
        let escaped = path.to_string_lossy().replace('\'', "''");
        let command = format!("(Get-Item '{escaped}').PSDrive.Free");
        let output = Command::new("powershell")
            .args(["-NoProfile", "-Command", &command])
            .output()
            .map_err(|error| CoreError::Io(error.to_string()))?;
        if !output.status.success() {
            return Err(CoreError::Io(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u64>()
            .map_err(|error| CoreError::Io(error.to_string()))
    }
}

pub fn freespace_margin(source_size: u64) -> u64 {
    (source_size / 100).max(64 * MIB)
}

pub fn freespace_ok(free: u64, source_size: u64) -> bool {
    free >= source_size.saturating_add(freespace_margin(source_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn margin_is_at_least_64_mib() {
        assert_eq!(freespace_margin(1), 64 * MIB);
    }

    #[test]
    fn margin_becomes_one_percent_for_large_sources() {
        assert_eq!(freespace_margin(10 * 1024 * MIB), 1024 * MIB / 10);
        assert!(freespace_ok(128 * MIB, 64 * MIB));
        assert!(!freespace_ok(127 * MIB, 64 * MIB));
    }
}
