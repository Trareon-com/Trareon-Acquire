#!/usr/bin/env bash
# Wave 3a: spike ewf-image write outside the workspace MSRV gate.
# Does not modify Trareon crates. Exit 0 only if a segment file is produced.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORK="${TMPDIR:-/tmp}/trareon-ewf-spike-$$"
RAW="$WORK/sample.raw"
OUT="$WORK/out"
mkdir -p "$OUT"
dd if=/dev/urandom of="$RAW" bs=1m count=4 status=none 2>/dev/null \
  || dd if=/dev/urandom of="$RAW" bs=1048576 count=4 2>/dev/null

mkdir -p "$WORK/crate"
cd "$WORK/crate"
cargo init --lib --name trareon_ewf_spike -q
cat >> Cargo.toml <<'EOF'
ewf-image = "0.2"
EOF

cat > src/lib.rs <<'EOF'
pub fn write_sample(raw: &std::path::Path, image: &std::path::Path) -> ewf_image::Result<()> {
    let mut input = std::fs::File::open(raw)?;
    let mut options = ewf_image::WriteOptions {
        format: ewf_image::WriteFormat::Ewf2Physical,
        compression: ewf_image::WriteCompression::Zlib,
        ..ewf_image::WriteOptions::default()
    };
    options.metadata.set_header_value("case_number", "TRAREON-SPIKE");
    options.metadata.set_header_value("acquiry_software", "trareon-ewf-spike");
    let mut writer = ewf_image::EwfWriter::create(image, options)?;
    std::io::copy(&mut input, &mut writer)?;
    let result = writer.finish()?;
    eprintln!("segments={}", result.segment_paths.len());
    for p in result.segment_paths {
        eprintln!("wrote {}", p.display());
    }
    Ok(())
}
EOF

cat > src/main.rs <<EOF
fn main() -> ewf_image::Result<()> {
    let raw = std::path::Path::new("$RAW");
    let image = std::path::Path::new("$OUT/spike.Ex01");
    trareon_ewf_spike::write_sample(raw, image)
}
EOF

# ewf-image MSRV is 1.96; workspace is 1.95 — spike only.
cargo run --ignore-rust-version --quiet

echo "SPIKE_OK workdir=$WORK"
echo "NOTE: do not label product UI as E01 until libewf oracle + Autopsy/FTK evidence"
ls -la "$OUT"
