# Commercial launch status

Last updated: 2026-07-17 (non-Windows commercial finish pack)
Plan: [`COMMERCIAL-90-DAY-DAILY-PLAN.md`](COMMERCIAL-90-DAY-DAILY-PLAN.md)

## Current day

**Software finish pack landed** — unsigned install docs, self-test, founder
build helper, Linux lab software prep, UI preflight/About, and commercial
drafts are in-repo. **All Windows hardware days are deferred** to
[`platform/WINDOWS-LAB-OPERATOR-PACK.md`](platform/WINDOWS-LAB-OPERATOR-PACK.md).

Remaining blockers are human/business gates (waitlist, payment, demo video)
plus the Windows pack and Linux root loop attach.

## Week 1 deliverables

| Deliverable | Status |
|-------------|--------|
| `COMMERCIAL-V1-SCOPE.md` | Done |
| `COMMERCIAL-V1-GAP-AUDIT.md` | Done |
| `COMMERCIAL-90-DAY-DAILY-PLAN.md` | Done |
| `macos-elevation-operator-guide.md` | Done |
| `lab_windows_bounded_smoke` example | Done |
| Broker `PhysicalDrive0` deny test | Done |
| Week-1 review, merge, and matrix update | Done |
| `disk10s1` operator smoke | **PASS** (1 MiB, SHA `445808af…`) |
| Windows lab inventory scaffold (Hari 8) | Done → collected in operator pack |
| Linux lab software prep | Done (inventory + example + template) |
| Unsigned install docs | Done (`docs/install/`) |
| Self-test / founder-build scripts | Done |

## Gates

| Gate | Target day | Status |
|------|------------|--------|
| Gate 0 public foundation | Done (M0/M1) | PASS |
| Gate 1 waitlist | 39 | **Human** — software ready; not opened |
| Gate 2 Founder preorder | 53 | **Human** — legal draft + ledger template ready |
| Gate 3 external spend (signing) | 80+ | Blocked (zero-cash) |
| Gate 4 paid release | 82+ | Not started |

## Deferred: Windows lab

See [`platform/WINDOWS-LAB-OPERATOR-PACK.md`](platform/WINDOWS-LAB-OPERATOR-PACK.md)
for Hari 8–14 / 22 / 64 / 88 in one checklist. Do not claim Windows raw PASS
until that pack is executed on a Windows host.

## Hari 3 result (disk10s1)

```text
source=/dev/disk10s1 bound=1048576
sha256=445808af80ff3a67e29fcd10131b690fc27d2243297186524b5cd7de4d3a63ff
verifier=VALID
```

## Session log

| Date | Note |
|------|------|
| 2026-07-17 | Hari 1–6 docs + Windows smoke example + broker test |
| 2026-07-17 | Slint cutover; docs sync; Hari 8 scaffold; disk10s1 PASS |
| 2026-07-17 | Non-Windows finish pack; Windows work collected in operator pack |
