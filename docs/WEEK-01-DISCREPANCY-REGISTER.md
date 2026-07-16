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
cargo tree -p traeron-core --locked | grep -i tauri     # no match
cargo tree -p traeron-verifier --locked | grep -i tauri # no match
```

`traeron-core` and `traeron-verifier` do not depend on Tauri, `wry`, or any
frontend/UI crate. Only `apps/traeron-acquire/src-tauri` depends on `tauri`
and, transitively, on `traeron-core`. The dependency direction is one-way:
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
| 11 | RAW and split-RAW | Implemented and tested (`with_split_segment_bytes`, boundary-size, zero-length-final-segment, final-short-segment, segment-order, reassembly) | `.fsnap` packaging (`package.rs`) still assumes a single `evidence.raw`; multi-segment packaging is not yet wired into Day 12/13 scope |
| 15 | CLI and fsnap draft | Implemented and tested — 6 golden fixtures (valid, mutated, truncated, removed-file, audit-discontinuous, unsupported-version), CLI exit-code assertions for each, `docs/fsnap-v0.1-read-contract.md` written. Also closed a real gap: `verify_fsnap` now enforces manifest schema/build-identity, which was previously declared in the JSON Schema but never checked in Rust | None known |
| 17–19 | Guided UI, guidance/accessibility, CoC/report preview | Partially covered (Task 7 delivered a simpler guided flow: source/output fields, confirmation checkbox, progress, result card) | No case-identity fields, no dedicated verifier-summary breakdown, no accessibility/keyboard audit, no Chain-of-Custody report rendering |
| 20 | Cross-platform CI | Implemented and hosted-CI green on 3 OS (PR #32) | 3 recovery cycles were needed (exceeds this runbook's own 2-cycle limit) — recorded honestly in `MASTER-CHECKLIST.md` |
| 21–30 | DevSecOps gates, fuzz, platform feasibility, performance, capability matrix, docs, adversarial review, freeze gate | Not started | Out of scope for this register; tracked as `NOT_STARTED` in `MASTER-CHECKLIST.md` |

## P0/P1 status

No open P0 (release-blocking) or P1 (high-severity) findings against the
implemented scope as of this register. The gaps above are scope not yet
attempted, not known defects in delivered code.

## RFC baseline

`docs/RFC-BASELINE.sha256` is unchanged by this integration gate; no RFC
amendment was made or required.
