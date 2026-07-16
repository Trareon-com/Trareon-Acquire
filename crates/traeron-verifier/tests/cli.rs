use std::process::Command;

#[test]
fn no_arguments_returns_usage_error() {
    let output = Command::new(env!("CARGO_BIN_EXE_traeron-verifier"))
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(64));
    assert!(String::from_utf8_lossy(&output.stderr).contains("usage:"));
}
