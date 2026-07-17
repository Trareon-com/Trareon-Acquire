#!/usr/bin/env bash
# Operator-only: Hari 3 disk10s1 bounded smoke (1 MiB) + independent verify.
# Requires interactive sudo. Does not commit evidence bytes.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

if [[ ! -e /dev/disk10s1 ]]; then
  echo "disk10s1 missing — attach tiny11 USB first"
  exit 1
fi

if mount | grep -q ' on /Volumes/tiny11 2311 '; then
  echo "tiny11 still mounted — unmount first:"
  echo "  diskutil unmount \"/Volumes/tiny11 2311\""
  exit 1
fi

echo "Running elevated bounded smoke on /dev/disk10s1 (1 MiB)…"
sudo cargo run -p trareon-core --example lab_raw_bounded_smoke -- \
  /dev/disk10s1 fixtures/lab-allowlists/tiny11-2311-disk10.json 1048576

echo "Independent verify…"
cargo run -q -p trareon-verifier -- verify /tmp/trareon-raw-bounded-lab/bounded-1048576.fsnap

echo "Paste WIN/RAW lines into docs/platform/m2-lab-tiny11-2311-disk10.md and COMMERCIAL-LAUNCH-STATUS.md"
