//! Read-only platform feasibility probes for Track C (RFC platform
//! adapters). These are spikes, not production adapters: they answer
//! "can this capability even be reached from the current privilege
//! level" without ever performing a raw-device read/write.

use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum RawDeviceAccessCapability {
    /// The device/control path could be opened with the access mode probed.
    Available,
    /// The path exists but the current privilege level was denied access.
    DeniedInsufficientPrivilege,
    /// The path does not exist, or the probe failed for a reason other
    /// than a permission boundary; treated as not validated rather than
    /// guessed at.
    NotValidated { reason: String },
}

/// Probes whether the current process can open `path` for read+write,
/// without ever reading or writing through it. Used to answer, for a
/// given OS's raw-device control surface (e.g. Linux `/dev/loop-control`),
/// whether the *current* privilege level can reach it — not whether raw
/// acquisition is implemented, which it isn't yet.
pub fn probe_read_write_access(path: &Path) -> RawDeviceAccessCapability {
    match std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
    {
        Ok(_) => RawDeviceAccessCapability::Available,
        Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
            RawDeviceAccessCapability::DeniedInsufficientPrivilege
        }
        Err(error) => RawDeviceAccessCapability::NotValidated {
            reason: error.to_string(),
        },
    }
}

#[cfg(target_os = "linux")]
pub mod linux {
    use super::{RawDeviceAccessCapability, probe_read_write_access};
    use std::path::Path;

    /// Feasibility probe for `/dev/loop-control`, the ioctl surface Linux
    /// uses to bind a backing file to a loop block device. This is the
    /// exact control path exercised manually on Kali (root: `Available`;
    /// unprivileged `kali` user not in group `disk`:
    /// `DeniedInsufficientPrivilege`, matching `losetup`'s own failure).
    pub fn probe_loop_control() -> RawDeviceAccessCapability {
        probe_read_write_access(Path::new("/dev/loop-control"))
    }
}

#[cfg(target_os = "windows")]
pub mod windows {
    use super::RawDeviceAccessCapability;
    use std::os::windows::fs::OpenOptionsExt;
    use std::path::Path;

    const FILE_SHARE_READ: u32 = 0x0000_0001;
    const FILE_SHARE_WRITE: u32 = 0x0000_0002;

    /// Feasibility probe for opening `\\.\PhysicalDrive0` (the first
    /// physical disk) with a read+write-capable handle and explicit share
    /// flags. This mirrors the manual PowerShell probe run on real
    /// hardware (`[System.IO.File]::Open(..., FileShare.ReadWrite)`),
    /// which found: elevated (UAC-elevated) process as an Administrators-
    /// group member -> succeeds; the *same account*, same machine, in a
    /// non-elevated PowerShell -> `Access to the path ... is denied.`.
    /// Never reads or writes through the handle once opened.
    pub fn probe_physical_drive_zero() -> RawDeviceAccessCapability {
        let mut options = std::fs::OpenOptions::new();
        options
            .read(true)
            .write(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE);
        match options.open(Path::new(r"\\.\PhysicalDrive0")) {
            Ok(_) => RawDeviceAccessCapability::Available,
            Err(error) if error.kind() == std::io::ErrorKind::PermissionDenied => {
                RawDeviceAccessCapability::DeniedInsufficientPrivilege
            }
            Err(error) => RawDeviceAccessCapability::NotValidated {
                reason: error.to_string(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonexistent_path_is_not_validated() {
        let result = probe_read_write_access(Path::new(
            "/this/path/does/not/exist/on/any/platform/trareon",
        ));
        assert!(matches!(
            result,
            RawDeviceAccessCapability::NotValidated { .. }
        ));
    }

    #[test]
    fn world_readable_writable_path_is_available() {
        // /dev/null exists with rw access for every user on Unix; on
        // platforms without it this test is skipped rather than guessed at.
        let null_path = Path::new("/dev/null");
        if !null_path.exists() {
            return;
        }
        assert_eq!(
            probe_read_write_access(null_path),
            RawDeviceAccessCapability::Available
        );
    }

    #[cfg(target_os = "linux")]
    #[test]
    fn linux_loop_control_probe_returns_without_panicking() {
        // The actual result depends on whether the CI runner is root and
        // in group `disk` — both are legitimate outcomes and neither is
        // asserted here; only that the probe classifies one of the three
        // known states instead of panicking.
        let result = linux::probe_loop_control();
        assert!(matches!(
            result,
            RawDeviceAccessCapability::Available
                | RawDeviceAccessCapability::DeniedInsufficientPrivilege
                | RawDeviceAccessCapability::NotValidated { .. }
        ));
    }

    #[cfg(target_os = "windows")]
    #[test]
    fn windows_physical_drive_zero_probe_returns_without_panicking() {
        // As on Linux, the actual result depends on the hosted runner's
        // elevation state and isn't asserted to a fixed value — only that
        // the probe classifies one of the three known states.
        let result = windows::probe_physical_drive_zero();
        assert!(matches!(
            result,
            RawDeviceAccessCapability::Available
                | RawDeviceAccessCapability::DeniedInsufficientPrivilege
                | RawDeviceAccessCapability::NotValidated { .. }
        ));
    }
}
