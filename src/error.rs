use thiserror::Error;

/// Collection of errors in the library
#[derive(Debug, Error, PartialEq)]
pub enum Error {
    /// This error appears when the syntax of the template to be rendered is wrong.
    #[error(transparent)]
    BadSyntax(crate::parser::Error),

    /// This error appears when the variable to be rendered does not exist.
    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    /// This error appears when the function to be rendered does not exist.
    #[error("Function not implemented: {0}")]
    FunctionNotImplemented(String),

    /// This error appears when the function to be rendered has suffered from an internal error.
    #[error("Error Processing Function: {0}")]
    Function(#[from] super::template::function::Error),
}
