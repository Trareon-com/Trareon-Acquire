# M2 lab media trial — tiny11 2311 (`disk10`)

Date: 2026-07-17  
Operator designation: `/Volumes/tiny11 2311`  
Allowlist: `fixtures/lab-allowlists/tiny11-2311-disk10.json`

## Inventory (redacted)

| Field | Value |
|---|---|
| Mount point | `/Volumes/tiny11 2311` |
| Device node | `/dev/disk10` (whole), `/dev/disk10s1` (partition), `/dev/rdisk10` (raw char) |
| FS | NTFS, **read-only** on this Mac (`nosuid, read-only`) |
| Apparent size | ~57 GiB (`df`) |
| System disk? | **No** (not `disk0` / `rdisk0`) |
| Privilege | Device nodes `root:operator` mode `640` |

## What was tried

1. **Allowlist gate** — `/dev/disk10`, `/dev/rdisk10`, `/dev/disk10s1` accepted with `human_approved: true`. Volume directory path correctly refused as non-file source.
2. **Raw open probe (no content read)** — `/dev/disk10`, `/dev/rdisk10`, `/dev/disk10s1` → `DeniedInsufficientPrivilege` (user not in `operator`).
3. **File-backed smoke** — acquired `autorun.inf` (128 bytes) from the volume to `/tmp/trareon-tiny11-lab/`, packaged + verified. SHA-256 `3378723cb5910e5f3afe1ee2200b8f0d08bab8c5d77f7ce9fcb5725aff525852`.
4. **Write to volume** — failed (`Read-only file system`); expected for NTFS without write driver.

## Capability claims

| Capability | Status |
|---|---|
| Lab allowlist for this removable media | Implemented (policy) |
| File-backed acquire of a file *on* the volume | PASS (smoke) |
| Raw-device content acquire of `disk10`/`rdisk10` | **NotValidated** (open denied; no elevate) |
| Writable staging on this volume | **NotValidated** (RO mount) |

## Explicitly not done

- No elevation / `operator` group change
- No full-disk image of the USB
- No commit of acquired evidence bytes
- No Official Production / Lab Beta exit claim

## Next

Operator runbook: [`m2-tiny11-disk10-operator-runbook.md`](m2-tiny11-disk10-operator-runbook.md)
(elevated open-only probe, optional `operator` membership, NTFS write options).
PR: https://github.com/Trareon-com/Trareon-Acquire/pull/56
