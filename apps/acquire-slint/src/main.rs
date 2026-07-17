#[cfg(feature = "gui")]
fn main() -> Result<(), slint::PlatformError> {
    use acquire_slint::{AppWindow, UiSnapshot, run_synthetic_demo};
    use slint::ComponentHandle;
    use std::cell::RefCell;
    use std::rc::Rc;

    let ui = AppWindow::new()?;
    let snapshot = Rc::new(RefCell::new(UiSnapshot::default()));

    let apply = |ui: &AppWindow, snap: &UiSnapshot| {
        ui.set_status_line(snap.status_line.clone().into());
        ui.set_last_package(snap.last_package.clone().into());
        ui.set_busy(snap.busy);
    };
    apply(&ui, &snapshot.borrow());

    let ui_weak = ui.as_weak();
    let snap_run = Rc::clone(&snapshot);
    ui.on_run_synthetic_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            {
                let mut snap = snap_run.borrow_mut();
                snap.set_busy(true);
                snap.status_line = "Running synthetic acquire…".into();
                apply(&ui, &snap);
            }

            let out = std::env::temp_dir().join("trareon-acquire-slint-demo");
            let result = run_synthetic_demo(&out);

            let mut snap = snap_run.borrow_mut();
            match result {
                Ok(package) => {
                    snap.set_ok(
                        package.display().to_string(),
                        "Acquire + package + verify OK (synthetic)",
                    );
                }
                Err(err) => snap.set_err(format!("FAILED: {err}")),
            }
            apply(&ui, &snap);
        }
    });

    let ui_weak = ui.as_weak();
    let snap_clear = Rc::clone(&snapshot);
    ui.on_clear_status_clicked(move || {
        if let Some(ui) = ui_weak.upgrade() {
            let mut snap = snap_clear.borrow_mut();
            snap.clear();
            apply(&ui, &snap);
        }
    });

    ui.run()
}

#[cfg(not(feature = "gui"))]
fn main() {
    eprintln!("acquire-slint built without `gui` feature");
    std::process::exit(1);
}
