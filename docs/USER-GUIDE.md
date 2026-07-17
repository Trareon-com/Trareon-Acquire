# Trareon Acquire — User Guide (M0 skeleton)

Result class: **Engineering Alpha — Lab Use Only**.

This is an operator skeleton for the current file-backed foundation demo.
It is not a production field guide and does not certify forensic fitness.

## Who this is for

Operators running the **synthetic / training** foundation demo on a
workstation, not live evidence acquisition from raw disks.

## Before you start

1. Confirm the source file is synthetic or training data — never real evidence in M0.
2. Read the lab banner in the app and [`docs/LEGAL-LIMITATIONS-DRAFT.md`](LEGAL-LIMITATIONS-DRAFT.md).
3. Prefer the walkthrough in [`docs/FOUNDATION-DEMO.md`](FOUNDATION-DEMO.md).
4. If you received a binary, its GPLv3 corresponding source is the repository
   at the source commit recorded with that build.

## In-app help mapping

| UI control | Purpose | Notes |
|---|---|---|
| Lab-use banner | States Engineering Alpha / not production | Always visible |
| Case identity | Operator-only note | **Not** sent to core; not in audit/verify |
| Source path | File-backed synthetic source | No raw-device picker in M0 |
| Output directory | Destination for acquire + package | Must be writable |
| Synthetic confirmation checkbox | Gate before Run | Required |
| Run | Invokes core acquire → package → verify | UI does not invent success |
| Cancel | Arms cooperative `cancel_flag` for in-flight demo | Shows Cancelled; never Verified Complete |
| Chain of Custody Summary | Displays core-returned status only | Failed / Cancelled / Verified Complete |
| About | Attribution + limitation links | See in-app About section |

## Supported workflow (M0 / early M1)

1. Build/test locally (`cargo test --workspace --locked --exclude acquire-slint`
   and `cargo test -p acquire-slint --features gui --locked`).
2. Launch the Slint demo (`cargo run -p acquire-slint --features gui`).
3. Pick a mode: **Guided** (Fill synthetic), **Standard** (browse paths), or
   **Expert** (raw-path warnings; Run remains file-backed).
4. Acquire a synthetic source; confirm checkbox.
5. Optionally Cancel an in-flight Run (cooperative cancel; incomplete checkpoint may remain on disk for resume tooling).
6. Optionally **Export CoC JSON** (operator note + status — outside crypto audit).
7. Independently verify the package with `trareon-verifier`.
8. Optionally import a verified package into Analysis indexes with `trareon-analysis import PACKAGE --index-dir DIR` (index must be outside the package).

## Not supported yet

- Raw-device / elevated acquisition (Days 23–25 probes only).
- Privileged raw-device broker elevation (protocol + `StubElevationHelper` only; no OS elevate).
- Signing, release channels, or certification claims.

See [`docs/CAPABILITY-MATRIX-M0.md`](CAPABILITY-MATRIX-M0.md) for the full matrix.
See [`docs/M1-MILESTONE-REVIEW.md`](M1-MILESTONE-REVIEW.md) for the file-backed Engineering Alpha exit.
