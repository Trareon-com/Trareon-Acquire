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
Fix billing in GitHub org settings, then re-run checks on PR #56.

## E. Bounded raw content smoke (1 MiB — not full disk)

After elevated open of `/dev/rdisk10` is **Available**, run a **bounded** sample
only. This never images the whole USB.

```bash
cd "/Users/user/Projects/Trareon/Trareon Acquire"
sudo cargo run -p trareon-core --example lab_raw_bounded_smoke -- \
  /dev/rdisk10 fixtures/lab-allowlists/tiny11-2311-disk10.json 1048576
```

Expect `RAW_BOUNDED_OK` with `bytes=1048576`. Output lands under
`/tmp/trareon-raw-bounded-lab/` (not committed).

To probe the partition node instead, unmount first (destructive to open
handles — only if you intend it):

```bash
# optional — makes disk10s1 not busy
diskutil unmount "/Volumes/tiny11 2311"
```

## Still out of policy without a new explicit gate

- Imaging entire `/dev/rdisk10` into the repo
- Claiming Storage Lab Beta exit / Official Production
- Touching `/dev/disk0` / system disk
