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
   - Package: `/tmp/trareon-raw-bounded-lab/bounded-1048576.fsnap` (not committed)
   - Independent re-verify: `trareon-verifier verify` → `VALID` same SHA/size
7. **Bounded raw content smoke (operator `sudo`, 64 MiB):**
   - Command: `lab_raw_bounded_smoke /dev/rdisk10 … 67108864`
   - Result: `RAW_BOUNDED_OK` bytes=`67108864`
   - SHA-256: `a0ff3432080bcd12f2e34f2a9ebb0c7b1388ae811ca7f45f4953eefc767cfe5f`
   - Package: `/tmp/trareon-raw-bounded-lab/bounded-67108864.fsnap` (not committed)
   - Independent verify (operator + agent re-check): `VALID` same SHA/size
8. **Full-disk raw acquire (operator `sudo`, gate → `/Volumes/Untitled`):**
   - Command: `lab_raw_full_disk --i-approve-full-disk /dev/rdisk10 … /Volumes/Untitled/trareon-lab 61524148224`
   - Result: `RAW_FULL_OK` bytes=`61524148224`
   - SHA-256: `23e039c2e71bfc8b90ce4d1e76c18a0edd3a1e7fac8f864dd4465a66751e1d6c`
   - Evidence: `/Volumes/Untitled/trareon-lab/tiny11-rdisk10-full.raw` (not committed)
   - Package: `/Volumes/Untitled/trareon-lab/tiny11-rdisk10-full.fsnap` (not committed)
   - Independent verify: `trareon-verifier verify` → `VALID` same SHA/size (~19 min)

## Capability claims

| Capability | Status |
|---|---|
| Lab allowlist for this removable media | Implemented (policy) |
| File-backed acquire of a file *on* the volume | PASS |
| Elevated open of `disk10` / `rdisk10` | PASS |
| Elevated open of mounted `disk10s1` | Busy while mounted |
| Bounded raw content sample (1 MiB of `rdisk10`) | **PASS** (lab smoke) |
| Bounded raw content sample (64 MiB of `rdisk10`) | **PASS** (lab smoke) |
| Full-disk raw acquire of `rdisk10` → Untitled | **PASS** (lab; not Lab Beta exit) |
| Writable staging on this volume | **NotValidated** (RO mount) |

## Explicitly not done

- No commit of acquired evidence bytes
- No Official Production / Lab Beta exit claim

## Agent elevation attempt (2026-07-17)

- `sudo -n` open-only probe: **failed** (`sudo: a password is required`) — agent cannot enter interactive admin password.
- Operator-run elevated steps (open, bounded samples, full-disk) are authoritative for this report.

## Next

- Optional: unmount volume then probe `/dev/disk10s1`
- Restore GitHub Actions billing for normal CI
