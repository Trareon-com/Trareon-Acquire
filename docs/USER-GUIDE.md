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

## In-app help mapping

| UI control | Purpose | Notes |
|---|---|---|
| Lab-use banner | States Engineering Alpha / not production | Always visible |
| Case identity | Operator-only note | **Not** sent to core; not in audit/verify |
| Source path | File-backed synthetic source | No raw-device picker in M0 |
| Output directory | Destination for acquire + package | Must be writable |
| Synthetic confirmation checkbox | Gate before Run | Required |
| Run | Invokes core acquire → package → verify | UI does not invent success |
| Chain of Custody Summary | Displays core-returned status only | Failed / Verified Complete |
| About | Attribution + limitation links | See in-app About section |

## Supported workflow (M0)

1. Build/test locally (`cargo test --workspace --locked`, `npm run build`).
2. Launch the Tauri demo (`npm run tauri --prefix apps/trareon-acquire -- dev`).
3. Acquire a synthetic source; confirm checkbox.
4. Independently verify the package with `trareon-verifier`.

## Not supported yet

- Raw-device / elevated acquisition (Days 23–25).
- UI cancellation of an in-flight acquire.
- Multi-segment split-RAW inside `.fsnap`.
- Signing, release channels, or certification claims.

See [`docs/CAPABILITY-MATRIX-M0.md`](CAPABILITY-MATRIX-M0.md) for the full matrix.
