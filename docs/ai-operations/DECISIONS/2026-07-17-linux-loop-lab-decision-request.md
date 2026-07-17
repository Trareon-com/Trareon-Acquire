# Decision request — Linux loop lab allowlist

- **Day / Task:** Commercial Hari 15–16 / M2-P02
- **Gate status:** `HUMAN_APPROVAL` **REQUIRED**
- **Request:** Approve one synthetic read-only loop device for Linux lab acquires
- **Required inventory:** `docs/platform/linux-lab-inventory.md`
- **Software ready:** `lab_linux_loop_bounded_smoke`,
  `fixtures/lab-allowlists/linux-loop-template.json`, portable fault suite
- **Forbidden until approval:** `human_approved: true` for any `/dev/loop*`;
  touching `nvme0n1` / system disk; claiming Linux raw PASS from Day 23 alone

## Acceptance options

| Option | Meaning |
|--------|---------|
| A | Approve inventoried RO loop for bounded Linux lab smoke |
| B | Defer — inventory incomplete |
| C | Reject |

## Evidence when accepting

- Filled inventory; exact `/dev/loopN`; confirmation system disk excluded
