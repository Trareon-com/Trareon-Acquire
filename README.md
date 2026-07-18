# Trareon Acquire

**Digital forensic acquisition for the lab bench** — custody-first, honest about limits, open to verify.

[![CI](https://github.com/Trareon-com/Trareon-Acquire/actions/workflows/ci.yml/badge.svg)](https://github.com/Trareon-com/Trareon-Acquire/actions/workflows/ci.yml)
[![License: GPL-3.0](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

Trareon Acquire is a multi-OS acquisition workstation (Slint + Rust) aimed at the same job as FTK Imager / Guymager / Magnet Acquire: **identify → acquire → seal → verify**, with native EN/ID copy and an open verifier.

> **Lab use / Engineering Alpha.** Builds are **UNSIGNED**. Do not claim production evidence fitness until live-gate and interop rows in the capability matrix are green.

![Acquire bench](docs/media/screenshots/01-acquire-bench.png)

## Why Trareon

| Strength | What you get |
|---|---|
| Honesty UX | `UNSIGNED` banner, **E01-lite** until libewf/Autopsy evidence, write-blocker “not detected” until confirmed |
| Court Path A | RAW / `.fsnap` + SHA-256 seal + CoC/QR — open as raw in Autopsy/FTK |
| Path B (EWF) | Real EWF1 writer via `ewf-image` (MSRV 1.96); UI stays **E01-lite** until oracle green |
| Open verify | `trareon-verifier` + in-app Tools (verify / hash / compare / export) |
| Operator workflow | Cases → Identify → Acquire → Triage/Tools/QMS/Boot — ISO/IEC 27037 language |

Full walkthrough: **[docs/tutorials/OPERATOR-TUTORIAL.md](docs/tutorials/OPERATOR-TUTORIAL.md)**

## Screenshots

| Acquire | Cases | Identify |
|:---:|:---:|:---:|
| ![Acquire](docs/media/screenshots/01-acquire-bench.png) | ![Cases](docs/media/screenshots/02-cases.png) | ![Identify](docs/media/screenshots/03-identify.png) |
| **Telemetry** | **Seal / CoC** | **Tools** |
| ![Telemetry](docs/media/screenshots/04-telemetry-coverage.png) | ![Seal](docs/media/screenshots/05-seal-coc.png) | ![Tools](docs/media/screenshots/06-tools.png) |
| **Triage** | **QMS / Boot** | **Help** |
| ![Triage](docs/media/screenshots/07-triage-analysis.png) | ![QMS](docs/media/screenshots/08-qms-boot.png) | ![Help](docs/media/screenshots/09-help.png) |

## Quick start

**Requirements:** Rust **1.96** ([`rust-toolchain.toml`](rust-toolchain.toml)). Prefer rustup (`~/.cargo/bin` ahead of Homebrew on macOS).

```bash
# Tests
cargo test --workspace --locked --exclude acquire-slint
cargo test -p acquire-slint --features gui --locked

# Launch desktop app
cargo run -p acquire-slint --features gui
```

Guided mode: **Load synthetic demo** → confirm → **Start acquire** → verify with:

```bash
cargo run -p trareon-verifier --locked -- verify "$TMPDIR/trareon-acquire-slint-demo/foundation.fsnap"
```

## Workflow at a glance

1. **Cases** — create case + examiner (custody start).
2. **Identify** — power / network / encryption / OoV checklist; save beside the case.
3. **Acquire** — pick disk or file, format (`.fsnap` default), write-blocker confirm, run with MiB/s + ETA + coverage map.
4. **Seal** — SHA-256 (+ optional SHA-512), CoC JSON + QR.
5. **Tools** — independent verify / compare / export.
6. **Triage** — read-only Analysis lite timeline (field triage, not a full exam suite).

## Repository layout

```
crates/trareon-core/       # Acquire, .fsnap, EWF/e01-lite, CoC, broker
crates/trareon-verifier/   # Independent CLI verifier
crates/trareon-analysis/   # Read-only Analysis lite
crates/trareon-ata/        # Write-blocker / HPA-DCO probes
apps/acquire-slint/        # Primary desktop UI (Slint)
docs/tutorials/            # Full operator tutorial
docs/media/screenshots/    # UI surfaces for this README
docs/format-interop/       # Path A RAW + Path B EWF evidence packs
```

## Documentation

| Doc | Purpose |
|---|---|
| [Operator tutorial](docs/tutorials/OPERATOR-TUTORIAL.md) | Step-by-step every surface |
| [User guide](docs/USER-GUIDE.md) | Short operator reference |
| [Capability matrix](docs/CAPABILITY-MATRIX-M0.md) | What is implemented vs NotValidated |
| [Format interop](docs/format-interop/README.md) | RAW court path + EWF spike/oracle |
| [Live-gate checklist](docs/live-gate-checklist.md) | Human lab gates (AI does not check these) |
| [Security](SECURITY.md) | Vulnerability reporting |

## Honest limits

- Live raw-disk / ATA HPA passthrough / hardware write-blocker VID enum still need **human live-gate** evidence.
- Marketing must match the capability matrix — no “E01” without `-lite` until `EVIDENCE.md` Path B is green.
- Analysis deep dive (Autopsy/AXIOM-class) is **out of scope**; use a separate Lab product later.

## License

[GPL-3.0-only](LICENSE). Binaries may be sold; corresponding source must be provided.

**Author:** Yusuf Shalahuddin Al Ayyubi As Sobari
