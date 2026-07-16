#[test]
fn build_identity_is_stable() {
    assert_eq!(
        trareon_core::build_identity(),
        "trareon-acquire-foundation-v1"
    );
}
