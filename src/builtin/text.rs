use crate::prelude::FuncResult;
use crate::template::validations;

/// Convert strings to lowercase.
///
/// This function takes a slice of strings and converts each string to lowercase.
/// It then concatenates the converted strings into a single string, separated by space.
///
/// # Arguments
///
/// * `args`: A slice of strings to be converted to lowercase.
///
/// # Returns
///
/// * A [`FuncResult`] containing the concatenated lowercase strings.
///
/// # Errors
///
/// This function can return an error of [`crate::function::FunctionError`] variant:
/// - `FunctionError::InvalidArgument` if there are insufficient input arguments.
#[cfg_attr(docsrs, doc(cfg(feature = "text")))]
#[cfg(feature = "text")]
pub fn to_lower(args: &[String]) -> FuncResult {
    validations::args_min_len(args, 1)?;

    Ok(args
        .iter()
        .map(|a| a.to_lowercase())
        .collect::<Vec<_>>()
        .join(" "))
}

/// Convert strings to uppercase.
///
/// This function takes a slice of strings and converts each string to uppercase.
/// It then concatenates the converted strings into a single string, separated by space.
///
/// # Arguments
///
/// * `args`: A slice of strings to be converted to uppercase.
///
/// # Returns
///
/// * A [`FuncResult`] containing the concatenated uppercase strings.
///
/// # Errors
///
/// This function can return an error of [`crate::function::FunctionError`] variant:
/// - `FunctionError::InvalidArgument` if there are insufficient input arguments.
#[cfg_attr(docsrs, doc(cfg(feature = "text")))]
#[cfg(feature = "text")]
pub fn to_upper(args: &[String]) -> FuncResult {
    validations::args_min_len(args, 1)?;

    Ok(args
        .iter()
        .map(|a| a.to_uppercase())
        .collect::<Vec<_>>()
        .join(" "))
}

/// Trim leading and trailing whitespace from strings.
///
/// This function takes a slice of strings and removes leading and trailing whitespace
/// from each string. It then concatenates the trimmed strings into a single string, separated by space.
///
/// # Arguments
///
/// * `args`: A slice of strings to be trimmed.
///
/// # Returns
///
/// * A [`FuncResult`] containing the concatenated trimmed strings.
///
/// # Errors
///
/// This function can return an error of [`crate::function::FunctionError`] variant:
/// - `FunctionError::InvalidArgument` if there are insufficient input arguments.
#[cfg_attr(docsrs, doc(cfg(feature = "text")))]
#[cfg(feature = "text")]
pub fn trim(args: &[String]) -> FuncResult {
    validations::args_min_len(args, 1)?;

    Ok(args.iter().map(|a| a.trim()).collect::<Vec<_>>().join(" "))
}
