use std::{env, path::Path, process::ExitCode};

use trareon_verifier::{compare, export, hash_only, info, verify};

fn usage() -> ExitCode {
    eprintln!(
        "usage:
  trareon-verifier verify PATH
  trareon-verifier verify --hash-only PATH
  trareon-verifier info PATH
  trareon-verifier compare A.fsnap B.fsnap
  trareon-verifier export PATH -o OUT"
    );
    ExitCode::from(64)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let Some(command) = args.get(1) else {
        return usage();
    };
    match command.as_str() {
        "verify" => cmd_verify(&args[2..]),
        "info" => cmd_info(&args[2..]),
        "compare" => cmd_compare(&args[2..]),
        "export" => cmd_export(&args[2..]),
        _ => usage(),
    }
}

fn cmd_verify(args: &[String]) -> ExitCode {
    let (is_hash_only, path) = match args {
        [flag, path] if flag == "--hash-only" => (true, path.as_str()),
        [path] => (false, path.as_str()),
        _ => return usage(),
    };
    let result = if is_hash_only {
        hash_only(Path::new(path)).map(|(hash, size)| format!("HASH {hash} {size}"))
    } else {
        verify(Path::new(path)).map(|manifest| {
            format!(
                "VALID {} {}",
                manifest.evidence_sha256, manifest.evidence_size
            )
        })
    };
    match result {
        Ok(output) => {
            println!("{output}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("INVALID {error}");
            ExitCode::from(2)
        }
    }
}

fn cmd_info(args: &[String]) -> ExitCode {
    let [path] = args else {
        return usage();
    };
    match info(Path::new(path)) {
        Ok(info) => {
            println!("schema={}", info.schema);
            println!("build_identity={}", info.build_identity);
            println!("evidence_sha256={}", info.evidence_sha256);
            println!("evidence_size={}", info.evidence_size);
            println!("evidence_path={}", info.evidence_path);
            println!("segments={}", info.segments);
            println!("audit_root={}", info.audit_root);
            println!("audit_events={}", info.audit_events);
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("INVALID {error}");
            ExitCode::from(2)
        }
    }
}

fn cmd_compare(args: &[String]) -> ExitCode {
    let [a, b] = args else {
        return usage();
    };
    let a_path = Path::new(a);
    let b_path = Path::new(b);
    match (compare(a_path, b_path), verify(a_path), verify(b_path)) {
        (Ok(true), Ok(a), Ok(_)) => {
            println!("IDENTICAL {} {}", a.evidence_sha256, a.evidence_size);
            ExitCode::SUCCESS
        }
        (Ok(false), Ok(a), Ok(b)) => {
            eprintln!("DIFFER");
            eprintln!("A {} {}", a.evidence_sha256, a.evidence_size);
            eprintln!("B {} {}", b.evidence_sha256, b.evidence_size);
            ExitCode::from(1)
        }
        (Err(error), _, _) | (_, Err(error), _) | (_, _, Err(error)) => {
            eprintln!("INVALID {error}");
            ExitCode::from(2)
        }
    }
}

fn cmd_export(args: &[String]) -> ExitCode {
    if args.len() != 3 || args[1] != "-o" {
        return usage();
    }
    let output = Path::new(&args[2]);
    match export(Path::new(&args[0]), output) {
        Ok(()) => {
            println!("EXPORTED {}", output.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("INVALID {error}");
            ExitCode::from(2)
        }
    }
}
