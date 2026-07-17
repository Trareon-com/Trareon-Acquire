# Linux lab inventory (Hari 15 software prep)

Status: **SOFTWARE PREP** — physical loop attach still needs root lab session.
Plan: [`m2-p02-linux-fault-injection-plan.md`](m2-p02-linux-fault-injection-plan.md)
Day 23 probe: [`day23-linux-feasibility.md`](day23-linux-feasibility.md)
Decision request: [`../ai-operations/DECISIONS/2026-07-17-linux-loop-lab-decision-request.md`](../ai-operations/DECISIONS/2026-07-17-linux-loop-lab-decision-request.md)
Template: `fixtures/lab-allowlists/linux-loop-template.json`
Example: `lab_linux_loop_bounded_smoke`

## Inventory checklist (fill on Linux lab host)

| Field | Value (operator) |
|-------|------------------|
| Distro / version | |
| Kernel | |
| Arch | |
| System disk (never touch) | |
| Lab image path | e.g. `/var/tmp/trareon-lab-loop.img` |
| Loop device | e.g. `/dev/loopN` |
| Attach mode | read-only (`losetup -r`) |
| Free staging space | |
| Root / `disk` group available? | |

## Safe discovery

```bash
lsblk -o NAME,SIZE,TYPE,MOUNTPOINT,RM,RO,MODEL
# Identify system disk and never use it
losetup -f   # next free loop (do not attach yet)
```

## After HUMAN_APPROVAL

1. Create RO loop from a synthetic image (not system disk).
2. Allowlist exact `/dev/loopN` with `human_approved: true`.
3. Bounded smoke via `lab_linux_loop_bounded_smoke`.
4. Optional fault: detach mid-read; dest-full on small tmpfs.

## Explicit non-claims

Portable CI fault suite ≠ Linux raw-acquire PASS.
