use thiserror::Error;

#[derive(Debug, Error)]
pub enum SrTemplateError {
    #[error("Invalid syntaxis")]
    BadSyntax(String),

    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    #[error("Function not imlemented: {0}")]
    FunctionNotImplemented(String),

    #[error("Function not supported: {0}")]
    FunctionNotSupported(String),
}
