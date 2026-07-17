use std::path::Path;
use std::process::Command;

#[test]
fn no_arguments_returns_usage_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_trareon-verifier"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(64));
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage:"));
}

fn fixtures_root() -> std::path::PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("fsnap-v0.1")
}

fn verify_fixture(name: &str) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_trareon-verifier"))
        .arg("verify")
        .arg(fixtures_root().join(name))
        .output()
        .unwrap()
}

#[test]
fn golden_valid_package_is_accepted() {
    let output = verify_fixture("valid");
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("VALID"));
}

#[test]
fn info_prints_manifest_fields() {
    let output = Command::new(env!("CARGO_BIN_EXE_trareon-verifier"))
        .arg("info")
        .arg(fixtures_root().join("valid"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("evidence_sha256="));
    assert!(stdout.contains("audit_events="));
}

#[test]
fn hash_only_skips_full_verify_path_but_reads_manifest() {
    let output = Command::new(env!("CARGO_BIN_EXE_trareon-verifier"))
        .args(["verify", "--hash-only"])
        .arg(fixtures_root().join("valid"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).starts_with("HASH"));
}

#[test]
fn integrity_accepts_valid() {
    let output = Command::new(env!("CARGO_BIN_EXE_trareon-verifier"))
        .arg("integrity")
        .arg(fixtures_root().join("valid"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("INTEGRITY_OK"));
}

#[test]
fn compare_identical_packages() {
    let output = Command::new(env!("CARGO_BIN_EXE_trareon-verifier"))
        .arg("compare")
        .arg(fixtures_root().join("valid"))
        .arg(fixtures_root().join("valid"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(0));
    assert!(String::from_utf8_lossy(&output.stdout).contains("IDENTICAL"));
}

#[test]
fn golden_mutated_evidence_is_rejected() {
    let output = verify_fixture("mutated");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("INVALID"));
}

#[test]
fn golden_truncated_evidence_is_rejected() {
    let output = verify_fixture("truncated");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn golden_removed_file_is_rejected() {
    let output = verify_fixture("removed-file");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn golden_audit_discontinuous_is_rejected() {
    let output = verify_fixture("audit-discontinuous");
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn golden_unsupported_version_is_rejected() {
    let output = verify_fixture("unsupported-version");
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).contains("unsupported fsnap manifest schema"));
}
