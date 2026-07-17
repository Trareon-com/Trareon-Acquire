use std::fs;
use tempfile::tempdir;
use trareon_acquire_app_lib::run_foundation_demo_inner;

#[test]
fn foundation_demo_produces_verified_complete_package() {
    let dir = tempdir().unwrap();
    let source = dir.path().join("source.img");
    let output_dir = dir.path().join("output");
    fs::write(&source, b"synthetic training fixture, not real evidence").unwrap();

    let result =
        run_foundation_demo_inner(source.to_str().unwrap(), output_dir.to_str().unwrap(), None)
            .expect("foundation demo should succeed on a synthetic fixture");

    assert_eq!(result.status, "verified_complete");
    assert!(trareon_core::verify_fsnap(std::path::Path::new(&result.package_path)).is_ok());
}
