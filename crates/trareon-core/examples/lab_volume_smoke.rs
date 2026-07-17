//! Lab smoke against an operator-designated removable volume.
//!
//! Usage:
//!   cargo run -p trareon-core --example lab_volume_smoke -- \
//!     "/Volumes/tiny11 2311" fixtures/lab-allowlists/tiny11-2311-disk10.json
//!
//! Does **not** open `/dev/rdisk*` for content. File-backed only.

use std::{env, fs, path::PathBuf};
use trareon_core::{
    AcquireRequest, acquire_file, assert_source_permitted, create_fsnap, load_lab_allowlist,
    verify_fsnap,
};

fn main() {
    let mut args = env::args().skip(1);
    let volume = PathBuf::from(args.next().unwrap_or_else(|| "/Volumes/tiny11 2311".into()));
    let allowlist_path = PathBuf::from(
        args.next()
            .unwrap_or_else(|| "fixtures/lab-allowlists/tiny11-2311-disk10.json".into()),
    );
    let allow = load_lab_allowlist(&allowlist_path).expect("allowlist");
    assert!(
        allow.human_approved,
        "refuse lab smoke without human_approved allowlist"
    );

    println!("allowlist_approved_by={}", allow.approved_by);

    match assert_source_permitted(&volume, Some(&allow)) {
        Ok(id) => println!("volume_policy OK kind={:?} digest={}", id.kind, id.identity_digest),
        Err(e) => println!("volume_policy_expected_non_file: {e}"),
    }

    for raw in ["/dev/disk10", "/dev/rdisk10", "/dev/disk10s1"] {
        match assert_source_permitted(std::path::Path::new(raw), Some(&allow)) {
            Ok(id) => println!("raw_policy OK {raw} kind={:?}", id.kind),
            Err(e) => println!("raw_policy {raw}: {e}"),
        }
    }

    // Prefer a tiny existing file on the volume (NTFS often mounts read-only on macOS).
    let source = volume.join("autorun.inf");
    let out_dir = PathBuf::from("/tmp/trareon-tiny11-lab");
    fs::create_dir_all(&out_dir).expect("tmpdir");
    let evidence = out_dir.join("evidence.raw");
    let package = out_dir.join("lab.fsnap");
    let _ = fs::remove_file(&evidence);

    let summary = acquire_file(
        &AcquireRequest::new(&source, &evidence).with_lab_allowlist_path(&allowlist_path),
    )
    .expect("file-backed acquire from lab volume file");
    create_fsnap(&evidence, &summary.audit_path, &package).expect("package");
    let manifest = verify_fsnap(&package).expect("verify");
    println!(
        "LAB_SMOKE_OK source={} bytes={} sha={} package={}",
        source.display(),
        manifest.evidence_size,
        manifest.evidence_sha256,
        package.display()
    );
    println!("RAW_DEVICE_ACQUIRE=NotValidated (no content read of raw device nodes)");
}
