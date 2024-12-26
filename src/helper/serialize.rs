use thiserror::Error;

/// An enumeration representing different errors that can occur while parsing arguments.
#[derive(Debug, Error, PartialEq)]
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
    /// Parses a slice of function arguments into a specific type.
    ///
    /// This function is part of the `FromArgs` trait, designed to facilitate
    /// the conversion of function arguments provided as strings into a concrete type `Self`.
    ///
    /// # Arguments
    ///
    /// * `args` - A slice of `String` values representing the arguments to parse.
    ///
    /// # Returns
    ///
    /// A `FromArgsResult<Self>` where:
    /// - `Ok(Self)` contains the parsed type if the conversion is successful.
    /// - `Err(FromArgsError)` provides details about why the conversion failed.
    ///
    /// # Example
    ///
    /// ```
    /// use srtemplate::prelude::{FromArgs, FromArgsResult};
    ///
    /// struct MyArgs {
    ///     value: i32,
    /// }
    ///
    /// impl FromArgs for MyArgs {
    ///     fn from_args(args: &[String]) -> FromArgsResult<Self> {
    ///         if args.len() != 1 {
    ///             return Err(srtemplate::prelude::FromArgsError::ArgumentNotExists("value".to_string(), 0));
    ///         }
    ///         let value = args[0].parse::<i32>().map_err(|_| srtemplate::prelude::FromArgsError::BadType(args[0].clone()))?;
    ///         Ok(MyArgs { value })
    ///     }
    /// }
    ///
    /// let args = vec!["42".to_string()];
    /// let parsed = MyArgs::from_args(&args);
    /// assert!(parsed.is_ok());
    /// ```
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The number of arguments does not match the expected input.
    /// - An argument cannot be parsed into the required type.
    fn from_args(args: &[String]) -> FromArgsResult<Self>
    where
        Self: Sized;
}
