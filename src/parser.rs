#[cfg(feature = "debug")]
use log::trace;
use nom::multi::many0;
use nom::{branch::alt, IResult};

mod function;
mod values;

#[cfg(test)]
mod test;

use function::function_parser;
use values::{text_parser, variable_parser};

use crate::parser::values::raw_string_parser;

/// Variants of the types of nodes that exist in the syntax
#[derive(Debug, PartialEq, Eq)]
pub enum TemplateNode {
    /// Variables to be rendered
    Variable(String),
    /// Functions to be rendered
    Function(String, Vec<TemplateNode>),
    /// Plain text, pass as variable
    InnerText(String),
    /// Plain text, this will be ignored in the rendering
    Text(String),
}

/// Parse a string input into a vector of `TemplateNode`s.
///
/// This function takes a string input and parses it into a vector of `TemplateNode`s, representing different elements of the template. It uses the `nom` parser combinator library to handle the parsing.
///
/// # Arguments
///
/// * `input`: The input string to be parsed as a template.
///
/// # Returns
///
/// An `IResult` containing the remaining unparsed input (if any) and a vector of `TemplateNode`s, representing the parsed elements of the template.
pub fn parser<'a>(
    input: &'a str,
    start: &'a str,
    close: &'a str,
) -> IResult<&'a str, Vec<TemplateNode>> {
    #[cfg(feature = "debug")]
    trace!("Start Parser: {input} with delimiters: {start} - {close}");
    many0(alt((
        function_parser(start, close),
        raw_string_parser(start, close),
        variable_parser(start, close),
        text_parser(start),
    )))(input)
}
