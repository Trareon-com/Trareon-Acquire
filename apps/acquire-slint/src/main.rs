#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use acquire_slint::{AppWindow, UiSnapshot, run_foundation_demo};
    use slint::ComponentHandle;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn apply(ui: &AppWindow, snap: &UiSnapshot) {
        ui.set_case_identity(snap.case_identity.clone().into());
        ui.set_source_path(snap.source_path.clone().into());
        ui.set_output_dir(snap.output_dir.clone().into());
        ui.set_confirmed_synthetic(snap.confirmed_synthetic);
        ui.set_status_line(snap.status_line.clone().into());
        ui.set_last_package(snap.last_package.clone().into());
        ui.set_evidence_sha256(snap.evidence_sha256.clone().into());
        ui.set_evidence_size(snap.evidence_size.clone().into());
        ui.set_busy(snap.busy);
    }

    fn sync_fields_from_ui(ui: &AppWindow, snap: &mut UiSnapshot) {
        snap.case_identity = ui.get_case_identity().as_str().to_string();
        snap.source_path = ui.get_source_path().as_str().to_string();
        snap.output_dir = ui.get_output_dir().as_str().to_string();
        snap.confirmed_synthetic = ui.get_confirmed_synthetic();
    }

    let ui = AppWindow::new()?;
    let snapshot = Arc::new(Mutex::new(UiSnapshot::default()));
    let cancel_flag = Arc::new(AtomicBool::new(false));
    apply(&ui, &snapshot.lock().expect("snapshot lock"));

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_browse_source_clicked(move || {
            if let Some(ui) = ui_weak.upgrade()
                && let Some(path) = rfd::FileDialog::new()
                    .set_title("Select synthetic source file")
                    .pick_file()
            {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                s.source_path = path.display().to_string();
                apply(&ui, &s);
            }
        });
    }

    {
        let ui_weak = ui.as_weak();
        let snap = Arc::clone(&snapshot);
        ui.on_browse_output_clicked(move || {
            if let Some(ui) = ui_weak.upgrade()
                && let Some(path) = rfd::FileDialog::new()
                    .set_title("Select output directory")
                    .pick_folder()
            {
                let mut s = snap.lock().expect("snapshot lock");
                sync_fields_from_ui(&ui, &mut s);
                s.output_dir = path.display().to_string();
                apply(&ui, &s);
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
                if !s.can_run() {
                    s.set_err("Confirm synthetic source and set source + output paths.");
                    apply(&ui, &s);
                    return;
                }
                cancel.store(false, Ordering::SeqCst);
                s.set_busy(true);
                s.status_line = "Running foundation demo…".into();
                apply(&ui, &s);
            }

            let source = PathBuf::from(ui.get_source_path().as_str());
            let output = PathBuf::from(ui.get_output_dir().as_str());
            let cancel_job = Arc::clone(&cancel);
            let ui_weak = ui.as_weak();
            let snap_done = Arc::clone(&snap);

            thread::spawn(move || {
                let result = run_foundation_demo(&source, &output, Some(cancel_job));
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ui_weak.upgrade() {
                        let mut s = snap_done.lock().expect("snapshot lock");
                        match result {
                            Ok(demo) => {
                                s.set_ok(
                                    demo.package_path,
                                    demo.evidence_sha256,
                                    demo.evidence_size,
                                    "Verified Complete",
                                );
                            }
                            Err(err) => {
                                let lower = err.to_lowercase();
                                if lower.contains("cancel") {
                                    s.set_err(format!("Cancelled: {err}"));
                                } else {
                                    s.set_err(format!("Failed: {err}"));
                                }
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
                    s.status_line = "Cancelling…".into();
                    apply(&ui, &s);
                }
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
