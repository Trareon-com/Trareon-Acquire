use std::path::Path;

use trareon_core::{AcquireRequest, acquire_file, create_fsnap, verify_fsnap};

#[derive(serde::Serialize)]
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
) -> Result<FoundationDemoResult, String> {
    let source_path = Path::new(source);
    if !source_path.is_file() {
        return Err(format!("source path is not a file: {source}"));
    }

    let output_dir = Path::new(output_dir);
    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;

    let evidence_path = output_dir.join("evidence.raw");
    let package_path = output_dir.join("foundation.fsnap");

    let summary = acquire_file(&AcquireRequest::new(source_path, &evidence_path))
        .map_err(|error| error.to_string())?;

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
fn run_foundation_demo(source: String, output_dir: String) -> Result<FoundationDemoResult, String> {
    run_foundation_demo_inner(&source, &output_dir)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_foundation_demo])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
