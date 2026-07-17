# Commercial launch status

Last updated: 2026-07-17 (Hari 3 disk10s1 PASS)
Plan: [`COMMERCIAL-90-DAY-DAILY-PLAN.md`](COMMERCIAL-90-DAY-DAILY-PLAN.md)

## Current day

**Hari 3 closed** — unmounted `/dev/disk10s1` bounded 1 MiB smoke PASS +
independent verify. Next blocker is Windows lab host work (Hari 8 inventory fill
→ allowlist approval → elevated smoke).

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
| Windows lab inventory scaffold (Hari 8) | Done (fill on Windows host) |

## Gates

| Gate | Target day | Status |
|------|------------|--------|
| Gate 0 public foundation | Done (M0/M1) | PASS |
| Gate 1 waitlist | 39 | Not started |
| Gate 2 Founder preorder | 53 | Not started |
| Gate 3 external spend (signing) | 80+ | Blocked (zero-cash) |
| Gate 4 paid release | 82+ | Not started |

## Hari 3 result (disk10s1)

```text
source=/dev/disk10s1 bound=1048576
sha256=445808af80ff3a67e29fcd10131b690fc27d2243297186524b5cd7de4d3a63ff
verifier=VALID
package=/tmp/trareon-raw-bounded-lab/bounded-1048576.fsnap  (not committed)
```

Re-run (if remounted, unmount first):

```bash
cd "/Users/user/Projects/Trareon/Trareon Acquire"
./scripts/operator-disk10s1-smoke.sh
```

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
| 2026-07-17 | Hari 3 disk10s1 1 MiB PASS + verify (Terminal sudo) |
