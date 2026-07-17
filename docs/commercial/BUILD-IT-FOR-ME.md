# Build It For Me — operator how-to (Hari 36)

Workflow: `.github/workflows/build-it-for-me.yml` (`workflow_dispatch`).

## Steps

1. Open Actions → **Build It For Me**.
2. Choose runner: `ubuntu-latest` / `windows-latest` / `macos-latest`.
3. Download artifact `trareon-acquire-community-build-<platform>`.
4. Read `COMMUNITY-SELF-BUILD.txt` (commit, class, GPLv3 source note).
5. Run `./scripts/self-test.sh` locally when possible; treat artifact as
   **Community/Self Build**, not Official Production.

## Checksums

Prefer `scripts/founder-build.sh` locally for `SHA256SUMS.txt`, or hash the
artifact binary and record it in the fulfillment ledger.
