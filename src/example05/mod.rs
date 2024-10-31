//! Example 05:
//!
//! This example shows how to assert that a function returns an error when it
//! can be done with a simple assert because we know the exact error value.

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
    use crate::example05::{function_returning_an_error, CustomError};

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

        assert_eq!(
            result.unwrap_err(),
            CustomError::ParseError("error".to_string())
        );
    }
}
