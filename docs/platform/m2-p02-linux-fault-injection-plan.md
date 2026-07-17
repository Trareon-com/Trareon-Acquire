# M2-P02 — Linux reference fault-injection harness plan

Status: **PLAN + portable synthetic suite** — physical loop attach still needs human/root lab session.

## Objective

On Kali/Ubuntu LTS lab hosts, exercise disconnect / destination-full / bad-sector
**substitutes** without touching the system disk. Map outcomes to
`docs/M1-FAILURE-MATRIX.md` (no false-complete). Cite Day 23.

## Portable suite (CI / any OS)

Implemented in `crates/trareon-core/tests/fault_injection.rs`:

| Fault | Portable substitute |
|---|---|
| System disk target | Hard-deny `PhysicalDrive0` / `nvme0n1` / `rdisk0` |
| Destination full | Parent path is a file |
| Empty source | Reject before acquire |
| Unsigned allowlist | Block-device suspect denied |

## Lab-only (human + root, allowlisted media)

1. Create synthetic backing file; `losetup -r` attach (Day 23).
2. Allowlist the loop path with human approval.
3. Faults: detach mid-read, fill destination FS, inject EIO via device-mapper if available.
4. Never use `nvme0n1` / root FS disk.

## Non-goals

Claiming Linux raw-acquire PASS from probes alone.
