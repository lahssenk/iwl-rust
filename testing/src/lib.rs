// NOTE: this func is not public, but it is still available
// to children modules
fn _return_same(a: usize) -> usize {
    a
}

// make this function public so that we can run an external integration test
pub fn return_same(a: usize) -> usize {
    a
}

// declare a local test
#[test]
fn local_test() {
    let input = 1;
    let res = _return_same(input);
    assert_eq!(res, input);
}

// declare the test in its own module and as a test module
// so that it only gets compiled and run with `cargo test`
#[cfg(test)]
mod tests {
    // bring the parent crate's (testing) symbols to scope
    // (here we bring the private function _return_same into scope)
    use super::*;

    #[test]
    fn isolated_test() {
        let input = 1;
        // need use full path because our func is not in the same module anymore
        let res = _return_same(input);
        assert_eq!(res, input);
    }
}

// documentation tests -----
// Code blocks in /// comments are automatically seen as Rust code
// The code will be compiled and executed as part of cargo test

/// Shortens a string to the given length.
///
/// ```
/// use testing::shorten_string;
/// assert_eq!(shorten_string("Hello World", 5), "Hello");
/// assert_eq!(shorten_string("Hello World", 20), "Hello World");
/// ```
pub fn shorten_string(s: &str, length: usize) -> &str {
    &s[..std::cmp::min(length, s.len())]
}
