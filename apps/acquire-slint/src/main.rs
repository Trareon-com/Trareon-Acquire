#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use acquire_slint::{
        AppWindow, UiMode, UiSnapshot, prefs::AcquirePrefs, preserve, recent,
        run_foundation_demo_with_progress, shell_ops, write_coc_summary,
    };
    use slint::ComponentHandle;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Instant;
    use trareon_core::{AcquireProgress, ProgressCallback, default_checkpoint_path};

    fn apply(ui: &AppWindow, snap: &UiSnapshot) {
        ui.set_mode_index(snap.mode.index());
        ui.set_mode_guidance(snap.guidance().into());
        ui.set_show_path_editors(snap.show_path_editors());
        ui.set_case_identity(snap.case_identity.clone().into());
        ui.set_case_id(snap.case_id.clone().into());
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
        ui.set_progress_fraction(snap.progress_fraction as f32);
        ui.set_progress_label(snap.progress_label.clone().into());
        ui.set_progress_phase(snap.progress_phase.clone().into());
        ui.set_about_text(snap.about_text().into());
        ui.set_write_blocker_line(snap.write_blocker_line().into());
        ui.set_write_blocker_ready(snap.write_blocker_ready());
        ui.set_wb_confirmed(snap.wb_confirmed);
        ui.set_blockish_source(snap.is_blockish_source());
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
        // Prefer live progress fraction when busy.
        if snap.busy {
            ui.set_coverage_ok(snap.progress_fraction as f32);
            ui.set_coverage_err(0.0);
        } else {
            ui.set_coverage_ok(if sealed { 1.0 } else { ok });
            ui.set_coverage_err(err);
        }
        ui.set_format_caption(shell_ops::format_label(ui.get_format_index()).into());
    }

    fn sync_fields_from_ui(ui: &AppWindow, snap: &mut UiSnapshot) {
        snap.case_identity = ui.get_case_identity().as_str().to_string();
        snap.case_id = ui.get_case_id().as_str().to_string();
        snap.source_path = ui.get_source_path().as_str().to_string();
        snap.output_dir = ui.get_output_dir().as_str().to_string();
        snap.confirmed_synthetic = ui.get_confirmed_synthetic();
        snap.wb_confirmed = ui.get_wb_confirmed();
        snap.oov_ack = ui.get_id_oov();
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
    let snapshot = Arc::new(Mutex::new(initial_snapshot));
    let cancel_flag = Arc::new(AtomicBool::new(false));
    let run_started = Arc::new(Mutex::new(None::<Instant>));
    ui.set_disk_list(shell_ops::list_disks_text().into());
    ui.set_panel_detail(shell_ops::refresh_cases_text().into());
    apply(&ui, &snapshot.lock().expect("snapshot lock"));

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_mode_changed(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                s.mode = UiMode::from_index(idx);
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
                // Update instrument chips only — avoid rewriting path fields (changed-loop).
                ui.set_write_blocker_line(s.write_blocker_line().into());
                ui.set_write_blocker_ready(s.write_blocker_ready());
                ui.set_blockish_source(s.is_blockish_source());
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
                s.clear_results();
                apply(&ui, &s);
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
                    let msg = if !s.can_run() {
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
                let result = if format_index == 0 && !write_sha512 {
                    run_foundation_demo_with_progress(
                        &source,
                        &output,
                        Some(cancel_job),
                        Some(progress_cb),
                    )
                    .map(|demo| {
                        (
                            demo.package_path,
                            demo.evidence_sha256,
                            demo.evidence_size,
                            String::new(),
                        )
                    })
                } else {
                    shell_ops::acquire_with_format(
                        &source,
                        &output,
                        format_index,
                        segment_mib,
                        write_sha512,
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
                    })
                };
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
                                let seal_note = if package_path.ends_with(".fsnap") {
                                    preserve::seal_package(
                                        &package,
                                        if s.case_id.trim().is_empty() {
                                            "TRAINING"
                                        } else {
                                            s.case_id.as_str()
                                        },
                                        examiner,
                                        &s.source_path,
                                    )
                                    .map(|coc| format!("; sealed {}", coc.evidence_id))
                                    .unwrap_or_else(|err| format!("; seal deferred: {err}"))
                                } else {
                                    "; container written (verify externally if not .fsnap)".into()
                                };
                                let ok = format!("{}{seal_note}", s.msg_verified());
                                let recent_entry =
                                    recent::RecentEntry::completed(&s.case_id, &package_path);
                                s.set_ok(package_path, evidence_sha256, evidence_size, ok);
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
                ui.set_disk_list(shell_ops::list_disks_text().into());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_use_first_disk_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let list = shell_ops::list_disks_text();
                if let Some(line) = list.lines().next()
                    && let Some(path) = line.split(" · ").next()
                {
                    let mut s = snap.lock().expect("snapshot lock");
                    sync_fields_from_ui(&ui, &mut s);
                    s.source_path = path.to_string();
                    apply(&ui, &s);
                    ui.set_panel_detail(format!("Source set to {path}").into());
                }
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_refresh_cases_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(shell_ops::refresh_cases_text().into());
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
                        ui.set_panel_detail(
                            format!("{msg}\n{}", shell_ops::refresh_cases_text()).into(),
                        );
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
                ui.set_panel_detail(shell_ops::identify_probe_text(&source).into());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_verify_package_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(
                    shell_ops::verify_tools_text(ui.get_last_package().as_str()).into(),
                );
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_hash_only_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(
                    shell_ops::hash_only_text(ui.get_last_package().as_str()).into(),
                );
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_run_qms_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(shell_ops::run_qms_text().into());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_run_triage_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                let out = ui.get_output_dir().as_str().to_string();
                ui.set_panel_detail(shell_ops::run_triage_text(&out).into());
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_boot_plan_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(
                    shell_ops::boot_plan_text(ui.get_boot_image_path().as_str(), "").into(),
                );
            }
        });
    }
    {
        let ui_weak = ui.as_weak();
        ui.on_analysis_lite_clicked(move || {
            if let Some(ui) = ui_weak.upgrade() {
                ui.set_panel_detail(
                    shell_ops::analysis_lite_text(
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
                        ui.set_panel_detail(msg.into());
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
                ui.set_panel_detail(
                    shell_ops::compare_packages_text(
                        ui.get_last_package().as_str(),
                        ui.get_compare_package_b().as_str(),
                    )
                    .into(),
                );
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
