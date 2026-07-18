use std::{env, fs::File, io::Cursor, path::PathBuf};

use trareon_core::{CaseMetadata, read_e01_to_raw, write_e01};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = env::args()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("usage: format_e01_smoke OUTPUT.E01")?;
    let data: Vec<u8> = (0..=255).cycle().take(256 * 1024).collect();
    let summary = write_e01(
        Cursor::new(&data),
        &output,
        &CaseMetadata {
            case_number: "INTEROP-SMOKE".into(),
            evidence_number: "SYNTHETIC".into(),
            examiner: "automated smoke".into(),
            description: "synthetic bytes only".into(),
            notes: "not evidence".into(),
        },
    )?;
    let mut decoded = Vec::new();
    read_e01_to_raw(&output, &mut decoded)?;
    if decoded != data {
        return Err("roundtrip bytes differ".into());
    }
    File::open(&output)?.sync_all()?;
    println!(
        "E01_SMOKE_OK sha256={} bytes={}",
        summary.sha256, summary.raw_size
    );
    Ok(())
}
