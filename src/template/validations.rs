use std::str::FromStr;

use crate::prelude::FunctionError;

pub type ValidationResult = Result<(), FunctionError>;

/// Checks if the number of arguments is at least `expected`.
/// If there are fewer arguments, it returns an error.
///
/// # Arguments
///
/// * `args` - A slice of strings representing the arguments.
/// * `expected` - The minimum number of arguments expected.
///
/// # Returns
///
/// An `Result<(), FunctionError>` where `Ok(())` indicates that the number of arguments
/// meets the minimum requirement, and `Err` contains an error if the requirement is not met.
pub const fn args_min_len(args: &[String], expected: usize) -> ValidationResult {
    if expected > args.len() {
        return Err(FunctionError::ArgumentsIncomplete(args.len(), expected));
    }
    Ok(())
}

/// Checks if the number of arguments is at most `expected`.
/// If there are more arguments, it returns an error.
///
/// # Arguments
///
/// * `args` - A slice of strings representing the arguments.
/// * `expected` - The maximum number of arguments expected.
///
/// # Returns
///
/// An `Result<(), FunctionError>` where `Ok(())` indicates that the number of arguments
/// meets the maximum requirement, and `Err` contains an error if the requirement is exceeded.
pub const fn args_max_len(args: &[String], expected: usize) -> ValidationResult {
    if args.len() > expected {
        return Err(FunctionError::ArgumentsIncomplete(args.len(), expected));
    }
    Ok(())
}

/// Checks if a string argument can be parsed into a specific type `T`.
/// If parsing fails, it returns an error.
///
/// # Arguments
///
/// * `arg` - A string representing the argument to be parsed.
///
/// # Returns
///
/// An `Result<(), FunctionError>` where `Ok(())` indicates that the argument
/// can be successfully parsed into type `T`, and `Err` contains an error if parsing fails.
///
/// # Example
///
/// ```no_run
/// use std::str::FromStr;
/// use srtemplate::prelude::validations::arg_type;
///
/// let valid_arg = "42";
/// let result = arg_type::<i32>(valid_arg.to_string());
/// assert!(result.is_ok());
///
/// let invalid_arg = "not_an_integer";
/// let result = arg_type::<i32>(invalid_arg.to_string());
/// assert!(result.is_err());
/// ```
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
