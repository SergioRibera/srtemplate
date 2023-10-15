use std::str::FromStr;

use crate::prelude::FunctionError;

pub type ValidationResult = Result<(), FunctionError>;

pub fn args_min_len(args: &[String], expected: usize) -> ValidationResult {
    if expected > args.len() {
        return Err(FunctionError::ArgumentsIncomplete(args.len(), expected));
    }
    Ok(())
}

pub fn args_max_len(args: &[String], expected: usize) -> ValidationResult {
    if args.len() > expected {
        return Err(FunctionError::ArgumentsIncomplete(args.len(), expected));
    }
    Ok(())
}

pub fn arg_type<T: FromStr>(arg: String) -> ValidationResult {
    if arg.parse::<T>().is_err() {
        return Err(FunctionError::InvalidType(arg));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::prelude::validations::{arg_type, args_min_len, args_max_len};

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
