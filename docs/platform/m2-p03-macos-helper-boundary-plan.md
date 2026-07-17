# M2-P03 — macOS helper boundary plan

Status: **PLAN + Apple Silicon lab evidence** — no FDA claim; Intel Mac remains `NotValidated`.

## Day 25 evidence (bound)

- SIP + Authenticated Root enabled on M4 Pro lab Mac
- Admin user not in `operator`: `/dev/rdisk0` → `DeniedInsufficientPrivilege`
- No content read performed on system disk

## tiny11 / `disk10` lab ladder (2026-07-17, Apple Silicon operator host)

| Step | Result |
|---|---|
| Unelevated open `disk10`/`rdisk10`/`disk10s1` | `DeniedInsufficientPrivilege` |
| Elevated open `disk10`/`rdisk10` | Available |
| Elevated open mounted `disk10s1` | Resource busy |
| Allowlisted bounded content sample (1 MiB `rdisk10`) | PASS + independent verifier VALID |
| Allowlisted bounded content sample (64 MiB `rdisk10`) | PASS + independent verifier VALID |
| Allowlisted full-disk acquire (`rdisk10` → Untitled) | PASS + independent verifier VALID |

Evidence: `docs/platform/m2-lab-tiny11-2311-disk10.md`, allowlist
`fixtures/lab-allowlists/tiny11-2311-disk10.json`.

## Required for any future helper

1. Explicit human gate + allowlist entry (never `rdisk0` system disk)
2. Reviewed helper design (see M2-P04) — not shell, not network
3. Exact OS, arch, SIP state, FDA status recorded
4. Capability matrix: privilege probe alone stays non-acquire; bounded sample ≠ full-disk
5. Operator `sudo` / `operator` group / reviewed helper — agent cannot enter interactive password

## Explicitly not claimed

- Full Disk Access as proof of forensic fitness
- Production readiness on macOS
- Intel Mac coverage
- Storage Lab Beta exit from a 1 MiB or 64 MiB sample
