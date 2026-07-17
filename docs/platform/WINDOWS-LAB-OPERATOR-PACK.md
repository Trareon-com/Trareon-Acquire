# Windows Lab Operator Pack (deferred)

Status: **COLLECTED FOR LATER** — do not claim Windows raw PASS until items below are executed on a Windows lab host.
Created: 2026-07-17
Parent plans: [`m2-p01-windows-narrow-storage-plan.md`](m2-p01-windows-narrow-storage-plan.md),
[`windows-lab-inventory.md`](windows-lab-inventory.md),
[`../ai-operations/DECISIONS/2026-07-17-windows-lab-media-decision-request.md`](../ai-operations/DECISIONS/2026-07-17-windows-lab-media-decision-request.md)

This pack consolidates commercial Hari **8–14**, **22**, **64**, and **88** so Mac-side
software work can finish without blocking on the Windows machine.

## Already ready on `main` (no Windows host required)

| Asset | Path |
|-------|------|
| Inventory scaffold | `docs/platform/windows-lab-inventory.md` |
| Media decision request | `docs/ai-operations/DECISIONS/2026-07-17-windows-lab-media-decision-request.md` |
| Allowlist template | `fixtures/lab-allowlists/windows-usb-template.json` |
| Bounded smoke example | `cargo run -p trareon-core --example lab_windows_bounded_smoke` |
| Broker hard-deny | `PhysicalDrive0` CI tests |
| Unsigned install draft | `docs/install/windows-unsigned.md` |

## Operator sequence (run only on Windows lab)

1. **Hari 8** — Fill inventory table; confirm removable `PhysicalDriveN` ≠ system/boot.
2. **Hari 8–9** — Accept decision request (Option A); copy template →
   `fixtures/lab-allowlists/windows-usb-<id>.json` with `human_approved: true`.
3. **Hari 10** — Non-elevated open → Access Denied; elevated open-only → Available.
4. **Hari 11** — Bounded 1 MiB + `trareon-verifier`:
   ```powershell
   cargo run -p trareon-core --example lab_windows_bounded_smoke -- `
     \\.\PhysicalDriveN fixtures/lab-allowlists/windows-usb-<id>.json 1048576
   cargo run -q -p trareon-verifier -- verify $env:TEMP\trareon-windows-bounded-lab\bounded-1048576.fsnap
   ```
5. **Hari 12** — Full-disk USB → **different** staging volume + new decision (never system disk).
6. **Hari 13** — Split-RAW + resume on that media.
7. **Hari 14** — Write `docs/platform/windows-lab-acquire.md` with exact SHA/size only.
8. **Hari 22** — Cancel mid-acquire on Windows (cooperative `cancel_flag`).
9. **Hari 64 / 88** — Second media / Win11 24H2 smoke if different build.

## Recording rules

- Never copy macOS tiny11 SHA into Windows matrix cells.
- Never allowlist `PhysicalDrive0`.
- Never commit evidence bytes.
- Update `docs/COMMERCIAL-LAUNCH-STATUS.md` and capability matrix only after verifier `VALID`.

## Exit of this pack

Pack is **closed** when Hari 11 minimum (bounded + verify) is recorded with Windows-only evidence.
