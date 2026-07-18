#!/usr/bin/env bash
# Build a portable verifier bundle with a minimal package inventory.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
OUT="${1:-$ROOT/dist/trareon-acquire-portable}"
STAGE="$OUT/stage"
ARCHIVE="$OUT/trareon-acquire-portable.zip"

rm -rf "$STAGE"
mkdir -p "$STAGE"
cd "$ROOT"

cargo build --release -p acquire-slint --bin trareon-acquire
cargo build --release -p trareon-verifier --bin trareon-verifier

cp target/release/trareon-acquire "$STAGE/"
cp target/release/trareon-verifier "$STAGE/"
cargo metadata --no-deps --format-version 1 | python3 -c '
import json, sys
metadata = json.load(sys.stdin)
packages = [{"name": p["name"], "version": p["version"]} for p in metadata["packages"]]
json.dump({"schema": "trareon.sbom.stub/1", "packages": packages}, sys.stdout, indent=2)
print()
' > "$STAGE/SBOM.json"

(
  cd "$STAGE"
  shasum -a 256 trareon-acquire trareon-verifier SBOM.json > SHA256SUMS
)
mkdir -p "$OUT"
rm -f "$ARCHIVE"
(
  cd "$STAGE"
  zip -q -r "$ARCHIVE" trareon-acquire trareon-verifier SBOM.json SHA256SUMS
)
echo "$ARCHIVE"
