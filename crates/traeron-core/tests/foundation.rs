#[test]
fn build_identity_is_stable() {
    assert_eq!(
        traeron_core::build_identity(),
        "traeron-acquire-foundation-v1"
    );
}
