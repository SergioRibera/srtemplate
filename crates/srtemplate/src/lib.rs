pub mod builtin;
mod error;
#[cfg(feature = "typed_args")]
pub mod helper;
mod parser;
mod render;
mod template;

pub use error::SrTemplateError;
pub use template::{function, SrTemplate, TemplateFunction};

pub mod prelude {
    pub use super::builtin::*;
    pub use super::error::SrTemplateError;
    pub use super::template::function::*;
    pub use super::template::validations;
    pub use super::{SrTemplate, TemplateFunction};

    #[cfg(feature = "typed_args")]
    pub use super::helper::serialize::*;
}
