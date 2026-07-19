#!/usr/bin/env bash
# Seed fixed synthetic .fsnap packages for manual GUI smoke (Tools / Recent / Compare).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export PATH="${HOME}/.rustup/toolchains/1.96.0-aarch64-apple-darwin/bin:${PATH}"
cd "$ROOT"
cargo test -p acquire-slint --lib \
  tests::seed_fixed_synthetic_packages_for_gui_smoke \
  -- --ignored --exact --nocapture
echo ""
echo "Paths:"
cat "$ROOT/tmp/synthetic-gui-smoke/PATHS.txt"
echo ""
echo "Launch GUI:"
echo "  cargo run -p acquire-slint --features gui"
