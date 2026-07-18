#!/usr/bin/env bash
# Builds an unsigned RC staging artifact. Human signing/notarization is intentionally excluded.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

OUT_DIR="${1:-dist/rc}"
mkdir -p "$OUT_DIR"

cargo build -p acquire-slint --features gui --release --locked
cargo test -p trareon-core --locked
cargo test -p trareon-analysis --locked

TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
BIN="$TARGET_DIR/release/trareon-acquire"
EXT=""
case "$(uname -s)" in
  MINGW*|MSYS*|CYGWIN*|Windows_NT) BIN="${BIN}.exe"; EXT=".exe" ;;
esac
[[ -f "$BIN" ]] || { echo "missing release binary: $BIN" >&2; exit 1; }

cp "$BIN" "$OUT_DIR/trareon-acquire${EXT}"
if command -v shasum >/dev/null 2>&1; then
  (cd "$OUT_DIR" && shasum -a 256 "trareon-acquire${EXT}" > SHA256SUMS.txt)
else
  (cd "$OUT_DIR" && sha256sum "trareon-acquire${EXT}" > SHA256SUMS.txt)
fi

{
  echo "SPDXVersion: SPDX-2.3"
  echo "DocumentName: Trareon-Acquire-RC-PENDING-HUMAN-SBOM"
  echo "DocumentComment: Stub only. Replace with an approved generated SBOM before release."
  echo "PackageName: trareon-acquire"
  echo "PackageVersion: $(git rev-parse HEAD)"
} > "$OUT_DIR/SBOM.spdx"

{
  echo "commit=$(git rev-parse HEAD)"
  echo "status=PENDING_HUMAN_SIGN_OFF"
  echo "signing=NOT_PERFORMED"
  echo "notarization=NOT_PERFORMED"
} > "$OUT_DIR/RELEASE-STATUS.txt"

printf 'RC_BUILD_READY dir=%s status=PENDING_HUMAN_SIGN_OFF\n' "$OUT_DIR"
