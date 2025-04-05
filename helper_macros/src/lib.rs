//! # srtemplate-macros Library Documentation
//!
//! This library provides procedural macros for generating function and variable handling code, particularly useful for template processing systems.
//!
//! ## Features
//!
//! - **Function attribute macro**: Transforms regular functions into argument-parsing functions
//! - **Variable derive macro**: Converts structs into collections of key-value variables
//! - **Text case conversion**: Comprehensive text case handling utilities
//!
//! ## Macros
//!
//! ### `#[function]` Attribute Macro
//!
//! Transforms a function to accept string arguments and perform automatic parsing.
//!
//! **Example**:
//! ```rust
//! #[srtemplate::function]
//! fn greet(name: String, age: i32) -> String {
//!     format!("Hello {}, you're {} years old", name, age)
//! }
//! ```
//!
//! **Generated Code**:
//! ```rust
//! fn greet(args: &[String]) -> srtemplate::prelude::FuncResult {
//!     srtemplate::prelude::validations::args_min_len(args, 2)?;
//!     srtemplate::prelude::validations::args_max_len(args, 2)?;
//!
//!     let name = args[0].parse::<String>().map_err(|_| srtemplate::prelude::FromArgsError::ParseFailed(0))?;
//!     let age = args[1].parse::<i32>().map_err(|_| srtemplate::prelude::FromArgsError::ParseFailed(1))?;
//!
//!     format!("Hello {}, you're {} years old", name, age)
//! }
//! ```
//!
//! ### `#[derive(Variable)]` Macro
//!
//! Generates a `variables()` method for structs that returns field names and values as strings.
//!
//! **Example**:
//! ```rust
//! #[derive(srtemplate::Variable)]
//! #[template(case = "snake")]
//! struct User {
//!     #[template(rename = "first_name")]
//!     name: String,
//!     age: u32,
//! }
//! ```
//!
//! **Generated Implementation**:
//! ```rust
//! impl<'variable> srtemplate::Variable<'variable> for User {
//!     fn variables(&self) -> impl Iterator<Item = (std::borrow::Cow<'variable, str>, String)> {
//!         [
//!             ("user.first_name".into(), self.name.to_string()),
//!             ("user.age".into(), self.age.to_string())
//!         ].into_iter()
//!     }
//! }
//! ```
//!
//! ## Text Case Conversion
//!
//! The library provides comprehensive text case conversion through the `TextCase` enum:
//!
//! ### Supported Cases
//!
//! | Case | Example | Conversion Method |
//! |------|---------|-------------------|
//! | `Lower` | `hello world` | `to_lowercase()` |
//! | `Upper` | `HELLO WORLD` | `to_uppercase()` |
//! | `Snake` | `hello_world` | Lowercase with underscores |
//! | `Camel` | `helloWorld` | First word lowercase, others capitalized |
//! | `Pascal` | `HelloWorld` | All words capitalized |
//! | `Kebab` | `hello-world` | Lowercase with hyphens |
//! | `ScreamingSnake` | `HELLO_WORLD` | Uppercase with underscores |
//!
//! **Usage Example**:
//! ```rust
//! use srtemplate::TextCase;
//!
//! let text = "hello_world-example";
//! let pascal_case = TextCase::Pascal.convert(text);
//! assert_eq!(pascal_case, "HelloWorldExample");
//! ```
//!
//! ## Error Handling
//!
//! The macros provide clear error messages for:
//! - Invalid function signatures in `#[function]`
//! - Unsupported types in `#[derive(Variable)]`
//! - Malformed attribute syntax
//! - Invalid case type specifications
//!
//! ## Limitations
//!
//! 1. `#[function]` macro:
//!    - Doesn't support methods (functions with `self` parameter)
//!    - Requires explicit type annotations
//!
//! 2. `#[derive(Variable)]` macro:
//!    - Currently only supports structs (not enums)
//!    - Field types must implement `ToString`
use function::gen_function;
use proc_macro::TokenStream;
use venial::{parse_item, Error, Item};

mod function;
mod variable;

/// # Function Macro Documentation
///
/// This module provides a procedural macro for generating functions that can parse arguments from a string slice and execute with type-safe parameters.
///
/// ## Overview
///
/// The macro transforms a Rust function into one that:
/// - Takes `&[String]` as input
/// - Validates argument count matches parameter count
/// - Parses each argument to the expected type
/// - Returns a `FuncResult` (`Result<String, FunctionError>`)
///
/// ## Usage
///
/// ### Basic Example
///
/// ```rust
/// #[function]
/// fn add(a: i32, b: i32) {
///     Ok((a + b).to_string())
/// }
/// ```
///
/// This generates:
///
/// ```rust
/// fn add(args: &[String]) -> srtemplate::prelude::FuncResult {
///     srtemplate::prelude::validations::args_min_len(args, 2)?;
///     srtemplate::prelude::validations::args_max_len(args, 2)?;
///
///     let a = args[0].parse::<i32>().map_err(|_| srtemplate::prelude::FromArgsError::ParseFailed(0))?;
///     let b = args[1].parse::<i32>().map_err(|_| srtemplate::prelude::FromArgsError::ParseFailed(1))?;
///
///     Ok((a + b).to_string())
/// }
/// ```
///
/// ### Visibility
///
/// The macro preserves the original function's visibility:
///
/// ```rust
/// #[function]
/// pub fn public_function(x: f64) {
///     Ok((x * 2.0).to_string())
/// }
/// ```
///
/// ### Error Handling
///
/// The generated function automatically handles:
/// - Argument count validation (min and max)
/// - Type parsing errors
///
/// ## Implementation Details
///
/// ### Function Transformation
///
/// The macro performs several transformations:
///
/// 1. **Argument Validation**: Adds checks for exact argument count matching parameter count
/// 2. **Parameter Parsing**: Generates code to parse each string argument to the corresponding parameter type
/// 3. **Body Preservation**: Keeps the original function body intact
///
/// ### Restrictions
///
/// 1. **No Receiver Parameters**: Functions with `self` parameters are rejected
/// 2. **Required Body**: Functions must have a body
/// 3. **Typed Parameters**: All parameters must have explicit type annotations
///
/// ### Error Cases
///
/// The macro will error if:
/// - The function has a `self` parameter
/// - Any parameter lacks a type annotation
/// - The function lacks a body
///
/// ## Technical Notes
///
/// The implementation uses:
/// - `venial` for parsing the function syntax
/// - `proc_macro2` and `quote` for code generation
/// - Span-preserving operations for better error reporting
///
/// The generated code relies on these types being in scope:
/// - `srtemplate::prelude::FuncResult`
/// - `srtemplate::prelude::FromArgsError`
/// - `srtemplate::prelude::validations` functions
#[proc_macro_attribute]
pub fn function(_: TokenStream, body: TokenStream) -> TokenStream {
    let func = match parse_item(body.into()) {
        Ok(Item::Function(func)) => Ok(func),
        Err(e) => Err(e),
        Ok(_) => Err(Error::new("Just support functions")),
    };

    func.and_then(gen_function)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

/// # Variable Derive Macro Documentation
///
/// This procedural macro generates an implementation of the `Variable` trait for structs, enabling conversion of struct fields into key-value pairs with various naming and casing options.
///
/// ## Struct-Level Attributes
///
/// These attributes are applied to the struct definition:
///
/// ### `#[template(rename = "name")]` or `#[template(alias = "name")]`
/// - **Purpose**: Renames the struct in the generated variable names
/// - **Example**:
///   ```rust
///   #[template(rename = "UserData")]
///   struct User {
///       // fields...
///   }
///   ```
///   Generates variables like `UserData.field_name` instead of `User.field_name`
///
/// ### `#[template(case = "case_type")]`
/// - **Purpose**: Sets the naming case for the struct name
/// - **Valid values**:
///   | Case | Example | Conversion Method |
///   |------|---------|-------------------|
///   | `Lower` | `hello world` | `to_lowercase()` |
///   | `Upper` | `HELLO WORLD` | `to_uppercase()` |
///   | `Snake` | `hello_world` | Lowercase with underscores |
///   | `Camel` | `helloWorld` | First word lowercase, others capitalized |
///   | `Pascal` | `HelloWorld` | All words capitalized |
///   | `Kebab` | `hello-world` | Lowercase with hyphens |
///   | `ScreamingSnake` | `HELLO_WORLD` | Uppercase with underscores |
/// - **Example**:
///   ```rust
///   #[template(case = "pascal")]
///   struct user_settings {
///       // fields...
///   }
///   ```
///   Generates variables with the struct name converted to PascalCase
///
/// ### `#[template(case_fields = "case_type")]`
/// - **Purpose**: Sets default naming case for all fields
/// - **Valid values**: Same as `case` attribute
/// - **Overrides**: Can be overridden by field-level case attributes
/// - **Default**: `snake_case`
/// - **Example**:
///   ```rust
///   #[template(case_fields = "scream")]
///   struct Config {
///       max_size: usize,
///   }
///   ```
///   Generates variable name `Config.MAX_SIZE`
///
/// ## Field-Level Attributes
///
/// These attributes are applied to individual struct fields:
///
/// ### `#[template(ignore)]`
/// - **Purpose**: Excludes the field from variable generation
/// - **Example**:
///   ```rust
///   struct Product {
///       id: u64,
///       #[template(ignore)]
///       internal_code: String,
///   }
///   ```
///   Only generates variable for `id`
///
/// ### `#[template(rename = "name")]` or `#[template(alias = "name")]`
/// - **Purpose**: Renames the field in the generated variable
/// - **Example**:
///   ```rust
///   struct Point {
///       #[template(rename = "x-coord")]
///       x: f64,
///   }
///   ```
///   Generates variable name `Point.x-coord`
///
/// ### `#[template(case = "case_type")]`
/// - **Purpose**: Sets naming case for this specific field
/// - **Overrides**: The struct-level `case_fields` setting
/// - **Example**:
///   ```rust
///   struct Settings {
///       #[template(case = "kebab")]
///       max_file_size: usize,
///   }
///   ```
///   Generates variable name `Settings.max-file-size`
///
/// ## Generated Implementation
///
/// The macro generates an implementation of the `Variable` trait that provides:
///
/// 1. A `variables()` method returning an iterator of `(Cow<str>, String)` tuples
/// 2. Proper case conversion according to the specified attributes
/// 3. Field filtering for ignored fields
///
/// ### Example Output
///
/// For this input:
/// ```rust
/// #[template(case = "pascal", case_fields = "kebab")]
/// struct UserProfile {
///     #[template(rename = "first-name")]
///     first_name: String,
///     #[template(ignore)]
///     password: String,
///     age: u8,
/// }
/// ```
///
/// The generated implementation would produce variables:
/// - `UserProfile.first-name`
/// - `UserProfile.age`
///
/// ## Supported Field Types
///
/// The macro works with:
/// - Named fields (standard structs)
/// - Tuple structs (fields accessed by index)
/// - Unit structs (treated as a single value)
///
/// ## Error Handling
///
/// The macro will error if:
/// - Applied to non-struct types (enums, unions, etc.)
/// - Invalid case type is specified
/// - Attribute syntax is malformed
#[proc_macro_derive(Variable, attributes(template))]
pub fn derive_variable(input: TokenStream) -> TokenStream {
    variable::derive(input.into())
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

#[derive(Clone, Copy, Debug)]
pub(crate) enum TextCase {
    Lower,
    Upper,
    Sentence,
    Snake,
    Camel,
    Pascal,
    Kebab,
    ScreamingSnake,
}

impl TextCase {
    pub fn convert(&self, s: &str) -> String {
        match self {
            TextCase::Lower => s.to_lowercase(),
            TextCase::Upper => s.to_uppercase(),
            _ => {
                let words = Self::split_into_words(s);
                self.process_words(words)
            }
        }
    }

    fn split_into_words(s: &str) -> Vec<String> {
        let mut words = Vec::new();
        let mut current_word = String::new();
        let mut prev_char = None::<char>;

        for c in s.chars() {
            if c.is_ascii_punctuation() || c.is_whitespace() {
                if !current_word.is_empty() {
                    words.push(current_word);
                    current_word = String::new();
                }
            } else if c.is_uppercase() {
                if let Some(prev) = prev_char {
                    if prev.is_lowercase() || prev.is_ascii_digit() {
                        if !current_word.is_empty() {
                            words.push(current_word);
                            current_word = String::new();
                        }
                    }
                }
                current_word.push(c);
            } else if c.is_lowercase() {
                if let Some(prev) = prev_char {
                    if prev.is_uppercase() && current_word.len() > 1 {
                        let last_char = current_word.pop().unwrap();
                        if !current_word.is_empty() {
                            words.push(current_word);
                        }
                        current_word = String::from(last_char);
                    }
                }
                current_word.push(c);
            } else {
                current_word.push(c);
            }
            prev_char = Some(c);
        }

        if !current_word.is_empty() {
            words.push(current_word);
        }

        words
    }

    fn process_words(&self, words: Vec<String>) -> String {
        match self {
            TextCase::Snake => words
                .iter()
                .map(|word| word.to_lowercase())
                .collect::<Vec<_>>()
                .join("_"),

            TextCase::Camel => {
                if words.is_empty() {
                    return String::new();
                }
                let mut processed = vec![words[0].to_lowercase()];
                processed.extend(words[1..].iter().map(|w| Self::capitalize(w)));
                processed.join("")
            }

            TextCase::Pascal => words
                .iter()
                .map(|word| Self::capitalize(word))
                .collect::<Vec<_>>()
                .join(""),

            TextCase::Kebab => words
                .iter()
                .map(|word| word.to_lowercase())
                .collect::<Vec<_>>()
                .join("-"),

            TextCase::ScreamingSnake => words
                .iter()
                .map(|word| word.to_uppercase())
                .collect::<Vec<_>>()
                .join("_"),

            _ => String::new(),
        }
    }

    fn capitalize(s: &str) -> String {
        let mut chars = s.chars();
        match chars.next() {
            None => String::new(),
            Some(first) => {
                let first_upper = first.to_uppercase().collect::<String>();
                let rest_lower: String = chars.collect::<String>().to_lowercase();
                format!("{}{}", first_upper, rest_lower)
            }
        }
    }
}

impl std::str::FromStr for TextCase {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "lower" | "lowercase" => Ok(TextCase::Lower),
            "upper" | "uppercase" => Ok(TextCase::Upper),
            "sentence" | "sentencecase" => Ok(TextCase::Sentence),
            "snake" | "snakecase" => Ok(TextCase::Snake),
            "camel" | "camelcase" => Ok(TextCase::Camel),
            "pascal" | "pascalcase" => Ok(TextCase::Pascal),
            "kebab" | "kebabcase" => Ok(TextCase::Kebab),
            "screamingsnake" | "screaming_snake" | "screaming-snake" | "screaming snake" => {
                Ok(TextCase::ScreamingSnake)
            }
            _ => Err(format!("Invalid text case: {}", s)),
        }
    }
}

impl<T: AsRef<str>> From<T> for TextCase {
    fn from(s: T) -> Self {
        s.as_ref()
            .parse()
            .unwrap_or_else(|_| panic!("Failed to convert '{}' into TextCase", s.as_ref()))
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!("lowercase".parse::<TextCase>(), Ok(TextCase::Lower));
        assert_eq!("UPPERCASE".parse::<TextCase>(), Ok(TextCase::Upper));
        assert_eq!("snake_case".parse::<TextCase>(), Ok(TextCase::Snake));
        assert_eq!("camelCase".parse::<TextCase>(), Ok(TextCase::Camel));
        assert_eq!("PascalCase".parse::<TextCase>(), Ok(TextCase::Pascal));
        assert_eq!("kebab-case".parse::<TextCase>(), Ok(TextCase::Kebab));
        assert_eq!(
            "SCREAMING_SNAKE".parse::<TextCase>(),
            Ok(TextCase::ScreamingSnake)
        );
        assert!("invalid_case".parse::<TextCase>().is_err());
    }

    #[test]
    fn test_into() {
        let lower: TextCase = "lowercase".into();
        assert!(matches!(lower, TextCase::Lower));

        let snake: TextCase = "snakecase".into();
        assert!(matches!(snake, TextCase::Snake));

        let screaming_snake: TextCase = "screaming_snake".into();
        assert!(matches!(screaming_snake, TextCase::ScreamingSnake));
    }

    #[test]
    fn test_case_conversions() {
        let input = "hello_world-example 123HTTPRequest";

        assert_eq!(
            TextCase::Lower.convert(input),
            "hello_world-example 123httpprequest"
        );
        assert_eq!(
            TextCase::Upper.convert(input),
            "HELLO_WORLD-EXAMPLE 123HTTPREQUEST"
        );
        assert_eq!(
            TextCase::Snake.convert(input),
            "hello_world_example_123_http_request"
        );
        assert_eq!(
            TextCase::Camel.convert(input),
            "helloWorldExample123HttpRequest"
        );
        assert_eq!(
            TextCase::Pascal.convert(input),
            "HelloWorldExample123HttpRequest"
        );
        assert_eq!(
            TextCase::Kebab.convert(input),
            "hello-world-example-123-http-request"
        );
        assert_eq!(
            TextCase::ScreamingSnake.convert(input),
            "HELLO_WORLD_EXAMPLE_123_HTTP_REQUEST"
        );
    }
}
