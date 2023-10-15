use crate::prelude::{FuncResult, FunctionError};
use crate::template::validations;

/// Retrieve and concatenate environment variable values by their names.
///
/// This function takes a slice of strings representing environment variable names.
/// It attempts to retrieve the values of these variables and concatenates them into a single string,
/// separated by space.
///
/// # Arguments
///
/// * `args`: A slice of strings representing environment variable names.
///
/// # Returns
///
/// * A [`FuncResult`] containing the concatenated environment variable values.
///
/// # Errors
///
/// This function can return an error of [`FunctionError`] variant:
/// - `FunctionError::InvalidArgument` if any of the specified environment variable names is invalid.
#[cfg_attr(docsrs, doc(cfg(feature = "os")))]
#[cfg(feature = "os")]
pub fn env(args: &[String]) -> FuncResult {
    validations::args_min_len(args, 1)?;

    Ok(args
        .iter()
        .map(|a| std::env::var(a).map_err(|_| FunctionError::InvalidArgument(a.clone())))
        .collect::<Result<Vec<String>, FunctionError>>()?
        .join(" "))
}
