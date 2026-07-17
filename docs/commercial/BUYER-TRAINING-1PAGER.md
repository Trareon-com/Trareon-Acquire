# Buyer training — 1 pager (Hari 79 draft)

## What Trareon Acquire does

Streams a **file-backed or allowlisted lab** source to RAW, records a hash-chained
audit, packages `.fsnap`, and verifies with an independent CLI. The UI never invents success.

## Safe first run

1. Run `./scripts/self-test.sh` (or CI equivalent).
2. `cargo run -p acquire-slint --features gui`
3. **Fill synthetic demo paths** → confirm checkbox → **Run**.
4. Optionally **Cancel** mid-run (status must not become Verified Complete).
5. Verify with `trareon-verifier`.

## Hard rules

- No real evidence until capability matrix says so for that OS/media.
- No `PhysicalDrive0` / system disks.
- Unsigned builds need SmartScreen/Gatekeeper steps — see `docs/install/`.
- GPLv3 source accompanies binaries.

## When something fails

- Failed / Cancelled ≠ Verified Complete.
- Keep the package; run `trareon-verifier` and file a GitHub issue with OS,
  commit SHA, and redacted logs (no evidence bytes).
