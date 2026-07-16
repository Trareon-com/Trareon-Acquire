//! Regenerates the golden `.fsnap` fixtures under `fixtures/fsnap-v0.1/`.
//!
//! These fixtures are committed to the repository; this is not run as part
//! of the normal test suite. Regenerate deliberately with:
//!
//! ```bash
//! cargo test -p traeron-verifier --test generate_fixtures -- --ignored --nocapture
//! ```

use std::fs;
use std::path::{Path, PathBuf};

use traeron_core::{AcquireRequest, acquire_file, create_fsnap};

fn fixtures_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("fixtures")
        .join("fsnap-v0.1")
}

fn build_valid_package(dir: &Path, package_name: &str) -> PathBuf {
    fs::create_dir_all(dir).unwrap();
    let source = dir.join("source.img");
    let evidence = dir.join("evidence.raw");
    let package = dir.join(package_name);
    fs::write(
        &source,
        b"deterministic golden fixture content, not real evidence",
    )
    .unwrap();

    let summary = acquire_file(&AcquireRequest::new(&source, &evidence)).unwrap();
    create_fsnap(&evidence, &summary.audit_path, &package).unwrap();
    package
}

fn copy_dir(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_dir(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), &target).unwrap();
        }
    }
}

#[test]
#[ignore]
fn regenerate_golden_fsnap_fixtures() {
    let root = fixtures_root();
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root).unwrap();

    let staging = tempfile::tempdir().unwrap();

    // valid: untouched package, must verify OK.
    let valid_src = build_valid_package(&staging.path().join("valid"), "case.fsnap");
    copy_dir(&valid_src, &root.join("valid"));

    // mutated: evidence bytes changed after packaging.
    let mutated_src = build_valid_package(&staging.path().join("mutated"), "case.fsnap");
    fs::write(
        mutated_src.join("acquisitions/0001/evidence.raw"),
        b"tampered bytes replacing the original evidence",
    )
    .unwrap();
    copy_dir(&mutated_src, &root.join("mutated"));

    // truncated: evidence shorter than the manifest's recorded size.
    let truncated_src = build_valid_package(&staging.path().join("truncated"), "case.fsnap");
    let evidence_path = truncated_src.join("acquisitions/0001/evidence.raw");
    let original = fs::read(&evidence_path).unwrap();
    fs::write(&evidence_path, &original[..original.len() / 2]).unwrap();
    copy_dir(&truncated_src, &root.join("truncated"));

    // removed-file: audit.jsonl deleted entirely.
    let removed_src = build_valid_package(&staging.path().join("removed-file"), "case.fsnap");
    fs::remove_file(removed_src.join("audit/audit.jsonl")).unwrap();
    copy_dir(&removed_src, &root.join("removed-file"));

    // audit-discontinuous: audit chain hash-linkage broken.
    let discontinuous_src =
        build_valid_package(&staging.path().join("audit-discontinuous"), "case.fsnap");
    let audit_path = discontinuous_src.join("audit/audit.jsonl");
    let contents = fs::read_to_string(&audit_path).unwrap();
    let mut lines: Vec<String> = contents.lines().map(|l| l.to_string()).collect();
    // Flip the recorded action on the first event without recomputing its
    // hash, breaking the chain the same way a tamper attempt would.
    lines[0] = lines[0].replace("\"action\":\"planned\"", "\"action\":\"modified\"");
    fs::write(&audit_path, lines.join("\n") + "\n").unwrap();
    copy_dir(&discontinuous_src, &root.join("audit-discontinuous"));

    // unsupported-version: manifest schema bumped to a value this build
    // does not recognize.
    let unsupported_src =
        build_valid_package(&staging.path().join("unsupported-version"), "case.fsnap");
    let manifest_path = unsupported_src.join("manifest/manifest.json");
    let manifest_contents = fs::read_to_string(&manifest_path).unwrap();
    let bumped = manifest_contents.replace(
        "\"trareon.fsnap.manifest/1\"",
        "\"trareon.fsnap.manifest/999\"",
    );
    assert_ne!(bumped, manifest_contents, "schema string must be present");
    fs::write(&manifest_path, bumped).unwrap();
    copy_dir(&unsupported_src, &root.join("unsupported-version"));

    println!("golden fixtures regenerated at {}", root.display());
}
