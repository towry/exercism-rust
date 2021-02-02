use nth_prime as np;

#[test]
// #[ignore]
fn test_is_prime() {
    assert_eq!(np::is_prime(2), true)
}

#[test]
// #[ignore]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
// #[ignore]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
// #[ignore]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}
