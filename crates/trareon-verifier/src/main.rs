use std::{env, path::Path, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 || args[1] != "verify" {
        eprintln!("usage: trareon-verifier verify PATH");
        return ExitCode::from(64);
    }

    match trareon_core::verify_fsnap(Path::new(&args[2])) {
        Ok(manifest) => {
            println!(
                "VALID {} {}",
                manifest.evidence_sha256, manifest.evidence_size
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("INVALID {error}");
            ExitCode::from(2)
        }
    }
}
