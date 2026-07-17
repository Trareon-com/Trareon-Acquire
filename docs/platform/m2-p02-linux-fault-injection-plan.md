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
| Broker system disk | `assert_broker_source_identity` hard-deny |
| Broker block w/o allowlist | Denied |

## Lab-only script outline (human + root, allowlisted loop)

```bash
# Do NOT use nvme0n1 / system disk
IMG=/var/tmp/trareon-lab-loop.img
dd if=/dev/zero of="$IMG" bs=1M count=64 status=none
LOOP=$(sudo losetup -f --show -r "$IMG")   # read-only loop
# author allowlist entry for $LOOP, human_approved=true, decision file
# bounded acquire with max_bytes; then:
#   sudo losetup -d "$LOOP"   # mid-read disconnect case
# destination-full: fill a small tmpfs and point output there
# never claim Linux raw-acquire PASS from probes alone
```

## Non-goals

Claiming Linux raw-acquire PASS from probes alone; using the host system disk.
