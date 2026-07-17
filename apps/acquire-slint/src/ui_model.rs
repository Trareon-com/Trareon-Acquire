//! Presentation helpers for the Acquire Slint shell (testable without a display).

/// Snapshot shown in the foundation Slint window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiSnapshot {
    pub case_identity: String,
    pub source_path: String,
    pub output_dir: String,
    pub confirmed_synthetic: bool,
    pub status_line: String,
    pub last_package: String,
    pub evidence_sha256: String,
    pub evidence_size: String,
    pub busy: bool,
}

impl Default for UiSnapshot {
    fn default() -> Self {
        Self {
            case_identity: String::new(),
            source_path: String::new(),
            output_dir: String::new(),
            confirmed_synthetic: false,
            status_line: "Ready — Lab Use Only (Engineering Alpha)".into(),
            last_package: "(none)".into(),
            evidence_sha256: "(none)".into(),
            evidence_size: "(none)".into(),
            busy: false,
        }
    }
}

impl UiSnapshot {
    pub fn can_run(&self) -> bool {
        self.confirmed_synthetic
            && !self.source_path.trim().is_empty()
            && !self.output_dir.trim().is_empty()
            && !self.busy
    }

    /// Elevation / allowlist preflight hint (Hari 26). Presentation-only.
    pub fn preflight_hint(&self) -> String {
        let path = self.source_path.trim().to_ascii_lowercase();
        if path.is_empty() {
            return String::new();
        }
        let blockish = path.contains(r"\\.\physicaldrive")
            || path.contains("/dev/rdisk")
            || path.contains("/dev/disk")
            || path.starts_with("/dev/loop")
            || path.starts_with("/dev/nvme")
            || path.starts_with("/dev/sd");
        if !blockish {
            return String::new();
        }
        if path.contains("physicaldrive0")
            || path == "/dev/rdisk0"
            || path == "/dev/disk0"
            || path.starts_with("/dev/nvme0n1")
            || path == "/dev/sda"
            || path.starts_with("/dev/sda")
        {
            return "PREFLIGHT DENY: system-disk-like path — hard-denied.".into();
        }
        "PREFLIGHT: block-device path — requires human-approved allowlist + elevation; UI foundation demo is file-backed only.".into()
    }

    pub fn set_busy(&mut self, busy: bool) {
        self.busy = busy;
    }

    pub fn set_ok(
        &mut self,
        package: impl Into<String>,
        sha: impl Into<String>,
        size: u64,
        detail: impl Into<String>,
    ) {
        self.busy = false;
        self.last_package = package.into();
        self.evidence_sha256 = sha.into();
        self.evidence_size = size.to_string();
        self.status_line = detail.into();
    }

    pub fn set_err(&mut self, detail: impl Into<String>) {
        self.busy = false;
        self.status_line = detail.into();
    }

    pub fn clear_results(&mut self) {
        self.status_line = "Ready — Lab Use Only (Engineering Alpha)".into();
        self.last_package = "(none)".into();
        self.evidence_sha256 = "(none)".into();
        self.evidence_size = "(none)".into();
        self.busy = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_run_requires_confirm_and_paths() {
        let mut snap = UiSnapshot::default();
        assert!(!snap.can_run());
        snap.source_path = "/tmp/a.img".into();
        snap.output_dir = "/tmp/out".into();
        assert!(!snap.can_run());
        snap.confirmed_synthetic = true;
        assert!(snap.can_run());
        snap.set_busy(true);
        assert!(!snap.can_run());
    }

    #[test]
    fn snapshot_ok_and_clear() {
        let mut snap = UiSnapshot::default();
        snap.set_busy(true);
        snap.set_ok("/tmp/demo.fsnap", "abcd", 1024, "Verified Complete");
        assert!(!snap.busy);
        assert_eq!(snap.evidence_size, "1024");
        snap.clear_results();
        assert_eq!(snap.last_package, "(none)");
    }

    #[test]
    fn preflight_warns_on_block_device_and_denies_system_disk() {
        let mut snap = UiSnapshot::default();
        assert!(snap.preflight_hint().is_empty());
        snap.source_path = "/tmp/synth.img".into();
        assert!(snap.preflight_hint().is_empty());
        snap.source_path = "/dev/disk10".into();
        assert!(snap.preflight_hint().contains("allowlist"));
        snap.source_path = r"\\.\PhysicalDrive0".into();
        assert!(snap.preflight_hint().contains("DENY"));
    }
}
