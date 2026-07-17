use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::State;
use trareon_core::{AcquireRequest, acquire_file, create_fsnap, verify_fsnap};

#[derive(Default)]
pub struct DemoCancelState {
    pub cancel: Arc<AtomicBool>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FoundationDemoResult {
    pub status: String,
    pub package_path: String,
    pub evidence_sha256: String,
    pub evidence_size: u64,
}

/// Plain Rust core of the foundation demo, independent of `tauri::AppHandle`
/// so it can be unit tested without a WebView.
pub fn run_foundation_demo_inner(
    source: &str,
    output_dir: &str,
    cancel_flag: Option<Arc<AtomicBool>>,
) -> Result<FoundationDemoResult, String> {
    let source_path = Path::new(source);
    if !source_path.is_file() {
        return Err(format!("source path is not a file: {source}"));
    }

    let output_dir = Path::new(output_dir);
    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;

    let evidence_path = output_dir.join("evidence.raw");
    let package_path = output_dir.join("foundation.fsnap");

    let mut request = AcquireRequest::new(source_path, &evidence_path);
    if let Some(flag) = cancel_flag {
        request = request.with_cancel_flag(flag);
    }

    let summary = acquire_file(&request).map_err(|error| error.to_string())?;

    create_fsnap(&evidence_path, &summary.audit_path, &package_path)
        .map_err(|error| error.to_string())?;

    let manifest = verify_fsnap(&package_path).map_err(|error| error.to_string())?;

    Ok(FoundationDemoResult {
        status: "verified_complete".to_string(),
        package_path: package_path.to_string_lossy().to_string(),
        evidence_sha256: manifest.evidence_sha256,
        evidence_size: manifest.evidence_size,
    })
}

#[tauri::command]
async fn run_foundation_demo(
    source: String,
    output_dir: String,
    state: State<'_, DemoCancelState>,
) -> Result<FoundationDemoResult, String> {
    let cancel = Arc::clone(&state.cancel);
    cancel.store(false, Ordering::SeqCst);
    let cancel_for_job = Arc::clone(&cancel);
    tauri::async_runtime::spawn_blocking(move || {
        run_foundation_demo_inner(&source, &output_dir, Some(cancel_for_job))
    })
    .await
    .map_err(|error| format!("demo task join failed: {error}"))?
}

#[tauri::command]
fn cancel_foundation_demo(state: State<'_, DemoCancelState>) -> Result<(), String> {
    state.cancel.store(true, Ordering::SeqCst);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DemoCancelState::default())
        .invoke_handler(tauri::generate_handler![
            run_foundation_demo,
            cancel_foundation_demo
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::AtomicBool;
    use tempfile::tempdir;

    #[test]
    fn prearmed_cancel_never_reports_verified_complete() {
        let dir = tempdir().unwrap();
        let source = dir.path().join("source.img");
        std::fs::write(&source, vec![0u8; 1024 * 64]).unwrap();
        let out = dir.path().join("out");
        std::fs::create_dir_all(&out).unwrap();
        let flag = Arc::new(AtomicBool::new(true));
        let err =
            run_foundation_demo_inner(source.to_str().unwrap(), out.to_str().unwrap(), Some(flag))
                .unwrap_err();
        assert!(err.to_lowercase().contains("cancel"));
    }
}
