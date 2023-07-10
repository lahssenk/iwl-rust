# Testing

# Unit tests
- annotate each test func with #[test]
- tests should be isolated in their own modules, declared as a test with annotation #[cfg(test)]

Run all tests in workspace with `cargo test`
Run only tests in a specific package with `cargo test -p <packageName>`
Run only lib tests (ignore doc tests etc) with `cargo test --lib`
Run a single test with `cargo test local_test -p testing`

# Integration tests
- integration tests should live in the /tests folder, testing code in the /src folder
- each file in the /tests dir is its own crate
- will only be built when running `cargo test`

