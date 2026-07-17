//! Bounded raw-device lab smoke (requires elevated privileges).
//!
//! Does **not** image the whole disk. Default bound: 1 MiB from `/dev/rdisk10`.
//! Output: `/tmp/trareon-raw-bounded-lab/bounded-<max_bytes>.{raw,fsnap}`.
//!
//! ```bash
//! sudo cargo run -p trareon-core --example lab_raw_bounded_smoke -- \
//!   /dev/rdisk10 fixtures/lab-allowlists/tiny11-2311-disk10.json 1048576
//! # optional larger bound (still not full disk):
//! # … 67108864
//! ```

use std::{env, fs, path::PathBuf, process};
use trareon_core::{AcquireRequest, acquire_file, create_fsnap, load_lab_allowlist, verify_fsnap};

fn main() {
    let mut args = env::args().skip(1);
    let source = PathBuf::from(args.next().unwrap_or_else(|| "/dev/rdisk10".into()));
    let allowlist_path = PathBuf::from(
        args.next()
            .unwrap_or_else(|| "fixtures/lab-allowlists/tiny11-2311-disk10.json".into()),
    );
    let max_bytes: u64 = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1024 * 1024);

    let allow = load_lab_allowlist(&allowlist_path).expect("allowlist");
    if !allow.human_approved {
        eprintln!("allowlist not human_approved");
        process::exit(2);
    }

    let out_dir = PathBuf::from("/tmp/trareon-raw-bounded-lab");
    fs::create_dir_all(&out_dir).expect("tmpdir");
    // Bound-specific names so a larger sample does not overwrite a smaller one.
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
            eprintln!("RAW_BOUNDED_FAIL {e}");
            eprintln!("hint: run under sudo if open is DeniedInsufficientPrivilege");
            process::exit(1);
        }
    };

    create_fsnap(&evidence, &summary.audit_path, &package).expect("package");
    let manifest = verify_fsnap(&package).expect("verify");
    println!(
        "RAW_BOUNDED_OK source={} bound={} bytes={} sha={} package={}",
        source.display(),
        max_bytes,
        manifest.evidence_size,
        manifest.evidence_sha256,
        package.display()
    );
    println!("NOTE: this is a bounded sample, not a full-disk acquisition");
}
