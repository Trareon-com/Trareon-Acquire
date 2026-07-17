# Rolling wave — M2 prompts from M1 exit evidence

Generated from M1 exit (`docs/M1-MILESTONE-REVIEW.md`, `docs/M1-FAILURE-MATRIX.md`,
Days 23–25 platform reports). Do not start destructive hardware work without
human allowlist approval.

## Entry gate (must be true)

- [x] M1 file-backed false-complete matrix exists
- [x] Analysis importer covers all six `.fsnap` v0.1 goldens
- [x] Checkpoint/resume covers non-split and split-RAW file-backed paths
- [x] Software lab-policy / plans / decision request authored (`docs/M2-SOFTWARE-PREP-STATUS.md`)
- [x] Operator designated removable media `/Volumes/tiny11 2311` → `/dev/disk10` (allowlist + file-backed smoke PASS)
- [x] Raw open of `disk10`/`rdisk10` Available under elevation (operator sudo probe 2026-07-17)
- [x] Bounded raw content sample (`lab_raw_bounded_smoke`, 1 MiB) under sudo — SHA `41fb8d9…` PASS + independent verify
- [x] Optional larger bound (64 MiB) on same allowlist — SHA `a0ff343…` PASS + independent verify (still not full disk)
- [ ] Allowlisted lab media + write-block process approved for **additional** devices as needed
- [ ] Full-disk acquire **run** of `rdisk10` → Untitled (gate PROVIDED; operator sudo pending — runbook §H)
- [ ] GitHub Actions billing/spending limit restored so PR checks can run

## Software prep (landed)

- Plans: `docs/platform/m2-p01-*.md` … `m2-p05-*.md`
- Decision: `docs/ai-operations/DECISIONS/2026-07-17-m2-lab-allowlist-required.md`
- Code: `lab_policy.rs` + `tests/fault_injection.rs`
- Lab trial: `docs/platform/m2-lab-tiny11-2311-disk10.md` + runbook `m2-tiny11-disk10-operator-runbook.md`

## Prompt pack (copy into agent sessions)

### M2-P01 — Windows narrow storage lab plan

Author a Windows-first lab plan that acquires only allowlisted removable media.
Require UAC elevation evidence, source identity stability, and independent
`trareon-verifier` post-check. Forbid system-disk targets. Cite Day 24 probe.

### M2-P02 — Linux reference fault-injection harness

On Kali/Ubuntu LTS lab hosts, define loop-device/synthetic fault cases
(disconnect, destination-full, bad-sector substitute) without touching the
system disk. Map each case to the M1 failure matrix semantics (no
false-complete). Cite Day 23.

### M2-P03 — macOS helper boundary (no FDA claim)

Document operator/helper requirements for `/dev/rdisk*` without claiming FDA
or production fitness. Intel Mac remains `NotValidated` until separate evidence.
Cite Day 25.

### M2-P04 — Privileged broker elevation (human-gated)

Replace `StubElevationHelper` only after a reviewed helper design, allowlisted
ops, and human gate. Shell-like payloads must remain denied. No network from
helper. **Progress:** allowlist binding + system-disk deny landed in core;
OS elevation still stub-only.


### M2-P05 — Capability matrix promotion rules

Update the capability matrix only with exact OS/hardware evidence. Never
promote privilege-probe PASS into raw-acquire PASS.

## Explicit non-goals for this wave

- Official Production classification
- Signing/notarization
- Court-admissibility claims
- Umbrella “all OS” language
