# M0 Adversarial Review (Synthetic / Local)

- **Date:** 2026-07-17
- **Product freeze base (main):** `9e24b15e8a562a033d70732a69f55b9bf5e1c1c3` (merge of Day 26 / PR #43)
- **Review tip (includes Day 27–28 docs/UI on this branch):** `72e855bac1a9cc9568a21855e5c3dda8f4cd3ca3`
- **Result class:** Engineering Alpha — Lab Use Only
- **Author role:** adversarial pass against frozen product behavior using existing deterministic reproducers (no production data, no raw disk)

## Method

Re-ran the workspace suite and UI production build, then mapped each Day 29 attack theme to an existing automated reproducer. No new dependencies. No silent weakening of assertions.

Commands (exit 0 unless noted):

```bash
cargo test --workspace --locked
npm run build --prefix apps/trareon-acquire
sh scripts/validate-ai-operations.sh
```

Security-boundary grep (no executable hits outside Tauri-generated docs):

```bash
rg -n "tauri-plugin-shell|shell:|fs:allow|unsafe\\s*\\{|continue-on-error|\\|\\| true" \
  apps/trareon-acquire/src-tauri crates apps/trareon-acquire/src \
  --glob '!node_modules/**' --glob '!**/gen/**'
```

## Attack themes → reproducers

| Theme | Reproducer / evidence | Outcome |
|---|---|---|
| Evidence tamper | Golden `mutated` / `truncated`; `package_verifier_rejects_modified_evidence` | Rejected (exit 2 / `Err`) |
| Audit tamper / discontinuity | Golden `audit-discontinuous`; `property_audit_tamper_breaks_continuity` | Rejected |
| False completion | State machine tests; `property_cancel_flag_never_reports_verified_complete` | No false `VerifiedComplete` |
| Path escape / extra files | `property_unsafe_manifest_paths_*`; `property_extra_package_file_breaks_containment` | Rejected without panic |
| Unsupported schema | Golden `unsupported-version` | Rejected |
| Privilege / shell boundary | Tauri `capabilities` = `core:default` only; boundary grep clean | No shell / broad FS grant found |
| Dependency / license / advisory | CI `security` job + `deny.toml` | Gates present; unmaintained advisories explicitly listed |
| Nondeterminism (hash) | Canonical hash tests; property audit determinism; perf two-run hash match | Stable on synthetic fixtures |
| Cancellation | Acquisition cancel tests; property cancel; perf cancel latency | `CoreError::Cancelled`; not complete |
| Performance equivalence | Day 26 baseline — measurement only, no optimization in-scope | Hash/bytes stable across repeated runs |

## P0 / P1 findings

**None newly discovered in this pass** that violate an M0 acceptance invariant already claimed as implemented.

Known residual **limitations** (not filed as new P0 because already tracked; they **do** block a production EAC claim at Day 30):

1. No UI path to `cancel_flag` (core cancel exists).
2. Split-RAW segments not packaged into `.fsnap`.
3. Raw-device paths Days 23–25 = `NotValidated` / `MANUAL_START`.
4. Full `cargo-fuzz` = `NotValidated`.
5. Peak RSS = `NotValidated`.
6. Independent Codex `Review` cells Days 01–28 still `NOT_STARTED`.

## Day 30 gate recommendation

- **Do not** auto-approve Day 30 Production-Directed EAC.
- Requires explicit **human** classification after reviewing this report + capability matrix + discrepancy register.
- File-backed synthetic slice may be described as Engineering Alpha lab demo only.

## NotValidated list (review scope)

- Raw-device acquire / elevated privilege on Linux, Windows, macOS.
- Formal accessibility audit tooling.
- Absolute memory ceiling (RSS) on all CI OSes.
- Full libFuzzer / cargo-fuzz campaign.
- Any court-admissibility or certification claim.
