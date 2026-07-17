# M2-P01 — Windows narrow storage lab plan

Status: **PLAN ONLY** — blocked on human allowlist (`DECISIONS/2026-07-17-m2-lab-allowlist-required.md`).

## Objective

Acquire **only** allowlisted removable media on Windows with UAC elevation evidence,
stable source identity, RAW/split-RAW, SHA-256, audit, `.fsnap`, and independent
`trareon-verifier` post-check. Cite Day 24: elevation gates `PhysicalDriveN` opens.

## Forbidden

- `\\.\PhysicalDrive0` and any system disk
- Non-elevated claims of raw access success
- Silent repair of packages

## Procedure (when allowlist approved)

1. Record exact OS build, arch, controller, enclosure, media model/serial (redact in public docs), sector size, privilege (elevated vs not).
2. Load `trareon.lab-allowlist/1` with `human_approved: true`.
3. Non-elevated probe: expect deny (Day 24 pattern).
4. Elevated acquire of allowlisted removable drive only via future adapter (not yet implemented).
5. Post-verify with `trareon-verifier`; import indexes with `trareon-analysis` outside package.
6. Update capability matrix with exact evidence — never promote probe PASS to acquire PASS without this run.

## Software already in place

- System-disk hard-deny + allowlist gate in `lab_policy.rs`
- File-backed split/resume + failure matrix from M1
