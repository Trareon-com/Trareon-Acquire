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
| Privilege | Device nodes `root:operator` mode `640`; elevated open via operator `sudo` |
| Host | MacBook Pro (operator `MBPU`), Darwin session 2026-07-17 |

## What was tried

1. **Allowlist gate** — `/dev/disk10`, `/dev/rdisk10`, `/dev/disk10s1` accepted with `human_approved: true`. Volume directory path correctly refused as non-file source.
2. **Raw open probe (unelevated)** — all three → `DeniedInsufficientPrivilege`.
3. **File-backed smoke** — acquired `autorun.inf` (128 bytes) from the volume to `/tmp/trareon-tiny11-lab/`, packaged + verified. SHA-256 `3378723cb5910e5f3afe1ee2200b8f0d08bab8c5d77f7ce9fcb5725aff525852`.
4. **Write to volume** — failed (`Read-only file system`); expected for NTFS without write driver.
5. **Elevated open-only probe (operator Terminal):**
   - `/dev/disk10` → **Available**
   - `/dev/rdisk10` → **Available**
   - `/dev/disk10s1` → `NotValidated` (`Resource busy`) while mounted
6. **Bounded raw content smoke (operator `sudo`, 1 MiB):**
   - Command: `lab_raw_bounded_smoke /dev/rdisk10 … 1048576`
   - Result: `RAW_BOUNDED_OK` bytes=`1048576`
   - SHA-256: `41fb8d926780c7eb45521713b3f5df286c2e06d1627df47ac03934059ff4c313`
   - Package: `/tmp/trareon-raw-bounded-lab/bounded.fsnap` (not committed)
   - Independent re-verify: `trareon-verifier verify` → `VALID` same SHA/size

## Capability claims

| Capability | Status |
|---|---|
| Lab allowlist for this removable media | Implemented (policy) |
| File-backed acquire of a file *on* the volume | PASS |
| Elevated open of `disk10` / `rdisk10` | PASS |
| Elevated open of mounted `disk10s1` | Busy while mounted |
| Bounded raw content sample (1 MiB of `rdisk10`) | **PASS** (lab smoke; not full disk) |
| Full-disk raw acquire of `rdisk10` | **NotValidated** / not requested |
| Writable staging on this volume | **NotValidated** (RO mount) |

## Explicitly not done

- No full-disk image of the USB (~57 GiB)
- No commit of acquired evidence bytes
- No Official Production / Lab Beta exit claim
- Bounded sample ≠ complete media acquisition

## Agent elevation attempt (2026-07-17)

- `sudo -n` open-only probe: **failed** (`sudo: a password is required`) — agent cannot enter interactive admin password.
- Operator-run elevated steps (open + 1 MiB bounded acquire) are authoritative for this report.

## Next

- Optional next gate: **64 MiB** bounded sample (runbook §F) — still not full disk
- Optional: unmount volume then probe `/dev/disk10s1`
- Full-disk acquire only with a new human gate
- Restore GitHub Actions billing for normal CI
