# EWF adoption spike (Wave 3a) — DONE (writer wired)

Date: 2026-07-18  
Workspace MSRV: Rust **1.96** (`rust-toolchain.toml`)  
Decision: **`ewf-image` 0.2** (Apache-2.0) is the Path B writer behind feature `ewf` (default on `trareon-core`).

## What shipped

| Item | Status |
|---|---|
| MSRV bump 1.95 → 1.96 | Done |
| `write_ewf_physical` / `write_e01` → ewf-image | Done (`crates/trareon-core/src/format/ewf.rs`) |
| `write_e01_lite` kept for lite round-trip / smoke | Done |
| Feature `libewf-oracle` (`ewfverify` probe) | Done |
| UI label without `-lite` | **Blocked** until Autopsy/FTK + `ewfverify` green in `EVIDENCE.md` |
| `scripts/ewf-spike.sh` | Still valid offline sandbox |

## Oracle gate (human / lab)

```sh
cargo test -p trareon-core --features ewf --lib format::ewf
# If libewf tools installed:
cargo test -p trareon-core --features libewf-oracle
ewfverify path/to/evidence.E01
```

Fill [EVIDENCE.md](EVIDENCE.md) Path B rows before marketing “E01” without `-lite`.
