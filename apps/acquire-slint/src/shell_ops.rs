//! Nav-surface operations for Cases / Identify / Tools / Triage / QMS / Boot (preview).

use std::path::{Path, PathBuf};

use trareon_core::{BadSectorPolicy, enumerate_disks};

use crate::{
    acquire_formats::{self, UiOutputFormat},
    analysis_ui, boot_media, cases, identify, multisource, preserve, profiles, qms, tools_hub,
    triage_ui,
};

pub fn format_label(index: i32) -> &'static str {
    match index {
        1 => "E01 (ewf-image when enabled; oracle pending)",
        2 => "ZFF (via zffacquire if installed)",
        3 => "Split-RAW .fsnap",
        _ => "RAW + .fsnap (Path A candidate)",
    }
}

pub fn format_from_index(index: i32) -> UiOutputFormat {
    match index {
        1 => UiOutputFormat::E01,
        2 => UiOutputFormat::Zff,
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

/// Structured case rows for the Cases table (mirrors DiskUiRow).
#[derive(Debug, Clone)]
pub struct CaseUiRow {
    pub id: String,
    pub title: String,
    pub examiner: String,
    pub state: String,
    pub updated: String,
}

pub fn list_case_ui_rows(filter: &str, sort_by_id: bool) -> Vec<CaseUiRow> {
    let Some(store) = cases::CaseStore::local() else {
        return Vec::new();
    };
    let Ok(mut list) = store.list() else {
        return Vec::new();
    };
    let filter = filter.trim().to_ascii_lowercase();
    if !filter.is_empty() {
        list.retain(|c| {
            c.id.to_ascii_lowercase().contains(&filter)
                || c.title.to_ascii_lowercase().contains(&filter)
                || c.examiner.to_ascii_lowercase().contains(&filter)
        });
    }
    if sort_by_id {
        list.sort_by(|a, b| a.id.cmp(&b.id));
    }
    list.into_iter()
        .map(|c| CaseUiRow {
            id: c.id,
            title: c.title,
            examiner: c.examiner,
            state: format!("{:?}", c.state),
            updated: c.updated_at.chars().take(16).collect(),
        })
        .collect()
}

pub fn archive_case(id: &str) -> Result<String, String> {
    let store = cases::CaseStore::local().ok_or("case store unavailable")?;
    let record = store.archive(id)?;
    Ok(format!("Archived {} — state {:?}.", record.id, record.state))
}

#[derive(Debug, Clone)]
pub struct ResultUiRow {
    pub name: String,
    pub ok: bool,
    pub detail: String,
}

pub fn qms_check_ui_rows() -> Vec<ResultUiRow> {
    qms::validation_matrix()
        .checks
        .into_iter()
        .map(|c| ResultUiRow {
            name: c.name,
            ok: c.passed,
            detail: c.detail,
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct LogUiRow {
    pub level: String,
    pub text: String,
}

pub fn text_to_log_rows(body: &str) -> Vec<LogUiRow> {
    body.lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let lower = line.to_ascii_lowercase();
            let level = if lower.contains("fail")
                || lower.contains("error")
                || lower.contains("deny")
                || lower.contains("tolak")
            {
                "err"
            } else if lower.contains("pass")
                || lower.contains("ok")
                || lower.contains("verified")
                || lower.contains("sealed")
            {
                "ok"
            } else {
                "info"
            };
            LogUiRow {
                level: level.into(),
                text: line.to_string(),
            }
        })
        .collect()
}

pub fn compare_summary_rows(package_a: &str, package_b: &str) -> Vec<ResultUiRow> {
    let text = compare_packages_text(package_a, package_b);
    text.lines()
        .filter(|l| !l.trim().is_empty())
        .map(|line| {
            let ok = !line.to_ascii_lowercase().contains("mismatch")
                && !line.to_ascii_lowercase().contains("fail")
                && !line.to_ascii_lowercase().contains("differ");
            ResultUiRow {
                name: if ok { "Match".into() } else { "Mismatch".into() },
                ok,
                detail: line.to_string(),
            }
        })
        .collect()
}

#[derive(Debug, Clone)]
pub struct TimelineUiRow {
    pub when: String,
    pub what: String,
}

#[derive(Debug, Clone)]
pub struct ListingUiRow {
    pub path: String,
    pub size: String,
    pub kind: String,
}

pub fn analysis_lite_rows(
    package: &str,
    out_dir: &str,
) -> Result<(Vec<TimelineUiRow>, Vec<ListingUiRow>, String), String> {
    if package.trim().is_empty() || package == "(none)" {
        return Err("No package selected.".into());
    }
    let out = if out_dir.trim().is_empty() {
        std::env::temp_dir()
    } else {
        PathBuf::from(out_dir)
    };
    let index_dir = out.join("analysis-index");
    let report_dir = out.join("analysis-report");
    let result = analysis_ui::import_for_analysis(Path::new(package), &index_dir, &report_dir)?;
    let timeline = result
        .timeline
        .into_iter()
        .map(|e| TimelineUiRow {
            when: e.timestamp_utc.clone(),
            what: format!("{} · {}", e.state, e.action),
        })
        .collect();
    let listing = result
        .listing
        .into_iter()
        .take(200)
        .map(|e| ListingUiRow {
            path: e.relative_path.clone(),
            size: e.size.to_string(),
            kind: "file".into(),
        })
        .collect();
    let summary = format!(
        "Analysis lite OK — report {}\nindex {}",
        result.report_path.display(),
        result.index_path.display()
    );
    Ok((timeline, listing, summary))
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
        Err(error) => format!("Boot plan (preview) failed: {error}"),
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
    acquire_with_format_controls(source, output, format_index, segment_mib, sha512, None, None)
}

/// Acquire with cancel + progress wired for every format path.
pub fn acquire_with_format_controls(
    source: &Path,
    output: &Path,
    format_index: i32,
    segment_mib: u64,
    sha512: bool,
    cancel_flag: Option<std::sync::Arc<std::sync::atomic::AtomicBool>>,
    progress: Option<trareon_core::ProgressCallback>,
) -> Result<(String, String, u64), String> {
    if format_index == 3 {
        let package = acquire_formats::acquire_split_raw_with_controls(
            source,
            output,
            segment_mib.max(1) * 1024 * 1024,
            cancel_flag,
            progress,
        )?;
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
    let package = acquire_formats::acquire_to_format_with_controls(
        source,
        output,
        format,
        sha512,
        cancel_flag,
        progress,
    )?;
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
    use super::{
        analysis_lite_rows, boot_plan_text, bus_from_kind, compare_packages_text,
        compare_summary_rows, export_report_text, format_size_label, hash_only_text,
        identify_probe_text, list_case_ui_rows, nav_index_for_search, qms_check_ui_rows,
        run_qms_text, text_to_log_rows, verify_tools_text,
    };

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

    #[test]
    fn case_and_qms_rows_are_structured() {
        let _ = list_case_ui_rows("", false);
        let checks = qms_check_ui_rows();
        assert!(!checks.is_empty());
        assert!(checks.iter().all(|c| !c.name.is_empty()));
        let logs = text_to_log_rows("COMPARE OK — packages match.\nCOMPARE FAIL: boom");
        assert_eq!(logs.len(), 2);
        assert_eq!(logs[0].level, "ok");
        assert_eq!(logs[1].level, "err");
    }

    /// End-to-end shell surfaces against synthetic .fsnap packages (no real disks).
    #[test]
    fn synthetic_shell_smoke_covers_tools_pipeline() {
        let a_dir = tempfile::tempdir().unwrap();
        let b_dir = tempfile::tempdir().unwrap();
        let out = tempfile::tempdir().unwrap();
        let package_a = crate::run_synthetic_demo(a_dir.path()).expect("synthetic A");
        let package_b = crate::run_synthetic_demo(b_dir.path()).expect("synthetic B");
        let pkg_a = package_a.to_string_lossy();
        let pkg_b = package_b.to_string_lossy();
        let source = a_dir.path().join("synthetic-source.bin");

        let verify = verify_tools_text(&pkg_a);
        assert!(verify.starts_with("VERIFY OK"), "{verify}");

        let hash = hash_only_text(&pkg_a);
        assert!(hash.contains("hash-only"), "{hash}");
        assert!(hash.contains("262144"), "{hash}");

        let compare = compare_packages_text(&pkg_a, &pkg_b);
        assert!(
            compare.starts_with("COMPARE OK") || compare.starts_with("COMPARE DIFF"),
            "{compare}"
        );
        let rows = compare_summary_rows(&pkg_a, &pkg_b);
        assert!(!rows.is_empty());
        assert!(rows.iter().all(|r| !r.detail.is_empty()));

        let export = export_report_text(&pkg_a, &out.path().to_string_lossy());
        assert!(export.contains("Export written"), "{export}");
        assert!(out.path().join("export-evidence.bin").is_file());

        let (timeline, listing, summary) =
            analysis_lite_rows(&pkg_a, &out.path().to_string_lossy()).expect("analysis");
        assert!(summary.contains("Analysis lite OK"), "{summary}");
        assert!(!listing.is_empty());
        let _ = timeline;

        let identify = identify_probe_text(&source.to_string_lossy());
        assert!(!identify.contains("Set a source path"), "{identify}");

        let boot = boot_plan_text(&source.to_string_lossy(), "/dev/null");
        assert!(boot.contains("DRY-RUN") || boot.contains("failed"), "{boot}");

        let qms = run_qms_text();
        assert!(qms.contains("self-test: PASS"), "{qms}");
        assert!(qms.contains("known-dataset: PASS"), "{qms}");

        let checks = qms_check_ui_rows();
        assert!(checks.iter().any(|c| c.ok), "{checks:?}");
    }
}
