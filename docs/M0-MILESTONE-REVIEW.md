# Milestone Review — M0 Day 30 (complete redo)

- **Milestone:** M0 complete → handoff to M1 Engineering Alpha rolling wave
- **Revision:** This supersedes the incomplete Day 30 package from PR #48,
  which was classified before Days 24–25 hardware evidence existed. Operator
  instruction 2026-07-17: recreate Day 30 now that all Month-01 days have
  implementation evidence.
- **Frozen SHA (audit base):** `4dd2828b37286f5d99de69825452502ec61bef53`
  (`main` tip including Days 23–25 / PRs #47–#50)
- **Evidence index complete:** yes — [`docs/ai-operations/EVIDENCE-INDEX.md`](ai-operations/EVIDENCE-INDEX.md)
- **Required tests:** re-verified on this redo (`cargo test --workspace --locked`,
  `npm run build --prefix apps/trareon-acquire`, `sh scripts/validate-ai-operations.sh`,
  `shasum -a 256 -c docs/RFC-BASELINE.sha256`). Note: `npm run check` is not a
  defined script in `apps/trareon-acquire`; production Vite build is the UI gate used.
- **Open P0 findings:** none
- **Open P1 findings:** none
- **Determinism evidence:** canonical-hash tests; property audit determinism;
  performance two-run hash match; golden `.fsnap` fixtures marked `-text`
- **Performance baseline/equivalence:** [`docs/performance/m0-day26-baseline.md`](performance/m0-day26-baseline.md)
  — measurement only; peak RSS `NotValidated`
- **Platform capability matrix:** [`docs/CAPABILITY-MATRIX-M0.md`](CAPABILITY-MATRIX-M0.md)
- **Capabilities NotValidated:** physical-disk content acquire on Linux/Windows/macOS
  (Days 23–25 are privilege/enumeration spikes only); Intel Mac; Ubuntu LTS separate
  from Kali; ThinkPad X270 Windows path; full `cargo-fuzz`; peak RSS; formal a11y
  audit; court/certification claims; UI path to `cancel_flag`; split-RAW packaging
  into `.fsnap`; privileged helper / FDA grant paths
- **Verifier independence:** `trareon-verifier` CLI + six golden fixtures under
  `fixtures/fsnap-v0.1/`; fail-closed, no silent repair
- **Documentation/limitations:** user guide, legal draft (`LEGAL_DRAFT_ONLY`),
  adversarial review, discrepancy register, platform reports Days 23–25
- **Security/supply-chain status:** CI `security` job + `deny.toml` + `SECURITY.md`;
  unmaintained advisories listed explicitly
- **Human approval:** PROVIDED — operator requested Day 30 redo after Days 23–25
  landed (`untuk day 30 buat ulang karena tadi belum lengkap semua harinya`)
- **Classification:** `ENGINEERING_ALPHA` as **Production-Directed Engineering Alpha Candidate — Lab Use Only**

`OFFICIAL_PRODUCTION` remains forbidden: raw-device *acquisition* (not merely
privilege probes) remains `NotValidated`, independent Codex `Review` cells remain
`NOT_STARTED`, and residual product limitations remain open.

## Month-01 day coverage (implementation)

| Band | Days | Status |
|---|---|---|
| Foundation + gap-fill | 01–20 | `IMPLEMENTED_UNREVIEWED` (PR #33) |
| DevSecOps / fuzz / perf | 21–22, 26 | `IMPLEMENTED_UNREVIEWED` (PRs #34, #37, #43) |
| Platform privilege spikes | 23–25 | `IMPLEMENTED_UNREVIEWED` + Human Gate `PROVIDED` (PRs #47, #49, #50) |
| Matrix / docs / adversarial | 27–29 | `IMPLEMENTED_UNREVIEWED` (PRs #44–#46) |
| EAC gate | 30 | This redo — Lab Use Only + Analysis freeze |

All Day 01–29 `Review` cells remain `NOT_STARTED`.

## `.fsnap` v0.1 Analysis freeze (reaffirmed)

**Decision:** freeze the **file-backed, single-`evidence.raw`** reader contract for
Trareon Analysis import against golden fixtures and
`docs/fsnap-v0.1-read-contract.md`.

**In scope:**

1. Package layout and fail-closed verification rules in the read contract.
2. Golden suite: `valid`, `mutated`, `truncated`, `removed-file`,
   `audit-discontinuous`, `unsupported-version`.
3. Schema string `trareon.fsnap.manifest/1` equality; unsupported schemas reject.

**Out of freeze scope:**

- Split-RAW multi-segment packaging.
- Raw-device acquisition paths (Linux/Windows/macOS privilege probes do **not**
  equal validated acquire adapters).
- Production compatibility / court-admissibility / certification guarantees.
- Silent migration across schema versions.

## Platform privilege summary (Days 23–25)

| OS | Finding | Implication for Track C |
|---|---|---|
| Linux (Kali) | Unprivileged user denied `losetup` / loop-control; root can attach synthetic loop RO | Privileged broker required |
| Windows 10 | Same Administrators account: elevated open `PhysicalDrive0` ok; non-elevated denied | UAC elevation required |
| macOS M4 Pro | SIP + Authenticated Root on; admin not in `operator`; `/dev/rdisk0` open denied | Explicit helper/elevation required |

## Handoff

Next: M1 rolling-wave decision using
[`docs/ai-operations/PHASE-MAPS/M1-ENGINEERING-ALPHA.md`](ai-operations/PHASE-MAPS/M1-ENGINEERING-ALPHA.md).
Prior incomplete Day 30 decision (PR #48) is superseded by
[`docs/ai-operations/DECISIONS/2026-07-17-day30-eac-complete-redo.md`](ai-operations/DECISIONS/2026-07-17-day30-eac-complete-redo.md).
