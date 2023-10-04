use thiserror::Error;

#[derive(Debug, Error)]
pub enum FunctionError {
    #[error("Invalid Function Arguments {0:?}")]
    InvalidArguments(Vec<String>),
    #[error("This function require {0} arguments, but found {1}")]
    ArgumentsIncomplete(u32, u32),
    #[error("Error calling the function: {0}")]
    RuntimeError(String),
}

trait CustomFunction<T> {
    fn validation(&self, _args: T) -> Result<(), FunctionError> {
        Ok(())
    }

    fn call(&self, args: T) -> Result<String, FunctionError>;
}
