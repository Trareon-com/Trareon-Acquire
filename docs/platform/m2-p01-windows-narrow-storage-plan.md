# M2-P01 — Windows narrow storage lab plan

Status: **PLAN ONLY** — needs Windows host + human allowlist for **that** removable media.
macOS tiny11 evidence does **not** promote Windows rows.

## Objective

Acquire **only** allowlisted removable media on Windows with UAC elevation evidence,
stable source identity, RAW/split-RAW, SHA-256, audit, `.fsnap`, and independent
`trareon-verifier` post-check. Cite Day 24: elevation gates `PhysicalDriveN` opens.

## Forbidden

- `\\.\PhysicalDrive0` and any system disk
- Non-elevated claims of raw access success
- Silent repair of packages
- Copying macOS SHA evidence into Windows matrix cells

## Software already in place

- System-disk hard-deny + allowlist gate in `lab_policy.rs`
- Broker deny for `PhysicalDrive0` / require allowlist for other `PhysicalDriveN`
- File-backed split/resume + failure matrix from M1
- `max_bytes` required for block-device acquires (full-disk only with explicit size/gate)

## Operator checklist (when Windows lab media is approved)

1. Record exact OS build, arch, controller, enclosure, media model/serial (redact public), sector size, elevated vs not.
2. Author `fixtures/lab-allowlists/<media>.json` with `human_approved: true` and decision file.
3. Non-elevated probe of `\\.\PhysicalDriveN` → expect Access Denied (Day 24 pattern).
4. Elevated probe open-only (no content read) → Available.
5. Bounded sample first (`max_bytes` 1 MiB / 64 MiB) via future Windows example or `acquire_file`.
6. Full-disk only with destination on a **different** volume with enough free space + new decision.
7. `trareon-verifier verify`; optional `trareon-analysis` import outside package.
8. Update capability matrix with exact Windows evidence only.

## Still missing (implementation)

- Windows-oriented lab example (parity with `lab_raw_bounded_smoke` / `lab_raw_full_disk`)
- Real UAC elevation helper (M2-P04)
- ThinkPad X270 / alternate hardware evidence (Day 24 noted `NotValidated`)
