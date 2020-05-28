# Data-Test
Rust data-test (aka table-test) helpers

[![Build Status](https://travis-ci.com/jhg/data-test-rs.svg?branch=master)](https://travis-ci.com/jhg/data-test-rs)
![GitHub](https://img.shields.io/github/license/jhg/data-test-rs)
![Crates.io](https://img.shields.io/crates/v/data-test)

## Getting Started
Add to your test dependencies:
```toml
[dev-dependencies]
data-test = "^0.1"
```

Example of usage:
```rust
#[cfg(test)]
mod tests {
    use data_test::data_test;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    data_test! {
        fn is_equal(input, expected) => {
            assert_eq!(input, expected);
        }
        - a (1, 1)
        - b (2, 2)
        - c (3, 3)
        - d (4, 4)
        - e (5, 5)
        - f ("hello world", "hello world")
    }
}
// cargo test output:
// test tests::it_works ... ok
// test tests::is_equal::a ... ok
// test tests::is_equal::b ... ok
// test tests::is_equal::c ... ok
// test tests::is_equal::d ... ok
// test tests::is_equal::e ... ok
// test tests::is_equal::f ... ok
```
