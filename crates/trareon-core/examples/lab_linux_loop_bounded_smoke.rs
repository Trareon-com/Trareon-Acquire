//! Bounded Linux loop-device lab smoke (requires root / disk group + allowlist).
//!
//! Attach a **synthetic** image read-only first (`losetup -r`). Never use the
//! system disk (`nvme0n1`, `sda`, …).
//!
//! ```bash
//! # After HUMAN_APPROVAL of a media-specific allowlist:
//! cargo run -p trareon-core --example lab_linux_loop_bounded_smoke -- \
//!   /dev/loopN fixtures/lab-allowlists/your-linux-allowlist.json 1048576
//! ```

use std::{env, fs, path::PathBuf, process};
use trareon_core::{AcquireRequest, acquire_file, create_fsnap, load_lab_allowlist, verify_fsnap};

fn main() {
    let mut args = env::args().skip(1);
    let source = PathBuf::from(args.next().unwrap_or_else(|| "/dev/loop0".into()));
    let allowlist_path = PathBuf::from(
        args.next()
            .unwrap_or_else(|| "fixtures/lab-allowlists/linux-loop-template.json".into()),
    );
    let max_bytes: u64 = args
        .next()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1024 * 1024);

    let source_str = source.to_string_lossy().to_ascii_lowercase();
    if source_str.contains("nvme0n1")
        || source_str == "/dev/sda"
        || source_str.starts_with("/dev/sda")
        || source_str == "/dev/disk0"
        || source_str == "/dev/rdisk0"
    {
        eprintln!("refusing: system-disk-like path is hard-denied");
        process::exit(2);
    }
    if !source_str.contains("/dev/loop") {
        eprintln!("refusing: this example is for /dev/loopN only");
        process::exit(2);
    }

    let allow = load_lab_allowlist(&allowlist_path).expect("allowlist");
    if !allow.human_approved {
        eprintln!("allowlist not human_approved — set human_approved:true after operator gate");
        process::exit(2);
    }

    let out_dir = env::temp_dir().join("trareon-linux-loop-bounded-lab");
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
            eprintln!("LINUX_LOOP_BOUNDED_FAIL {e}");
            eprintln!("hint: root/disk group; RO losetup; allowlist must list exact path");
            process::exit(1);
        }
    };

    create_fsnap(&evidence, &summary.audit_path, &package).expect("package");
    let manifest = verify_fsnap(&package).expect("verify");
    println!(
        "LINUX_LOOP_BOUNDED_OK source={} bound={} bytes={} sha={} package={}",
        source.display(),
        max_bytes,
        manifest.evidence_size,
        manifest.evidence_sha256,
        package.display()
    );
    println!("NOTE: bounded sample only; not full-disk acquisition");
}
