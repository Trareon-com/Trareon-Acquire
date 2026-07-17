//! Day 26 performance baseline — measure, do not optimize.
//!
//! Resource bounds for the CI smoke path:
//! - Synthetic sizes: 64 KiB and 1 MiB only (hosted CI must stay fast).
//! - Larger local sizes (8 MiB) are exercised by the ignored report writer.
//! - Peak RSS remains `NotValidated` here: no portable in-process sampler is
//!   added (would need a new dependency or OS-specific code). Memory safety
//!   for M0 is asserted via the streaming buffer contract instead.
//!
//! Run the documented report path twice (Day 26 acceptance):
//! ```text
//! TRAREON_WRITE_PERF_REPORT=1 cargo test -p trareon-core --test performance --locked -- --ignored --nocapture
//! ```

use std::{
    fs,
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use chrono::Utc;
use tempfile::tempdir;
use trareon_core::{
    AcquireRequest, AcquisitionState, CoreError, acquire_file, create_fsnap, verify_fsnap,
};

const KIB: usize = 1024;
const MIB: usize = 1024 * KIB;

#[derive(Clone, Debug)]
struct PhaseTiming {
    acquire_ms: f64,
    package_ms: f64,
    verify_ms: f64,
    sha256: String,
    bytes: u64,
}

fn duration_ms(d: Duration) -> f64 {
    d.as_secs_f64() * 1000.0
}

fn fill_pattern(len: usize) -> Vec<u8> {
    (0u8..=255).cycle().take(len).collect()
}

fn run_pipeline(bytes: &[u8]) -> PhaseTiming {
    let dir = tempdir().expect("tempdir");
    let source = dir.path().join("source.img");
    let raw = dir.path().join("evidence.raw");
    let package = dir.path().join("case.fsnap");
    fs::write(&source, bytes).expect("write source");

    let t0 = Instant::now();
    let summary = acquire_file(&AcquireRequest::new(&source, &raw)).expect("acquire");
    let acquire_ms = duration_ms(t0.elapsed());
    assert_eq!(summary.state, AcquisitionState::AcquiredUnverified);
    assert_eq!(fs::read(&raw).expect("read raw"), bytes);

    let t1 = Instant::now();
    create_fsnap(&raw, &summary.audit_path, &package).expect("package");
    let package_ms = duration_ms(t1.elapsed());

    let t2 = Instant::now();
    let manifest = verify_fsnap(&package).expect("verify");
    let verify_ms = duration_ms(t2.elapsed());
    assert_eq!(manifest.evidence_sha256, summary.sha256);

    PhaseTiming {
        acquire_ms,
        package_ms,
        verify_ms,
        sha256: summary.sha256,
        bytes: summary.bytes_written,
    }
}

fn measure_cancel_latency(source_bytes: usize) -> f64 {
    let dir = tempdir().expect("tempdir");
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    fs::write(&source, fill_pattern(source_bytes)).expect("write source");

    let flag = Arc::new(AtomicBool::new(false));
    let request = AcquireRequest::new(&source, &output).with_cancel_flag(Arc::clone(&flag));

    // Arm cancel before the first read so cancellation is cooperative and immediate.
    flag.store(true, Ordering::SeqCst);
    let t0 = Instant::now();
    let err = acquire_file(&request).expect_err("must cancel");
    let cancel_ms = duration_ms(t0.elapsed());
    assert!(matches!(err, CoreError::Cancelled));
    cancel_ms
}

#[test]
fn performance_baseline_ci_sizes_are_byte_hash_stable_across_two_runs() {
    for &size in &[64 * KIB, MIB] {
        let bytes = fill_pattern(size);
        let a = run_pipeline(&bytes);
        let b = run_pipeline(&bytes);
        assert_eq!(a.bytes, size as u64);
        assert_eq!(a.sha256, b.sha256, "hash must match across repeated runs");

        // Soft sanity: timings are finite and non-negative (no optimization gate).
        assert!(a.acquire_ms >= 0.0 && b.acquire_ms >= 0.0);
        assert!(a.package_ms >= 0.0 && a.verify_ms >= 0.0);
    }
}

#[test]
fn performance_cancel_latency_is_measurable_and_never_false_complete() {
    let cancel_ms = measure_cancel_latency(4 * MIB);
    assert!(
        cancel_ms < 5_000.0,
        "cancel should return promptly on a synthetic 4 MiB source; got {cancel_ms} ms"
    );
}

#[test]
fn performance_streaming_buffer_stays_bounded() {
    // Contract check: AcquireRequest default buffer is 1 MiB and must not grow with source size.
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output = dir.path().join("evidence.raw");
    fs::write(&source, fill_pattern(8 * MIB)).unwrap();
    let request = AcquireRequest::new(&source, &output);
    assert_eq!(
        request.buffer_size, MIB,
        "default streaming buffer must stay 1 MiB (memory-bounded acquire)"
    );
    let summary = acquire_file(&request).unwrap();
    assert_eq!(summary.bytes_written, (8 * MIB) as u64);
}

fn write_report(path: &Path, rows: &[(usize, PhaseTiming, PhaseTiming)], cancel_ms: f64) {
    let mut out = String::new();
    out.push_str("# M0 Day 26 — Performance baseline\n\n");
    out.push_str(&format!(
        "- Generated (UTC): `{}`\n",
        Utc::now().to_rfc3339()
    ));
    out.push_str(&format!("- Host OS: `{}`\n", std::env::consts::OS));
    out.push_str(&format!("- Arch: `{}`\n", std::env::consts::ARCH));
    out.push_str("- Rustc (build): see `rust-toolchain.toml` / CI matrix\n");
    out.push_str(
        "- Peak RSS: `NotValidated` (no portable in-process sampler; streaming buffer fixed at 1 MiB)\n",
    );
    out.push_str(
        "- Scope note: Day 26 entry gate asked for Day 25 review `EXPECTED_PASS`; Days 23–25 remain `MANUAL_START`/`NOT_STARTED`. Baseline still collected for synthetic file-backed paths only.\n\n",
    );
    out.push_str("## Pipeline timings (two consecutive runs per size)\n\n");
    out.push_str(
        "| Size | Run | Acquire (ms) | Package (ms) | Verify (ms) | Throughput acquire (MiB/s) | SHA-256 |\n",
    );
    out.push_str("|---:|---:|---:|---:|---:|---:|---|\n");
    for (size, a, b) in rows {
        for (run, t) in [(1, a), (2, b)] {
            let mib = *size as f64 / MIB as f64;
            let thr = if t.acquire_ms > 0.0 {
                mib / (t.acquire_ms / 1000.0)
            } else {
                f64::INFINITY
            };
            out.push_str(&format!(
                "| {} KiB | {} | {:.3} | {:.3} | {:.3} | {:.3} | `{}` |\n",
                size / KIB,
                run,
                t.acquire_ms,
                t.package_ms,
                t.verify_ms,
                thr,
                &t.sha256[..16]
            ));
        }
        assert_eq!(a.sha256, b.sha256);
        let var_acquire = (a.acquire_ms - b.acquire_ms).abs();
        out.push_str(&format!(
            "| _(variance)_ | — | Δ acquire {:.3} ms | Δ package {:.3} ms | Δ verify {:.3} ms | — | match |\n",
            var_acquire,
            (a.package_ms - b.package_ms).abs(),
            (a.verify_ms - b.verify_ms).abs()
        ));
    }
    out.push_str(&format!(
        "\n## Cancellation latency\n\n- Armed-before-start cancel on 4 MiB synthetic source: **{cancel_ms:.3} ms** to `CoreError::Cancelled`.\n"
    ));
    out.push_str(
        "\n## Equivalence checks\n\n- Output bytes equal source for every measured size.\n- SHA-256 identical across the two consecutive runs per size.\n- No optimization changes landed with this baseline.\n",
    );
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("create report dir");
    }
    fs::write(path, out).expect("write report");
}

#[test]
#[ignore = "writes docs/performance/m0-day26-baseline.md; run twice per Day 26 acceptance"]
fn performance_baseline_write_documented_report() {
    let sizes = [64 * KIB, MIB, 8 * MIB];
    let mut rows = Vec::new();
    for size in sizes {
        let bytes = fill_pattern(size);
        let a = run_pipeline(&bytes);
        let b = run_pipeline(&bytes);
        assert_eq!(a.sha256, b.sha256);
        rows.push((size, a, b));
    }
    let cancel_ms = measure_cancel_latency(4 * MIB);

    let report =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/performance/m0-day26-baseline.md");

    if std::env::var_os("TRAREON_WRITE_PERF_REPORT").is_some() {
        write_report(&report, &rows, cancel_ms);
        eprintln!("wrote {}", report.display());
    } else {
        eprintln!(
            "skip writing report (set TRAREON_WRITE_PERF_REPORT=1); cancel_ms={cancel_ms:.3}"
        );
    }
}
