//! Presentation helpers for the Acquire Slint shell (testable without a display).

/// Operator progressive-disclosure mode (Hari 46–48).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UiMode {
    /// Synthetic-only path: prefer Fill demo; hide path browsing.
    Guided,
    /// Standard file paths + browse (foundation demo).
    Standard,
    /// Raw-path warnings and elevation/allowlist preflight always emphasized.
    Expert,
}

impl UiMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Guided => "guided",
            Self::Standard => "standard",
            Self::Expert => "expert",
        }
    }

    pub fn from_index(i: i32) -> Self {
        match i {
            0 => Self::Guided,
            2 => Self::Expert,
            _ => Self::Standard,
        }
    }

    pub fn index(self) -> i32 {
        match self {
            Self::Guided => 0,
            Self::Standard => 1,
            Self::Expert => 2,
        }
    }

    pub fn guidance(self) -> &'static str {
        match self {
            Self::Guided => {
                "Guided: press Fill synthetic demo paths → confirm → Run. Cancel anytime. No raw disks."
            }
            Self::Standard => {
                "Standard: set file source + output (or Browse). Removable USB raw acquire is lab-gated separately."
            }
            Self::Expert => {
                "Expert: raw/block paths need human-approved allowlist + elevation. System disks are hard-denied. Foundation Run remains file-backed."
            }
        }
    }
}

/// Snapshot shown in the foundation Slint window.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UiSnapshot {
    pub mode: UiMode,
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
            mode: UiMode::Guided,
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

    pub fn show_path_editors(&self) -> bool {
        !matches!(self.mode, UiMode::Guided)
    }

    /// Elevation / allowlist preflight hint (Hari 26 / Expert).
    pub fn preflight_hint(&self) -> String {
        let path = self.source_path.trim().to_ascii_lowercase();
        let blockish = !path.is_empty()
            && (path.contains(r"\\.\physicaldrive")
                || path.contains("/dev/rdisk")
                || path.contains("/dev/disk")
                || path.starts_with("/dev/loop")
                || path.starts_with("/dev/nvme")
                || path.starts_with("/dev/sd"));

        if matches!(self.mode, UiMode::Expert) && !blockish {
            return "EXPERT: type a raw path only with an approved allowlist; Run here is still file-backed.".into();
        }
        if path.is_empty() || !blockish {
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

    /// Minimal CoC/report card JSON (operator note + core results). Not forensic CoC.
    pub fn coc_report_json(&self) -> String {
        format!(
            "{{\n  \"schema\": \"trareon.ui-coc-summary/1\",\n  \"mode\": \"{}\",\n  \"case_identity_operator_note\": {},\n  \"status\": {},\n  \"package\": {},\n  \"evidence_sha256\": {},\n  \"evidence_size\": {},\n  \"disclaimer\": \"Operator note is outside the cryptographic audit; Lab Use Only.\"\n}}\n",
            self.mode.as_str(),
            json_str(&self.case_identity),
            json_str(&self.status_line),
            json_str(&self.last_package),
            json_str(&self.evidence_sha256),
            json_str(&self.evidence_size),
        )
    }
}

fn json_str(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
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
        let mut snap = UiSnapshot {
            mode: UiMode::Standard,
            ..UiSnapshot::default()
        };
        assert!(snap.preflight_hint().is_empty());
        snap.source_path = "/tmp/synth.img".into();
        assert!(snap.preflight_hint().is_empty());
        snap.source_path = "/dev/disk10".into();
        assert!(snap.preflight_hint().contains("allowlist"));
        snap.source_path = r"\\.\PhysicalDrive0".into();
        assert!(snap.preflight_hint().contains("DENY"));
    }

    #[test]
    fn guided_hides_path_editors_expert_always_hints() {
        let mut snap = UiSnapshot::default();
        assert_eq!(snap.mode, UiMode::Guided);
        assert!(!snap.show_path_editors());
        snap.mode = UiMode::Standard;
        assert!(snap.show_path_editors());
        snap.mode = UiMode::Expert;
        snap.source_path.clear();
        assert!(snap.preflight_hint().contains("EXPERT"));
    }

    #[test]
    fn coc_report_json_includes_mode_and_disclaimer() {
        let mut snap = UiSnapshot::default();
        snap.mode = UiMode::Standard;
        snap.case_identity = "TRN-1".into();
        snap.set_ok("/tmp/p.fsnap", "deadbeef", 8, "Verified Complete");
        let j = snap.coc_report_json();
        assert!(j.contains("\"mode\": \"standard\""));
        assert!(j.contains("TRN-1"));
        assert!(j.contains("Lab Use Only"));
    }
}
