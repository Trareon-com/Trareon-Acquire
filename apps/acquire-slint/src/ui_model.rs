//! Presentation helpers for the Acquire Slint shell (testable without a display).

use crate::identify::{IdentifyRecord, identify_blocks_live};
use crate::prefs::AcquirePrefs;
use std::path::Path;

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
}

/// UI language preference.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UiLocale {
    #[default]
    En,
    Id,
}

impl UiLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::En => "en",
            Self::Id => "id",
        }
    }

    /// Coerce unknown values to English (Lab prefs contract).
    pub fn parse(raw: &str) -> Self {
        match raw.trim().to_ascii_lowercase().as_str() {
            "id" => Self::Id,
            _ => Self::En,
        }
    }
}

/// Sentinel stored in package/hash/size fields when empty.
pub const NONE_SENTINEL: &str = "(none)";

/// Snapshot shown in the foundation Slint window.
#[derive(Debug, Clone, PartialEq)]
pub struct UiSnapshot {
    pub mode: UiMode,
    pub dark_mode: bool,
    pub locale: UiLocale,
    pub case_identity: String,
    pub case_id: String,
    pub source_path: String,
    pub output_dir: String,
    pub confirmed_synthetic: bool,
    pub wb_confirmed: bool,
    pub oov_ack: bool,
    pub allowlist_path: String,
    pub emergency_override_reason: String,
    pub pre_hash_hex: String,
    pub free_space_ok: Option<bool>,
    pub stop_reason: Option<crate::preserve::StopReason>,
    pub identify_record: IdentifyRecord,
    pub status_line: String,
    pub last_package: String,
    pub evidence_sha256: String,
    pub evidence_size: String,
    pub busy: bool,
    pub progress_fraction: f64,
    pub progress_label: String,
    pub progress_phase: String,
}

impl Default for UiSnapshot {
    fn default() -> Self {
        let locale = UiLocale::En;
        Self {
            mode: UiMode::Guided,
            dark_mode: false,
            locale,
            case_identity: String::new(),
            case_id: String::new(),
            source_path: String::new(),
            output_dir: String::new(),
            confirmed_synthetic: false,
            wb_confirmed: false,
            oov_ack: false,
            allowlist_path: String::new(),
            emergency_override_reason: String::new(),
            pre_hash_hex: String::new(),
            free_space_ok: None,
            stop_reason: None,
            identify_record: IdentifyRecord::default(),
            status_line: ready_status(locale).into(),
            last_package: NONE_SENTINEL.into(),
            evidence_sha256: NONE_SENTINEL.into(),
            evidence_size: NONE_SENTINEL.into(),
            busy: false,
            progress_fraction: 0.0,
            progress_label: String::new(),
            progress_phase: String::new(),
        }
    }
}

impl UiSnapshot {
    pub fn from_prefs(prefs: &AcquirePrefs) -> Self {
        let mut snapshot = Self {
            dark_mode: prefs.dark_mode,
            output_dir: prefs.last_output_dir.clone(),
            ..Self::default()
        };
        snapshot.set_locale(&prefs.locale);
        snapshot
    }

    pub fn about_text(&self) -> String {
        format!(
            "Trareon Acquire\nBuild: {}\nLab-use shell — not validated for production evidence.",
            trareon_core::build_identity()
        )
    }

    pub fn set_dark_mode(&mut self, dark: bool) {
        self.dark_mode = dark;
    }

    pub fn set_locale(&mut self, locale: impl AsRef<str>) {
        self.locale = UiLocale::parse(locale.as_ref());
        // Refresh idle status if still on the default ready line.
        if !self.busy && self.last_package == NONE_SENTINEL {
            self.status_line = ready_status(self.locale).into();
        }
    }

    pub fn can_start_guided(&self) -> bool {
        self.confirmed_synthetic
            && !self.source_path.trim().is_empty()
            && !self.output_dir.trim().is_empty()
            && !self.busy
    }

    pub fn can_start_expert(&self) -> bool {
        self.can_start_guided()
            && !self.case_id.trim().is_empty()
            && (!self.is_blockish_source() || self.wb_confirmed)
            && self.free_space_ok != Some(false)
            && (!self.requires_identify_gate()
                || !self.emergency_override_reason.trim().is_empty()
                || (!identify_blocks_live(&self.identify_record) && self.oov_ack))
    }

    /// True when acquisition needs the identification checklist rather than file-demo checks.
    pub fn requires_identify_gate(&self) -> bool {
        matches!(self.mode, UiMode::Expert) || self.is_blockish_source()
    }

    pub fn is_blockish_source(&self) -> bool {
        path_looks_blockish(&self.source_path)
    }

    /// Audit text to append whenever an operator explicitly bypasses the identification gate.
    pub fn override_audit_line(&self) -> Option<String> {
        let reason = self.emergency_override_reason.trim();
        (!reason.is_empty()).then(|| format!("EMERGENCY IDENTIFY OVERRIDE: {reason}"))
    }

    pub fn can_run(&self) -> bool {
        if matches!(self.mode, UiMode::Guided) {
            self.can_start_guided()
        } else {
            self.can_start_expert()
        }
    }

    pub fn save_draft(&self) {
        crate::draft::WizardDraft::from_snapshot(self).save();
    }

    pub fn restore_draft(&mut self) {
        if let Some(draft) = crate::draft::WizardDraft::load() {
            draft.apply_to(self);
        }
    }

    pub fn clear_draft() {
        crate::draft::WizardDraft::clear();
    }

    pub fn show_path_editors(&self) -> bool {
        !matches!(self.mode, UiMode::Guided)
    }

    pub fn guidance(&self) -> &'static str {
        guidance(self.mode, self.locale)
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
            return expert_hint(self.locale).into();
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
            return deny_system_disk(self.locale).into();
        }
        let wb = trareon_ata::probe_write_blocker(std::path::Path::new(&self.source_path));
        let wb_line = match wb {
            trareon_ata::WriteBlockerState::Detected { vendor, .. } => {
                format!("write-blocker: detected ({vendor})")
            }
            trareon_ata::WriteBlockerState::NotApplicable => String::new(),
            trareon_ata::WriteBlockerState::ManualConfirmed { note } => {
                format!("write-blocker: manual ({note})")
            }
            trareon_ata::WriteBlockerState::NotDetected => {
                "write-blocker: NOT detected — confirm hardware blocker before acquire".into()
            }
            trareon_ata::WriteBlockerState::Uncertain { reason } => {
                format!("write-blocker: uncertain ({reason})")
            }
        };
        let hpa = match trareon_ata::detect_hpa_dco(std::path::Path::new(&self.source_path)) {
            Ok(r) => match r.status {
                trareon_ata::DetectionStatus::Ok if r.is_hidden_area() => {
                    "HPA/DCO: hidden area indicated".into()
                }
                trareon_ata::DetectionStatus::Ok => "HPA/DCO: no hidden area in probe".into(),
                trareon_ata::DetectionStatus::Unavailable { reason } => {
                    format!("HPA/DCO: {reason}")
                }
            },
            Err(e) => format!("HPA/DCO: {e}"),
        };
        let base = allowlist_hint(self.locale);
        format!("{base} | {wb_line} | {hpa}")
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

    /// Persist an operator pause reason adjacent to the resumable checkpoint.
    pub fn set_paused(&mut self, checkpoint: &Path) -> Result<(), String> {
        crate::preserve::write_stop_reason(checkpoint, crate::preserve::StopReason::Paused)?;
        self.busy = false;
        self.stop_reason = Some(crate::preserve::StopReason::Paused);
        self.status_line = "Paused".into();
        Ok(())
    }

    /// Persist an operator cancellation reason adjacent to the resumable checkpoint.
    pub fn set_cancelled(&mut self, checkpoint: &Path) -> Result<(), String> {
        crate::preserve::write_stop_reason(checkpoint, crate::preserve::StopReason::Cancelled)?;
        self.busy = false;
        self.stop_reason = Some(crate::preserve::StopReason::Cancelled);
        self.status_line = "Cancelled".into();
        Ok(())
    }

    pub fn clear_results(&mut self) {
        self.status_line = ready_status(self.locale).into();
        self.last_package = NONE_SENTINEL.into();
        self.evidence_sha256 = NONE_SENTINEL.into();
        self.evidence_size = NONE_SENTINEL.into();
        self.busy = false;
        self.progress_fraction = 0.0;
        self.progress_label.clear();
        self.progress_phase.clear();
    }

    pub fn msg_need_confirm_paths(&self) -> &'static str {
        match self.locale {
            UiLocale::Id => "Konfirmasi sumber sintetis dan isi jalur sumber + keluaran.",
            UiLocale::En => "Confirm synthetic source and set source + output paths.",
        }
    }

    pub fn msg_running(&self) -> &'static str {
        match self.locale {
            UiLocale::Id => "Menjalankan demo fondasi…",
            UiLocale::En => "Running foundation demo…",
        }
    }

    pub fn msg_cancelling(&self) -> &'static str {
        match self.locale {
            UiLocale::Id => "Membatalkan…",
            UiLocale::En => "Cancelling…",
        }
    }

    pub fn msg_verified(&self) -> &'static str {
        match self.locale {
            UiLocale::Id => "Verifikasi Selesai",
            UiLocale::En => "Verified Complete",
        }
    }

    pub fn msg_cancelled(&self, err: &str) -> String {
        match self.locale {
            UiLocale::Id => format!("Dibatalkan: {err}"),
            UiLocale::En => format!("Cancelled: {err}"),
        }
    }

    pub fn msg_failed(&self, err: &str) -> String {
        match self.locale {
            UiLocale::Id => format!("Gagal: {err}"),
            UiLocale::En => format!("Failed: {err}"),
        }
    }

    pub fn msg_no_package(&self) -> &'static str {
        match self.locale {
            UiLocale::Id => "Belum ada paket — jalankan demo yang berhasil dulu.",
            UiLocale::En => "No package to export — Run a successful demo first.",
        }
    }

    pub fn msg_coc_written(&self, path: &str) -> String {
        match self.locale {
            UiLocale::Id => format!("Ringkasan CoC ditulis: {path}"),
            UiLocale::En => format!("CoC summary written: {path}"),
        }
    }

    pub fn msg_coc_failed(&self, err: &str) -> String {
        match self.locale {
            UiLocale::Id => format!("Ekspor CoC gagal: {err}"),
            UiLocale::En => format!("CoC export failed: {err}"),
        }
    }

    pub fn dialog_source_title(&self) -> &'static str {
        match self.locale {
            UiLocale::Id => "Pilih berkas sumber sintetis",
            UiLocale::En => "Select synthetic source file",
        }
    }

    pub fn dialog_output_title(&self) -> &'static str {
        match self.locale {
            UiLocale::Id => "Pilih direktori keluaran",
            UiLocale::En => "Select output directory",
        }
    }

    /// Minimal CoC/report card JSON (operator note + core results). Not forensic CoC.
    pub fn coc_report_json(&self) -> String {
        format!(
            "{{\n  \"schema\": \"trareon.ui-coc-summary/1\",\n  \"mode\": \"{}\",\n  \"locale\": \"{}\",\n  \"case_identity_operator_note\": {},\n  \"status\": {},\n  \"package\": {},\n  \"evidence_sha256\": {},\n  \"evidence_size\": {},\n  \"disclaimer\": \"Operator note is outside the cryptographic audit; Lab Use Only.\"\n}}\n",
            self.mode.as_str(),
            self.locale.as_str(),
            json_str(&self.case_identity),
            json_str(&self.status_line),
            json_str(&self.last_package),
            json_str(&self.evidence_sha256),
            json_str(&self.evidence_size),
        )
    }
}

fn ready_status(locale: UiLocale) -> &'static str {
    match locale {
        UiLocale::Id => "Siap — Hanya Lab",
        UiLocale::En => "Ready — Lab Use Only",
    }
}

fn guidance(mode: UiMode, locale: UiLocale) -> &'static str {
    match (mode, locale) {
        (UiMode::Guided, UiLocale::En) => {
            "Confirm the checkbox, then Start acquire. Cancel anytime. No raw disks."
        }
        (UiMode::Guided, UiLocale::Id) => {
            "Centang konfirmasi, lalu Mulai akuisisi. Batal kapan saja. Tanpa disk raw."
        }
        (UiMode::Standard, UiLocale::En) => {
            "Browse source + output, confirm training use, then Start acquire."
        }
        (UiMode::Standard, UiLocale::Id) => {
            "Jelajah sumber + keluaran, konfirmasi pelatihan, lalu Mulai akuisisi."
        }
        (UiMode::Expert, UiLocale::En) => {
            "Raw paths need an approved allowlist + elevation. System disks are denied. This demo stays file-backed."
        }
        (UiMode::Expert, UiLocale::Id) => {
            "Jalur raw butuh allowlist + elevasi yang disetujui. Disk sistem ditolak. Demo ini tetap berbasis berkas."
        }
    }
}

fn expert_hint(locale: UiLocale) -> &'static str {
    match locale {
        UiLocale::Id => {
            "AHLI: ketik jalur raw hanya dengan allowlist yang disetujui; Akuisisi di sini tetap berbasis berkas."
        }
        UiLocale::En => {
            "EXPERT: type a raw path only with an approved allowlist; Run here is still file-backed."
        }
    }
}

fn deny_system_disk(locale: UiLocale) -> &'static str {
    match locale {
        UiLocale::Id => "PREFLIGHT TOLAK: jalur mirip disk sistem — ditolak keras.",
        UiLocale::En => "PREFLIGHT DENY: system-disk-like path — hard-denied.",
    }
}

fn allowlist_hint(locale: UiLocale) -> &'static str {
    match locale {
        UiLocale::Id => {
            "PREFLIGHT: jalur perangkat blok — butuh allowlist + elevasi yang disetujui; demo fondasi UI hanya berbasis berkas."
        }
        UiLocale::En => {
            "PREFLIGHT: block-device path — requires human-approved allowlist + elevation; UI foundation demo is file-backed only."
        }
    }
}

fn json_str(s: &str) -> String {
    let escaped = s.replace('\\', "\\\\").replace('"', "\\\"");
    format!("\"{escaped}\"")
}

fn path_looks_blockish(path: &str) -> bool {
    let path = path.trim().to_ascii_lowercase();
    !path.is_empty()
        && (path.contains(r"\\.\physicaldrive")
            || path.contains("/dev/rdisk")
            || path.contains("/dev/disk")
            || path.starts_with("/dev/loop")
            || path.starts_with("/dev/nvme")
            || path.starts_with("/dev/sd"))
}

/// Whether source and destination resolve to the same filesystem volume.
///
/// A same-volume destination can exhaust the source volume during acquisition and is rejected
/// before a live acquisition starts.
pub fn dest_equals_source_volume(source: &Path, destination: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        let source = source.metadata();
        let destination = destination.metadata();
        matches!((source, destination), (Ok(source), Ok(destination)) if source.dev() == destination.dev())
    }
    #[cfg(windows)]
    {
        let source = source.to_string_lossy();
        let destination = destination.to_string_lossy();
        source
            .get(..2)
            .zip(destination.get(..2))
            .is_some_and(|(left, right)| left.eq_ignore_ascii_case(right))
    }
    #[cfg(not(any(unix, windows)))]
    {
        false
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
    fn standard_and_expert_reject_orphan_runs() {
        let mut snap = UiSnapshot {
            mode: UiMode::Standard,
            source_path: "/tmp/a.img".into(),
            output_dir: "/tmp/out".into(),
            confirmed_synthetic: true,
            ..UiSnapshot::default()
        };
        assert!(!snap.can_run());
        assert!(!snap.can_start_expert());
        snap.case_id = "CASE-1".into();
        assert!(snap.can_run());
        snap.mode = UiMode::Expert;
        assert!(!snap.can_start_expert());
        snap.oov_ack = true;
        snap.identify_record.oov_ack = true;
        snap.identify_record.checklist_complete = crate::identify::IdentifyChecklist {
            power: true,
            network: true,
            encryption: true,
            anti_forensics: true,
        };
        assert!(snap.can_start_expert());
    }

    #[test]
    fn snapshot_ok_and_clear() {
        let mut snap = UiSnapshot::default();
        snap.set_busy(true);
        snap.set_ok("/tmp/demo.fsnap", "abcd", 1024, "Verified Complete");
        assert!(!snap.busy);
        assert_eq!(snap.evidence_size, "1024");
        snap.clear_results();
        assert_eq!(snap.last_package, NONE_SENTINEL);
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
        let mut snap = UiSnapshot {
            mode: UiMode::Standard,
            case_identity: "TRN-1".into(),
            ..UiSnapshot::default()
        };
        snap.set_ok("/tmp/p.fsnap", "deadbeef", 8, "Verified Complete");
        let j = snap.coc_report_json();
        assert!(j.contains("\"mode\": \"standard\""));
        assert!(j.contains("TRN-1"));
        assert!(j.contains("Lab Use Only"));
        assert!(j.contains("\"locale\": \"en\""));
    }

    #[test]
    fn prefs_default_light_en() {
        let snap = UiSnapshot::default();
        assert!(!snap.dark_mode);
        assert_eq!(snap.locale, UiLocale::En);
    }

    #[test]
    fn snapshot_uses_persisted_theme_locale_and_output() {
        let prefs = AcquirePrefs {
            dark_mode: true,
            locale: "id".into(),
            last_output_dir: "/tmp/acquire".into(),
            ..AcquirePrefs::default()
        };
        let snap = UiSnapshot::from_prefs(&prefs);
        assert!(snap.dark_mode);
        assert_eq!(snap.locale, UiLocale::Id);
        assert_eq!(snap.output_dir, "/tmp/acquire");
    }

    #[test]
    fn set_dark_mode_round_trips() {
        let mut snap = UiSnapshot::default();
        snap.set_dark_mode(false);
        assert!(!snap.dark_mode);
        snap.set_dark_mode(true);
        assert!(snap.dark_mode);
    }

    #[test]
    fn set_locale_accepts_id_and_en_and_coerces() {
        let mut snap = UiSnapshot::default();
        snap.set_locale("id");
        assert_eq!(snap.locale, UiLocale::Id);
        assert!(snap.status_line.contains("Siap"));
        snap.set_locale("en");
        assert_eq!(snap.locale, UiLocale::En);
        snap.set_locale("fr");
        assert_eq!(snap.locale, UiLocale::En);
        snap.set_locale("");
        assert_eq!(snap.locale, UiLocale::En);
    }

    #[test]
    fn indonesian_guidance_and_preflight() {
        let mut snap = UiSnapshot::default();
        snap.set_locale("id");
        assert!(snap.guidance().contains("konfirmasi") || snap.guidance().contains("Centang"));
        snap.mode = UiMode::Expert;
        assert!(snap.preflight_hint().contains("AHLI"));
        snap.source_path = "/dev/disk10".into();
        assert!(snap.preflight_hint().contains("allowlist"));
        snap.source_path = "/dev/disk0".into();
        assert!(snap.preflight_hint().contains("TOLAK"));
    }

    #[test]
    fn expert_gate_requires_write_blocker_capacity_and_identify() {
        let mut snap = UiSnapshot {
            mode: UiMode::Expert,
            case_id: "CASE-1".into(),
            source_path: "/dev/disk10".into(),
            output_dir: "/tmp/out".into(),
            confirmed_synthetic: true,
            ..UiSnapshot::default()
        };
        assert!(!snap.can_start_expert());
        snap.wb_confirmed = true;
        snap.free_space_ok = Some(false);
        assert!(!snap.can_start_expert());
        snap.free_space_ok = Some(true);
        snap.emergency_override_reason = "supervisor approved emergency collection".into();
        assert!(snap.can_start_expert());
        assert!(snap.override_audit_line().unwrap().contains("EMERGENCY"));
    }

    #[test]
    fn pause_and_cancel_write_semantic_stop_reasons() {
        let temp = tempfile::tempdir().unwrap();
        let checkpoint = temp.path().join("acquire.checkpoint.json");
        let mut snap = UiSnapshot::default();
        snap.set_busy(true);
        snap.set_paused(&checkpoint).unwrap();
        assert_eq!(snap.stop_reason, Some(crate::preserve::StopReason::Paused));
        snap.set_cancelled(&checkpoint).unwrap();
        assert_eq!(
            snap.stop_reason,
            Some(crate::preserve::StopReason::Cancelled)
        );
        assert_eq!(
            std::fs::read_to_string(temp.path().join(".acquire-stop-reason")).unwrap(),
            "cancelled\n"
        );
    }

    #[test]
    fn same_volume_destination_is_rejected() {
        let temp = tempfile::tempdir().unwrap();
        let source = temp.path().join("source.bin");
        std::fs::write(&source, b"source").unwrap();
        assert!(dest_equals_source_volume(&source, temp.path()));
    }
}
