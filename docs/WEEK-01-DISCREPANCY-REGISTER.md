# Week 1 Integration Gate — Discrepancy Register

Result class: **Engineering Alpha** foundation slice. This register accompanies
the Day 07 (Week 1 integration) checkpoint of
`docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md` and
`docs/ai-operations/MASTER-CHECKLIST.md`. It does not replace either document.

## Clean-check evidence (run twice, identical)

```
cargo test --workspace --all-targets --locked
```

Both runs on `feat/m0-t01-workspace-core-boundary` (commit
`3c462b0...` and later) produced identical `test result: ok` counts across
every test binary — no flaky or order-dependent tests were observed.

## Dependency boundary

```
cargo tree -p trareon-core --locked | grep -i tauri     # no match
cargo tree -p trareon-verifier --locked | grep -i tauri # no match
```

`trareon-core` and `trareon-verifier` do not depend on Tauri, `wry`, or any
frontend/UI crate. Only `apps/trareon-acquire/src-tauri` depends on `tauri`
and, transitively, on `trareon-core`. The dependency direction is one-way:
UI depends on core, core never depends on UI. This matches the RFC/roadmap
requirement that core and verifier compile without Tauri.

## Known discrepancies between the Day-by-day runbook and delivered code

The M0 foundation slice was implemented via the coarser-grained
`docs/superpowers/plans/2026-07-17-trareon-acquire-foundation.md` (8 tasks,
single-author, no per-Day independent Codex review), not via
`docs/ai-operations/`'s Day-by-day cadence. See
`docs/ai-session-log/2026-07-17-claude-code-foundation-complete.md` and
`docs/ai-session-log/2026-07-17-claude-code-pr32-ci-fixes.md` for the full
trail. As of this register:

| Day | Scope | Status | Gap |
|---:|---|---|---|
| 01–05, 08–09, 12–14, 16 | Baseline, workspace, domain, audit, streaming, identity, manifest, package, verifier, Tauri boundary | Implemented, tested, hosted-CI green on 3 OS | No independent Codex review yet |
| 06 | Canonical hashing determinism/version rejection | Implemented and tested | None known |
| 07 | This integration gate | In progress (this file) | None known |
| 10 | Failure semantics | Cancellation implemented and tested (`CoreError::Cancelled`, cooperative `cancel_flag`); destination-write-failure tested via a portable deterministic substitute (blocked path component) | Retry-boundary and literal destination-full simulation intentionally not implemented — that belongs to raw-device bad-sector policy in Track B/M2, not the file-backed M0 engine |
| 11 | RAW and split-RAW | Implemented and tested (`with_split_segment_bytes`, …) | Split packaging now via `create_fsnap_from_segments` + optional `evidence_segments` (classic single-file Analysis freeze retained) |
| 15 | CLI and fsnap draft | Implemented and tested — 6 golden fixtures (valid, mutated, truncated, removed-file, audit-discontinuous, unsupported-version), CLI exit-code assertions for each, `docs/fsnap-v0.1-read-contract.md` written. Also closed a real gap: `verify_fsnap` now enforces manifest schema/build-identity, which was previously declared in the JSON Schema but never checked in Rust | None known |
| 17–19 | Guided UI, guidance/accessibility, CoC/report preview | Case identity field (operator note only, explicitly not part of verification), `<label for>` + `aria-describedby` + `aria-live` + `aria-busy` accessibility wiring, and a Chain-of-Custody-style report card added; report status is still driven only by the core's returned status, never invented by the UI | UI Cancel landed in M1; manual a11y checklist PASS (`docs/accessibility/M1-A11Y-CHECKLIST.md`); automated scanner still open |
| 20 | Cross-platform CI | Implemented and hosted-CI green on 3 OS (PR #32) | 3 recovery cycles were needed (exceeds this runbook's own 2-cycle limit) — recorded honestly in `MASTER-CHECKLIST.md` |
| 21 | DevSecOps gates | Implemented via PR #34 (merged); rename path fix via PR #35 (merged) | Independent Codex review still `NOT_STARTED` |
| 22 | Property and fuzz baseline | Implemented via PR #37 (`feat/m0-d22-fuzz`, frozen SHA `f225d33`) — bounded property suite + synthetic corpus + CI smoke; hosted CI `PASS_3OS` | Full `cargo-fuzz`/libFuzzer remains `NotValidated` (needs dependency/toolchain review). Day 22 entry gate asked for Day 21 independent review `EXPECTED_PASS`, which has not happened yet |
| 23 | Linux raw-device privilege-boundary feasibility | Implemented via PR #47 (`feat/m0-d23-linux`, frozen SHA `b377e3b`) on real Kali hardware (human-operated relay, no direct network path from this session) — `crates/trareon-core/src/platform.rs` probe, `docs/platform/day23-linux-feasibility.md` | Only loop-device/synthetic-file feasibility tested; real physical disk attach/read intentionally out of scope (system disk excluded per policy) |
| 24 | Windows raw-device UAC-elevation feasibility | Implemented via PR #49 (`feat/m0-d24-windows`, frozen SHA `f465ac3`) on operator Windows 10 Pro — `windows::probe_physical_drive_zero()`, `docs/platform/day24-windows-feasibility.md` | Handle-open only; X270 / older Windows remain `NotValidated` |
| 25 | macOS raw-device privilege-boundary feasibility | Implemented on M4 Pro Mac — `macos::probe_rdisk0`, `docs/platform/day25-macos-feasibility.md` | Open denied without `operator`/helper; Intel/`hdiutil` attach/`FDA` helper `NotValidated` |
| 26 | Performance baseline | Implemented via PR #43 (merged) — harness + synthetic timings + CI smoke; no optimization | Peak RSS `NotValidated`; Day 25 entry gate not formally met |
| 27 | Capability matrix + fsnap freeze candidate | Implemented — `docs/CAPABILITY-MATRIX-M0.md`; later promoted to Analysis freeze at Day 30 | Split-RAW packaging and Official Production still out of scope |
| 28 | User guide / About / legal draft | Implemented on `feat/m0-d28-docs` — USER-GUIDE, LEGAL-LIMITATIONS-DRAFT, About UI | Legal text remains draft; no certification claim |
| 29 | Adversarial review | Implemented — `docs/ADVERSARIAL-REVIEW-M0.md`; no new P0/P1 | Residuals block Official Production; Lab-Use-Only EAC still allowed with human approval |
| 30 | EAC / fsnap Analysis freeze | Complete redo after Days 23–25 — milestone/risk/evidence/decision supersede PR #48 | Classification Lab Use Only; Codex reviews still `NOT_STARTED`; raw acquire still `NotValidated` |

| post-M0 | Privileged broker protocol spike | `broker.rs` typed allowlist + deny shell; returns `NotImplemented` | `StubElevationHelper` trait added in M1; still no OS elevate |
| post-M0 | Cursor independent review Days 01–29 | `docs/INDEPENDENT-REVIEW-M0-CURSOR.md` | Codex review still not performed; substitute documented |
| M1 | UI cancel + checkpoint/resume + Analysis importer | PR #53 merged | Initial slice; split resume + exit bundle follow on `feat/m1-exit-complete` |
| M1 exit | Failure matrix, split resume, 6-golden Analysis, a11y checklist, M2 prompts | PR #54 | File-backed Engineering Alpha exit; raw acquire still NotValidated |
| M2 prep | Lab allowlist policy + plans P01–P05 + decision request | PR #55 | HUMAN_APPROVAL_REQUIRED before raw/loop content acquire |
| M2 lab | tiny11 disk10 allowlist + elevated open + 1 MiB/64 MiB bounded raw PASS | PRs #56–#60 + operator sudo | Full-disk still NotValidated; Lab Beta exit not claimed |

## P0/P1 status

No open P0 (release-blocking) or P1 (high-severity) findings against the
implemented scope as of this register. The gaps above are scope not yet
attempted, not known defects in delivered code.

## RFC baseline

`docs/RFC-BASELINE.sha256` is unchanged by this integration gate; no RFC
amendment was made or required.
