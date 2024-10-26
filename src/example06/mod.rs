//! Example 06:
//!
//! This example shows how to assert that a function returns an error variant in
//! an enum when you don't know or don't care about the exact error value.

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
    use crate::example06::{function_returning_an_error, CustomError};

    #[test]
    fn test_function_returning_not_found() {
        let result = function_returning_an_error(CustomError::NotFound);

        assert!(result.is_err());

        assert_eq!(result.unwrap_err(), CustomError::NotFound);
    }

    #[test]
    fn test_function_returning_parse_error() {
        let result = function_returning_an_error(CustomError::ParseError("error".to_string()));

        assert!(result.is_err());

        if let CustomError::ParseError(_error) = result.unwrap_err() {
        } else {
            panic!("Unexpected error variant");
        }
    }

    #[allow(clippy::match_like_matches_macro)]
    fn is_parse_error(error: CustomError) -> bool {
        match error {
            CustomError::ParseError(_error) => true,
            _ => false,
        }
    }

    #[test]
    fn test_function_returning_parse_error_with_custom_assert() {
        let result = function_returning_an_error(CustomError::ParseError("error".to_string()));

        assert!(result.is_err());

        assert!(is_parse_error(result.unwrap_err()));
    }
}
