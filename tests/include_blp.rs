use gtk_blueprint::include_blp;

#[test]
fn successful() {
    let blueprint = include_blp!("tests/sample.blp");
    let expected = include_str!("expected.ui");
    assert_eq!(blueprint, expected)
}