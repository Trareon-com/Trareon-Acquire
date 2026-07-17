use std::{env, path::Path, process::ExitCode};

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 || args[1] != "import" || args[3] != "--index-dir" {
        eprintln!("usage: trareon-analysis import PACKAGE --index-dir DIR");
        return ExitCode::from(64);
    }

    match trareon_analysis::import_fsnap_readonly(Path::new(&args[2]), Path::new(&args[4])) {
        Ok((manifest, index_path)) => {
            println!(
                "IMPORTED {} {} {}",
                manifest.evidence_sha256,
                manifest.evidence_size,
                index_path.display()
            );
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("REJECTED {error}");
            ExitCode::from(2)
        }
    }
}
