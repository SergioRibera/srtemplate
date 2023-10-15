use thiserror::Error;

pub type FuncResult = Result<String, FunctionError>;

/// An enumeration representing various errors that can occur while processing functions.
#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("Invalid Function Arguments {0}")]
    InvalidArgument(String),

    #[error("Invalid Function Argument Type for {0}")]
    InvalidType(String),

    #[cfg(feature = "typed_args")]
    #[error("Convert type from arguments failed: {0}")]
    ConvertArgsFailed(#[from] crate::helper::serialize::FromArgsError),

    #[error("This function require {0} arguments, but found {1}")]
    ArgumentsIncomplete(usize, usize),

    #[error("Error calling the function: {0}")]
    RuntimeError(String),
}
