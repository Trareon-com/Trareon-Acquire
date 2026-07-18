#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use acquire_slint::{
        AppWindow, UiMode, UiSnapshot, prefs::AcquirePrefs, preserve, recent,
        run_foundation_demo_with_progress, write_coc_summary,
    };
    use slint::ComponentHandle;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;
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
    }

    fn sync_fields_from_ui(ui: &AppWindow, snap: &mut UiSnapshot) {
        snap.case_identity = ui.get_case_identity().as_str().to_string();
        snap.case_id = ui.get_case_id().as_str().to_string();
        snap.source_path = ui.get_source_path().as_str().to_string();
        snap.output_dir = ui.get_output_dir().as_str().to_string();
        snap.confirmed_synthetic = ui.get_confirmed_synthetic();
        snap.wb_confirmed = ui.get_wb_confirmed();
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

            let progress_ui = ui.as_weak();
            let progress_snap = Arc::clone(&snap);
            let progress_cb: ProgressCallback = Arc::new(move |p: AcquireProgress| {
                let fraction = p.fraction().unwrap_or(0.0);
                let label = match p.bytes_total {
                    Some(total) => format!("{} / {} B", p.bytes_done, total),
                    None => format!("{} B", p.bytes_done),
                };
                let phase = p.phase.as_str().to_ascii_uppercase();
                let ui_weak = progress_ui.clone();
                let snap = Arc::clone(&progress_snap);
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak.upgrade() {
                        let mut s = snap.lock().expect("snapshot lock");
                        s.progress_fraction = fraction;
                        s.progress_label = label;
                        s.progress_phase = phase;
                        if s.busy {
                            apply(&ui, &s);
                        }
                    }
                });
            });

            thread::spawn(move || {
                let result = run_foundation_demo_with_progress(
                    &source,
                    &output,
                    Some(cancel_job),
                    Some(progress_cb),
                );
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak.upgrade() {
                        let mut s = snap_done.lock().expect("snapshot lock");
                        match result {
                            Ok(demo) => {
                                let package = PathBuf::from(&demo.package_path);
                                let examiner = if s.case_id.trim().is_empty() {
                                    "lab-operator"
                                } else {
                                    s.case_id.as_str()
                                };
                                let seal_note = preserve::seal_package(
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
                                .unwrap_or_else(|err| format!("; seal deferred: {err}"));
                                let ok = format!("{}{seal_note}", s.msg_verified());
                                let recent_entry =
                                    recent::RecentEntry::completed(&s.case_id, &demo.package_path);
                                s.set_ok(
                                    demo.package_path,
                                    demo.evidence_sha256,
                                    demo.evidence_size,
                                    ok,
                                );
                                s.progress_fraction = 1.0;
                                s.progress_phase = "DONE".into();
                                s.progress_label = format!("{} B", demo.evidence_size);
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
                apply(&ui, &s);
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
