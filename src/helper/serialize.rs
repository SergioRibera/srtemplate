use thiserror::Error;

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

pub trait FromArgs {
    fn from_args(args: &[String]) -> FromArgsResult<Self>
    where
        Self: Sized;
}
