#[cfg_attr(docsrs, doc(cfg(feature = "math")))]
#[cfg(feature = "math")]
pub(crate) mod math;
#[cfg_attr(docsrs, doc(cfg(feature = "os")))]
#[cfg(feature = "os")]
pub(crate) mod os;
#[cfg_attr(docsrs, doc(cfg(feature = "text")))]
#[cfg(feature = "text")]
pub(crate) mod text;

#[cfg(feature = "typed_args")]
use crate::helper::serialize::FromArgs;
#[cfg(feature = "typed_args")]
use crate::prelude::FunctionError;

/// Converts a slice of `String` arguments into a typed object `T`.
///
/// This function utilizes the `FromArgs` trait to parse and convert
/// a slice of `String` arguments into a specific type `T`.
/// If the conversion fails, it returns a [`FunctionError`] with details about the error.
///
/// # Arguments
///
/// * `args` - A slice of `String` arguments to be converted.
///
/// # Returns
///
/// A `Result` where:
/// - `Ok(T)` contains the successfully parsed and typed object.
/// - `Err(FunctionError)` provides details about why the conversion failed.
///
/// # Example
///
/// ```
/// use srtemplate::prelude::{to_typed_args, FromArgs, FromArgsResult};
///
/// struct MyArgs {
///     name: String,
///     count: u8,
/// }
///
/// impl FromArgs for MyArgs {
///     fn from_args(args: &[String]) -> FromArgsResult<Self> {
///         if args.len() != 2 {
///             return Err(srtemplate::prelude::FromArgsError::ArgumentsIncomplete(args.len(), 2));
///         }
///         let name = args[0].clone();
///         let count = args[1].parse::<u8>().map_err(|_| srtemplate::prelude::FromArgsError::BadType(args[1].clone()))?;
///         Ok(MyArgs { name, count })
///     }
/// }
///
/// fn my_function(args: &[String]) -> Result<String, srtemplate::prelude::FunctionError> {
///     let my_args = to_typed_args::<MyArgs>(args)?;
///     Ok(format!("Name: {}, Count: {}", my_args.name, my_args.count))
/// }
/// ```
///
/// # Errors
///
/// Returns an error if:
/// - The number of arguments does not meet the expected count for type `T`.
/// - Any argument cannot be parsed into the required type `T`.
///
/// # Feature Flags
///
/// This function requires the `typed_args` feature to be enabled.
#[cfg_attr(docsrs, doc(cfg(feature = "typed_args")))]
#[cfg(feature = "typed_args")]
pub fn to_typed_args<T: FromArgs>(args: &[String]) -> Result<T, FunctionError> {
    Ok(T::from_args(args)?)
}
