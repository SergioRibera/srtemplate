#![cfg_attr(docsrs, feature(doc_cfg))]
//!
//! The library features are specified in your Cargo.toml file.
//! - `text`: Text processing functions.
//! - `os`: Functions related to the operating system.
//! - `math`: Mathematical functions.
//! - `typed_args`: Enables typed arguments, if specified.
//! - `debug`: Enable log for library
//! - `default`: Enable all features excepts `debug` feature
//!
//! To enable specific features, you can include them in your dependencies, like:
//!
//! ```toml
//! [dependencies]
//! srtemplate = { version = "0.1", default-features = false, features = ["text", "os"] }
//! ```
//!
//! Make sure to include the required features you need in your application.
//!
//! ## Example
//! ```no_run
//! use srtemplate::SrTemplate;
//!
//! let mut ctx = SrTemplate::default();
//! ctx.add_variable("var", &"World");
//! ctx.add_variable("otherVar", &"Other");
//! ctx.add_variable("number", &85u8);
//!
//! let template = "Hello {{ var }}! This is {{ otherVar }} and this is number: {{number}}";
//! println!("Rendered: {}", ctx.render(template).unwrap());
//! ```
//!
//! To see all function implemented for template syntax see [wiki](https://github.com/SergioRibera/srtemplate/wiki/Template-Syntaxis#builtin-functions)

/// The `builtin` module provides a set of built-in functions for `SrTemplate`.
pub mod builtin;

/// The `error` module defines custom error types for `SrTemplate`.
mod error;

/// The `helper` module contains utility functions and traits, and it's available when the `typed_args` feature is enabled.
#[cfg(feature = "typed_args")]
pub mod helper;

/// The `parser` module is responsible for parsing template strings into nodes.
mod parser;

/// The `render` module provides functions for rendering template nodes.
mod render;

/// The `template` module contains the core functionality for `SrTemplate`, including the `function` module for custom functions.
mod template;

/// Re-exports the `SrTemplateError` type for convenient use.
pub use error::SrTemplateError;

/// Re-exports the [`template::function`], [`template::SrTemplate`], [`template::TemplateFunction`] type for convenient use.
pub use template::{function, SrTemplate, TemplateFunction};

/// The `prelude` module re-exports common items for easier use of `SrTemplate`.
pub mod prelude {
    pub use super::builtin::*;
    pub use super::error::SrTemplateError;
    pub use super::template::function::*;
    pub use super::template::validations;
    pub use super::{SrTemplate, TemplateFunction};

    /// When the `typed_args` feature is enabled, this module re-exports serialization related items.
    #[cfg(feature = "typed_args")]
    pub use super::helper::serialize::*;
}
