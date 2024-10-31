# Testing error variants in Rust using `assert_matches!` and alternatives

In Rust, handling errors in enums allows for detailed error information, but testing these errors can become tricky when you don’t care about every detail. This article shows various ways to test for error types, including exact matches, variant checks without specific values, and using the `matches!` and `assert_matches!` macros for simpler assertions.

## Introduction

Suppose you have an error enum, `CustomError`, with various error types, some of which carry additional information:

```rust
#[derive(Debug, PartialEq)]
enum CustomError {
    NotFound,
    Unauthorized,
    NetworkError,
    ParseError(String), // This includes additional data we might want to ignore
    Unknown,
}
```

When testing functions that return errors of this type, you may want to check for a specific error variant. However, you may not always know or care about every value inside these variants. Let’s walk through three examples of how to handle these cases.

## Example 05: Asserting the exact error value

When you know the precise error, you can assert the entire error value directly.

```rust
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
```

Here, we check both that the result is an error and that it matches exactly `CustomError::NotFound` or `CustomError::ParseError("error")`. This approach is straightforward, but it requires knowing the exact value of the error, which isn’t always practical.

## Example 06: Asserting only the error variant without the value

When you only care about the error variant (e.g., `ParseError`) but not the exact value, you can use a custom assertion to check for the variant without worrying about its contents.

```rust
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
```

In this example, the `is_parse_error` helper function allows us to check only for the variant without worrying about the content. This is helpful when the inner data isn’t essential for the test.

## Example 07: Using `matches!` or `assert_matches!` for cleaner variant assertions

Using the `matches!` or `assert_matches!` macro simplifies the process even further, allowing us to check just the variant directly without writing additional code.

```rust
#[cfg(test)]
fn function_returning_an_error(error: CustomError) -> Result<(), CustomError> {
    Err(error)
}

#[cfg(test)]
mod tests {
    use std::assert_matches::assert_matches;

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
```

Here, the `assert_matches!` macro lets you assert that the result contains a specific variant, while `_` or `..` can be used to ignore the inner data. This approach provides cleaner, more readable assertions without additional helper functions.

The `..` syntax is commonly referred to as "pattern wildcards" or "pattern ignoring". When used in the context of structs, tuples, and enum variants, it’s often called "elision" or "rest pattern". The exact term varies depending on the structure being matched, but "rest pattern" is frequently used in Rust documentation and discussions when referring to `..` as a way to ignore data.

## Choosing the right approach

1. **Exact error matching (Example 05)**: Use this approach if you need to assert the exact value of an error, including its inner data.
2. **Variant-only matching with helper function (Example 06)**: Useful if you care only about the variant but want to avoid external macros.
3. **`assert_matches!` Macro (Example 07)**: Ideal for concise, readable tests focusing only on the variant type.

## Conclusion

Testing errors in Rust can be done in multiple ways, each with its trade-offs. For more complex enums with nested data, `matches!` and `assert_matches!` makes tests cleaner and easier to read by allowing you to focus on the variant without needing to specify every detail. These techniques ensure robust tests that maintain clarity, even as your error types evolve.
