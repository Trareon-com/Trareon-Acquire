//! Bounded Windows physical-drive lab smoke (requires elevated admin/UAC).
//!
//! Does **not** image the whole disk. Use allowlisted `\\.\PhysicalDriveN`
//! where N is a **removable** USB — never `PhysicalDrive0`.
//!
//! ```powershell
//! # Elevated PowerShell or admin terminal:
//! cargo run -p trareon-core --example lab_windows_bounded_smoke -- `
//!   \\.\PhysicalDriveN fixtures/lab-allowlists/your-windows-allowlist.json 1048576
//! ```

use std::{env, fs, path::PathBuf, process};
use trareon_core::{AcquireRequest, acquire_file, create_fsnap, load_lab_allowlist, verify_fsnap};

fn main() {
    let mut args = env::args().skip(1);
    let source = PathBuf::from(args.next().unwrap_or_else(|| r"\\.\PhysicalDrive1".into()));
    let allowlist_path = PathBuf::from(
        args.next()
            .unwrap_or_else(|| "fixtures/lab-allowlists/windows-usb-template.json".into()),
    );
    let max_bytes: u64 = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1024 * 1024);

    let source_str = source.to_string_lossy().to_ascii_lowercase();
    if source_str.contains("physicaldrive0") {
        eprintln!("refusing: PhysicalDrive0 is hard-denied (system disk)");
        process::exit(2);
    }

    let allow = load_lab_allowlist(&allowlist_path).expect("allowlist");
    if !allow.human_approved {
        eprintln!("allowlist not human_approved — set human_approved:true after operator gate");
        process::exit(2);
    }

    let out_dir = PathBuf::from(env::temp_dir()).join("trareon-windows-bounded-lab");
    fs::create_dir_all(&out_dir).expect("tmpdir");
    let stem = format!("bounded-{max_bytes}");
    let evidence = out_dir.join(format!("{stem}.raw"));
    let package = out_dir.join(format!("{stem}.fsnap"));
    let _ = fs::remove_file(&evidence);

    let summary = match acquire_file(
        &AcquireRequest::new(&source, &evidence)
            .with_lab_allowlist_path(&allowlist_path)
            .with_max_bytes(max_bytes)
            .with_buffer_size(64 * 1024),
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("WIN_BOUNDED_FAIL {e}");
            eprintln!(
                "hint: run elevated; confirm N from Get-Disk; allowlist must list exact path"
            );
            process::exit(1);
        }
    };

    create_fsnap(&evidence, &summary.audit_path, &package).expect("package");
    let manifest = verify_fsnap(&package).expect("verify");
    println!(
        "WIN_BOUNDED_OK source={} bound={} bytes={} sha={} package={}",
        source.display(),
        max_bytes,
        manifest.evidence_size,
        manifest.evidence_sha256,
        package.display()
    );
    println!("NOTE: bounded sample only; not full-disk acquisition");
}
