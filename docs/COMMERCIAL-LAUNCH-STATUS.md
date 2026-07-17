# Commercial launch status

Last updated: 2026-07-17 (Hari 8 software prep + disk10s1 ready)
Plan: [`COMMERCIAL-90-DAY-DAILY-PLAN.md`](COMMERCIAL-90-DAY-DAILY-PLAN.md)

## Current day

**Hari 8 software prep done** — Windows lab inventory scaffold and media
decision request are in-repo. Fill the inventory on a Windows host, then accept
the decision before any elevated Windows smoke. Hari 3 `disk10s1` is unmounted
and ready for operator `sudo` (agent cannot enter the password).

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
| `disk10s1` operator smoke | **Waiting operator** (volume unmounted) |
| Windows lab inventory scaffold (Hari 8) | Done (fill on Windows host) |

## Gates

| Gate | Target day | Status |
|------|------------|--------|
| Gate 0 public foundation | Done (M0/M1) | PASS |
| Gate 1 waitlist | 39 | Not started |
| Gate 2 Founder preorder | 53 | Not started |
| Gate 3 external spend (signing) | 80+ | Blocked (zero-cash) |
| Gate 4 paid release | 82+ | Not started |

## Operator commands pending (Hari 3)

Volume check (2026-07-17): `/dev/disk10s1` present, **Mounted: No**, unelevated
open → `EACCES`. Unmount step can be skipped unless it remounts.

```bash
cd "/Users/user/Projects/Trareon/Trareon Acquire"
# optional if remounted:
# diskutil unmount "/Volumes/tiny11 2311"
./scripts/operator-disk10s1-smoke.sh
```

Or manually:

```bash
sudo cargo run -p trareon-core --example lab_raw_bounded_smoke -- \
  /dev/disk10s1 fixtures/lab-allowlists/tiny11-2311-disk10.json 1048576
cargo run -q -p trareon-verifier -- verify /tmp/trareon-raw-bounded-lab/bounded-1048576.fsnap
```

Paste output to update `docs/platform/m2-lab-tiny11-2311-disk10.md`.

## Windows next (Hari 8–11)

1. Fill [`docs/platform/windows-lab-inventory.md`](platform/windows-lab-inventory.md) on the Windows lab host.
2. Accept [`docs/ai-operations/DECISIONS/2026-07-17-windows-lab-media-decision-request.md`](ai-operations/DECISIONS/2026-07-17-windows-lab-media-decision-request.md).
3. Author media-specific allowlist from `fixtures/lab-allowlists/windows-usb-template.json`.
4. Elevated open-only, then `lab_windows_bounded_smoke` 1 MiB + verify.

## Session log

| Date | Note |
|------|------|
| 2026-07-17 | Hari 1–6 docs + Windows smoke example + broker test; CI expected green on PR |
| 2026-07-17 | Week-1 branch merged; Slint replaced Tauri as the primary desktop shell |
| 2026-07-17 | Hari 8 inventory + Windows media decision request; disk10s1 unmounted/ready |
