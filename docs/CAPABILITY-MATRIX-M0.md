# Trareon Acquire — M0 Capability & Limitation Matrix

Result class: **Production-Directed Engineering Alpha Candidate — Lab Use Only**.

This matrix consolidates evidence already present in the repository as of
`main` tip `6594840` (includes Day 23 / PR #47) plus Day 30 human
classification. Capabilities without exact hardware or independent Codex
review evidence remain `NotValidated`. This document does **not** certify
Official Production readiness.

Authoritative day tracking: [`docs/ai-operations/MASTER-CHECKLIST.md`](ai-operations/MASTER-CHECKLIST.md).  
Known gaps: [`docs/WEEK-01-DISCREPANCY-REGISTER.md`](WEEK-01-DISCREPANCY-REGISTER.md).  
Milestone: [`docs/M0-MILESTONE-REVIEW.md`](M0-MILESTONE-REVIEW.md).  
Risks: [`docs/M0-RISK-REGISTER.md`](M0-RISK-REGISTER.md).  
`.fsnap` reader contract: [`docs/fsnap-v0.1-read-contract.md`](fsnap-v0.1-read-contract.md).

## Platform evidence (hosted CI)

| Platform | Compile + unit/integration tests | Security job | Raw-device acquire | Notes |
|---|---|---|---|---|
| ubuntu-latest | PASS | PASS | `NotValidated` (physical disk) | Day 23 Kali: loop/privilege probe only |
| windows-latest | PASS | n/a (security job is Ubuntu) | `NotValidated` | Day 24 `MANUAL_START` |
| macos-latest | PASS | n/a | Privilege probe only | Day 25 M4 Pro: open `/dev/rdisk0` denied; physical acquire still `NotValidated` |

Hosted CI is **portability evidence for the file-backed slice**, not proof of
read-only raw-device acquisition on Windows/macOS.

## Capability matrix

| Capability | Status | Evidence | Limitations |
|---|---|---|---|
| File-backed streaming acquire | Implemented | `trareon-core` acquisition tests; foundation demo | Synthetic/file sources only |
| SHA-256 of acquired bytes | Implemented | Acquisition summary + independent re-hash in verify | — |
| Append-only audit hash chain | Implemented | Audit tests + package verify | — |
| Cooperative cancel (`cancel_flag`) | Implemented (core) | Acquisition / property / performance tests | **No UI path** to arm cancel |
| Split-RAW segment writes | Implemented (core) | Acquisition split tests | **Not packaged** into `.fsnap` yet |
| `.fsnap` v0.1 create/verify | Implemented + **Analysis-frozen** | Package tests + 6 golden fixtures + CLI | Single `evidence.raw` only |
| Independent verifier CLI | Implemented | `trareon-verifier` CLI tests | Exit 0 / 2 only; no repair |
| Guided synthetic UI | Partial | App.svelte CoC card + a11y labels | No formal a11y audit; no cancel control |
| Cross-OS CI matrix | Implemented | GitHub Actions `test` × 3 OS | See Day 20 recovery-cycle note |
| DevSecOps gates | Implemented | `deny.toml`, CI `security` job, `SECURITY.md` | Unmaintained advisories explicitly ignored |
| Bounded property tests | Implemented | `tests/properties.rs` + fuzz corpus docs | Full `cargo-fuzz` `NotValidated` |
| Performance baseline (synthetic) | Implemented | `tests/performance.rs` + `docs/performance/` | Peak RSS `NotValidated`; no optimization |
| Linux raw-device privilege probe | Feasibility spike | Day 23 / PR #47 + `platform.rs` | Physical disk attach/read `NotValidated`; Ubuntu LTS HW not separately recorded |
| macOS raw-device privilege probe | Feasibility spike | Day 25 + `platform::macos::probe_rdisk0` | Open denied without `operator`/helper; Intel Mac `NotValidated`; synthetic `hdiutil` attach `NotValidated` |
| Raw-device Windows | `NotValidated` | None | Day 24 requires lab device |
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
- Raw-device and elevated Windows paths remain `NotValidated`; macOS physical acquire remains `NotValidated` (privilege probe only).
- Any court-admissibility or Official Production claim.

## Independent review status

All Day 01–29 `Review` cells remain `NOT_STARTED`. Closing GitHub issues
for implementation does **not** equal `EXPECTED_PASS` Codex review.
Day 30 human classification is recorded in
[`docs/ai-operations/DECISIONS/2026-07-17-day30-eac-human-approval.md`](ai-operations/DECISIONS/2026-07-17-day30-eac-human-approval.md).
