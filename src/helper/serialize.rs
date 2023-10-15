use thiserror::Error;

/// An enumeration representing different errors that can occur while parsing arguments.
#[derive(Debug, Error)]
pub enum FromArgsError {
    #[error("Invalid Type: {0}")]
    BadType(String),

    #[error("Argument of type \"{0}\" not exists, argument index: {1}")]
    ArgumentNotExists(String, usize),

    #[error("Parse Variable Failed in position: {0}")]
    ParseFailed(usize),
}

pub type FromArgsResult<T> = Result<T, FromArgsError>;

/// A trait for parsing function arguments into a specified type.
pub trait FromArgs {
    /// Parse function arguments into the specified type.
    fn from_args(args: &[String]) -> FromArgsResult<Self>
    where
        Self: Sized;
}
