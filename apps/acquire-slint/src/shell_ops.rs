//! Nav-surface operations for Cases / Identify / Tools / Triage / QMS / Boot.

use std::path::{Path, PathBuf};

use trareon_core::{BadSectorPolicy, enumerate_disks};

use crate::{
    acquire_formats::{self, UiOutputFormat},
    analysis_ui, boot_media, cases, identify, multisource, preserve, profiles, qms, tools_hub,
    triage_ui,
};

pub fn format_label(index: i32) -> &'static str {
    match index {
        1 => "E01-lite label (ewf-image writer; oracle pending)",
        2 => "AFF4-lite (subset)",
        3 => "VMDK wrapper (subset)",
        4 => "VHD wrapper (subset)",
        5 => "QCOW2 wrapper (subset)",
        6 => "DMG wrapper (subset)",
        7 => "Split-RAW .fsnap",
        8 => "ZFF (via zffacquire if installed)",
        _ => "RAW + .fsnap (court path A)",
    }
}

pub fn format_from_index(index: i32) -> UiOutputFormat {
    match index {
        1 => UiOutputFormat::E01,
        2 => UiOutputFormat::Aff4,
        3 => UiOutputFormat::Vmdk,
        4 => UiOutputFormat::Vhd,
        5 => UiOutputFormat::Qcow2,
        6 => UiOutputFormat::Dmg,
        8 => UiOutputFormat::Zff,
        _ => UiOutputFormat::RawFsnap,
    }
}

pub fn list_disks_text() -> String {
    match enumerate_disks() {
        Ok(rows) if rows.is_empty() => "No disks reported by the host enumerator.".into(),
        Ok(rows) => rows
            .into_iter()
            .map(|row| {
                format!(
                    "{} · {} · {} B · {}",
                    row.path, row.name, row.size_bytes, row.kind
                )
            })
            .collect::<Vec<_>>()
            .join("\n"),
        Err(error) => format!("Disk enum unavailable: {error}"),
    }
}

/// Structured disk rows for the Evidence Control Room table (Fase 1).
#[derive(Debug, Clone)]
pub struct DiskUiRow {
    pub path: String,
    pub device: String,
    pub model: String,
    pub serial: String,
    pub size: String,
    pub bus: String,
}

/// Exact HOST DISK DEVICES rows from the Evidence Control Room mock (Fase 1 visual SoT).
pub fn mock_bench_disk_rows() -> Vec<DiskUiRow> {
    vec![
        DiskUiRow {
            path: r"\\.\PhysicalDrive0".into(),
            device: "Disk 0".into(),
            model: "Samsung SSD 870 EVO 1TB".into(),
            serial: "S6PYNF0T123456Z".into(),
            size: "931.51 GB".into(),
            bus: "SATA".into(),
        },
        DiskUiRow {
            path: r"\\.\PhysicalDrive1".into(),
            device: "Disk 1".into(),
            model: "WDC WD20EFRX-68EUZN0".into(),
            serial: "WD-WCC4M2LXC123".into(),
            size: "1.82 TB".into(),
            bus: "SATA".into(),
        },
        DiskUiRow {
            path: r"\\.\PhysicalDrive2".into(),
            device: "Disk 2".into(),
            model: "Seagate Expansion Desk 4TB".into(),
            serial: "NA7QJ2S8".into(),
            size: "3.64 TB".into(),
            bus: "USB".into(),
        },
        DiskUiRow {
            path: r"\\.\PhysicalDrive3".into(),
            device: "Disk 3".into(),
            model: "Kingston DataTraveler 3.0".into(),
            serial: "001CC0EC0E1F12345678".into(),
            size: "28.87 GB".into(),
            bus: "USB".into(),
        },
        DiskUiRow {
            path: r"\\.\PhysicalDrive4".into(),
            device: "Disk 4".into(),
            model: "SanDisk Ultra USB 3.0".into(),
            serial: "4C530001060924105116".into(),
            size: "57.81 GB".into(),
            bus: "USB".into(),
        },
    ]
}

pub fn list_disk_ui_rows() -> Vec<DiskUiRow> {
    match enumerate_disks() {
        Ok(rows) if !rows.is_empty() => rows
            .into_iter()
            .enumerate()
            .map(|(i, row)| {
                let size = format_size_label(row.size_bytes);
                let bus = bus_from_kind(&row.kind);
                DiskUiRow {
                    path: row.path.clone(),
                    device: format!("Disk {i}"),
                    model: if row.name.trim().is_empty() {
                        row.path.clone()
                    } else {
                        row.name
                    },
                    serial: "—".into(),
                    size,
                    bus,
                }
            })
            .collect(),
        // Honest empty: do not inject Tableau/mock fixtures into the live shell.
        _ => Vec::new(),
    }
}

/// Demo disk table for Guided "Fill demo" visual preview only (not host truth).
pub fn demo_disk_ui_rows() -> Vec<DiskUiRow> {
    mock_bench_disk_rows()
}

fn format_size_label(bytes: u64) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    const GIB: f64 = MIB * 1024.0;
    const TIB: f64 = GIB * 1024.0;
    let b = bytes as f64;
    if b >= TIB {
        format!("{:.1} TB", b / TIB)
    } else if b >= GIB {
        format!("{:.0} GB", b / GIB)
    } else if b >= MIB {
        format!("{:.0} MB", b / MIB)
    } else if bytes == 0 {
        "—".into()
    } else {
        format!("{bytes} B")
    }
}

fn bus_from_kind(kind: &str) -> String {
    let k = kind.to_ascii_lowercase();
    if k.contains("usb") {
        "USB".into()
    } else if k.contains("nvme") {
        "NVMe".into()
    } else if k.contains("sata") || k.contains("disk") || k.contains("ata") {
        "SATA".into()
    } else if kind.trim().is_empty() {
        "—".into()
    } else {
        kind.to_string()
    }
}

pub fn refresh_cases_text() -> String {
    let Some(store) = cases::CaseStore::local() else {
        return "Case store unavailable (no local data directory).".into();
    };
    match store.list() {
        Ok(list) if list.is_empty() => {
            "No cases yet. Create one with title + examiner to start ISO Identify.".into()
        }
        Ok(list) => list
            .into_iter()
            .map(|c| format!("{} · {} · {:?} · {}", c.id, c.title, c.state, c.examiner))
            .collect::<Vec<_>>()
            .join("\n"),
        Err(error) => format!("Case list failed: {error}"),
    }
}

pub fn create_case(title: &str, examiner: &str, notes: &str) -> Result<String, String> {
    let store = cases::CaseStore::local().ok_or("case store unavailable")?;
    let record = store.create(title, examiner, notes)?;
    Ok(format!(
        "Created {} — state {:?}. Open Identify next.",
        record.id, record.state
    ))
}

pub fn identify_probe_text(source: &str) -> String {
    if source.trim().is_empty() {
        return "Set a source path on Acquire, then probe encryption here.".into();
    }
    let status = identify::probe_encryption(Path::new(source));
    format!("{:?} — {}", status.state, status.note)
}

pub fn save_identify_for_case(
    case_id: &str,
    power: bool,
    network: bool,
    encryption: bool,
    anti: bool,
    oov: bool,
) -> Result<String, String> {
    let store = cases::CaseStore::local().ok_or("case store unavailable")?;
    let Some(record) = store.open(case_id)? else {
        return Err(format!("case not found: {case_id}"));
    };
    let case_path = store.path_for(&record.id)?;
    let record_id = identify::IdentifyRecord {
        checklist_complete: identify::IdentifyChecklist {
            power,
            network,
            encryption,
            anti_forensics: anti,
        },
        oov_ack: oov,
        power_state: if power {
            "documented".into()
        } else {
            String::new()
        },
        network_isolated: network,
        ..Default::default()
    };
    identify::save_beside_case(&case_path, &record_id)?;
    let blocked = identify::identify_blocks_live(&record_id);
    Ok(if blocked {
        format!(
            "Identify saved beside {}. Live acquire still blocked until checklist + OoV complete.",
            record.id
        )
    } else {
        format!("Identify complete for {}. Acquire may proceed.", record.id)
    })
}

pub fn verify_tools_text(package: &str) -> String {
    if package.trim().is_empty() || package == "(none)" {
        return "No package yet. Run Acquire or browse a .fsnap.".into();
    }
    let path = Path::new(package);
    match tools_hub::verify_package(path) {
        Ok(result) => format!(
            "VERIFY OK · sha256={} · size={} · sig={:?}",
            result.manifest.evidence_sha256, result.manifest.evidence_size, result.signature
        ),
        Err(error) => format!("VERIFY FAIL: {error}"),
    }
}

pub fn hash_only_text(package: &str) -> String {
    if package.trim().is_empty() || package == "(none)" {
        return "No package selected.".into();
    }
    match tools_hub::hash_only(Path::new(package)) {
        Ok((hash, size)) => format!("hash-only · {hash} · {size} B"),
        Err(error) => format!("hash-only failed: {error}"),
    }
}

pub fn compare_packages_text(a: &str, b: &str) -> String {
    if a.trim().is_empty() || a == "(none)" || b.trim().is_empty() {
        return "Compare needs two package paths (set last package + browse a second).".into();
    }
    match tools_hub::compare(Path::new(a), Path::new(b)) {
        Ok(true) => "COMPARE OK — packages match.".into(),
        Ok(false) => "COMPARE DIFF — packages differ.".into(),
        Err(error) => format!("COMPARE FAIL: {error}"),
    }
}

pub fn export_report_text(package: &str, out_dir: &str) -> String {
    if package.trim().is_empty() || package == "(none)" {
        return "Seal or browse a package first.".into();
    }
    let dest = if out_dir.trim().is_empty() {
        std::env::temp_dir().join("trareon-export.bin")
    } else {
        PathBuf::from(out_dir).join("export-evidence.bin")
    };
    match tools_hub::export(Path::new(package), &dest) {
        Ok(()) => format!("Export written under {}", dest.display()),
        Err(error) => format!("Export failed: {error}"),
    }
}

pub fn profile_caption(index: i32) -> &'static str {
    match index {
        0 => "Quick — selected paths only (logical)",
        2 => "Deep — full source (default acquire path)",
        3 => "Custom — operator paths (use Expert)",
        _ => "Standard — full file/disk acquire",
    }
}

pub fn run_quick_profile_text(source: &str, out_dir: &str) -> String {
    if source.trim().is_empty() || out_dir.trim().is_empty() {
        return "Quick profile needs a source directory + output.".into();
    }
    let root = Path::new(source);
    if !root.is_dir() {
        return "Quick profile is logical/fixture collection — pick a directory source, or use Standard/Deep for full acquire.".into();
    }
    let Ok(entries) = std::fs::read_dir(root) else {
        return "Quick profile: cannot read source directory.".into();
    };
    let names: Vec<String> = entries
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
        .take(5)
        .filter_map(|e| e.file_name().into_string().ok())
        .collect();
    if names.is_empty() {
        return "Quick profile: no top-level files in source directory.".into();
    }
    let refs: Vec<&str> = names.iter().map(String::as_str).collect();
    let out = PathBuf::from(out_dir).join("profile-quick");
    match profiles::collect_quick_profile(root, &out, &refs) {
        Ok(report) => format!(
            "Quick profile → {}\ncopied={} excluded={}",
            out.display(),
            report.copied.len(),
            report.excluded.len()
        ),
        Err(error) => format!("Quick profile: {error}"),
    }
}

pub fn source_kind_caption(index: i32) -> String {
    match index {
        1 => "File — path editor / browse".into(),
        2 => match crate::sources_ext::probe_avml() {
            crate::sources_ext::LiveAdapterStatus::Available { binary } => {
                format!("RAM — avml available ({})", binary.display())
            }
            crate::sources_ext::LiveAdapterStatus::Unavailable { reason } => reason,
        },
        3 => match crate::sources_ext::probe_fuji() {
            crate::sources_ext::LiveAdapterStatus::Available { binary } => {
                format!("Snapshot/macOS — fuji helper ({})", binary.display())
            }
            crate::sources_ext::LiveAdapterStatus::Unavailable { reason } => {
                format!("Snapshot — {reason}")
            }
        },
        _ => "Disk — host enumerator / PhysicalDrive".into(),
    }
}

pub fn retry_policy_caption(index: i32) -> String {
    BadSectorPolicy::from_index(index).custody_note()
}

pub fn export_full_coc(
    package: &str,
    case_id: &str,
    examiner: &str,
    device_id: &str,
    media_num: u16,
    seq_num: u16,
    description: &str,
) -> Result<String, String> {
    if package.trim().is_empty() || package == "(none)" {
        return Err("No package to seal into CoC.".into());
    }
    let path = Path::new(package);
    // Full CoC + QR via preserve for .fsnap packages; otherwise write JSON summary beside output.
    if path.is_dir() || path.extension().and_then(|e| e.to_str()) == Some("fsnap") {
        let coc = preserve::seal_package_with_ids(
            path,
            if case_id.is_empty() {
                "TRAINING"
            } else {
                case_id
            },
            if examiner.is_empty() {
                "operator"
            } else {
                examiner
            },
            if device_id.is_empty() {
                "device"
            } else {
                device_id
            },
            media_num.max(1),
            seq_num.max(1),
        )?;
        let parent = path.parent().unwrap_or_else(|| Path::new("."));
        Ok(format!(
            "CoC {} · QR {}\nmedia={} seq={} · {}\n{}",
            parent.join("coc.json").display(),
            parent.join("qr.png").display(),
            media_num,
            seq_num,
            description,
            coc.evidence_id
        ))
    } else {
        Err("Full CoC+QR requires a .fsnap package (court path A). Use Export CoC summary for lite containers.".into())
    }
}

pub fn multisource_text(source_a: &str, source_b: &str, out_dir: &str) -> String {
    if source_a.trim().is_empty() || source_b.trim().is_empty() || out_dir.trim().is_empty() {
        return "Multisource needs source A, source B path, and output dir.".into();
    }
    let out = PathBuf::from(out_dir).join("multisource");
    match multisource::run_parallel_two_sources(Path::new(source_a), Path::new(source_b), &out) {
        Ok(run) => format!(
            "Multisource {:?}\nA: {}\nB: {}\ngovernor: {}",
            run.recommendation,
            run.first_package.display(),
            run.second_package.display(),
            run.governor_note.display()
        ),
        Err(error) => format!("Multisource failed: {error}"),
    }
}

pub fn run_qms_text() -> String {
    let a = qms::self_test();
    let b = qms::known_dataset_suite();
    format!(
        "self-test: {} — {}\nknown-dataset: {} — {}",
        if a.passed { "PASS" } else { "FAIL" },
        a.detail,
        if b.passed { "PASS" } else { "FAIL" },
        b.detail
    )
}

pub fn run_triage_text(out_dir: &str) -> String {
    let out = if out_dir.trim().is_empty() {
        std::env::temp_dir().join("trareon-triage.json")
    } else {
        PathBuf::from(out_dir).join("triage.json")
    };
    match triage_ui::run_live_triage(&out) {
        Ok(run) => format!(
            "Triage bundle: {}\n{}",
            run.path.display(),
            run.footprint_note
        ),
        Err(error) => format!("Triage failed: {error}"),
    }
}

pub fn boot_plan_text(image: &str, dest: &str) -> String {
    if image.trim().is_empty() {
        return "Set boot image path (file) to build a dry-run plan.".into();
    }
    let dest = if dest.trim().is_empty() {
        "/dev/null"
    } else {
        dest
    };
    match boot_media::prepare_boot_write_plan(Path::new(image), Path::new(dest)) {
        Ok(plan) => format!(
            "DRY-RUN plan\nimage: {}\ndest: {}\n{}",
            plan.image.display(),
            plan.destination_device.display(),
            plan.warnings.join("\n")
        ),
        Err(error) => format!("Boot plan failed: {error}"),
    }
}

pub fn analysis_lite_text(package: &str, out_dir: &str) -> String {
    if package.trim().is_empty() || package == "(none)" {
        return "Seal a package first, then import read-only Analysis lite.".into();
    }
    let base = if out_dir.trim().is_empty() {
        std::env::temp_dir().join("trareon-analysis")
    } else {
        PathBuf::from(out_dir).join("analysis")
    };
    let index = base.join("index");
    let report = base.join("report");
    match analysis_ui::import_for_analysis(Path::new(package), &index, &report) {
        Ok(result) => {
            let events: Vec<_> = result
                .timeline
                .iter()
                .take(8)
                .map(|e| format!("· {} — {} / {}", e.timestamp_utc, e.state, e.action))
                .collect();
            format!(
                "Analysis lite (read-only)\nreport: {}\nindex: {}\ntimeline (first {}):\n{}",
                result.report_path.display(),
                result.index_path.display(),
                events.len(),
                events.join("\n")
            )
        }
        Err(error) => format!("Analysis lite failed: {error}"),
    }
}

pub fn acquire_with_format(
    source: &Path,
    output: &Path,
    format_index: i32,
    segment_mib: u64,
    sha512: bool,
) -> Result<(String, String, u64), String> {
    if format_index == 7 {
        let package =
            acquire_formats::acquire_split_raw(source, output, segment_mib.max(1) * 1024 * 1024)?;
        let demo = crate::tools_hub::verify_package(&package).map(|v| {
            (
                package.display().to_string(),
                v.manifest.evidence_sha256,
                v.manifest.evidence_size,
            )
        })?;
        return Ok(demo);
    }
    let format = format_from_index(format_index);
    let package = acquire_formats::acquire_to_format_with_sha512(source, output, format, sha512)?;
    if format == UiOutputFormat::RawFsnap {
        let verified = tools_hub::verify_package(&package)?;
        return Ok((
            package.display().to_string(),
            verified.manifest.evidence_sha256,
            verified.manifest.evidence_size,
        ));
    }
    // Non-fsnap: hash the written container for display.
    let (hash, size) = tools_hub::hash_only(&package)?;
    Ok((package.display().to_string(), hash, size))
}

pub fn throughput_eta(
    bytes_done: u64,
    bytes_total: Option<u64>,
    elapsed_secs: f64,
) -> (String, String) {
    if elapsed_secs <= 0.05 || bytes_done == 0 {
        return ("— MB/s".into(), "ETA —".into());
    }
    let mib_s = (bytes_done as f64 / (1024.0 * 1024.0)) / elapsed_secs;
    let throughput = format!("{mib_s:.2} MiB/s");
    let eta = match bytes_total {
        Some(total) if total > bytes_done && mib_s > 0.0 => {
            let remain = (total - bytes_done) as f64 / (1024.0 * 1024.0);
            let secs = remain / mib_s;
            if secs >= 3600.0 {
                format!("ETA {:.0}h", secs / 3600.0)
            } else if secs >= 60.0 {
                format!("ETA {:.0}m", secs / 60.0)
            } else {
                format!("ETA {:.0}s", secs)
            }
        }
        _ => "ETA —".into(),
    };
    (throughput, eta)
}

pub fn coverage_fractions(
    bytes_done: u64,
    bytes_total: Option<u64>,
    had_error: bool,
) -> (f32, f32) {
    let ok = match bytes_total.filter(|t| *t > 0) {
        Some(total) => (bytes_done as f32 / total as f32).clamp(0.0, 1.0),
        None => 0.0,
    };
    let err = if had_error { 0.08 } else { 0.0 };
    (ok, err)
}

/// Format coverage for the status bar (Evidence · Coverage · mode).
pub fn coverage_status_text(ok: f32, err: f32) -> String {
    if ok <= 0.0 && err <= 0.0 {
        return "—".into();
    }
    if err > 0.0 {
        format!("{:.0}% ok · {:.0}% err", ok * 100.0, err * 100.0)
    } else {
        format!("{:.0}%", ok * 100.0)
    }
}

/// Map header search query → nav-index (0 Acquire … 8 About).
pub fn nav_index_for_search(query: &str) -> Option<i32> {
    let q = query.trim().to_lowercase();
    if q.is_empty() {
        return None;
    }
    let pairs: &[(&[&str], i32)] = &[
        (&["acquire", "akuisisi"], 0),
        (&["case", "cases", "kasus"], 1),
        (&["ident", "identify", "identifikasi"], 2),
        (&["triage"], 3),
        (&["tool", "tools", "alat"], 4),
        (&["qms"], 5),
        (&["boot"], 6),
        (&["help", "bantuan", "sop", "setting"], 7),
        (&["about", "tentang"], 8),
    ];
    for (keys, idx) in pairs {
        if keys.iter().any(|k| q.contains(k)) {
            return Some(*idx);
        }
    }
    None
}

pub fn mode_status_label(mode_index: i32) -> &'static str {
    match mode_index {
        0 => "Guided",
        2 => "Expert",
        _ => "Standard",
    }
}

pub fn evidence_status_count(has_package: bool) -> &'static str {
    if has_package { "1" } else { "0" }
}

pub fn custody_timeline(case_id: &str, package: &str, sha: &str, sealed: bool) -> String {
    let mut lines = vec![
        "1 · Case context recorded (ISO Identify)".to_string(),
        "2 · Source prepared / synthetic confirmed".to_string(),
    ];
    if sealed {
        lines.push(format!(
            "3 · Acquired → package {}",
            if package == "(none)" { "—" } else { package }
        ));
        lines.push(format!(
            "4 · Sealed SHA-256 {} · case {}",
            if sha == "(none)" {
                "—"
            } else {
                &sha[..sha.len().min(16)]
            },
            if case_id.is_empty() {
                "TRAINING"
            } else {
                case_id
            }
        ));
        lines.push("5 · Export CoC / independent verify required".into());
    } else {
        lines.push("3 · Acquire pending".into());
        lines.push("4 · Seal pending".into());
    }
    lines.join("\n")
}

#[cfg(test)]
mod disk_ui_tests {
    use super::{bus_from_kind, format_size_label, nav_index_for_search};

    #[test]
    fn size_and_bus_labels_match_bench_table() {
        assert_eq!(format_size_label(0), "—");
        assert_eq!(format_size_label(1024 * 1024 * 1024), "1 GB");
        assert_eq!(bus_from_kind("disk"), "SATA");
        assert_eq!(bus_from_kind("USB"), "USB");
    }

    #[test]
    fn search_jumps_to_known_nav() {
        assert_eq!(nav_index_for_search("qms"), Some(5));
        assert_eq!(nav_index_for_search("Cases"), Some(1));
        assert_eq!(nav_index_for_search("about"), Some(8));
        assert_eq!(nav_index_for_search("xyz"), None);
    }
}
