# Performance baselines (M0)

Day 26 collects **measurement-only** baselines for the file-backed synthetic acquisition path. Do not land optimizations in the same change that establishes the baseline.

## CI smoke

```bash
cargo test -p trareon-core --test performance --locked
```

Exercises 64 KiB and 1 MiB pipelines (two runs each for hash stability), cancel latency, and the 1 MiB streaming-buffer contract.

## Documented report (run twice)

```bash
TRAREON_WRITE_PERF_REPORT=1 cargo test -p trareon-core --test performance --locked -- --ignored --nocapture
```

Run the command **twice**. Each run overwrites `docs/performance/m0-day26-baseline.md` with host timings. Day 26 acceptance requires both runs to exit 0; the committed report is a sample from the author machine (macOS aarch64) and is not a cross-platform SLA.

## NotValidated

- Peak RSS / absolute memory ceiling on hosted runners (no portable sampler added; would need a new dependency or OS-specific code).
- Raw-device / elevated paths (Days 23–25).
- Any performance claim beyond Engineering Alpha synthetic fixtures.
