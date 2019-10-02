/* *** Standard *** */
#[test]
#[should_panic]
fn test_ko() {
    assert_eq!(true, false);
}
#[test]
fn test_ok() {
    assert_eq!(true, true);
}
