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
