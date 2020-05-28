//! # Data-Test
//! 
//! Aka table-test.


#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]


/// Create sub module with tests for multiple input data
/// 
/// # Examples
/// ```no_run
/// #[cfg(test)]
/// mod tests {
///     use data_test::data_test;
/// 
///     #[test]
///     fn it_works() {
///         assert_eq!(2 + 2, 4);
///     }
/// 
///     data_test!{
///         fn is_equal(input, expected) => {
///             assert_eq!(input, expected);
///         }
///         - a (1, 1)
///         - b (2, 2)
///         - c (3, 3)
///         - d (4, 4)
///         - e (5, 5)
///         - f ("hello world", "hello world")
///     }
/// }
/// // cargo test output:
/// // test tests::it_works ... ok
/// // test tests::is_equal::a ... ok
/// // test tests::is_equal::b ... ok
/// // test tests::is_equal::c ... ok
/// // test tests::is_equal::d ... ok
/// // test tests::is_equal::e ... ok
/// // test tests::is_equal::f ... ok
/// ```
// NOTE: the doc test only can check if compile but doc test does not run tests inside of doc test, then use no_run attribute
#[macro_export]
macro_rules! data_test {
    {
        $prefix:ident $input:pat => $main:block
        $(- $name:ident $($value:tt),*)*
    } => {
        mod $prefix {
            $(
                #[test]
                fn $name () {
                    let $input = ($($value),*);
                    $main
                }
            )*
        }
    };
    {$(
        $prefix:ident $input:pat => $main:block
        $(- $name:ident $($value:tt),*)*
    )*} => {$(
        $crate::data_test!{
            $prefix $input => $main
            $(- $name ($($value),*))*
        }
    )*};
}


#[cfg(test)]
mod tests {
    use super::data_test;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    data_test!{
        is_equal(input, expected) => {
            assert_eq!(input, expected);
        }
        - a (1, 1)
        - b (1 + 1, 2)
        - c ("hello", "hello")
        - d ("\u{1f9ea}", "\u{1f9ea}")
    }

    data_test!{
        is_not_equal(input, expected) => {
            assert_ne!(input, expected);
        }
        - a (1, 2)
        - b 1, 3
        - c (1, 4)

        is_not_zero input => {
            assert_ne!(input, 0);
        }
        - a 1
        - b (2)
        - c 3

        test_multiplication(first, second, expected) => {
            assert_eq!(first * second, expected);
        }
        - a (2, 2,   4)
        - b (2, 3,   6)
        - c (3, 3,   9)
        - d (9, 9, 9*9)
    }
}
