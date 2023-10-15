#[cfg(feature = "os")]
mod os;
#[cfg(feature = "math")]
mod math;
#[cfg(feature = "text")]
mod text;

#[cfg(feature = "os")]
pub use os::*;
#[cfg(feature = "math")]
pub use math::*;
#[cfg(feature = "text")]
pub use text::*;

#[cfg(feature = "typed_args")]
use crate::helper::serialize::FromArgs;
#[cfg(feature = "typed_args")]
use crate::prelude::FunctionError;

#[cfg(feature = "typed_args")]
#[must_use]
pub fn to_typed_args<T: FromArgs>(args: &[String]) -> Result<T, FunctionError> {
    Ok(T::from_args(args)?)
}
