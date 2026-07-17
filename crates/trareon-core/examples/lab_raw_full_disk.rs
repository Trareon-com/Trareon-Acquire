//! Full-disk lab acquire for an **already allowlisted** raw device.
//!
//! Requires an explicit `--i-approve-full-disk` flag. Destination must have
//! enough free space (tiny11 / `rdisk10` ≈ 57 GiB → use `/Volumes/Untitled`).
//!
//! Prefer unmounting the volume first so the partition is not busy:
//! `diskutil unmount "/Volumes/tiny11 2311"`
//!
//! ```bash
//! sudo cargo run -p trareon-core --example lab_raw_full_disk -- \
//!   --i-approve-full-disk \
//!   /dev/rdisk10 \
//!   fixtures/lab-allowlists/tiny11-2311-disk10.json \
//!   /Volumes/Untitled/trareon-lab \
//!   61524148224
//! ```

use std::{env, fs, path::PathBuf, process, time::Instant};
use trareon_core::{AcquireRequest, acquire_file, create_fsnap, load_lab_allowlist, verify_fsnap};

fn usage() -> ! {
    eprintln!(
        "usage: lab_raw_full_disk --i-approve-full-disk <source> <allowlist.json> <out_dir> <disk_bytes>"
    );
    process::exit(64);
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    let approved = args
        .iter()
        .position(|a| a == "--i-approve-full-disk")
        .map(|i| {
            args.remove(i);
            true
        })
        .unwrap_or(false);
    if !approved {
        eprintln!("refusing: pass --i-approve-full-disk (human full-disk gate)");
        usage();
    }
    if args.len() != 4 {
        usage();
    }

    let source = PathBuf::from(&args[0]);
    let allowlist_path = PathBuf::from(&args[1]);
    let out_dir = PathBuf::from(&args[2]);
    let disk_bytes: u64 = args[3].parse().unwrap_or_else(|_| usage());

    if disk_bytes < 1024 * 1024 {
        eprintln!("disk_bytes too small: {disk_bytes}");
        process::exit(2);
    }

    let allow = load_lab_allowlist(&allowlist_path).expect("allowlist");
    if !allow.human_approved {
        eprintln!("allowlist not human_approved");
        process::exit(2);
    }

    fs::create_dir_all(&out_dir).expect("out_dir");
    let evidence = out_dir.join("tiny11-rdisk10-full.raw");
    let package = out_dir.join("tiny11-rdisk10-full.fsnap");
    let checkpoint = out_dir.join("tiny11-rdisk10-full.checkpoint.json");

    let resume = evidence.exists() || checkpoint.exists();
    if package.exists() && !resume {
        eprintln!(
            "refusing: package already exists at {} (remove deliberately to re-acquire)",
            package.display()
        );
        process::exit(3);
    }
    if resume {
        eprintln!(
            "RAW_FULL_RESUME existing evidence/checkpoint under {}",
            out_dir.display()
        );
    }

    eprintln!(
        "RAW_FULL_START source={} dest={} bound_bytes={} (~{:.1} GiB) resume={}",
        source.display(),
        evidence.display(),
        disk_bytes,
        disk_bytes as f64 / (1024.0 * 1024.0 * 1024.0),
        resume
    );
    eprintln!("NOTE: this can take a long time; keep the USB connected");

    let started = Instant::now();
    let request = AcquireRequest::new(&source, &evidence)
        .with_lab_allowlist_path(&allowlist_path)
        .with_max_bytes(disk_bytes)
        .with_buffer_size(1024 * 1024)
        .with_checkpoint_path(&checkpoint)
        .with_resume(resume);

    let summary = match acquire_file(&request) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("RAW_FULL_FAIL {e}");
            eprintln!("hint: sudo; unmount volume if Resource busy; re-run to resume");
            process::exit(1);
        }
    };

    if let Err(e) = create_fsnap(&evidence, &summary.audit_path, &package) {
        eprintln!("RAW_FULL_PACKAGE_FAIL {e}");
        process::exit(1);
    }
    let manifest = match verify_fsnap(&package) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("RAW_FULL_VERIFY_FAIL {e}");
            process::exit(1);
        }
    };

    let secs = started.elapsed().as_secs().max(1);
    let mib_s = (manifest.evidence_size as f64 / (1024.0 * 1024.0)) / secs as f64;
    println!(
        "RAW_FULL_OK source={} bound={} bytes={} sha={} package={} elapsed_s={} mib_s={mib_s:.1}",
        source.display(),
        disk_bytes,
        manifest.evidence_size,
        manifest.evidence_sha256,
        package.display(),
        secs
    );
    if manifest.evidence_size != disk_bytes {
        println!(
            "NOTE: bytes_read ({}) != declared disk_bytes ({}) — record as anomaly",
            manifest.evidence_size, disk_bytes
        );
    }
}
