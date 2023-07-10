// each file in this dir is its own crate.
use testing;

#[test]
fn integration_test() {
    let input = 1;
    let res = testing::return_same(input);
    assert_eq!(input, res);
}
