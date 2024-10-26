//! Example 07:
//!
//! This example shows how to assert that a function returns an error variant in
//! an enum when you don't know or don't care about the exact error value, using
//! the `matches!` macro.
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
#[cfg(test)]
enum CustomError {
    NotFound,
    Unauthorized,
    NetworkError,
    ParseError(String),
    Unknown,
}

#[cfg(test)]
fn function_returning_an_error(error: CustomError) -> Result<(), CustomError> {
    Err(error)
}

#[cfg(test)]
mod tests {
    //use std::assert_matches::assert_matches;

    use crate::example07::{function_returning_an_error, CustomError};

    #[test]
    fn test_function_returning_not_found() {
        let result = function_returning_an_error(CustomError::NotFound);

        assert!(result.is_err());

        assert_eq!(result.unwrap_err(), CustomError::NotFound);
    }

    #[test]
    fn test_function_returning_parse_error_with_matches_macro() {
        let result = function_returning_an_error(CustomError::ParseError("error".to_string()));

        assert!(matches!(
            result.unwrap_err(),
            CustomError::ParseError(_error)
        ));
    }

    /*

    #[test]
    fn test_function_returning_parse_error_with_assert_matches_macro() {
        let result = function_returning_an_error(CustomError::ParseError("error".to_string()));

        assert_matches!(result.unwrap_err(), CustomError::ParseError(_error));
    }

    #[test]
    fn test_function_returning_parse_error_with_assert_matches_macro_v2() {
        let result = function_returning_an_error(CustomError::ParseError("error".to_string()));

        // You can also use the `..` syntax
        assert_matches!(result.unwrap_err(), CustomError::ParseError(..));
    }

    */
}
