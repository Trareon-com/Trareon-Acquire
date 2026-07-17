# Trareon Acquire — M0 Capability & Limitation Matrix

Result class: **Production-Directed Engineering Alpha Candidate — Lab Use Only**.

This matrix consolidates evidence as of `main` tip `4dd2828` (Days 01–29
implemented, including Days 23–25 privilege spikes) plus the Day 30 **complete
redo** human classification. Capabilities without exact hardware acquisition
evidence or independent Codex review remain `NotValidated`. This document does
**not** certify Official Production readiness.

Authoritative day tracking: [`docs/ai-operations/MASTER-CHECKLIST.md`](ai-operations/MASTER-CHECKLIST.md).  
Known gaps: [`docs/WEEK-01-DISCREPANCY-REGISTER.md`](WEEK-01-DISCREPANCY-REGISTER.md).  
Milestone: [`docs/M0-MILESTONE-REVIEW.md`](M0-MILESTONE-REVIEW.md).  
Risks: [`docs/M0-RISK-REGISTER.md`](M0-RISK-REGISTER.md).  
`.fsnap` reader contract: [`docs/fsnap-v0.1-read-contract.md`](fsnap-v0.1-read-contract.md).

## Platform evidence (hosted CI + lab)

| Platform | Compile + unit/integration tests | Security job | Raw-device acquire | Privilege probe |
|---|---|---|---|---|
| ubuntu-latest / Kali lab | PASS (CI); Kali HW Day 23 | PASS (CI) | `NotValidated` (physical disk) | Day 23: loop-control / `losetup` group boundary |
| windows-latest / Win10 lab | PASS (CI); Win10 HW Day 24 | n/a on Windows job | `NotValidated` (content read) | Day 24: UAC elevation gates `PhysicalDrive0` |
| macos-latest / M4 Pro lab | PASS (CI); M4 Pro HW Day 25 | n/a | `NotValidated` (content read) | Day 25: `/dev/rdisk0` denied without `operator`/helper |

Hosted CI is **portability evidence for the file-backed slice**. Days 23–25
validate privilege boundaries only — not production raw-device adapters.

## Capability matrix

| Capability | Status | Evidence | Limitations |
|---|---|---|---|
| File-backed streaming acquire | Implemented | `trareon-core` acquisition tests; foundation demo | Synthetic/file sources only |
| SHA-256 of acquired bytes | Implemented | Acquisition summary + independent re-hash in verify | — |
| Append-only audit hash chain | Implemented | Audit tests + package verify | — |
| Cooperative cancel (`cancel_flag`) | Implemented (core + UI) | Acquisition / property / Tauri cancel tests | Resume of cancelled file-backed acquire is M1 (non-split) |
| Checkpoint / resume (file-backed) | Implemented (non-split + split-RAW) | `checkpoint.rs` + acquisition resume tests | Physical-media resume still M2 |
| Split-RAW segment writes | Implemented (core + package) | Acquisition + `create_fsnap_from_segments` | Optional `evidence_segments`; single-file Analysis goldens unchanged |
| `.fsnap` v0.1 create/verify | Implemented + **Analysis-frozen** | Package tests + 6 golden fixtures + CLI | Single `evidence.raw` only |
| Independent verifier CLI | Implemented | `trareon-verifier` CLI tests | Exit 0 / 2 only; no repair |
| Analysis read-only importer | Implemented (M1 exit) | `trareon-analysis` covers all 6 goldens + immutability | Indexes outside package; no repair/upgrade |
| Guided synthetic UI | Partial | App.svelte CoC + Cancel + a11y labels | Manual a11y checklist PASS; automated scanner open |
| Cross-OS CI matrix | Implemented | GitHub Actions `test` × 3 OS | See Day 20 recovery-cycle note |
| DevSecOps gates | Implemented | `deny.toml`, CI `security` job, `SECURITY.md` | Unmaintained advisories explicitly ignored |
| Bounded property tests | Implemented | `tests/properties.rs` + fuzz corpus docs | Full `cargo-fuzz` `NotValidated` |
| Performance baseline (synthetic) | Implemented | `tests/performance.rs` + `docs/performance/` | Peak RSS `NotValidated`; no optimization |
| Linux raw-device privilege probe | Feasibility spike | Day 23 / PR #47 + `platform::linux` | Physical disk attach/read `NotValidated`; Ubuntu LTS HW not separately recorded |
| Windows raw-device UAC probe | Feasibility spike | Day 24 / PR #49 + `platform::windows` | Non-elevated denied; content read `NotValidated`; X270 `NotValidated` |
| macOS raw-device privilege probe | Feasibility spike | Day 25 / PR #50 + `platform::macos` | Open denied without `operator`/helper; Intel/`hdiutil`/`FDA` `NotValidated` |
| macOS allowlisted bounded raw sample | Lab smoke PASS | `disk10`/`rdisk10` tiny11 USB; 1 MiB `max_bytes`; report `m2-lab-tiny11-2311-disk10.md` | Full-disk NotValidated; NTFS volume RO; not Lab Beta exit |
| Privileged broker protocol | Spike only | `broker.rs` + `StubElevationHelper` | Helper trait exists; OS elevation still `NotImplemented` |
| Lab source allowlist / system-disk deny | Implemented (M2 prep) | `lab_policy.rs` + fault_injection tests | Human-approved allowlist required for block/raw |
| Signing / release / certification | Out of scope | Day 30 human gate | Lab Use Only; no Official Production |

## `.fsnap` v0.1 Analysis freeze

**Status:** **frozen for Analysis** (Engineering Alpha / Lab Use Only) for the
**file-backed, single-`evidence.raw`** layout, contingent on:

1. Golden fixtures under `fixtures/fsnap-v0.1/` remaining byte-stable (`-text` in `.gitattributes`).
2. `docs/fsnap-v0.1-read-contract.md` remaining the fail-closed reader contract.
3. No silent-repair behavior in `verify_fsnap` / `trareon-verifier`.
4. Explicit rejection of unsupported `manifest.schema` values.

**Still out of freeze scope (not production format guarantees):**

- Split-RAW multi-segment packaging.
- Independent Codex review for Days 01–29 still `NOT_STARTED`.
- Raw-device *acquisition* remains `NotValidated` (privilege probes ≠ acquire adapters).
- Any court-admissibility or Official Production claim.

## Independent review status

Day 01–29 `Review` cells set to `EXPECTED_PASS` via Cursor substitute review (`docs/INDEPENDENT-REVIEW-M0-CURSOR.md`); Codex review still not performed. Closing GitHub issues
for implementation does **not** equal `EXPECTED_PASS` Codex review.
Day 30 complete-redo classification is recorded in
[`docs/ai-operations/DECISIONS/2026-07-17-day30-eac-complete-redo.md`](ai-operations/DECISIONS/2026-07-17-day30-eac-complete-redo.md).
