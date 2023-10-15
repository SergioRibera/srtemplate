use thiserror::Error;

#[derive(Debug, Error)]
pub enum SrTemplateError {
    #[error("Invalid syntaxis")]
    BadSyntax(String),

    #[error("Variable not found: {0}")]
    VariableNotFound(String),

    #[error("Function not imlemented: {0}")]
    FunctionNotImplemented(String),

    #[error("Error Processing Function: {0}")]
    Function(#[from] super::template::function::FunctionError),
}
