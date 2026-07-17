# Commercial v1 — gap audit (Day 2)

Date: 2026-07-17
Baseline evidence: `docs/platform/m2-lab-tiny11-2311-disk10.md`, Days 23–25 probes

## Summary

| Area | Status | Blocker untuk jual murah 3 OS |
|------|--------|-------------------------------|
| File-backed acquire + verifier | **PASS** | — |
| macOS raw lab (tiny11) | **PASS** (bounded + full) | Perlu media ke-2 untuk klaim lebar |
| macOS `disk10s1` mounted | **NotValidated** | Operator unmount (Hari 3) |
| Windows raw acquire | **NotValidated** | Adapter + lab USB (Hari 8–14) |
| Linux loop acquire | **NotValidated** | Root lab session (Hari 15–20) |
| Real elevation helper | **Stub only** | Hari 40+; until then `sudo`/UAC manual |
| UI cancel wired | **PASS** (Slint → `cancel_flag`) | Hari 24 closed early |
| Signing/notarization | **Deferred** | Zero-cash; unsigned + docs |
| Waitlist / Founder page | **Not started** | Hari 39–53 |

## macOS (current)

| Item | State |
|------|-------|
| Allowlist + policy | Implemented |
| Elevated open `rdisk10` | PASS |
| 1 MiB / 64 MiB / full-disk | PASS + verifier |
| Broker stub + allowlist bind | Implemented |
| Helper binary | Not implemented |
| Intel Mac | NotValidated |

## Windows (gap)

| Item | State |
|------|-------|
| UAC probe `PhysicalDrive0` | Day 24 PASS (open only) |
| `PhysicalDriveN` content acquire | **Not started** |
| Allowlist for removable USB | Template + Hari 8 inventory/decision request (approval pending) |
| Bounded smoke example | Added `lab_windows_bounded_smoke` (operator elevated) |
| Split/resume on raw | Not tested |

## Linux (gap)

| Item | State |
|------|-------|
| `loop-control` probe | Day 23 PASS |
| Physical loop attach acquire | Not started |
| Fault injection portable tests | Implemented in CI |
| Physical disconnect fault | Not started |

## Commercial / ops (gap)

| Item | State |
|------|-------|
| `COMMERCIAL-V1-SCOPE.md` | Done (Day 1) |
| 90-day daily plan | Done |
| Unsigned install guides | Not started (Hari 33–35) |
| Build script Founder artifact | Not started (Hari 55) |
| Payment + fulfillment ledger | Not started (Hari 51–54) |

## Priority order (next 7 days)

1. Operator Hari 3: `disk10s1` smoke (`./scripts/operator-disk10s1-smoke.sh`)
2. Windows lab inventory fill + allowlist approval (Hari 8–9)
3. Windows elevated open + bounded 1 MiB (Hari 10–11)
4. Linux loop Hari 15–17
