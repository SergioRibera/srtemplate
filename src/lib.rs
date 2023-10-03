pub mod builtin;
mod error;
mod parser;
mod render;
mod template;

pub use template::{SrTemplate, TemplateFunction};
pub use parser::{parser, TemplateNode};
pub use render::render_nodes;
