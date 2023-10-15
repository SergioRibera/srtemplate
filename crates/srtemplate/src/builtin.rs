#[cfg(feature = "os")]
mod os;
#[cfg(feature = "number")]
mod number;
#[cfg(feature = "text")]
mod text;

#[cfg(feature = "os")]
pub use os::*;
#[cfg(feature = "number")]
pub use number::*;
#[cfg(feature = "text")]
pub use text::*;

#[cfg(feature = "typed_args")]
use crate::helper::serialize::FromArgs;
#[cfg(feature = "typed_args")]
use crate::prelude::FunctionError;

#[cfg(feature = "typed_args")]
pub fn to_typed_args<T: FromArgs>(args: &[String]) -> Result<T, FunctionError> {
    Ok(T::from_args(args)?)
}
