use std::str::FromStr;

use crate::prelude::FunctionError;

pub type ValidationResult = Result<(), FunctionError>;

/// Validates that the number of arguments is at least `expected`.
///
/// # Arguments
///
/// * `args` - A slice of strings representing the arguments.
/// * `expected` - The minimum number of arguments required.
///
/// # Returns
///
/// A `ValidationResult` where:
/// - `Ok(())` indicates that the number of arguments meets the minimum requirement.
/// - `Err(FunctionError::ArgumentsIncomplete)` occurs if the number of arguments is less than the expected minimum.
///
/// # Example
///
/// ```
/// use srtemplate::prelude::validations::args_min_len;
///
/// let args = vec!["arg1".to_string(), "arg2".to_string()];
/// assert!(args_min_len(&args, 2).is_ok());
/// assert!(args_min_len(&args, 3).is_err());
/// ```
///
/// # Errors
///
/// Returns an error if the number of arguments is less than the specified minimum.
pub const fn args_min_len(args: &[String], expected: usize) -> ValidationResult {
    if expected > args.len() {
        return Err(FunctionError::ArgumentsIncomplete(args.len(), expected));
    }
    Ok(())
}

/// Validates that the number of arguments does not exceed `expected`.
///
/// # Arguments
///
/// * `args` - A slice of strings representing the arguments.
/// * `expected` - The maximum number of arguments allowed.
///
/// # Returns
///
/// A `ValidationResult` where:
/// - `Ok(())` indicates that the number of arguments is within the allowed limit.
/// - `Err(FunctionError::ArgumentsIncomplete)` occurs if the number of arguments exceeds the limit.
///
/// # Example
///
/// ```
/// use srtemplate::prelude::validations::args_max_len;
///
/// let args = vec!["arg1".to_string(), "arg2".to_string()];
/// assert!(args_max_len(&args, 2).is_ok());
/// assert!(args_max_len(&args, 1).is_err());
/// ```
///
/// # Errors
///
/// Returns an error if the number of arguments exceeds the specified maximum.
pub const fn args_max_len(args: &[String], expected: usize) -> ValidationResult {
    if args.len() > expected {
        return Err(FunctionError::ArgumentsIncomplete(args.len(), expected));
    }
    Ok(())
}

/// Validates if a string argument can be parsed into a specific type `T`.
///
/// # Arguments
///
/// * `arg` - A string representing the argument to be validated.
///
/// # Returns
///
/// A `ValidationResult` where:
/// - `Ok(())` indicates that the argument can be successfully parsed into type `T`.
/// - `Err(FunctionError::InvalidType)` occurs if parsing fails.
///
/// # Example
///
/// ```
/// use srtemplate::prelude::validations::arg_type;
///
/// let valid_arg = "42";
/// assert!(arg_type::<i32>(valid_arg.to_string()).is_ok());
///
/// let invalid_arg = "not_an_integer";
/// assert!(arg_type::<i32>(invalid_arg.to_string()).is_err());
/// ```
///
/// # Errors
///
/// Returns an error if the argument cannot be parsed into the specified type.
pub fn arg_type<T: FromStr>(arg: String) -> ValidationResult {
    if arg.parse::<T>().is_err() {
        return Err(FunctionError::InvalidType(arg));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::prelude::validations::{arg_type, args_max_len, args_min_len};

    #[test]
    fn test_arg_min_len() {
        let args = vec!["Some".to_string(), "Other".to_string(), "Again".to_string()];

        assert!(args_min_len(&args, 2).is_ok());
    }

    #[test]
    fn test_arg_min_err_len() {
        let args = vec!["Some".to_string(), "Other".to_string(), "Again".to_string()];

        assert!(args_min_len(&args, 10).is_err());
    }

    #[test]
    fn test_arg_max_len() {
        let args = vec!["Some".to_string(), "Other".to_string(), "Again".to_string()];

        assert!(args_max_len(&args, 3).is_ok());
    }

    #[test]
    fn test_arg_max_out_len() {
        let args = vec!["Some".to_string(), "Other".to_string(), "Again".to_string()];

        assert!(args_max_len(&args, 1).is_err());
    }

    #[test]
    fn test_arg_type() {
        assert!(arg_type::<u32>("54".to_string()).is_ok());
        assert!(arg_type::<f32>("3.5".to_string()).is_ok());
        assert!(arg_type::<i32>("-54".to_string()).is_ok());
        assert!(arg_type::<u32>("3.5".to_string()).is_err());
        assert!(arg_type::<u32>("-54".to_string()).is_err());
    }
}
