#[cfg_attr(docsrs, doc(cfg(feature = "math")))]
#[cfg(feature = "math")]
mod math;
#[cfg_attr(docsrs, doc(cfg(feature = "os")))]
#[cfg(feature = "os")]
mod os;
#[cfg_attr(docsrs, doc(cfg(feature = "text")))]
#[cfg(feature = "text")]
mod text;

#[cfg(feature = "typed_args")]
use crate::helper::serialize::FromArgs;
#[cfg(feature = "typed_args")]
use crate::prelude::FunctionError;

/// Converts a slice of `String` arguments to a typed object `T`.
///
/// This function takes a slice of `String` arguments and attempts to convert them into a typed object `T`. It returns a `Result` containing the typed object or a [`FunctionError`] in case of an error.
///
/// # Arguments
///
/// * `args`: A slice of `String` arguments to convert.
///
/// # Returns
///
/// A `Result` where `Ok` contains the typed object `T`, and `Err` holds a [`FunctionError`] if an error occurs.
///
/// # Example
///
/// ```no_run
/// fn custom_function(args: &[String]) -> FuncResult {
///     let (_a, _b) = to_typed_args::<(String, u8)>(args)?;

///     Ok(String::from("Test"))
/// }
/// ```
#[cfg_attr(docsrs, doc(cfg(feature = "typed_args")))]
#[cfg(feature = "typed_args")]
pub fn to_typed_args<T: FromArgs>(args: &[String]) -> Result<T, FunctionError> {
    Ok(T::from_args(args)?)
}
