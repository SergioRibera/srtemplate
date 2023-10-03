pub mod builtin;
mod error;
mod parser;
mod render;
mod template;

pub use template::{SrTemplate, TemplateFunction};

pub mod prelude {
    pub use super::builtin::*;
    pub use super::{SrTemplate, TemplateFunction};
}
