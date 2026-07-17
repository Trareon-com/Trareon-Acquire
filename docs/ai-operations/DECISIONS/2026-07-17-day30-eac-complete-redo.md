# Decision Record — Day 30 EAC complete redo

- **Day / Task:** M0-D30 (complete redo)
- **Frozen SHA (audit base):** `4dd2828b37286f5d99de69825452502ec61bef53`
- **Gate status:** `HUMAN_APPROVAL_REQUIRED` resolved by operator instruction to redo
- **Incident category:** HUMAN-APPROVAL-REQUIRED (planned Day 30 gate) + process correction
- **RFC/spec section affected:** Engineering Alpha classification; `.fsnap` v0.1 Analysis reader compatibility
- **Observed evidence:** MASTER-CHECKLIST Days 01–29 all `IMPLEMENTED_UNREVIEWED` including Days 23–25 hardware privilege spikes (PRs #47, #49, #50); zero open P0/P1; adversarial review still advises against Official Production; golden fixtures + verifier present
- **Conflicting evidence or ambiguity:** Prior Day 30 (PR #48) classified while Days 24–25 were still incomplete — that package is **superseded**, not silently amended in place without a new decision
- **Safe default if no decision:** Keep PR #48 wording and leave Days 24–25 as post-hoc notes only (rejected by operator)
- **Option A — leave PR #48 as sole Day 30 record:** preserves incomplete freeze narrative
- **Option B — redo Day 30 against tip including Days 23–25, reaffirm Lab-Use-Only EAC + Analysis freeze:** matches operator request and Month-01 exit evidence
- **Recommended option and reason:** Option B
- **Exact authority or command requested:** Operator chat on 2026-07-17: `untuk day 30 buat ulang karena tadi belum lengkap semua harinya`
- **Actions explicitly not performed:** publish/release/sign; Official Production claim; claiming Days 23–25 as validated raw-disk *acquisition*; silent schema migration

## Supersedes

[`2026-07-17-day30-eac-human-approval.md`](2026-07-17-day30-eac-human-approval.md) (PR #48 incomplete freeze base `6594840`).

AI must not treat this record as authority to publish, sign, or declare Official Production.
