#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use acquire_slint::{
        AppWindow, CaseLine, DiskLine, ListingLine, LogLine, RecentLine, ResultLine, TimelineLine,
        UiMode, UiSnapshot, draft::WizardDraft, prefs::AcquirePrefs, preserve, recent, shell_ops,
        write_coc_summary,
    };
    use slint::{ComponentHandle, ModelRc, SharedString, VecModel};
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Instant;
    use trareon_core::{AcquireProgress, ProgressCallback, default_checkpoint_path};

    fn load_disk_rows(ui: &AppWindow) {
        let rows: Vec<DiskLine> = shell_ops::list_disk_ui_rows()
            .into_iter()
            .map(|row| DiskLine {
                path: row.path.into(),
                device: row.device.into(),
                model: row.model.into(),
                serial: row.serial.into(),
                size: row.size.into(),
                bus: row.bus.into(),
            })
            .collect();
        ui.set_disk_rows(ModelRc::new(VecModel::from(rows)));
        ui.set_disk_list(shell_ops::list_disks_text().into());
    }

    fn load_case_rows(ui: &AppWindow) {
        let filter = ui.get_case_filter().as_str().to_string();
        let rows: Vec<CaseLine> = shell_ops::list_case_ui_rows(&filter, ui.get_case_sort_by_id())
            .into_iter()
            .map(|row| CaseLine {
                id: row.id.into(),
                title: row.title.into(),
                examiner: row.examiner.into(),
                state: row.state.into(),
                updated: row.updated.into(),
            })
            .collect();
        ui.set_case_rows(ModelRc::new(VecModel::from(rows)));
    }

    fn set_log_rows(ui: &AppWindow, body: &str) {
        let rows: Vec<LogLine> = shell_ops::text_to_log_rows(body)
            .into_iter()
            .map(|row| LogLine {
                level: row.level.into(),
                text: row.text.into(),
            })
            .collect();
        ui.set_log_rows(ModelRc::new(VecModel::from(rows)));
    }

    fn set_result_rows(ui: &AppWindow, rows: Vec<shell_ops::ResultUiRow>) {
        let rows: Vec<ResultLine> = rows
            .into_iter()
            .map(|row| ResultLine {
                name: row.name.into(),
                ok: row.ok,
                detail: row.detail.into(),
            })
            .collect();
        ui.set_result_rows(ModelRc::new(VecModel::from(rows)));
    }

    fn load_recent_rows(ui: &AppWindow) {
        let rows: Vec<RecentLine> = recent::list()
            .into_iter()
            .map(|row| RecentLine {
                case_id: row.case_id.into(),
                package_path: row.package_path.into(),
                completed_at: row.completed_at.into(),
            })
            .collect();
        ui.set_recent_rows(ModelRc::new(VecModel::from(rows)));
    }

    fn show_toast(ui: &AppWindow, text: &str) {
        ui.set_toast_text(text.into());
        ui.set_show_toast(true);
    }

    fn copy_to_clipboard(text: &str) {
        #[cfg(target_os = "macos")]
        {
            let _ = std::process::Command::new("pbcopy")
                .stdin(std::process::Stdio::piped())
                .spawn()
                .and_then(|mut child| {
                    use std::io::Write;
                    if let Some(stdin) = child.stdin.as_mut() {
                        stdin.write_all(text.as_bytes())?;
                    }
                    child.wait()?;
                    Ok(())
                });
        }
        #[cfg(not(target_os = "macos"))]
        {
            let _ = text;
        }
    }

    fn refresh_draft_banner(ui: &AppWindow) {
        if let Some(draft) = WizardDraft::load() {
            let summary = format!(
                "Draft: case={} source={}",
                if draft.case_id.is_empty() {
                    "—"
                } else {
                    draft.case_id.as_str()
                },
                if draft.source_path.is_empty() {
                    "—"
                } else {
                    draft.source_path.as_str()
                }
            );
            ui.set_draft_summary(summary.into());
            ui.set_show_draft_banner(true);
        } else {
            ui.set_show_draft_banner(false);
            ui.set_draft_summary(SharedString::default());
        }
    }

    fn apply_disk_details(ui: &AppWindow, index: i32, write_blocker_ready: bool) {
        let rows = shell_ops::list_disk_ui_rows();
        let Some(row) = rows.get(index as usize) else {
            ui.set_selected_disk(-1);
            ui.set_detail_interface("—".into());
            ui.set_detail_sectors("—".into());
            ui.set_detail_sector_size("—".into());
            ui.set_detail_partitions("—".into());
            ui.set_detail_type("—".into());
            ui.set_detail_readonly("—".into());
            ui.set_detail_readonly_ok(false);
            return;
        };
        ui.set_selected_disk(index);
        ui.set_detail_interface(row.bus.clone().into());
        ui.set_detail_sectors("Unavailable / not probed".into());
        ui.set_detail_sector_size("Unavailable / not probed".into());
        ui.set_detail_partitions("Unavailable / not probed".into());
        ui.set_detail_type("Unavailable / not probed".into());
        ui.set_detail_readonly(
            if write_blocker_ready {
                "Yes (operator-confirmed / blocked)"
            } else {
                "Confirm before write"
            }
            .into(),
        );
        ui.set_detail_readonly_ok(write_blocker_ready);
    }

    fn apply_instrument(ui: &AppWindow, snap: &UiSnapshot) {
        let line = snap.write_blocker_line();
        ui.set_write_blocker_line(line.clone().into());
        let secondary = if snap.is_blockish_source() {
            if snap.wb_confirmed {
                "Operator confirmed HW block"
            } else {
                "Enumerator: OS disk list — verify HW"
            }
        } else {
            "File / non-blockish source"
        };
        ui.set_write_blocker_secondary(secondary.into());
        ui.set_write_blocker_ready(snap.write_blocker_ready());
        ui.set_wb_confirmed(snap.wb_confirmed);
        ui.set_blockish_source(snap.is_blockish_source());
    }

    fn apply_footer(ui: &AppWindow, snap: &UiSnapshot) {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        ui.set_footer_time(now.into());
        let user = if snap.case_identity.trim().is_empty() {
            std::env::var("USER")
                .or_else(|_| std::env::var("USERNAME"))
                .unwrap_or_else(|_| "examiner".into())
        } else {
            snap.case_identity.clone()
        };
        ui.set_footer_user(user.into());
        let host = std::env::var("HOSTNAME")
            .or_else(|_| std::env::var("COMPUTERNAME"))
            .unwrap_or_else(|_| "local".into());
        ui.set_footer_host(host.into());
        ui.set_app_version("Version 0.1.0".into());
        ui.set_app_build(format!("Build {}", trareon_core::build_identity()).into());
        let disk_n = shell_ops::list_disk_ui_rows().len();
        ui.set_status_evidence_text(disk_n.to_string().into());
        ui.set_status_coverage_text(
            shell_ops::coverage_status_text(ui.get_coverage_ok(), ui.get_coverage_err()).into(),
        );
        ui.set_status_mode_text(shell_ops::mode_status_label(ui.get_mode_index()).into());
    }

    fn apply(ui: &AppWindow, snap: &UiSnapshot) {
        ui.set_mode_index(snap.mode.index());
        ui.set_mode_guidance(snap.guidance().into());
        ui.set_show_path_editors(snap.show_path_editors());
        ui.set_case_identity(snap.case_identity.clone().into());
        ui.set_case_id(snap.case_id.clone().into());
        if ui.get_case_verify().is_empty() || ui.get_case_verify() == ui.get_case_id() {
            ui.set_case_verify(snap.case_id.clone().into());
        }
        ui.set_source_path(snap.source_path.clone().into());
        ui.set_output_dir(snap.output_dir.clone().into());
        ui.set_confirmed_synthetic(snap.confirmed_synthetic);
        ui.set_status_line(snap.status_line.clone().into());
        ui.set_last_package(snap.last_package.clone().into());
        ui.set_evidence_sha256(snap.evidence_sha256.clone().into());
        ui.set_evidence_size(snap.evidence_size.clone().into());
        let hint = snap.preflight_hint();
        ui.set_preflight_hint(hint.clone().into());
        ui.set_preflight_is_deny(hint.contains("DENY") || hint.contains("TOLAK"));
        ui.set_busy(snap.busy);
        ui.set_dark_mode(snap.dark_mode);
        ui.set_locale(snap.locale.as_str().into());
        ui.set_case_subtitle(ui.get_case_title());
        ui.set_progress_fraction(snap.progress_fraction as f32);
        ui.set_progress_label(snap.progress_label.clone().into());
        ui.set_progress_phase(snap.progress_phase.clone().into());
        ui.set_about_text(snap.about_text().into());
        ui.set_emergency_override(snap.emergency_override_reason.clone().into());
        ui.set_id_power(snap.identify_record.checklist_complete.power);
        ui.set_id_network(snap.identify_record.checklist_complete.network);
        ui.set_id_encryption(snap.identify_record.checklist_complete.encryption);
        ui.set_id_anti(snap.identify_record.checklist_complete.anti_forensics);
        ui.set_id_oov(snap.identify_record.oov_ack);
        apply_instrument(ui, snap);
        let sealed = snap.evidence_sha256 != acquire_slint::NONE_SENTINEL;
        ui.set_custody_timeline_text(
            shell_ops::custody_timeline(
                &snap.case_id,
                &snap.last_package,
                &snap.evidence_sha256,
                sealed,
            )
            .into(),
        );
        let (ok, err) = shell_ops::coverage_fractions(
            (snap.progress_fraction * snap.evidence_size.parse::<f64>().unwrap_or(1.0)) as u64,
            snap.evidence_size.parse().ok(),
            snap.progress_phase.contains("FAIL"),
        );
        if snap.busy {
            ui.set_coverage_ok(snap.progress_fraction as f32);
            ui.set_coverage_err(0.0);
        } else {
            ui.set_coverage_ok(if sealed { 1.0 } else { ok });
            ui.set_coverage_err(err);
        }
        apply_footer(ui, snap);
        ui.set_format_caption(shell_ops::format_label(ui.get_format_index()).into());
        ui.set_free_space_ok(snap.free_space_ok != Some(false));
        set_log_rows(
            ui,
            &shell_ops::custody_timeline(
                &snap.case_id,
                &snap.last_package,
                &snap.evidence_sha256,
                sealed,
            ),
        );
        // SHA-512 default: off in Guided, on otherwise (unless user already toggled via UI bind).
        if matches!(snap.mode, UiMode::Guided) && !snap.busy {
            // leave write_sha512 as UI state; guided default applied once at mode change
        }
    }

    fn sync_fields_from_ui(ui: &AppWindow, snap: &mut UiSnapshot) {
        snap.case_identity = ui.get_case_identity().as_str().to_string();
        snap.case_id = ui.get_case_id().as_str().to_string();
        snap.case_verify = ui.get_case_verify().as_str().to_string();
        snap.source_path = ui.get_source_path().as_str().to_string();
        snap.output_dir = ui.get_output_dir().as_str().to_string();
        snap.confirmed_synthetic = ui.get_confirmed_synthetic();
        snap.wb_confirmed = ui.get_wb_confirmed();
        snap.oov_ack = ui.get_id_oov();
        snap.emergency_override_reason = ui.get_emergency_override().as_str().to_string();
        snap.dark_mode = ui.get_dark_mode();
        snap.identify_record.oov_ack = ui.get_id_oov();
        snap.identify_record.checklist_complete.power = ui.get_id_power();
        snap.identify_record.checklist_complete.network = ui.get_id_network();
        snap.identify_record.checklist_complete.encryption = ui.get_id_encryption();
        snap.identify_record.checklist_complete.anti_forensics = ui.get_id_anti();
        snap.refresh_preflight_gates();
    }

    fn save_prefs(snap: &UiSnapshot) {
        AcquirePrefs {
            dark_mode: snap.dark_mode,
            locale: snap.locale.as_str().into(),
            examiner: snap.case_identity.clone(),
            last_output_dir: snap.output_dir.clone(),
            ..AcquirePrefs::default()
        }
        .save();
    }

    let ui = AppWindow::new()?;
    let mut initial_snapshot = UiSnapshot::from_prefs(&AcquirePrefs::load());
    initial_snapshot.restore_draft();
    // Prefer Standard for live ECR density; Guided remains available via mode chips.
    if matches!(initial_snapshot.mode, UiMode::Guided)
        && initial_snapshot.source_path.trim().is_empty()
    {
        initial_snapshot.mode = UiMode::Standard;
    }
    let snapshot = Arc::new(Mutex::new(initial_snapshot));
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let run_started = Arc::new(Mutex::new(None::<Instant>));
    {
        let s = snapshot.lock().expect("snapshot lock");
        apply(&ui, &s);
        ui.set_write_sha512(!matches!(s.mode, UiMode::Guided));
        ui.set_segment_mib("4096".into());
        if !s.case_identity.trim().is_empty() {
            ui.set_case_identity(s.case_identity.clone().into());
        }
    }
    load_disk_rows(&ui);
    load_case_rows(&ui);
    load_recent_rows(&ui);
    refresh_draft_banner(&ui);
    ui.set_panel_detail(shell_ops::refresh_cases_text().into());

    {
        let ui_weak = ui.as_weak();
        ui.on_refresh_case_rows(move || {
            if let Some(ui) = ui_weak.upgrade() {
                load_case_rows(&ui);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_case_selected(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let rows = shell_ops::list_case_ui_rows(
                    ui.get_case_filter().as_str(),
                    ui.get_case_sort_by_id(),
                );
                if let Some(row) = rows.get(idx as usize) {
                    let mut s = snap.lock().expect("snapshot lock");
                    s.case_id = row.id.clone();
                    s.case_identity = row.examiner.clone();
                    ui.set_selected_case(idx);
                    ui.set_case_title(row.title.clone().into());
                    ui.set_case_verify(row.id.clone().into());
                    apply(&ui, &s);
                    show_toast(&ui, &format!("Case {}", row.id));
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_copy_text_clicked(move |text| {
            if let Some(ui) = ui_weak.upgrade() {
                copy_to_clipboard(text.as_str());
                show_toast(&ui, "Copied");
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_pause_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                let out = if s.output_dir.trim().is_empty() {
                    std::env::temp_dir()
                } else {
                    PathBuf::from(&s.output_dir)
                };
                let checkpoint = default_checkpoint_path(&out);
                if let Err(err) = s.set_paused(&checkpoint) {
                    ui.set_panel_detail(err.into());
                } else {
                    s.status_line = "Paused — checkpoint written.".into();
                    ui.set_paused(true);
                    apply(&ui, &s);
                    show_toast(&ui, "Paused");
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_draft_resume_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                s.restore_draft();
                apply(&ui, &s);
                ui.set_show_draft_banner(false);
                ui.set_nav_index(0);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_draft_discard_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                WizardDraft::clear();
                ui.set_show_draft_banner(false);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        let cancel_flag = Arc::clone(&cancel_flag);
        ui.on_confirm_accepted(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let action = ui.get_confirm_action().as_str().to_string();
                match action.as_str() {
                    "archive" => {
                        let id = ui.get_case_id().as_str().to_string();
                        match shell_ops::archive_case(&id) {
                            Ok(msg) => {
                                ui.set_panel_detail(msg.into());
                                load_case_rows(&ui);
                                show_toast(&ui, "Archived");
                            }
                            Err(err) => ui.set_panel_detail(err.into()),
                        }
                    }
                    "discard-draft" => {
                        WizardDraft::clear();
                        ui.set_show_draft_banner(false);
                    }
                    "clear" => {
                        let mut s = snap.lock().expect("snapshot lock");
                        s.clear_results();
                        apply(&ui, &s);
                        cancel_flag.store(false, Ordering::SeqCst);
                    }
                    _ => {}
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_open_user_guide_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let guide = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/USER-GUIDE.md");
                #[cfg(target_os = "macos")]
                {
                    let _ = std::process::Command::new("open").arg(&guide).spawn();
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let _ = guide;
                }
                show_toast(&ui, "User guide");
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_recent_selected(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let list = recent::list();
                if let Some(entry) = list.get(idx as usize) {
                    let mut s = snap.lock().expect("snapshot lock");
                    s.last_package = entry.package_path.clone();
                    if !entry.case_id.is_empty() {
                        s.case_id = entry.case_id.clone();
                    }
                    apply(&ui, &s);
                    ui.set_nav_index(4);
                    set_log_rows(&ui, &shell_ops::verify_tools_text(&entry.package_path));
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_mode_changed(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                s.mode = UiMode::from_index(idx);
                ui.set_write_sha512(!matches!(s.mode, UiMode::Guided));
                apply(&ui, &s);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_instrument_refresh(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                apply_instrument(&ui, &s);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_theme_toggled(move |dark| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                s.set_dark_mode(dark);
                save_prefs(&s);
                apply(&ui, &s);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_locale_toggled(move |loc| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                s.set_locale(loc.as_str());
                save_prefs(&s);
                apply(&ui, &s);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_search_submitted(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let q = ui.get_search_query();
                if q.trim().is_empty() {
                    return;
                }
                if let Some(idx) = shell_ops::nav_index_for_search(q.as_str()) {
                    ui.set_nav_index(idx);
                    show_toast(&ui, &format!("→ {}", q.trim()));
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_browse_source_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let title = {
                    let s = snap.lock().expect("snapshot lock");
                    s.dialog_source_title().to_string()
                };
                if let Some(path) = rfd::FileDialog::new().set_title(&title).pick_file() {
                    let mut s = snap.lock().expect("snapshot lock");
                    sync_fields_from_ui(&ui, &mut s);
                    s.source_path = path.display().to_string();
                    apply(&ui, &s);
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_browse_output_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let title = {
                    let s = snap.lock().expect("snapshot lock");
                    s.dialog_output_title().to_string()
                };
                if let Some(path) = rfd::FileDialog::new().set_title(&title).pick_folder() {
                    let mut s = snap.lock().expect("snapshot lock");
                    sync_fields_from_ui(&ui, &mut s);
                    s.output_dir = path.display().to_string();
                    apply(&ui, &s);
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_fill_synthetic_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let out = std::env::temp_dir().join("trareon-acquire-slint-demo");
                let _ = std::fs::create_dir_all(&out);
                let source = out.join("synthetic-source.bin");
                if !source.is_file() {
                    let _ = std::fs::write(&source, vec![0u8; 8 * 1024 * 1024]);
                }
                let mut s = snap.lock().expect("snapshot lock");
                s.source_path = source.display().to_string();
                s.output_dir = out.display().to_string();
                s.confirmed_synthetic = true;
                if s.case_id.trim().is_empty() {
                    s.case_id = "DEMO-GUIDED".into();
                }
                s.clear_results();
                apply(&ui, &s);
                ui.set_case_verify(s.case_id.clone().into());
                ui.set_case_subtitle("Guided synthetic demo".into());
                // Preview table (labeled demo) — host truth remains empty until Refresh.
                let rows: Vec<DiskLine> = shell_ops::demo_disk_ui_rows()
                    .into_iter()
                    .map(|row| DiskLine {
                        path: row.path.into(),
                        device: row.device.into(),
                        model: row.model.into(),
                        serial: row.serial.into(),
                        size: row.size.into(),
                        bus: row.bus.into(),
                    })
                    .collect();
                ui.set_disk_rows(ModelRc::new(VecModel::from(rows)));
                apply_disk_details(&ui, 0, true);
                ui.set_panel_detail(
                    "Guided demo filled — synthetic source + preview disk table (not host enum)."
                        .into(),
                );
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        let cancel = Arc::clone(&cancel_flag);
        let run_started = Arc::clone(&run_started);
        ui.on_run_clicked(move || {
            let Some(ui) = ui_weak.upgrade() else {
                return;
            };
            {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                let gated = if matches!(s.mode, UiMode::Guided) {
                    s.can_start_guided()
                } else {
                    s.can_start_expert()
                };
                if !gated {
                    let msg = if s.dest_volume_conflict() {
                        "DENY: destination is on the same volume as the block source — choose another disk.".into()
                    } else if s.free_space_ok == Some(false) {
                        "DENY: destination free space is insufficient for the source size.".into()
                    } else if !s.case_verify_ok() {
                        "DENY: case verify does not match case id.".into()
                    } else if !s.can_run() {
                        s.msg_need_confirm_paths().to_string()
                    } else {
                        format!(
                            "Preflight blocked: {}",
                            s.override_audit_line().unwrap_or_else(|| {
                                "case / write-blocker / capacity / identify checklist".into()
                            })
                        )
                    };
                    s.set_err(msg);
                    apply(&ui, &s);
                    return;
                }
                let preflight = s.preflight_hint();
                if preflight.contains("DENY") || preflight.contains("TOLAK") {
                    s.set_err(preflight);
                    apply(&ui, &s);
                    return;
                }
                cancel.store(false, Ordering::SeqCst);
                s.save_draft();
                s.set_busy(true);
                s.status_line = s.msg_running().into();
                s.progress_fraction = 0.0;
                s.progress_label = String::new();
                s.progress_phase = "PREFLIGHT".into();
                apply(&ui, &s);
            }

            let source = PathBuf::from(ui.get_source_path().as_str());
            let output = PathBuf::from(ui.get_output_dir().as_str());
            let cancel_job = Arc::clone(&cancel);
            let ui_weak = ui.as_weak();
            let snap_done = Arc::clone(&snap);

            let format_index = ui.get_format_index();
            let write_sha512 = ui.get_write_sha512();
            let segment_mib: u64 = ui.get_segment_mib().as_str().parse().unwrap_or(64);
            *run_started.lock().expect("run clock") = Some(Instant::now());

            let progress_ui = ui.as_weak();
            let progress_snap = Arc::clone(&snap);
            let clock = Arc::clone(&run_started);
            let progress_cb: ProgressCallback = Arc::new(move |p: AcquireProgress| {
                let fraction = p.fraction().unwrap_or(0.0);
                let label = match p.bytes_total {
                    Some(total) => format!("{} / {} B", p.bytes_done, total),
                    None => format!("{} B", p.bytes_done),
                };
                let phase = p.phase.as_str().to_ascii_uppercase();
                let elapsed = clock
                    .lock()
                    .ok()
                    .and_then(|g| g.map(|t| t.elapsed().as_secs_f64()))
                    .unwrap_or(0.0);
                let (throughput, eta) =
                    shell_ops::throughput_eta(p.bytes_done, p.bytes_total, elapsed);
                let (cov_ok, cov_err) =
                    shell_ops::coverage_fractions(p.bytes_done, p.bytes_total, false);
                let ui_weak = progress_ui.clone();
                let snap = Arc::clone(&progress_snap);
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak.upgrade() {
                        let mut s = snap.lock().expect("snapshot lock");
                        s.progress_fraction = fraction;
                        s.progress_label = label;
                        s.progress_phase = phase;
                        ui.set_throughput_label(throughput.into());
                        ui.set_eta_label(eta.into());
                        ui.set_coverage_ok(cov_ok);
                        ui.set_coverage_err(cov_err);
                        if s.busy {
                            apply(&ui, &s);
                        }
                    }
                });
            });

            thread::spawn(move || {
                let result = shell_ops::acquire_with_format_controls(
                    &source,
                    &output,
                    format_index,
                    segment_mib,
                    write_sha512,
                    Some(cancel_job),
                    Some(progress_cb),
                )
                .map(|(pkg, hash, size)| {
                    let sha512 = if write_sha512 {
                        let path = PathBuf::from(&pkg);
                        let side = path.with_extension(format!(
                            "{}.sha512",
                            path.extension().and_then(|e| e.to_str()).unwrap_or("")
                        ));
                        std::fs::read_to_string(side)
                            .unwrap_or_default()
                            .trim()
                            .to_string()
                    } else {
                        String::new()
                    };
                    (pkg, hash, size, sha512)
                });
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak.upgrade() {
                        let mut s = snap_done.lock().expect("snapshot lock");
                        match result {
                            Ok((package_path, evidence_sha256, evidence_size, sha512)) => {
                                let package = PathBuf::from(&package_path);
                                let examiner = if s.case_identity.trim().is_empty() {
                                    "lab-operator"
                                } else {
                                    s.case_identity.as_str()
                                };
                                let (status_msg, sealed_ok) = if package_path.ends_with(".fsnap")
                                    || PathBuf::from(&package_path)
                                        .extension()
                                        .and_then(|e| e.to_str())
                                        == Some("fsnap")
                                    || PathBuf::from(&package_path).is_dir()
                                {
                                    match preserve::seal_package(
                                        &package,
                                        if s.case_id.trim().is_empty() {
                                            "TRAINING"
                                        } else {
                                            s.case_id.as_str()
                                        },
                                        examiner,
                                        &s.source_path,
                                    ) {
                                        Ok(coc) => (
                                            format!(
                                                "{} — sealed {}",
                                                s.msg_verified(),
                                                coc.evidence_id
                                            ),
                                            true,
                                        ),
                                        Err(err) => (UiSnapshot::msg_seal_pending(&err), false),
                                    }
                                } else {
                                    (
                                        format!(
                                            "ACQUIRED — container written (external verify if not .fsnap)"
                                        ),
                                        false,
                                    )
                                };
                                let recent_entry =
                                    recent::RecentEntry::completed(&s.case_id, &package_path);
                                s.set_ok(
                                    package_path,
                                    evidence_sha256,
                                    evidence_size,
                                    status_msg,
                                );
                                let _ = sealed_ok;
                                s.progress_fraction = 1.0;
                                s.progress_phase = "DONE".into();
                                s.progress_label = format!("{evidence_size} B");
                                if !sha512.is_empty() {
                                    ui.set_evidence_sha512(sha512.into());
                                }
                                recent::append(recent_entry);
                                UiSnapshot::clear_draft();
                            }
                            Err(err) => {
                                let lower = err.to_lowercase();
                                let evidence = PathBuf::from(&s.output_dir).join("evidence.raw");
                                let checkpoint = default_checkpoint_path(&evidence);
                                if lower.contains("cancel") {
                                    let _ = preserve::write_stop_reason(
                                        &checkpoint,
                                        preserve::StopReason::Cancelled,
                                    );
                                    let msg = s.msg_cancelled(&err);
                                    s.set_err(msg);
                                } else {
                                    let msg = s.msg_failed(&err);
                                    s.set_err(msg);
                                }
                                s.progress_phase = "FAILED".into();
                            }
                        }
                        apply(&ui, &s);
                    }
                });
            });
        });
    }

    {
        let cancel = Arc::clone(&cancel_flag);
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_cancel_clicked(move || {
            cancel.store(true, Ordering::SeqCst);
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                if s.busy {
                    s.status_line = s.msg_cancelling().into();
                    apply(&ui, &s);
                }
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_export_coc_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                if s.last_package == acquire_slint::NONE_SENTINEL {
                    let msg = s.msg_no_package().to_string();
                    s.set_err(msg);
                    apply(&ui, &s);
                    return;
                }
                let path = PathBuf::from(&s.output_dir).join("ui-coc-summary.json");
                match write_coc_summary(&path, &s.coc_report_json()) {
                    Ok(()) => {
                        s.status_line = s.msg_coc_written(&path.display().to_string());
                    }
                    Err(err) => {
                        let msg = s.msg_coc_failed(&err);
                        s.set_err(msg);
                    }
                }
                apply(&ui, &s);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_clear_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                s.clear_results();
                ui.set_evidence_sha512("(none)".into());
                apply(&ui, &s);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        ui.on_refresh_disks_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                load_disk_rows(&ui);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_use_first_disk_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let rows = shell_ops::list_disk_ui_rows();
                if let Some(row) = rows.first() {
                    let mut s = snap.lock().expect("snapshot lock");
                    sync_fields_from_ui(&ui, &mut s);
                    s.source_path = row.path.clone();
                    apply(&ui, &s);
                    apply_disk_details(&ui, 0, s.write_blocker_ready());
                    ui.set_panel_detail(format!("Source set to {}", row.path).into());
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_disk_selected(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let rows = shell_ops::list_disk_ui_rows();
                if let Some(row) = rows.get(idx as usize) {
                    let mut s = snap.lock().expect("snapshot lock");
                    sync_fields_from_ui(&ui, &mut s);
                    s.source_path = row.path.clone();
                    if s.output_dir.trim().is_empty() || s.output_dir.contains("/Acquire/") {
                        let case = if s.case_id.trim().is_empty() {
                            "NOCASE"
                        } else {
                            s.case_id.as_str()
                        };
                        let leaf = row
                            .device
                            .to_ascii_lowercase()
                            .replace(' ', "");
                        let home = std::env::var("HOME").unwrap_or_else(|_| ".".into());
                        s.output_dir = format!("{home}/Evidence/{case}/Acquire/{leaf}");
                    }
                    let wb = s.write_blocker_ready();
                    apply(&ui, &s);
                    apply_disk_details(&ui, idx, wb);
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_refresh_cases_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(shell_ops::refresh_cases_text().into());
                load_case_rows(&ui);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_create_case_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let title = ui.get_case_title().as_str().to_string();
                let examiner = ui.get_case_identity().as_str().to_string();
                match shell_ops::create_case(&title, &examiner, "") {
                    Ok(msg) => {
                        let mut s = snap.lock().expect("snapshot lock");
                        if let Some(id) = msg.split_whitespace().nth(1) {
                            s.case_id = id.to_string();
                        }
                        s.case_identity = examiner;
                        apply(&ui, &s);
                        ui.set_case_verify(s.case_id.clone().into());
                        ui.set_case_subtitle(
                            if title.trim().is_empty() {
                                "New case".into()
                            } else {
                                title.into()
                            },
                        );
                        ui.set_panel_detail(
                            format!(
                                "{msg}\n{}\nNext: Identify checklist, then Acquire.",
                                shell_ops::refresh_cases_text()
                            )
                            .into(),
                        );
                        load_case_rows(&ui);
                        show_toast(&ui, "Case created");
                        ui.set_nav_index(2);
                    }
                    Err(err) => ui.set_panel_detail(err.into()),
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_save_identify_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let case_id = ui.get_case_id().as_str().to_string();
                match shell_ops::save_identify_for_case(
                    &case_id,
                    ui.get_id_power(),
                    ui.get_id_network(),
                    ui.get_id_encryption(),
                    ui.get_id_anti(),
                    ui.get_id_oov(),
                ) {
                    Ok(msg) => {
                        let mut s = snap.lock().expect("snapshot lock");
                        s.oov_ack = ui.get_id_oov();
                        apply(&ui, &s);
                        ui.set_panel_detail(msg.into());
                    }
                    Err(err) => ui.set_panel_detail(err.into()),
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_probe_encryption_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let source = ui.get_source_path().as_str().to_string();
                let text = shell_ops::identify_probe_text(&source);
                ui.set_panel_detail(text.clone().into());
                set_log_rows(&ui, &text);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_verify_package_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let text = shell_ops::verify_tools_text(ui.get_last_package().as_str());
                ui.set_panel_detail(text.clone().into());
                set_log_rows(&ui, &text);
                if text.to_ascii_lowercase().contains("ok")
                    || text.to_ascii_lowercase().contains("valid")
                {
                    show_toast(&ui, "Verify OK");
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_hash_only_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let text = shell_ops::hash_only_text(ui.get_last_package().as_str());
                ui.set_panel_detail(text.clone().into());
                set_log_rows(&ui, &text);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_run_qms_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let text = shell_ops::run_qms_text();
                ui.set_panel_detail(text.clone().into());
                set_result_rows(&ui, shell_ops::qms_check_ui_rows());
                set_log_rows(&ui, &text);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_run_triage_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let out = ui.get_output_dir().as_str().to_string();
                let text = shell_ops::run_triage_text(&out);
                ui.set_panel_detail(text.clone().into());
                set_log_rows(&ui, &text);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_boot_plan_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let text = shell_ops::boot_plan_text(ui.get_boot_image_path().as_str(), "");
                ui.set_panel_detail(text.clone().into());
                set_log_rows(&ui, &text);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_analysis_lite_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                match shell_ops::analysis_lite_rows(
                    ui.get_last_package().as_str(),
                    ui.get_output_dir().as_str(),
                ) {
                    Ok((timeline, listing, summary)) => {
                        ui.set_panel_detail(summary.clone().into());
                        set_log_rows(&ui, &summary);
                        let trows: Vec<TimelineLine> = timeline
                            .into_iter()
                            .map(|row| TimelineLine {
                                when: row.when.into(),
                                what: row.what.into(),
                            })
                            .collect();
                        let lrows: Vec<ListingLine> = listing
                            .into_iter()
                            .map(|row| ListingLine {
                                path: row.path.into(),
                                size: row.size.into(),
                                kind: row.kind.into(),
                            })
                            .collect();
                        ui.set_timeline_rows(ModelRc::new(VecModel::from(trows)));
                        ui.set_listing_rows(ModelRc::new(VecModel::from(lrows)));
                    }
                    Err(err) => {
                        ui.set_panel_detail(err.clone().into());
                        set_log_rows(&ui, &err);
                    }
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_format_changed(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_format_index(idx);
                ui.set_format_caption(shell_ops::format_label(idx).into());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_profile_changed(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_profile_index(idx);
                ui.set_profile_caption(shell_ops::profile_caption(idx).into());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_source_kind_changed(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_source_kind_index(idx);
                let caption = shell_ops::source_kind_caption(idx);
                ui.set_source_kind_caption(caption.clone().into());
                if idx == 2 || idx == 3 {
                    ui.set_panel_detail(caption.into());
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_retry_changed(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_retry_index(idx);
                ui.set_retry_caption(shell_ops::retry_policy_caption(idx).into());
                // Cosmetically reserve coverage error band when fail-closed is selected.
                if idx == 2 {
                    ui.set_coverage_err(0.0);
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_export_full_coc_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                let media = ui.get_coc_media().parse().unwrap_or(1);
                let seq = ui.get_coc_seq().parse().unwrap_or(1);
                match shell_ops::export_full_coc(
                    &s.last_package,
                    &s.case_id,
                    &s.case_identity,
                    ui.get_coc_device().as_str(),
                    media,
                    seq,
                    ui.get_coc_description().as_str(),
                ) {
                    Ok(msg) => {
                        s.status_line = msg.clone();
                        ui.set_panel_detail(msg.clone().into());
                        set_log_rows(&ui, &msg);
                        if let Some(parent) = PathBuf::from(&s.last_package).parent() {
                            let qr = parent.join("qr.png");
                            if qr.is_file() {
                                ui.set_qr_image_path(qr.display().to_string().into());
                            }
                        }
                        show_toast(&ui, "CoC exported");
                    }
                    Err(err) => {
                        s.set_err(err.clone());
                        ui.set_panel_detail(err.into());
                    }
                }
                apply(&ui, &s);
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_compare_packages_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let a = ui.get_last_package().as_str().to_string();
                let b = ui.get_compare_package_b().as_str().to_string();
                let text = shell_ops::compare_packages_text(&a, &b);
                ui.set_panel_detail(text.clone().into());
                set_log_rows(&ui, &text);
                let rows: Vec<ResultLine> = shell_ops::compare_summary_rows(&a, &b)
                    .into_iter()
                    .map(|row| ResultLine {
                        name: row.name.into(),
                        ok: row.ok,
                        detail: row.detail.into(),
                    })
                    .collect();
                ui.set_compare_rows(ModelRc::new(VecModel::from(rows)));
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_browse_compare_clicked(move || {
            if let Some(ui) = ui_weak.upgrade()
                && let Some(path) = rfd::FileDialog::new().pick_folder()
            {
                ui.set_compare_package_b(path.display().to_string().into());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_export_report_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(
                    shell_ops::export_report_text(
                        ui.get_last_package().as_str(),
                        ui.get_output_dir().as_str(),
                    )
                    .into(),
                );
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_run_profile_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(
                    shell_ops::run_quick_profile_text(
                        ui.get_source_path().as_str(),
                        ui.get_output_dir().as_str(),
                    )
                    .into(),
                );
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_multisource_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(
                    shell_ops::multisource_text(
                        ui.get_source_path().as_str(),
                        ui.get_multisource_b().as_str(),
                        ui.get_output_dir().as_str(),
                    )
                    .into(),
                );
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_browse_package_clicked(move || {
            if let Some(ui) = ui_weak.upgrade()
                && let Some(path) = rfd::FileDialog::new()
                    .add_filter("fsnap", &["fsnap"])
                    .pick_folder()
            {
                let mut s = snap.lock().expect("snapshot lock");
                s.last_package = path.display().to_string();
                apply(&ui, &s);
                ui.set_panel_detail(shell_ops::verify_tools_text(s.last_package.as_str()).into());
            }
        });
    }

    ui.run()
}

#[cfg(not(feature = "gui"))]
fn main() {
    eprintln!("acquire-slint built without `gui` feature");
    std::process::exit(1);
}
