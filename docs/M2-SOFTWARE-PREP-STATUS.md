# M2 software prep status

Date: 2026-07-17  
Branch tip: `main` (post PR #59)

## Completed without hardware gate

- Lab allowlist schema + system-disk hard-deny (`lab_policy.rs`)
- Acquire preflight wired to allowlist; block devices require `max_bytes`
- Portable fault-injection tests
- Plans M2-P01…P05 + decision for human allowlist
- Example `lab_raw_bounded_smoke` (bound-specific output names)

## Completed with operator-designated media (`disk10` / tiny11)

- Allowlist + decision + lab report + operator runbook
- File-backed volume smoke PASS
- Elevated open of `disk10`/`rdisk10` PASS
- Bounded raw sample **1 MiB** PASS (SHA `41fb8d9…`) + independent verify
- Bounded raw sample **64 MiB** PASS (SHA `a0ff343…`) + independent verify

## Still blocked / NotValidated (HUMAN_APPROVAL_REQUIRED)

- Unmount + `disk10s1` open/sample
- Full-disk acquire of allowlisted media
- Real OS elevation helper (M2-P04; `StubElevationHelper` remains)
- Writable staging on NTFS volume (RO mount)
- Lab Beta exit / Windows narrow-storage RC entry
- GitHub Actions billing restore for normal PR checks

Operator next (optional): runbook §G unmount + `disk10s1`, or explicit full-disk gate.
