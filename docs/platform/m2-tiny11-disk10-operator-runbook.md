# Operator runbook — next steps for tiny11 / `disk10`

Lab media already designated: `/Volumes/tiny11 2311` → `/dev/disk10`.  
Allowlist: `fixtures/lab-allowlists/tiny11-2311-disk10.json`.  
Trial report: `docs/platform/m2-lab-tiny11-2311-disk10.md`.

Current account is **admin** but **not** in group `operator`. Raw nodes are
`root:operator` mode `640`, so open fails without elevation/helper.

## A. File-backed only (already PASS)

```bash
cargo run -p trareon-core --example lab_volume_smoke -- \
  "/Volumes/tiny11 2311" fixtures/lab-allowlists/tiny11-2311-disk10.json
```

## B. Enable raw open (human-operated — choose one)

### B1. Temporary elevated probe (preferred for one-shot lab)

In Terminal (will prompt for admin password):

```bash
# Open-only probe — does NOT image the disk
sudo python3 - <<'PY'
from pathlib import Path
for p in ["/dev/disk10", "/dev/rdisk10", "/dev/disk10s1"]:
    try:
        open(p, "rb").close()
        print(p, "Available")
    except PermissionError:
        print(p, "DeniedInsufficientPrivilege")
    except OSError as e:
        print(p, "NotValidated", e)
PY
```

Record the printed lines into the lab report. **Do not** `dd` the whole USB
unless you explicitly intend a full raw acquire in a later gated step.

### B2. Add user to `operator` (persistent; reboot/re-login)

```bash
sudo dseditgroup -o edit -a "$(whoami)" -t user operator
# then log out/in and re-check: id | tr ',' '\n' | grep operator
```

## C. Writable staging on the volume

NTFS is mounted **read-only** on this Mac. Options:

1. Remount with a paid/commercial NTFS writer, or
2. Keep evidence output under `/tmp` / local APFS (current smoke path), or
3. Reformat **only if** the USB is expendable lab media (destructive — separate human gate).

## D. GitHub Actions

PR CI may fail with billing/spending-limit errors unrelated to this branch.
Fix billing in GitHub org settings, then re-run checks on open PRs.

## E. Bounded raw content smoke (1 MiB — PASS)

```bash
cd "/Users/user/Projects/Trareon/Trareon Acquire"
sudo cargo run -p trareon-core --example lab_raw_bounded_smoke -- \
  /dev/rdisk10 fixtures/lab-allowlists/tiny11-2311-disk10.json 1048576
```

Recorded: `RAW_BOUNDED_OK` bytes=`1048576`, SHA
`41fb8d926780c7eb45521713b3f5df286c2e06d1627df47ac03934059ff4c313`.
Package naming (current example): `/tmp/trareon-raw-bounded-lab/bounded-1048576.fsnap`.

## F. Larger bound (64 MiB — PASS, still not full disk)

```bash
cd "/Users/user/Projects/Trareon/Trareon Acquire"
sudo cargo run -p trareon-core --example lab_raw_bounded_smoke -- \
  /dev/rdisk10 fixtures/lab-allowlists/tiny11-2311-disk10.json 67108864
cargo run -q -p trareon-verifier -- verify /tmp/trareon-raw-bounded-lab/bounded-67108864.fsnap
```

Recorded: `RAW_BOUNDED_OK` bytes=`67108864`, SHA
`a0ff3432080bcd12f2e34f2a9ebb0c7b1388ae811ca7f45f4953eefc767cfe5f`, verifier `VALID`.

## G. Partition node after unmount (optional)

```bash
diskutil unmount "/Volumes/tiny11 2311"
# then elevated open / bounded smoke against /dev/disk10s1
```

## H. Full-disk acquire → `/Volumes/Untitled` (human-gated)

Decision: `docs/ai-operations/DECISIONS/2026-07-17-m2-fulldisk-rdisk10-to-untitled.md`  
Declared size: `61524148224` bytes. Destination ExFAT USB (`disk11`), **not** internal Mac (~3.5 GiB free).

```bash
# Recommended: unmount the tiny11 volume first
diskutil unmount "/Volumes/tiny11 2311"

cd "/Users/user/Projects/Trareon/Trareon Acquire"
sudo cargo run -p trareon-core --example lab_raw_full_disk -- \
  --i-approve-full-disk \
  /dev/rdisk10 \
  fixtures/lab-allowlists/tiny11-2311-disk10.json \
  /Volumes/Untitled/trareon-lab \
  61524148224

# After RAW_FULL_OK (may take a long time):
cargo run -q -p trareon-verifier -- verify /Volumes/Untitled/trareon-lab/tiny11-rdisk10-full.fsnap
```

Re-run the same command to **resume** if interrupted (checkpoint under the out dir).  
Do **not** point destination at `/Volumes/tiny11 2311` or at Macintosh HD.

## Still out of policy without a new explicit gate

- Imaging entire `/dev/rdisk10` to any path **other than** the approved Untitled lab folder without a new decision
- Claiming Storage Lab Beta exit / Official Production
- Touching `/dev/disk0` / system disk
