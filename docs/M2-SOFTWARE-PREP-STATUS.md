# M2 software prep status

Date: 2026-07-17  
Branch tip: in progress (broker allowlist binding + plan depth)

## Completed without hardware gate

- Lab allowlist schema + system-disk hard-deny (`lab_policy.rs`)
- Acquire preflight wired to allowlist; block devices require `max_bytes`
- Portable fault-injection tests (incl. broker identity gates)
- Plans M2-P01…P05 + decisions for allowlist / full-disk→Untitled
- Examples: `lab_raw_bounded_smoke`, `lab_raw_full_disk`
- Broker: allowlist-bound evaluation; `StubElevationHelper` still never elevates

## Completed with operator-designated media (`disk10` / tiny11)

- Allowlist + decision + lab report + operator runbook
- File-backed volume smoke PASS
- Elevated open of `disk10`/`rdisk10` PASS
- Bounded raw sample **1 MiB** PASS (SHA `41fb8d9…`) + independent verify
- Bounded raw sample **64 MiB** PASS (SHA `a0ff343…`) + independent verify
- Full-disk **PASS** (SHA `23e039c…`, 61524148224 bytes) + independent verify on Untitled

## Still blocked / NotValidated (HUMAN_APPROVAL_REQUIRED)

- Unmount + `disk10s1` open/sample (optional)
- Real OS elevation helper (M2-P04; stub remains)
- Windows narrow-storage lab host + media allowlist (M2-P01)
- Linux physical loop fault session (M2-P02)
- Writable staging on NTFS volume (RO mount)
- Lab Beta exit / Windows RC entry
- GitHub Actions billing restore for normal PR checks
