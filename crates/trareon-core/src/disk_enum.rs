#[cfg(any(target_os = "linux", target_os = "macos", target_os = "windows"))]
use std::process::Command;

#[cfg(target_os = "linux")]
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DiskRow {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
    pub kind: String,
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum EnumError {
    #[error("disk enumeration is unavailable: {0}")]
    Unavailable(String),
}

pub fn enumerate_disks() -> Result<Vec<DiskRow>, EnumError> {
    #[cfg(target_os = "linux")]
    {
        let output = Command::new("lsblk")
            .args(["-J", "-b", "-o", "PATH,NAME,SIZE,TYPE"])
            .output()
            .map_err(|error| EnumError::Unavailable(error.to_string()))?;
        if !output.status.success() {
            return Err(EnumError::Unavailable(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        parse_lsblk(&String::from_utf8_lossy(&output.stdout))
    }
    #[cfg(target_os = "macos")]
    {
        let output = Command::new("diskutil")
            .arg("list")
            .output()
            .map_err(|error| EnumError::Unavailable(error.to_string()))?;
        if !output.status.success() {
            return Err(EnumError::Unavailable(
                String::from_utf8_lossy(&output.stderr).trim().to_string(),
            ));
        }
        Ok(parse_diskutil_text(&String::from_utf8_lossy(
            &output.stdout,
        )))
    }
    #[cfg(target_os = "windows")]
    {
        enumerate_windows_disks()
    }
    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(EnumError::Unavailable(
            "unsupported operating system".to_string(),
        ))
    }
}

#[cfg(target_os = "linux")]
#[derive(Deserialize)]
struct LsblkOutput {
    blockdevices: Vec<LsblkDevice>,
}

#[cfg(target_os = "linux")]
#[derive(Deserialize)]
struct LsblkDevice {
    path: Option<String>,
    name: String,
    size: u64,
    #[serde(rename = "type")]
    kind: String,
}

#[cfg(target_os = "linux")]
fn parse_lsblk(input: &str) -> Result<Vec<DiskRow>, EnumError> {
    let parsed: LsblkOutput =
        serde_json::from_str(input).map_err(|error| EnumError::Unavailable(error.to_string()))?;
    Ok(parsed
        .blockdevices
        .into_iter()
        .filter_map(|disk| {
            disk.path.map(|path| DiskRow {
                path,
                name: disk.name,
                size_bytes: disk.size,
                kind: disk.kind,
            })
        })
        .collect())
}

#[cfg(target_os = "macos")]
fn parse_diskutil_text(input: &str) -> Vec<DiskRow> {
    input
        .lines()
        .filter_map(|line| {
            let disk_start = line.find("/dev/disk")?;
            let path = line[disk_start..].split_whitespace().next()?.to_string();
            let before = line[..disk_start].trim();
            let size_bytes = before
                .split('(')
                .nth(1)?
                .split_whitespace()
                .next()?
                .parse()
                .ok()?;
            Some(DiskRow {
                name: before.split('(').next()?.trim().to_string(),
                path,
                size_bytes,
                kind: "disk".to_string(),
            })
        })
        .collect()
}

#[cfg(target_os = "windows")]
fn enumerate_windows_disks() -> Result<Vec<DiskRow>, EnumError> {
    // CIM JSON: Index, Size, Model — maps to \\.\PhysicalDriveN
    let output = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            "Get-CimInstance Win32_DiskDrive | Select-Object Index,Size,Model,MediaType | ConvertTo-Json -Compress",
        ])
        .output()
        .map_err(|error| EnumError::Unavailable(error.to_string()))?;
    if !output.status.success() {
        return Err(EnumError::Unavailable(
            String::from_utf8_lossy(&output.stderr).trim().to_string(),
        ));
    }
    parse_windows_cim_json(&String::from_utf8_lossy(&output.stdout))
}

/// Parse PowerShell `ConvertTo-Json` output from Win32_DiskDrive (unit-tested on all hosts).
pub fn parse_windows_cim_json(input: &str) -> Result<Vec<DiskRow>, EnumError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Ok(Vec::new());
    }
    // PowerShell emits either one object or an array.
    let value: serde_json::Value =
        serde_json::from_str(trimmed).map_err(|error| EnumError::Unavailable(error.to_string()))?;
    let items = match value {
        serde_json::Value::Array(items) => items,
        other => vec![other],
    };
    Ok(items
        .into_iter()
        .filter_map(|item| {
            let index = item.get("Index")?.as_u64()?;
            let size = item.get("Size").and_then(|v| v.as_u64()).unwrap_or(0);
            let model = item
                .get("Model")
                .and_then(|v| v.as_str())
                .unwrap_or("disk")
                .trim()
                .to_string();
            let media = item
                .get("MediaType")
                .and_then(|v| v.as_str())
                .unwrap_or("disk");
            Some(DiskRow {
                path: format!(r"\\.\PhysicalDrive{index}"),
                name: model,
                size_bytes: size,
                kind: media.to_string(),
            })
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(target_os = "linux")]
    #[test]
    fn parses_lsblk_rows_without_inventing_paths() {
        let rows = parse_lsblk(r#"{"blockdevices":[{"path":"/dev/sda","name":"sda","size":1024,"type":"disk"},{"name":"ghost","size":1,"type":"disk"}]}"#).unwrap();
        assert_eq!(
            rows,
            vec![DiskRow {
                path: "/dev/sda".into(),
                name: "sda".into(),
                size_bytes: 1024,
                kind: "disk".into()
            }]
        );
    }

    #[test]
    fn windows_cim_json_maps_physical_drive_paths() {
        let rows = parse_windows_cim_json(
            r#"[{"Index":1,"Size":128,"Model":"USB Disk","MediaType":"External hard disk media"}]"#,
        )
        .unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0].path, r"\\.\PhysicalDrive1");
        assert_eq!(rows[0].size_bytes, 128);
        assert_eq!(rows[0].name, "USB Disk");
    }
}
