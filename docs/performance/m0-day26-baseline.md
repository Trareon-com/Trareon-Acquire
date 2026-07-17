# M0 Day 26 — Performance baseline

- Generated (UTC): `2026-07-17T01:41:00.579725+00:00`
- Host OS: `macos`
- Arch: `aarch64`
- Rustc (build): see `rust-toolchain.toml` / CI matrix
- Peak RSS: `NotValidated` (no portable in-process sampler; streaming buffer fixed at 1 MiB)
- Scope note: Day 26 entry gate asked for Day 25 review `EXPECTED_PASS`; Days 23–25 remain `MANUAL_START`/`NOT_STARTED`. Baseline still collected for synthetic file-backed paths only.

## Pipeline timings (two consecutive runs per size)

| Size | Run | Acquire (ms) | Package (ms) | Verify (ms) | Throughput acquire (MiB/s) | SHA-256 |
|---:|---:|---:|---:|---:|---:|---|
| 64 KiB | 1 | 11.975 | 17.510 | 2.624 | 5.219 | `7daca2095d043826` |
| 64 KiB | 2 | 8.990 | 14.576 | 2.277 | 6.952 | `7daca2095d043826` |
| _(variance)_ | — | Δ acquire 2.985 ms | Δ package 2.934 ms | Δ verify 0.347 ms | — | match |
| 1024 KiB | 1 | 20.883 | 24.489 | 13.485 | 47.886 | `fbbab289f7f94b25` |
| 1024 KiB | 2 | 20.707 | 24.735 | 13.343 | 48.293 | `fbbab289f7f94b25` |
| _(variance)_ | — | Δ acquire 0.176 ms | Δ package 0.246 ms | Δ verify 0.142 ms | — | match |
| 8192 KiB | 1 | 107.909 | 112.812 | 95.843 | 74.137 | `7d212b9c884f5c77` |
| 8192 KiB | 2 | 125.044 | 142.993 | 94.914 | 63.977 | `7d212b9c884f5c77` |
| _(variance)_ | — | Δ acquire 17.135 ms | Δ package 30.182 ms | Δ verify 0.929 ms | — | match |

## Cancellation latency

- Armed-before-start cancel on 4 MiB synthetic source: **6.816 ms** to `CoreError::Cancelled`.

## Equivalence checks

- Output bytes equal source for every measured size.
- SHA-256 identical across the two consecutive runs per size.
- No optimization changes landed with this baseline.
