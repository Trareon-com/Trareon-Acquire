# macOS elevation — operator guide (commercial v1)

Status: **operator documentation** — not FDA, not Official Production.

## Paths to read allowlisted raw devices

| Method | Cost | Persistence | Agent can automate? |
|--------|------|-------------|---------------------|
| `sudo` one-shot in Terminal | Rp0 | Per command | **No** (password) |
| User in `operator` group | Rp0 | After re-login | No (still no password for agent) |
| Reviewed helper binary (M2-P04) | Dev time | Yes | Future |

Device nodes are typically `root:operator` mode `640` for `/dev/rdisk*`.

## Recommended lab flow (tiny11 / disk10)

1. Designate media in allowlist (`human_approved: true`).
2. Prefer output on **different** USB (e.g. `/Volumes/Untitled`), not internal disk.
3. Optional: `diskutil unmount "/Volumes/tiny11 2311"` before partition/raw acquire.
4. Run bounded smoke before full-disk:

```bash
sudo cargo run -p trareon-core --example lab_raw_bounded_smoke -- \
  /dev/rdisk10 fixtures/lab-allowlists/tiny11-2311-disk10.json 1048576
```

5. Full-disk only with explicit gate + `lab_raw_full_disk --i-approve-full-disk`.

## Group `operator` (optional persistent)

```bash
sudo dseditgroup -o edit -a "$(whoami)" -t user operator
# log out and back in
id | tr ',' '\n' | grep operator
```

## What we do not claim

- Full Disk Access (FDA) as forensic fitness proof
- Works on every macOS version or Intel without separate evidence
- Agent/CI can acquire raw without human `sudo`

Evidence ladder: `docs/platform/m2-p03-macos-helper-boundary-plan.md`
