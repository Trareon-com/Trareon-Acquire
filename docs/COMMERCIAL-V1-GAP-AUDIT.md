# Commercial v1 — gap audit (Day 2)

Date: 2026-07-17
Last sync: 2026-07-17 (software pack closed on `main` @ post-#75)
Baseline evidence: `docs/platform/m2-lab-tiny11-2311-disk10.md`, Days 23–25 probes

## Summary

| Area | Status | Blocker untuk jual murah 3 OS |
|------|--------|-------------------------------|
| File-backed acquire + verifier | **PASS** | — |
| Slint Guided/Standard/Expert + CoC export | **PASS** | — |
| Commercial drafts (waitlist/Founder/freeze/articles) | **PASS** (drafts) | Live open = human Gate 1/2 |
| macOS raw lab (tiny11) | **PASS** (bounded + full) | Media ke-2 untuk klaim lebar |
| macOS `disk10s1` mounted | **PASS while unmounted** (1 MiB) | Remount still busy historically |
| Windows raw acquire | **NotValidated** | [`WINDOWS-LAB-OPERATOR-PACK.md`](platform/WINDOWS-LAB-OPERATOR-PACK.md) |
| Linux loop acquire | **NotValidated** | Root lab ([`linux-loop-lab.md`](platform/linux-loop-lab.md)) |
| Real elevation helper | **Stub only** | Hari 40+; until then `sudo`/UAC manual |
| UI cancel wired | **PASS** (Slint → `cancel_flag`) | — |
| Signing/notarization | **Deferred** | Zero-cash; unsigned + docs |

## macOS (current)

| Item | State |
|------|-------|
| Allowlist + policy | Implemented |
| Elevated open `rdisk10` | PASS |
| 1 MiB / 64 MiB / full-disk | PASS + verifier |
| Broker stub + allowlist bind | Implemented |
| Helper binary | Not implemented |
| Intel Mac | NotValidated |
| `disk10s1` unmounted bounded sample | **PASS** (1 MiB, SHA `445808af…`) |

## Windows (gap) — collected pack

| Item | State |
|------|-------|
| UAC probe `PhysicalDrive0` | Day 24 PASS (open only) |
| `PhysicalDriveN` content acquire | **Deferred** → operator pack |
| Allowlist for removable USB | Template + inventory/decision request ready |
| Bounded smoke example | `lab_windows_bounded_smoke` ready |
| Split/resume on raw | Not tested (pack Hari 13) |

Do **not** invent Windows PASS. Execute [`platform/WINDOWS-LAB-OPERATOR-PACK.md`](platform/WINDOWS-LAB-OPERATOR-PACK.md).

## Linux (gap)

| Item | State |
|------|-------|
| `loop-control` probe | Day 23 PASS |
| Software prep (example + template + scaffold) | Done |
| Physical loop attach acquire | Human root lab |
| Fault injection portable tests | Implemented in CI |
| Physical disconnect fault | Human lab |

## Commercial / ops (gap)

| Item | State |
|------|-------|
| `COMMERCIAL-V1-SCOPE.md` | Done |
| 90-day daily plan | Done; remaining rows are `[W]` / `[H]` / `[~]` lab |
| Unsigned install guides | Done (`docs/install/`) |
| Build script Founder artifact | Done (`scripts/founder-build.sh`) |
| Fulfillment ledger template | Done (`docs/commercial/fulfillment-ledger.csv`) |
| Payment channel + Gate 1/2 live | Human |
| Demo video / outreach / signing cash | Human |

## Priority order (after software close)

1. Run Windows operator pack on a real Windows host (Hari 8–11 minimum).
2. Human: open Gate 1 waitlist from draft.
3. Linux root loop attach when a Linux host is available.
4. Gate 2 Founder only after payment channel exists.
