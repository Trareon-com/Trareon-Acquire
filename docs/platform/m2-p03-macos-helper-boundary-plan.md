# M2-P03 — macOS helper boundary plan

Status: **PLAN ONLY** — no FDA claim; Intel Mac remains `NotValidated`.

## Day 25 evidence (bound)

- SIP + Authenticated Root enabled on M4 Pro lab Mac
- Admin user not in `operator`: `/dev/rdisk0` → `DeniedInsufficientPrivilege`
- No content read performed

## Required for any future helper

1. Explicit human gate + allowlist entry (never `rdisk0` system disk)
2. Reviewed helper design (see M2-P04) — not shell, not network
3. Exact OS, arch, SIP state, FDA status recorded
4. Capability matrix stays `NotValidated` until content acquire evidence exists

## Explicitly not claimed

- Full Disk Access as proof of forensic fitness
- Production readiness on macOS
- Intel Mac coverage
