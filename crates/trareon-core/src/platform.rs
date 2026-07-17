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

#[cfg(target_os = "macos")]
pub mod macos {
    use super::{RawDeviceAccessCapability, probe_read_write_access};
    use std::path::Path;

    /// Feasibility probe for `/dev/rdisk0`, the character-device node macOS
    /// exposes for the first whole-disk raw path. Day 25 observed that an
    /// unprivileged admin user (not in group `operator`) receives
    /// `DeniedInsufficientPrivilege` on both O_RDONLY and O_RDWR opens,
    /// with SIP and Authenticated Root enabled — matching Track C's need
    /// for an explicitly authorized elevation/helper path. This probe
    /// never reads or writes through the handle.
    pub fn probe_rdisk0() -> RawDeviceAccessCapability {
        probe_read_write_access(Path::new("/dev/rdisk0"))
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

    #[cfg(target_os = "macos")]
    #[test]
    fn macos_rdisk0_probe_returns_without_panicking() {
        // Hosted CI and developer Macs differ (operator membership, SIP,
        // TCC). Assert only that the probe returns a known variant.
        let result = macos::probe_rdisk0();
        assert!(matches!(
            result,
            RawDeviceAccessCapability::Available
                | RawDeviceAccessCapability::DeniedInsufficientPrivilege
                | RawDeviceAccessCapability::NotValidated { .. }
        ));
    }
}
