use std::{
    env, fs,
    path::{Path, PathBuf},
    process::ExitCode,
};

use trareon_core::{FsnapManifestV1, verify_fsnap};

fn usage() -> ExitCode {
    eprintln!(
        "usage:
  trareon-verifier verify PATH
  trareon-verifier verify --hash-only PATH
  trareon-verifier info PATH
  trareon-verifier compare A.fsnap B.fsnap
  trareon-verifier export PATH -o OUT
  trareon-verifier integrity PATH"
    );
    ExitCode::from(64)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return usage();
    }
    match args[1].as_str() {
        "verify" => cmd_verify(&args[2..]),
        "info" => cmd_info(&args[2..]),
        "compare" => cmd_compare(&args[2..]),
        "export" => cmd_export(&args[2..]),
        "integrity" => cmd_integrity(&args[2..]),
        _ => usage(),
    }
}

fn cmd_verify(args: &[String]) -> ExitCode {
    let (hash_only, path) = match args {
        [flag, path] if flag == "--hash-only" => (true, path.as_str()),
        [path] => (false, path.as_str()),
        _ => return usage(),
    };
    if hash_only {
        match load_manifest(Path::new(path)) {
            Ok(m) => {
                println!("HASH {} {}", m.evidence_sha256, m.evidence_size);
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("INVALID {e}");
                ExitCode::from(2)
            }
        }
    } else {
        match verify_fsnap(Path::new(path)) {
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
}

fn cmd_info(args: &[String]) -> ExitCode {
    let [path] = args else {
        return usage();
    };
    match load_manifest(Path::new(path)) {
        Ok(m) => {
            let segments = m
                .evidence_segments
                .as_ref()
                .map(|s| s.len())
                .unwrap_or(1);
            let audit_lines = fs::read_to_string(Path::new(path).join(&m.audit_relative_path))
                .map(|t| t.lines().count())
                .unwrap_or(0);
            println!("schema={}", m.schema);
            println!("build_identity={}", m.build_identity);
            println!("evidence_sha256={}", m.evidence_sha256);
            println!("evidence_size={}", m.evidence_size);
            println!("evidence_path={}", m.evidence_relative_path);
            println!("segments={}", segments);
            println!("audit_root={}", m.audit_root);
            println!("audit_events={}", audit_lines);
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("INVALID {e}");
            ExitCode::from(2)
        }
    }
}

fn cmd_compare(args: &[String]) -> ExitCode {
    let [a, b] = args else {
        return usage();
    };
    match (verify_fsnap(Path::new(a)), verify_fsnap(Path::new(b))) {
        (Ok(ma), Ok(mb)) => {
            if ma.evidence_sha256 == mb.evidence_sha256
                && ma.evidence_size == mb.evidence_size
                && ma.audit_root == mb.audit_root
            {
                println!("IDENTICAL {} {}", ma.evidence_sha256, ma.evidence_size);
                ExitCode::SUCCESS
            } else {
                eprintln!("DIFFER");
                eprintln!("A {} {}", ma.evidence_sha256, ma.evidence_size);
                eprintln!("B {} {}", mb.evidence_sha256, mb.evidence_size);
                ExitCode::from(1)
            }
        }
        (Err(e), _) | (_, Err(e)) => {
            eprintln!("INVALID {e}");
            ExitCode::from(2)
        }
    }
}

fn cmd_export(args: &[String]) -> ExitCode {
    // export PATH -o OUT
    if args.len() != 3 || args[1] != "-o" {
        return usage();
    }
    let package = Path::new(&args[0]);
    let out = Path::new(&args[2]);
    match verify_fsnap(package) {
        Ok(m) => {
            if let Some(segs) = &m.evidence_segments {
                if let Err(e) = export_segments(package, segs, out) {
                    eprintln!("INVALID {e}");
                    return ExitCode::from(2);
                }
            } else {
                let src = package.join(&m.evidence_relative_path);
                if let Err(e) = fs::copy(&src, out) {
                    eprintln!("INVALID {e}");
                    return ExitCode::from(2);
                }
            }
            println!("EXPORTED {}", out.display());
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("INVALID {e}");
            ExitCode::from(2)
        }
    }
}

fn cmd_integrity(args: &[String]) -> ExitCode {
    let [path] = args else {
        return usage();
    };
    match verify_fsnap(Path::new(path)) {
        Ok(m) => {
            println!(
                "INTEGRITY_OK {} {} audit_root={}",
                m.evidence_sha256, m.evidence_size, m.audit_root
            );
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("INVALID {e}");
            ExitCode::from(2)
        }
    }
}

fn load_manifest(package: &Path) -> Result<FsnapManifestV1, String> {
    let path = package.join("manifest/manifest.json");
    let bytes = fs::read(&path).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map_err(|e| e.to_string())
}

fn export_segments(
    package: &Path,
    segs: &[trareon_core::EvidenceSegmentV1],
    out: &Path,
) -> Result<(), String> {
    use std::io::Write;
    let mut file = fs::File::create(out).map_err(|e| e.to_string())?;
    for seg in segs {
        let bytes = fs::read(package.join(&seg.relative_path)).map_err(|e| e.to_string())?;
        file.write_all(&bytes).map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[allow(dead_code)]
fn _pathbuf(s: &str) -> PathBuf {
    PathBuf::from(s)
}
