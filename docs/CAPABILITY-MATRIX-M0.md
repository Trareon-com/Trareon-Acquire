# Trareon Acquire — M0 Capability & Limitation Matrix

Result class: **Engineering Alpha — Lab Use Only**.

This matrix consolidates evidence already present in the repository as of
merge commit `9e24b15` (Day 26 / PR #43 on `main`). Capabilities without
exact hardware or independent Codex review evidence are marked
`NotValidated`. This document does **not** certify production readiness.

Authoritative day tracking: [`docs/ai-operations/MASTER-CHECKLIST.md`](ai-operations/MASTER-CHECKLIST.md).  
Known gaps: [`docs/WEEK-01-DISCREPANCY-REGISTER.md`](WEEK-01-DISCREPANCY-REGISTER.md).  
`.fsnap` reader contract: [`docs/fsnap-v0.1-read-contract.md`](fsnap-v0.1-read-contract.md).

## Platform evidence (hosted CI)

| Platform | Compile + unit/integration tests | Security job | Raw-device acquire | Notes |
|---|---|---|---|---|
| ubuntu-latest | PASS (CI on merged foundation / Day 21–26 PRs) | PASS | `NotValidated` | Day 23 `MANUAL_START` |
| windows-latest | PASS | n/a (security job is Ubuntu) | `NotValidated` | Day 24 `MANUAL_START` |
| macos-latest | PASS | n/a | `NotValidated` | Day 25 `MANUAL_START`; Homebrew `aws/tap` untapped in CI |

Hosted CI is **portability evidence for the file-backed slice**, not proof of
read-only raw-device acquisition.

## Capability matrix

| Capability | Status | Evidence | Limitations |
|---|---|---|---|
| File-backed streaming acquire | Implemented | `trareon-core` acquisition tests; foundation demo | Synthetic/file sources only |
| SHA-256 of acquired bytes | Implemented | Acquisition summary + independent re-hash in verify | — |
| Append-only audit hash chain | Implemented | Audit tests + package verify | — |
| Cooperative cancel (`cancel_flag`) | Implemented (core) | Acquisition / property / performance tests | **No UI path** to arm cancel |
| Split-RAW segment writes | Implemented (core) | Acquisition split tests | **Not packaged** into `.fsnap` yet |
| `.fsnap` v0.1 create/verify | Implemented | Package tests + 6 golden fixtures + CLI | Single `evidence.raw` only; draft contract |
| Independent verifier CLI | Implemented | `trareon-verifier` CLI tests | Exit 0 / 2 only; no repair |
| Guided synthetic UI | Partial | App.svelte CoC card + a11y labels | No formal a11y audit; no cancel control |
| Cross-OS CI matrix | Implemented | GitHub Actions `test` × 3 OS | See Day 20 recovery-cycle note |
| DevSecOps gates | Implemented | `deny.toml`, CI `security` job, `SECURITY.md` | Unmaintained advisories explicitly ignored |
| Bounded property tests | Implemented | `tests/properties.rs` + fuzz corpus docs | Full `cargo-fuzz` `NotValidated` |
| Performance baseline (synthetic) | Implemented | `tests/performance.rs` + `docs/performance/` | Peak RSS `NotValidated`; no optimization |
| Raw-device Linux/Windows/macOS | `NotValidated` | None | Days 23–25 require lab devices |
| Signing / release / certification | Out of scope | — | Day 30 requires human gate |

## `.fsnap` v0.1 compatibility freeze candidate

**Candidate status:** freeze candidate for Analysis readers of the
**Engineering Alpha** file-backed layout, contingent on:

1. Golden fixtures under `fixtures/fsnap-v0.1/` remaining byte-stable (`-text` in `.gitattributes`).
2. `docs/fsnap-v0.1-read-contract.md` remaining the fail-closed reader contract.
3. No silent-repair behavior in `verify_fsnap` / `trareon-verifier`.
4. Explicit rejection of unsupported `manifest.schema` values.

**Blocks a hard freeze claim (breaking ambiguities still open):**

- Split-RAW multi-segment packaging not in the package layout.
- Independent Codex review for Days 01–26 still `NOT_STARTED`.
- Day 30 human EAC classification not performed.
- Raw-device and elevated paths remain `NotValidated`.

Until Day 30 human approval, treat `.fsnap` v0.1 as a **draft freeze
candidate**, not a production format guarantee.

## Independent review status

All Day 01–26 `Review` cells remain `NOT_STARTED`. Closing GitHub issues
for implementation does **not** equal `EXPECTED_PASS` Codex review.
