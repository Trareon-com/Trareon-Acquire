# Commercial launch status

Last updated: 2026-07-17 (week 1 + Slint cutover)
Plan: [`COMMERCIAL-90-DAY-DAILY-PLAN.md`](COMMERCIAL-90-DAY-DAILY-PLAN.md)

## Current day

**Week 1 complete early** — week-1 work is merged and the Slint shell is now
the primary UI. Next automatable milestone is Hari 8 Windows lab inventory;
the Hari 3 `disk10s1` smoke still requires the operator.

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
| `disk10s1` operator smoke | **Waiting operator** |

## Gates

| Gate | Target day | Status |
|------|------------|--------|
| Gate 0 public foundation | Done (M0/M1) | PASS |
| Gate 1 waitlist | 39 | Not started |
| Gate 2 Founder preorder | 53 | Not started |
| Gate 3 external spend (signing) | 80+ | Blocked (zero-cash) |
| Gate 4 paid release | 82+ | Not started |

## Operator commands pending (Hari 3)

```bash
diskutil unmount "/Volumes/tiny11 2311"
cd "/Users/user/Projects/Trareon/Trareon Acquire"
sudo cargo run -p trareon-core --example lab_raw_bounded_smoke -- \
  /dev/disk10s1 fixtures/lab-allowlists/tiny11-2311-disk10.json 1048576
cargo run -q -p trareon-verifier -- verify /tmp/trareon-raw-bounded-lab/bounded-1048576.fsnap
```

Paste output to update lab report.

## Session log

| Date | Note |
|------|------|
| 2026-07-17 | Hari 1–6 docs + Windows smoke example + broker test; CI expected green on PR |
| 2026-07-17 | Week-1 branch merged; Slint replaced Tauri as the primary desktop shell |
